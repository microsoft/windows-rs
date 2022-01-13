#[cfg(feature = "Win32_Foundation")]
pub trait IComprehensiveSpellCheckProviderImpl: Sized {
    fn ComprehensiveCheck(&mut self, text: super::Foundation::PWSTR) -> ::windows::core::Result<IEnumSpellingError>;
}
#[cfg(feature = "Win32_Foundation")]
impl IComprehensiveSpellCheckProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComprehensiveSpellCheckProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComprehensiveSpellCheckProviderVtbl {
        unsafe extern "system" fn ComprehensiveCheck<Impl: IComprehensiveSpellCheckProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: super::Foundation::PWSTR, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ComprehensiveCheck(::core::mem::transmute_copy(&text)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), ComprehensiveCheck: ComprehensiveCheck::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComprehensiveSpellCheckProvider as ::windows::core::Interface>::IID
    }
}
pub trait IEnumCodePageImpl: Sized {
    fn Clone(&mut self) -> ::windows::core::Result<IEnumCodePage>;
    fn Next(&mut self, celt: u32, rgelt: *mut MIMECPINFO, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
}
impl IEnumCodePageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumCodePageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumCodePageVtbl {
        unsafe extern "system" fn Clone<Impl: IEnumCodePageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumCodePageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut MIMECPINFO, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumCodePageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumCodePageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
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
    fn Clone(&mut self) -> ::windows::core::Result<IEnumRfc1766>;
    fn Next(&mut self, celt: u32, rgelt: *mut RFC1766INFO, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
}
impl IEnumRfc1766Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumRfc1766Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumRfc1766Vtbl {
        unsafe extern "system" fn Clone<Impl: IEnumRfc1766Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumRfc1766Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut RFC1766INFO, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumRfc1766Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumRfc1766Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
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
    fn Clone(&mut self) -> ::windows::core::Result<IEnumScript>;
    fn Next(&mut self, celt: u32, rgelt: *mut SCRIPTINFO, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
}
impl IEnumScriptVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumScriptImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumScriptVtbl {
        unsafe extern "system" fn Clone<Impl: IEnumScriptImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumScriptImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut SCRIPTINFO, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumScriptImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumScriptImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
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
    fn Next(&mut self) -> ::windows::core::Result<ISpellingError>;
}
impl IEnumSpellingErrorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSpellingErrorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumSpellingErrorVtbl {
        unsafe extern "system" fn Next<Impl: IEnumSpellingErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Next: Next::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumSpellingError as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMLangCodePagesImpl: Sized {
    fn GetCharCodePages(&mut self, chsrc: u16) -> ::windows::core::Result<u32>;
    fn GetStrCodePages(&mut self, pszsrc: super::Foundation::PWSTR, cchsrc: i32, dwprioritycodepages: u32, pdwcodepages: *mut u32, pcchcodepages: *mut i32) -> ::windows::core::Result<()>;
    fn CodePageToCodePages(&mut self, ucodepage: u32) -> ::windows::core::Result<u32>;
    fn CodePagesToCodePage(&mut self, dwcodepages: u32, udefaultcodepage: u32) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMLangCodePagesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMLangCodePagesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMLangCodePagesVtbl {
        unsafe extern "system" fn GetCharCodePages<Impl: IMLangCodePagesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, chsrc: u16, pdwcodepages: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCharCodePages(::core::mem::transmute_copy(&chsrc)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcodepages = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStrCodePages<Impl: IMLangCodePagesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsrc: super::Foundation::PWSTR, cchsrc: i32, dwprioritycodepages: u32, pdwcodepages: *mut u32, pcchcodepages: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStrCodePages(::core::mem::transmute_copy(&pszsrc), ::core::mem::transmute_copy(&cchsrc), ::core::mem::transmute_copy(&dwprioritycodepages), ::core::mem::transmute_copy(&pdwcodepages), ::core::mem::transmute_copy(&pcchcodepages)).into()
        }
        unsafe extern "system" fn CodePageToCodePages<Impl: IMLangCodePagesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ucodepage: u32, pdwcodepages: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CodePageToCodePages(::core::mem::transmute_copy(&ucodepage)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcodepages = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CodePagesToCodePage<Impl: IMLangCodePagesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcodepages: u32, udefaultcodepage: u32, pucodepage: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CodePagesToCodePage(::core::mem::transmute_copy(&dwcodepages), ::core::mem::transmute_copy(&udefaultcodepage)) {
                ::core::result::Result::Ok(ok__) => {
                    *pucodepage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn Initialize(&mut self, uisrccodepage: u32, uidstcodepage: u32, dwproperty: u32) -> ::windows::core::Result<()>;
    fn GetSourceCodePage(&mut self) -> ::windows::core::Result<u32>;
    fn GetDestinationCodePage(&mut self) -> ::windows::core::Result<u32>;
    fn GetProperty(&mut self) -> ::windows::core::Result<u32>;
    fn DoConversion(&mut self, psrcstr: *const u8, pcsrcsize: *mut u32, pdststr: *mut u8, pcdstsize: *mut u32) -> ::windows::core::Result<()>;
    fn DoConversionToUnicode(&mut self, psrcstr: super::Foundation::PSTR, pcsrcsize: *mut u32, pdststr: super::Foundation::PWSTR, pcdstsize: *mut u32) -> ::windows::core::Result<()>;
    fn DoConversionFromUnicode(&mut self, psrcstr: super::Foundation::PWSTR, pcsrcsize: *mut u32, pdststr: super::Foundation::PSTR, pcdstsize: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMLangConvertCharsetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMLangConvertCharsetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMLangConvertCharsetVtbl {
        unsafe extern "system" fn Initialize<Impl: IMLangConvertCharsetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uisrccodepage: u32, uidstcodepage: u32, dwproperty: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&uisrccodepage), ::core::mem::transmute_copy(&uidstcodepage), ::core::mem::transmute_copy(&dwproperty)).into()
        }
        unsafe extern "system" fn GetSourceCodePage<Impl: IMLangConvertCharsetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puisrccodepage: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSourceCodePage() {
                ::core::result::Result::Ok(ok__) => {
                    *puisrccodepage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDestinationCodePage<Impl: IMLangConvertCharsetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puidstcodepage: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDestinationCodePage() {
                ::core::result::Result::Ok(ok__) => {
                    *puidstcodepage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Impl: IMLangConvertCharsetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwproperty: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwproperty = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoConversion<Impl: IMLangConvertCharsetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psrcstr: *const u8, pcsrcsize: *mut u32, pdststr: *mut u8, pcdstsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DoConversion(::core::mem::transmute_copy(&psrcstr), ::core::mem::transmute_copy(&pcsrcsize), ::core::mem::transmute_copy(&pdststr), ::core::mem::transmute_copy(&pcdstsize)).into()
        }
        unsafe extern "system" fn DoConversionToUnicode<Impl: IMLangConvertCharsetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psrcstr: super::Foundation::PSTR, pcsrcsize: *mut u32, pdststr: super::Foundation::PWSTR, pcdstsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DoConversionToUnicode(::core::mem::transmute_copy(&psrcstr), ::core::mem::transmute_copy(&pcsrcsize), ::core::mem::transmute_copy(&pdststr), ::core::mem::transmute_copy(&pcdstsize)).into()
        }
        unsafe extern "system" fn DoConversionFromUnicode<Impl: IMLangConvertCharsetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psrcstr: super::Foundation::PWSTR, pcsrcsize: *mut u32, pdststr: super::Foundation::PSTR, pcdstsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DoConversionFromUnicode(::core::mem::transmute_copy(&psrcstr), ::core::mem::transmute_copy(&pcsrcsize), ::core::mem::transmute_copy(&pdststr), ::core::mem::transmute_copy(&pcdstsize)).into()
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
    fn GetFontCodePages(&mut self, hdc: super::Graphics::Gdi::HDC, hfont: super::Graphics::Gdi::HFONT) -> ::windows::core::Result<u32>;
    fn MapFont(&mut self, hdc: super::Graphics::Gdi::HDC, dwcodepages: u32, hsrcfont: super::Graphics::Gdi::HFONT) -> ::windows::core::Result<super::Graphics::Gdi::HFONT>;
    fn ReleaseFont(&mut self, hfont: super::Graphics::Gdi::HFONT) -> ::windows::core::Result<()>;
    fn ResetFontMapping(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IMLangFontLinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMLangFontLinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMLangFontLinkVtbl {
        unsafe extern "system" fn GetFontCodePages<Impl: IMLangFontLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: super::Graphics::Gdi::HDC, hfont: super::Graphics::Gdi::HFONT, pdwcodepages: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontCodePages(::core::mem::transmute_copy(&hdc), ::core::mem::transmute_copy(&hfont)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcodepages = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MapFont<Impl: IMLangFontLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: super::Graphics::Gdi::HDC, dwcodepages: u32, hsrcfont: super::Graphics::Gdi::HFONT, phdestfont: *mut super::Graphics::Gdi::HFONT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MapFont(::core::mem::transmute_copy(&hdc), ::core::mem::transmute_copy(&dwcodepages), ::core::mem::transmute_copy(&hsrcfont)) {
                ::core::result::Result::Ok(ok__) => {
                    *phdestfont = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseFont<Impl: IMLangFontLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hfont: super::Graphics::Gdi::HFONT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseFont(::core::mem::transmute_copy(&hfont)).into()
        }
        unsafe extern "system" fn ResetFontMapping<Impl: IMLangFontLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResetFontMapping().into()
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
    fn GetFontCodePages(&mut self, hdc: super::Graphics::Gdi::HDC, hfont: super::Graphics::Gdi::HFONT) -> ::windows::core::Result<u32>;
    fn ReleaseFont(&mut self, hfont: super::Graphics::Gdi::HFONT) -> ::windows::core::Result<()>;
    fn ResetFontMapping(&mut self) -> ::windows::core::Result<()>;
    fn MapFont(&mut self, hdc: super::Graphics::Gdi::HDC, dwcodepages: u32, chsrc: u16) -> ::windows::core::Result<super::Graphics::Gdi::HFONT>;
    fn GetFontUnicodeRanges(&mut self, hdc: super::Graphics::Gdi::HDC, puiranges: *const u32) -> ::windows::core::Result<UNICODERANGE>;
    fn GetScriptFontInfo(&mut self, sid: u8, dwflags: u32, puifonts: *mut u32, pscriptfont: *mut tagSCRIPFONTINFO) -> ::windows::core::Result<()>;
    fn CodePageToScriptID(&mut self, uicodepage: u32) -> ::windows::core::Result<u8>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IMLangFontLink2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMLangFontLink2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMLangFontLink2Vtbl {
        unsafe extern "system" fn GetFontCodePages<Impl: IMLangFontLink2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: super::Graphics::Gdi::HDC, hfont: super::Graphics::Gdi::HFONT, pdwcodepages: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontCodePages(::core::mem::transmute_copy(&hdc), ::core::mem::transmute_copy(&hfont)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcodepages = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseFont<Impl: IMLangFontLink2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hfont: super::Graphics::Gdi::HFONT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseFont(::core::mem::transmute_copy(&hfont)).into()
        }
        unsafe extern "system" fn ResetFontMapping<Impl: IMLangFontLink2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResetFontMapping().into()
        }
        unsafe extern "system" fn MapFont<Impl: IMLangFontLink2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: super::Graphics::Gdi::HDC, dwcodepages: u32, chsrc: u16, pfont: *mut super::Graphics::Gdi::HFONT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MapFont(::core::mem::transmute_copy(&hdc), ::core::mem::transmute_copy(&dwcodepages), ::core::mem::transmute_copy(&chsrc)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfont = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontUnicodeRanges<Impl: IMLangFontLink2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: super::Graphics::Gdi::HDC, puiranges: *const u32, puranges: *mut UNICODERANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontUnicodeRanges(::core::mem::transmute_copy(&hdc), ::core::mem::transmute_copy(&puiranges)) {
                ::core::result::Result::Ok(ok__) => {
                    *puranges = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetScriptFontInfo<Impl: IMLangFontLink2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sid: u8, dwflags: u32, puifonts: *mut u32, pscriptfont: *mut tagSCRIPFONTINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetScriptFontInfo(::core::mem::transmute_copy(&sid), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&puifonts), ::core::mem::transmute_copy(&pscriptfont)).into()
        }
        unsafe extern "system" fn CodePageToScriptID<Impl: IMLangFontLink2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uicodepage: u32, psid: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CodePageToScriptID(::core::mem::transmute_copy(&uicodepage)) {
                ::core::result::Result::Ok(ok__) => {
                    *psid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn BreakLineML(&mut self, psrcmlstr: ::core::option::Option<IMLangString>, lsrcpos: i32, lsrclen: i32, cmincolumns: i32, cmaxcolumns: i32, pllinelen: *mut i32, plskiplen: *mut i32) -> ::windows::core::Result<()>;
    fn BreakLineW(&mut self, locale: u32, pszsrc: super::Foundation::PWSTR, cchsrc: i32, cmaxcolumns: i32, pcchline: *mut i32, pcchskip: *mut i32) -> ::windows::core::Result<()>;
    fn BreakLineA(&mut self, locale: u32, ucodepage: u32, pszsrc: super::Foundation::PSTR, cchsrc: i32, cmaxcolumns: i32, pcchline: *mut i32, pcchskip: *mut i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMLangLineBreakConsoleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMLangLineBreakConsoleImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMLangLineBreakConsoleVtbl {
        unsafe extern "system" fn BreakLineML<Impl: IMLangLineBreakConsoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psrcmlstr: ::windows::core::RawPtr, lsrcpos: i32, lsrclen: i32, cmincolumns: i32, cmaxcolumns: i32, pllinelen: *mut i32, plskiplen: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BreakLineML(::core::mem::transmute(&psrcmlstr), ::core::mem::transmute_copy(&lsrcpos), ::core::mem::transmute_copy(&lsrclen), ::core::mem::transmute_copy(&cmincolumns), ::core::mem::transmute_copy(&cmaxcolumns), ::core::mem::transmute_copy(&pllinelen), ::core::mem::transmute_copy(&plskiplen)).into()
        }
        unsafe extern "system" fn BreakLineW<Impl: IMLangLineBreakConsoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, locale: u32, pszsrc: super::Foundation::PWSTR, cchsrc: i32, cmaxcolumns: i32, pcchline: *mut i32, pcchskip: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BreakLineW(::core::mem::transmute_copy(&locale), ::core::mem::transmute_copy(&pszsrc), ::core::mem::transmute_copy(&cchsrc), ::core::mem::transmute_copy(&cmaxcolumns), ::core::mem::transmute_copy(&pcchline), ::core::mem::transmute_copy(&pcchskip)).into()
        }
        unsafe extern "system" fn BreakLineA<Impl: IMLangLineBreakConsoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, locale: u32, ucodepage: u32, pszsrc: super::Foundation::PSTR, cchsrc: i32, cmaxcolumns: i32, pcchline: *mut i32, pcchskip: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BreakLineA(::core::mem::transmute_copy(&locale), ::core::mem::transmute_copy(&ucodepage), ::core::mem::transmute_copy(&pszsrc), ::core::mem::transmute_copy(&cchsrc), ::core::mem::transmute_copy(&cmaxcolumns), ::core::mem::transmute_copy(&pcchline), ::core::mem::transmute_copy(&pcchskip)).into()
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
    fn Sync(&mut self, fnoaccess: super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetLength(&mut self) -> ::windows::core::Result<i32>;
    fn SetMLStr(&mut self, ldestpos: i32, ldestlen: i32, psrcmlstr: ::core::option::Option<::windows::core::IUnknown>, lsrcpos: i32, lsrclen: i32) -> ::windows::core::Result<()>;
    fn GetMLStr(&mut self, lsrcpos: i32, lsrclen: i32, punkouter: ::core::option::Option<::windows::core::IUnknown>, dwclscontext: u32, piid: *const ::windows::core::GUID, ppdestmlstr: *mut ::core::option::Option<::windows::core::IUnknown>, pldestpos: *mut i32, pldestlen: *mut i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMLangStringVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMLangStringImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMLangStringVtbl {
        unsafe extern "system" fn Sync<Impl: IMLangStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fnoaccess: super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Sync(::core::mem::transmute_copy(&fnoaccess)).into()
        }
        unsafe extern "system" fn GetLength<Impl: IMLangStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pllen: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLength() {
                ::core::result::Result::Ok(ok__) => {
                    *pllen = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMLStr<Impl: IMLangStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ldestpos: i32, ldestlen: i32, psrcmlstr: *mut ::core::ffi::c_void, lsrcpos: i32, lsrclen: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMLStr(::core::mem::transmute_copy(&ldestpos), ::core::mem::transmute_copy(&ldestlen), ::core::mem::transmute(&psrcmlstr), ::core::mem::transmute_copy(&lsrcpos), ::core::mem::transmute_copy(&lsrclen)).into()
        }
        unsafe extern "system" fn GetMLStr<Impl: IMLangStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsrcpos: i32, lsrclen: i32, punkouter: *mut ::core::ffi::c_void, dwclscontext: u32, piid: *const ::windows::core::GUID, ppdestmlstr: *mut *mut ::core::ffi::c_void, pldestpos: *mut i32, pldestlen: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMLStr(::core::mem::transmute_copy(&lsrcpos), ::core::mem::transmute_copy(&lsrclen), ::core::mem::transmute(&punkouter), ::core::mem::transmute_copy(&dwclscontext), ::core::mem::transmute_copy(&piid), ::core::mem::transmute_copy(&ppdestmlstr), ::core::mem::transmute_copy(&pldestpos), ::core::mem::transmute_copy(&pldestlen)).into()
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
    fn SetAStr(&mut self, ldestpos: i32, ldestlen: i32, ucodepage: u32, pszsrc: super::Foundation::PSTR, cchsrc: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> ::windows::core::Result<()>;
    fn SetStrBufA(&mut self, ldestpos: i32, ldestlen: i32, ucodepage: u32, psrcbuf: ::core::option::Option<IMLangStringBufA>, pcchactual: *mut i32, plactuallen: *mut i32) -> ::windows::core::Result<()>;
    fn GetAStr(&mut self, lsrcpos: i32, lsrclen: i32, ucodepagein: u32, pucodepageout: *mut u32, pszdest: super::Foundation::PSTR, cchdest: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> ::windows::core::Result<()>;
    fn GetStrBufA(&mut self, lsrcpos: i32, lsrcmaxlen: i32, pudestcodepage: *mut u32, ppdestbuf: *mut ::core::option::Option<IMLangStringBufA>, pldestlen: *mut i32) -> ::windows::core::Result<()>;
    fn LockAStr(&mut self, lsrcpos: i32, lsrclen: i32, lflags: i32, ucodepagein: u32, cchrequest: i32, pucodepageout: *mut u32, ppszdest: *mut super::Foundation::PSTR, pcchdest: *mut i32, pldestlen: *mut i32) -> ::windows::core::Result<()>;
    fn UnlockAStr(&mut self, pszsrc: super::Foundation::PSTR, cchsrc: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> ::windows::core::Result<()>;
    fn SetLocale(&mut self, ldestpos: i32, ldestlen: i32, locale: u32) -> ::windows::core::Result<()>;
    fn GetLocale(&mut self, lsrcpos: i32, lsrcmaxlen: i32, plocale: *mut u32, pllocalepos: *mut i32, pllocalelen: *mut i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMLangStringAStrVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMLangStringAStrImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMLangStringAStrVtbl {
        unsafe extern "system" fn SetAStr<Impl: IMLangStringAStrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ldestpos: i32, ldestlen: i32, ucodepage: u32, pszsrc: super::Foundation::PSTR, cchsrc: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAStr(::core::mem::transmute_copy(&ldestpos), ::core::mem::transmute_copy(&ldestlen), ::core::mem::transmute_copy(&ucodepage), ::core::mem::transmute_copy(&pszsrc), ::core::mem::transmute_copy(&cchsrc), ::core::mem::transmute_copy(&pcchactual), ::core::mem::transmute_copy(&plactuallen)).into()
        }
        unsafe extern "system" fn SetStrBufA<Impl: IMLangStringAStrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ldestpos: i32, ldestlen: i32, ucodepage: u32, psrcbuf: ::windows::core::RawPtr, pcchactual: *mut i32, plactuallen: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrBufA(::core::mem::transmute_copy(&ldestpos), ::core::mem::transmute_copy(&ldestlen), ::core::mem::transmute_copy(&ucodepage), ::core::mem::transmute(&psrcbuf), ::core::mem::transmute_copy(&pcchactual), ::core::mem::transmute_copy(&plactuallen)).into()
        }
        unsafe extern "system" fn GetAStr<Impl: IMLangStringAStrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsrcpos: i32, lsrclen: i32, ucodepagein: u32, pucodepageout: *mut u32, pszdest: super::Foundation::PSTR, cchdest: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAStr(::core::mem::transmute_copy(&lsrcpos), ::core::mem::transmute_copy(&lsrclen), ::core::mem::transmute_copy(&ucodepagein), ::core::mem::transmute_copy(&pucodepageout), ::core::mem::transmute_copy(&pszdest), ::core::mem::transmute_copy(&cchdest), ::core::mem::transmute_copy(&pcchactual), ::core::mem::transmute_copy(&plactuallen)).into()
        }
        unsafe extern "system" fn GetStrBufA<Impl: IMLangStringAStrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsrcpos: i32, lsrcmaxlen: i32, pudestcodepage: *mut u32, ppdestbuf: *mut ::windows::core::RawPtr, pldestlen: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStrBufA(::core::mem::transmute_copy(&lsrcpos), ::core::mem::transmute_copy(&lsrcmaxlen), ::core::mem::transmute_copy(&pudestcodepage), ::core::mem::transmute_copy(&ppdestbuf), ::core::mem::transmute_copy(&pldestlen)).into()
        }
        unsafe extern "system" fn LockAStr<Impl: IMLangStringAStrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsrcpos: i32, lsrclen: i32, lflags: i32, ucodepagein: u32, cchrequest: i32, pucodepageout: *mut u32, ppszdest: *mut super::Foundation::PSTR, pcchdest: *mut i32, pldestlen: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LockAStr(::core::mem::transmute_copy(&lsrcpos), ::core::mem::transmute_copy(&lsrclen), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&ucodepagein), ::core::mem::transmute_copy(&cchrequest), ::core::mem::transmute_copy(&pucodepageout), ::core::mem::transmute_copy(&ppszdest), ::core::mem::transmute_copy(&pcchdest), ::core::mem::transmute_copy(&pldestlen)).into()
        }
        unsafe extern "system" fn UnlockAStr<Impl: IMLangStringAStrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsrc: super::Foundation::PSTR, cchsrc: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnlockAStr(::core::mem::transmute_copy(&pszsrc), ::core::mem::transmute_copy(&cchsrc), ::core::mem::transmute_copy(&pcchactual), ::core::mem::transmute_copy(&plactuallen)).into()
        }
        unsafe extern "system" fn SetLocale<Impl: IMLangStringAStrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ldestpos: i32, ldestlen: i32, locale: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLocale(::core::mem::transmute_copy(&ldestpos), ::core::mem::transmute_copy(&ldestlen), ::core::mem::transmute_copy(&locale)).into()
        }
        unsafe extern "system" fn GetLocale<Impl: IMLangStringAStrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsrcpos: i32, lsrcmaxlen: i32, plocale: *mut u32, pllocalepos: *mut i32, pllocalelen: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetLocale(::core::mem::transmute_copy(&lsrcpos), ::core::mem::transmute_copy(&lsrcmaxlen), ::core::mem::transmute_copy(&plocale), ::core::mem::transmute_copy(&pllocalepos), ::core::mem::transmute_copy(&pllocalelen)).into()
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
    fn GetStatus(&mut self, plflags: *mut i32, pcchbuf: *mut i32) -> ::windows::core::Result<()>;
    fn LockBuf(&mut self, cchoffset: i32, cchmaxlock: i32, ppszbuf: *mut *mut super::Foundation::CHAR, pcchbuf: *mut i32) -> ::windows::core::Result<()>;
    fn UnlockBuf(&mut self, pszbuf: super::Foundation::PSTR, cchoffset: i32, cchwrite: i32) -> ::windows::core::Result<()>;
    fn Insert(&mut self, cchoffset: i32, cchmaxinsert: i32) -> ::windows::core::Result<i32>;
    fn Delete(&mut self, cchoffset: i32, cchdelete: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMLangStringBufAVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMLangStringBufAImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMLangStringBufAVtbl {
        unsafe extern "system" fn GetStatus<Impl: IMLangStringBufAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plflags: *mut i32, pcchbuf: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStatus(::core::mem::transmute_copy(&plflags), ::core::mem::transmute_copy(&pcchbuf)).into()
        }
        unsafe extern "system" fn LockBuf<Impl: IMLangStringBufAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchoffset: i32, cchmaxlock: i32, ppszbuf: *mut *mut super::Foundation::CHAR, pcchbuf: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LockBuf(::core::mem::transmute_copy(&cchoffset), ::core::mem::transmute_copy(&cchmaxlock), ::core::mem::transmute_copy(&ppszbuf), ::core::mem::transmute_copy(&pcchbuf)).into()
        }
        unsafe extern "system" fn UnlockBuf<Impl: IMLangStringBufAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszbuf: super::Foundation::PSTR, cchoffset: i32, cchwrite: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnlockBuf(::core::mem::transmute_copy(&pszbuf), ::core::mem::transmute_copy(&cchoffset), ::core::mem::transmute_copy(&cchwrite)).into()
        }
        unsafe extern "system" fn Insert<Impl: IMLangStringBufAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchoffset: i32, cchmaxinsert: i32, pcchactual: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Insert(::core::mem::transmute_copy(&cchoffset), ::core::mem::transmute_copy(&cchmaxinsert)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcchactual = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IMLangStringBufAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchoffset: i32, cchdelete: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete(::core::mem::transmute_copy(&cchoffset), ::core::mem::transmute_copy(&cchdelete)).into()
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
    fn GetStatus(&mut self, plflags: *mut i32, pcchbuf: *mut i32) -> ::windows::core::Result<()>;
    fn LockBuf(&mut self, cchoffset: i32, cchmaxlock: i32, ppszbuf: *mut *mut u16, pcchbuf: *mut i32) -> ::windows::core::Result<()>;
    fn UnlockBuf(&mut self, pszbuf: super::Foundation::PWSTR, cchoffset: i32, cchwrite: i32) -> ::windows::core::Result<()>;
    fn Insert(&mut self, cchoffset: i32, cchmaxinsert: i32) -> ::windows::core::Result<i32>;
    fn Delete(&mut self, cchoffset: i32, cchdelete: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMLangStringBufWVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMLangStringBufWImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMLangStringBufWVtbl {
        unsafe extern "system" fn GetStatus<Impl: IMLangStringBufWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plflags: *mut i32, pcchbuf: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStatus(::core::mem::transmute_copy(&plflags), ::core::mem::transmute_copy(&pcchbuf)).into()
        }
        unsafe extern "system" fn LockBuf<Impl: IMLangStringBufWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchoffset: i32, cchmaxlock: i32, ppszbuf: *mut *mut u16, pcchbuf: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LockBuf(::core::mem::transmute_copy(&cchoffset), ::core::mem::transmute_copy(&cchmaxlock), ::core::mem::transmute_copy(&ppszbuf), ::core::mem::transmute_copy(&pcchbuf)).into()
        }
        unsafe extern "system" fn UnlockBuf<Impl: IMLangStringBufWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszbuf: super::Foundation::PWSTR, cchoffset: i32, cchwrite: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnlockBuf(::core::mem::transmute_copy(&pszbuf), ::core::mem::transmute_copy(&cchoffset), ::core::mem::transmute_copy(&cchwrite)).into()
        }
        unsafe extern "system" fn Insert<Impl: IMLangStringBufWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchoffset: i32, cchmaxinsert: i32, pcchactual: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Insert(::core::mem::transmute_copy(&cchoffset), ::core::mem::transmute_copy(&cchmaxinsert)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcchactual = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IMLangStringBufWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchoffset: i32, cchdelete: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete(::core::mem::transmute_copy(&cchoffset), ::core::mem::transmute_copy(&cchdelete)).into()
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
    fn SetWStr(&mut self, ldestpos: i32, ldestlen: i32, pszsrc: super::Foundation::PWSTR, cchsrc: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> ::windows::core::Result<()>;
    fn SetStrBufW(&mut self, ldestpos: i32, ldestlen: i32, psrcbuf: ::core::option::Option<IMLangStringBufW>, pcchactual: *mut i32, plactuallen: *mut i32) -> ::windows::core::Result<()>;
    fn GetWStr(&mut self, lsrcpos: i32, lsrclen: i32, pszdest: super::Foundation::PWSTR, cchdest: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> ::windows::core::Result<()>;
    fn GetStrBufW(&mut self, lsrcpos: i32, lsrcmaxlen: i32, ppdestbuf: *mut ::core::option::Option<IMLangStringBufW>, pldestlen: *mut i32) -> ::windows::core::Result<()>;
    fn LockWStr(&mut self, lsrcpos: i32, lsrclen: i32, lflags: i32, cchrequest: i32, ppszdest: *mut super::Foundation::PWSTR, pcchdest: *mut i32, pldestlen: *mut i32) -> ::windows::core::Result<()>;
    fn UnlockWStr(&mut self, pszsrc: super::Foundation::PWSTR, cchsrc: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> ::windows::core::Result<()>;
    fn SetLocale(&mut self, ldestpos: i32, ldestlen: i32, locale: u32) -> ::windows::core::Result<()>;
    fn GetLocale(&mut self, lsrcpos: i32, lsrcmaxlen: i32, plocale: *mut u32, pllocalepos: *mut i32, pllocalelen: *mut i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMLangStringWStrVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMLangStringWStrImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMLangStringWStrVtbl {
        unsafe extern "system" fn SetWStr<Impl: IMLangStringWStrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ldestpos: i32, ldestlen: i32, pszsrc: super::Foundation::PWSTR, cchsrc: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWStr(::core::mem::transmute_copy(&ldestpos), ::core::mem::transmute_copy(&ldestlen), ::core::mem::transmute_copy(&pszsrc), ::core::mem::transmute_copy(&cchsrc), ::core::mem::transmute_copy(&pcchactual), ::core::mem::transmute_copy(&plactuallen)).into()
        }
        unsafe extern "system" fn SetStrBufW<Impl: IMLangStringWStrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ldestpos: i32, ldestlen: i32, psrcbuf: ::windows::core::RawPtr, pcchactual: *mut i32, plactuallen: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrBufW(::core::mem::transmute_copy(&ldestpos), ::core::mem::transmute_copy(&ldestlen), ::core::mem::transmute(&psrcbuf), ::core::mem::transmute_copy(&pcchactual), ::core::mem::transmute_copy(&plactuallen)).into()
        }
        unsafe extern "system" fn GetWStr<Impl: IMLangStringWStrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsrcpos: i32, lsrclen: i32, pszdest: super::Foundation::PWSTR, cchdest: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetWStr(::core::mem::transmute_copy(&lsrcpos), ::core::mem::transmute_copy(&lsrclen), ::core::mem::transmute_copy(&pszdest), ::core::mem::transmute_copy(&cchdest), ::core::mem::transmute_copy(&pcchactual), ::core::mem::transmute_copy(&plactuallen)).into()
        }
        unsafe extern "system" fn GetStrBufW<Impl: IMLangStringWStrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsrcpos: i32, lsrcmaxlen: i32, ppdestbuf: *mut ::windows::core::RawPtr, pldestlen: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStrBufW(::core::mem::transmute_copy(&lsrcpos), ::core::mem::transmute_copy(&lsrcmaxlen), ::core::mem::transmute_copy(&ppdestbuf), ::core::mem::transmute_copy(&pldestlen)).into()
        }
        unsafe extern "system" fn LockWStr<Impl: IMLangStringWStrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsrcpos: i32, lsrclen: i32, lflags: i32, cchrequest: i32, ppszdest: *mut super::Foundation::PWSTR, pcchdest: *mut i32, pldestlen: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LockWStr(::core::mem::transmute_copy(&lsrcpos), ::core::mem::transmute_copy(&lsrclen), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&cchrequest), ::core::mem::transmute_copy(&ppszdest), ::core::mem::transmute_copy(&pcchdest), ::core::mem::transmute_copy(&pldestlen)).into()
        }
        unsafe extern "system" fn UnlockWStr<Impl: IMLangStringWStrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsrc: super::Foundation::PWSTR, cchsrc: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnlockWStr(::core::mem::transmute_copy(&pszsrc), ::core::mem::transmute_copy(&cchsrc), ::core::mem::transmute_copy(&pcchactual), ::core::mem::transmute_copy(&plactuallen)).into()
        }
        unsafe extern "system" fn SetLocale<Impl: IMLangStringWStrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ldestpos: i32, ldestlen: i32, locale: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLocale(::core::mem::transmute_copy(&ldestpos), ::core::mem::transmute_copy(&ldestlen), ::core::mem::transmute_copy(&locale)).into()
        }
        unsafe extern "system" fn GetLocale<Impl: IMLangStringWStrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsrcpos: i32, lsrcmaxlen: i32, plocale: *mut u32, pllocalepos: *mut i32, pllocalelen: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetLocale(::core::mem::transmute_copy(&lsrcpos), ::core::mem::transmute_copy(&lsrcmaxlen), ::core::mem::transmute_copy(&plocale), ::core::mem::transmute_copy(&pllocalepos), ::core::mem::transmute_copy(&pllocalelen)).into()
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
    fn GetNumberOfCodePageInfo(&mut self) -> ::windows::core::Result<u32>;
    fn GetCodePageInfo(&mut self, uicodepage: u32) -> ::windows::core::Result<MIMECPINFO>;
    fn GetFamilyCodePage(&mut self, uicodepage: u32) -> ::windows::core::Result<u32>;
    fn EnumCodePages(&mut self, grfflags: u32) -> ::windows::core::Result<IEnumCodePage>;
    fn GetCharsetInfo(&mut self, charset: super::Foundation::BSTR) -> ::windows::core::Result<MIMECSETINFO>;
    fn IsConvertible(&mut self, dwsrcencoding: u32, dwdstencoding: u32) -> ::windows::core::Result<()>;
    fn ConvertString(&mut self, pdwmode: *mut u32, dwsrcencoding: u32, dwdstencoding: u32, psrcstr: *const u8, pcsrcsize: *mut u32, pdststr: *mut u8, pcdstsize: *mut u32) -> ::windows::core::Result<()>;
    fn ConvertStringToUnicode(&mut self, pdwmode: *mut u32, dwencoding: u32, psrcstr: super::Foundation::PSTR, pcsrcsize: *mut u32, pdststr: super::Foundation::PWSTR, pcdstsize: *mut u32) -> ::windows::core::Result<()>;
    fn ConvertStringFromUnicode(&mut self, pdwmode: *mut u32, dwencoding: u32, psrcstr: super::Foundation::PWSTR, pcsrcsize: *mut u32, pdststr: super::Foundation::PSTR, pcdstsize: *mut u32) -> ::windows::core::Result<()>;
    fn ConvertStringReset(&mut self) -> ::windows::core::Result<()>;
    fn GetRfc1766FromLcid(&mut self, locale: u32) -> ::windows::core::Result<super::Foundation::BSTR>;
    fn GetLcidFromRfc1766(&mut self, plocale: *mut u32, bstrrfc1766: super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn EnumRfc1766(&mut self) -> ::windows::core::Result<IEnumRfc1766>;
    fn GetRfc1766Info(&mut self, locale: u32) -> ::windows::core::Result<RFC1766INFO>;
    fn CreateConvertCharset(&mut self, uisrccodepage: u32, uidstcodepage: u32, dwproperty: u32) -> ::windows::core::Result<IMLangConvertCharset>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMultiLanguageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMultiLanguageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMultiLanguageVtbl {
        unsafe extern "system" fn GetNumberOfCodePageInfo<Impl: IMultiLanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccodepage: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNumberOfCodePageInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *pccodepage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCodePageInfo<Impl: IMultiLanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uicodepage: u32, pcodepageinfo: *mut MIMECPINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCodePageInfo(::core::mem::transmute_copy(&uicodepage)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcodepageinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFamilyCodePage<Impl: IMultiLanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uicodepage: u32, puifamilycodepage: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFamilyCodePage(::core::mem::transmute_copy(&uicodepage)) {
                ::core::result::Result::Ok(ok__) => {
                    *puifamilycodepage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumCodePages<Impl: IMultiLanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfflags: u32, ppenumcodepage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumCodePages(::core::mem::transmute_copy(&grfflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumcodepage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCharsetInfo<Impl: IMultiLanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, charset: ::core::mem::ManuallyDrop<super::Foundation::BSTR>, pcharsetinfo: *mut MIMECSETINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCharsetInfo(::core::mem::transmute_copy(&charset)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcharsetinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsConvertible<Impl: IMultiLanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsrcencoding: u32, dwdstencoding: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsConvertible(::core::mem::transmute_copy(&dwsrcencoding), ::core::mem::transmute_copy(&dwdstencoding)).into()
        }
        unsafe extern "system" fn ConvertString<Impl: IMultiLanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmode: *mut u32, dwsrcencoding: u32, dwdstencoding: u32, psrcstr: *const u8, pcsrcsize: *mut u32, pdststr: *mut u8, pcdstsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConvertString(::core::mem::transmute_copy(&pdwmode), ::core::mem::transmute_copy(&dwsrcencoding), ::core::mem::transmute_copy(&dwdstencoding), ::core::mem::transmute_copy(&psrcstr), ::core::mem::transmute_copy(&pcsrcsize), ::core::mem::transmute_copy(&pdststr), ::core::mem::transmute_copy(&pcdstsize)).into()
        }
        unsafe extern "system" fn ConvertStringToUnicode<Impl: IMultiLanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmode: *mut u32, dwencoding: u32, psrcstr: super::Foundation::PSTR, pcsrcsize: *mut u32, pdststr: super::Foundation::PWSTR, pcdstsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConvertStringToUnicode(::core::mem::transmute_copy(&pdwmode), ::core::mem::transmute_copy(&dwencoding), ::core::mem::transmute_copy(&psrcstr), ::core::mem::transmute_copy(&pcsrcsize), ::core::mem::transmute_copy(&pdststr), ::core::mem::transmute_copy(&pcdstsize)).into()
        }
        unsafe extern "system" fn ConvertStringFromUnicode<Impl: IMultiLanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmode: *mut u32, dwencoding: u32, psrcstr: super::Foundation::PWSTR, pcsrcsize: *mut u32, pdststr: super::Foundation::PSTR, pcdstsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConvertStringFromUnicode(::core::mem::transmute_copy(&pdwmode), ::core::mem::transmute_copy(&dwencoding), ::core::mem::transmute_copy(&psrcstr), ::core::mem::transmute_copy(&pcsrcsize), ::core::mem::transmute_copy(&pdststr), ::core::mem::transmute_copy(&pcdstsize)).into()
        }
        unsafe extern "system" fn ConvertStringReset<Impl: IMultiLanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConvertStringReset().into()
        }
        unsafe extern "system" fn GetRfc1766FromLcid<Impl: IMultiLanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, locale: u32, pbstrrfc1766: *mut super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRfc1766FromLcid(::core::mem::transmute_copy(&locale)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrrfc1766 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLcidFromRfc1766<Impl: IMultiLanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plocale: *mut u32, bstrrfc1766: ::core::mem::ManuallyDrop<super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetLcidFromRfc1766(::core::mem::transmute_copy(&plocale), ::core::mem::transmute_copy(&bstrrfc1766)).into()
        }
        unsafe extern "system" fn EnumRfc1766<Impl: IMultiLanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumrfc1766: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumRfc1766() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumrfc1766 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRfc1766Info<Impl: IMultiLanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, locale: u32, prfc1766info: *mut RFC1766INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRfc1766Info(::core::mem::transmute_copy(&locale)) {
                ::core::result::Result::Ok(ok__) => {
                    *prfc1766info = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateConvertCharset<Impl: IMultiLanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uisrccodepage: u32, uidstcodepage: u32, dwproperty: u32, ppmlangconvertcharset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateConvertCharset(::core::mem::transmute_copy(&uisrccodepage), ::core::mem::transmute_copy(&uidstcodepage), ::core::mem::transmute_copy(&dwproperty)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmlangconvertcharset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn GetNumberOfCodePageInfo(&mut self) -> ::windows::core::Result<u32>;
    fn GetCodePageInfo(&mut self, uicodepage: u32, langid: u16) -> ::windows::core::Result<MIMECPINFO>;
    fn GetFamilyCodePage(&mut self, uicodepage: u32) -> ::windows::core::Result<u32>;
    fn EnumCodePages(&mut self, grfflags: u32, langid: u16) -> ::windows::core::Result<IEnumCodePage>;
    fn GetCharsetInfo(&mut self, charset: super::Foundation::BSTR) -> ::windows::core::Result<MIMECSETINFO>;
    fn IsConvertible(&mut self, dwsrcencoding: u32, dwdstencoding: u32) -> ::windows::core::Result<()>;
    fn ConvertString(&mut self, pdwmode: *mut u32, dwsrcencoding: u32, dwdstencoding: u32, psrcstr: *const u8, pcsrcsize: *mut u32, pdststr: *mut u8, pcdstsize: *mut u32) -> ::windows::core::Result<()>;
    fn ConvertStringToUnicode(&mut self, pdwmode: *mut u32, dwencoding: u32, psrcstr: super::Foundation::PSTR, pcsrcsize: *mut u32, pdststr: super::Foundation::PWSTR, pcdstsize: *mut u32) -> ::windows::core::Result<()>;
    fn ConvertStringFromUnicode(&mut self, pdwmode: *mut u32, dwencoding: u32, psrcstr: super::Foundation::PWSTR, pcsrcsize: *mut u32, pdststr: super::Foundation::PSTR, pcdstsize: *mut u32) -> ::windows::core::Result<()>;
    fn ConvertStringReset(&mut self) -> ::windows::core::Result<()>;
    fn GetRfc1766FromLcid(&mut self, locale: u32) -> ::windows::core::Result<super::Foundation::BSTR>;
    fn GetLcidFromRfc1766(&mut self, plocale: *mut u32, bstrrfc1766: super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn EnumRfc1766(&mut self, langid: u16) -> ::windows::core::Result<IEnumRfc1766>;
    fn GetRfc1766Info(&mut self, locale: u32, langid: u16) -> ::windows::core::Result<RFC1766INFO>;
    fn CreateConvertCharset(&mut self, uisrccodepage: u32, uidstcodepage: u32, dwproperty: u32) -> ::windows::core::Result<IMLangConvertCharset>;
    fn ConvertStringInIStream(&mut self, pdwmode: *mut u32, dwflag: u32, lpfallback: super::Foundation::PWSTR, dwsrcencoding: u32, dwdstencoding: u32, pstmin: ::core::option::Option<super::System::Com::IStream>, pstmout: ::core::option::Option<super::System::Com::IStream>) -> ::windows::core::Result<()>;
    fn ConvertStringToUnicodeEx(&mut self, pdwmode: *mut u32, dwencoding: u32, psrcstr: super::Foundation::PSTR, pcsrcsize: *mut u32, pdststr: super::Foundation::PWSTR, pcdstsize: *mut u32, dwflag: u32, lpfallback: super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn ConvertStringFromUnicodeEx(&mut self, pdwmode: *mut u32, dwencoding: u32, psrcstr: super::Foundation::PWSTR, pcsrcsize: *mut u32, pdststr: super::Foundation::PSTR, pcdstsize: *mut u32, dwflag: u32, lpfallback: super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn DetectCodepageInIStream(&mut self, dwflag: u32, dwprefwincodepage: u32, pstmin: ::core::option::Option<super::System::Com::IStream>, lpencoding: *mut DetectEncodingInfo, pnscores: *mut i32) -> ::windows::core::Result<()>;
    fn DetectInputCodepage(&mut self, dwflag: u32, dwprefwincodepage: u32, psrcstr: super::Foundation::PSTR, pcsrcsize: *mut i32, lpencoding: *mut DetectEncodingInfo, pnscores: *mut i32) -> ::windows::core::Result<()>;
    fn ValidateCodePage(&mut self, uicodepage: u32, hwnd: super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn GetCodePageDescription(&mut self, uicodepage: u32, lcid: u32, lpwidecharstr: super::Foundation::PWSTR, cchwidechar: i32) -> ::windows::core::Result<()>;
    fn IsCodePageInstallable(&mut self, uicodepage: u32) -> ::windows::core::Result<()>;
    fn SetMimeDBSource(&mut self, dwsource: MIMECONTF) -> ::windows::core::Result<()>;
    fn GetNumberOfScripts(&mut self) -> ::windows::core::Result<u32>;
    fn EnumScripts(&mut self, dwflags: u32, langid: u16) -> ::windows::core::Result<IEnumScript>;
    fn ValidateCodePageEx(&mut self, uicodepage: u32, hwnd: super::Foundation::HWND, dwfiodcontrol: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IMultiLanguage2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMultiLanguage2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMultiLanguage2Vtbl {
        unsafe extern "system" fn GetNumberOfCodePageInfo<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccodepage: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNumberOfCodePageInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *pccodepage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCodePageInfo<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uicodepage: u32, langid: u16, pcodepageinfo: *mut MIMECPINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCodePageInfo(::core::mem::transmute_copy(&uicodepage), ::core::mem::transmute_copy(&langid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcodepageinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFamilyCodePage<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uicodepage: u32, puifamilycodepage: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFamilyCodePage(::core::mem::transmute_copy(&uicodepage)) {
                ::core::result::Result::Ok(ok__) => {
                    *puifamilycodepage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumCodePages<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfflags: u32, langid: u16, ppenumcodepage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumCodePages(::core::mem::transmute_copy(&grfflags), ::core::mem::transmute_copy(&langid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumcodepage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCharsetInfo<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, charset: ::core::mem::ManuallyDrop<super::Foundation::BSTR>, pcharsetinfo: *mut MIMECSETINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCharsetInfo(::core::mem::transmute_copy(&charset)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcharsetinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsConvertible<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsrcencoding: u32, dwdstencoding: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsConvertible(::core::mem::transmute_copy(&dwsrcencoding), ::core::mem::transmute_copy(&dwdstencoding)).into()
        }
        unsafe extern "system" fn ConvertString<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmode: *mut u32, dwsrcencoding: u32, dwdstencoding: u32, psrcstr: *const u8, pcsrcsize: *mut u32, pdststr: *mut u8, pcdstsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConvertString(::core::mem::transmute_copy(&pdwmode), ::core::mem::transmute_copy(&dwsrcencoding), ::core::mem::transmute_copy(&dwdstencoding), ::core::mem::transmute_copy(&psrcstr), ::core::mem::transmute_copy(&pcsrcsize), ::core::mem::transmute_copy(&pdststr), ::core::mem::transmute_copy(&pcdstsize)).into()
        }
        unsafe extern "system" fn ConvertStringToUnicode<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmode: *mut u32, dwencoding: u32, psrcstr: super::Foundation::PSTR, pcsrcsize: *mut u32, pdststr: super::Foundation::PWSTR, pcdstsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConvertStringToUnicode(::core::mem::transmute_copy(&pdwmode), ::core::mem::transmute_copy(&dwencoding), ::core::mem::transmute_copy(&psrcstr), ::core::mem::transmute_copy(&pcsrcsize), ::core::mem::transmute_copy(&pdststr), ::core::mem::transmute_copy(&pcdstsize)).into()
        }
        unsafe extern "system" fn ConvertStringFromUnicode<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmode: *mut u32, dwencoding: u32, psrcstr: super::Foundation::PWSTR, pcsrcsize: *mut u32, pdststr: super::Foundation::PSTR, pcdstsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConvertStringFromUnicode(::core::mem::transmute_copy(&pdwmode), ::core::mem::transmute_copy(&dwencoding), ::core::mem::transmute_copy(&psrcstr), ::core::mem::transmute_copy(&pcsrcsize), ::core::mem::transmute_copy(&pdststr), ::core::mem::transmute_copy(&pcdstsize)).into()
        }
        unsafe extern "system" fn ConvertStringReset<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConvertStringReset().into()
        }
        unsafe extern "system" fn GetRfc1766FromLcid<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, locale: u32, pbstrrfc1766: *mut super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRfc1766FromLcid(::core::mem::transmute_copy(&locale)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrrfc1766 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLcidFromRfc1766<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plocale: *mut u32, bstrrfc1766: ::core::mem::ManuallyDrop<super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetLcidFromRfc1766(::core::mem::transmute_copy(&plocale), ::core::mem::transmute_copy(&bstrrfc1766)).into()
        }
        unsafe extern "system" fn EnumRfc1766<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: u16, ppenumrfc1766: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumRfc1766(::core::mem::transmute_copy(&langid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumrfc1766 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRfc1766Info<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, locale: u32, langid: u16, prfc1766info: *mut RFC1766INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRfc1766Info(::core::mem::transmute_copy(&locale), ::core::mem::transmute_copy(&langid)) {
                ::core::result::Result::Ok(ok__) => {
                    *prfc1766info = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateConvertCharset<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uisrccodepage: u32, uidstcodepage: u32, dwproperty: u32, ppmlangconvertcharset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateConvertCharset(::core::mem::transmute_copy(&uisrccodepage), ::core::mem::transmute_copy(&uidstcodepage), ::core::mem::transmute_copy(&dwproperty)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmlangconvertcharset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertStringInIStream<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmode: *mut u32, dwflag: u32, lpfallback: super::Foundation::PWSTR, dwsrcencoding: u32, dwdstencoding: u32, pstmin: ::windows::core::RawPtr, pstmout: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConvertStringInIStream(::core::mem::transmute_copy(&pdwmode), ::core::mem::transmute_copy(&dwflag), ::core::mem::transmute_copy(&lpfallback), ::core::mem::transmute_copy(&dwsrcencoding), ::core::mem::transmute_copy(&dwdstencoding), ::core::mem::transmute(&pstmin), ::core::mem::transmute(&pstmout)).into()
        }
        unsafe extern "system" fn ConvertStringToUnicodeEx<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmode: *mut u32, dwencoding: u32, psrcstr: super::Foundation::PSTR, pcsrcsize: *mut u32, pdststr: super::Foundation::PWSTR, pcdstsize: *mut u32, dwflag: u32, lpfallback: super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConvertStringToUnicodeEx(::core::mem::transmute_copy(&pdwmode), ::core::mem::transmute_copy(&dwencoding), ::core::mem::transmute_copy(&psrcstr), ::core::mem::transmute_copy(&pcsrcsize), ::core::mem::transmute_copy(&pdststr), ::core::mem::transmute_copy(&pcdstsize), ::core::mem::transmute_copy(&dwflag), ::core::mem::transmute_copy(&lpfallback)).into()
        }
        unsafe extern "system" fn ConvertStringFromUnicodeEx<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmode: *mut u32, dwencoding: u32, psrcstr: super::Foundation::PWSTR, pcsrcsize: *mut u32, pdststr: super::Foundation::PSTR, pcdstsize: *mut u32, dwflag: u32, lpfallback: super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConvertStringFromUnicodeEx(::core::mem::transmute_copy(&pdwmode), ::core::mem::transmute_copy(&dwencoding), ::core::mem::transmute_copy(&psrcstr), ::core::mem::transmute_copy(&pcsrcsize), ::core::mem::transmute_copy(&pdststr), ::core::mem::transmute_copy(&pcdstsize), ::core::mem::transmute_copy(&dwflag), ::core::mem::transmute_copy(&lpfallback)).into()
        }
        unsafe extern "system" fn DetectCodepageInIStream<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflag: u32, dwprefwincodepage: u32, pstmin: ::windows::core::RawPtr, lpencoding: *mut DetectEncodingInfo, pnscores: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DetectCodepageInIStream(::core::mem::transmute_copy(&dwflag), ::core::mem::transmute_copy(&dwprefwincodepage), ::core::mem::transmute(&pstmin), ::core::mem::transmute_copy(&lpencoding), ::core::mem::transmute_copy(&pnscores)).into()
        }
        unsafe extern "system" fn DetectInputCodepage<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflag: u32, dwprefwincodepage: u32, psrcstr: super::Foundation::PSTR, pcsrcsize: *mut i32, lpencoding: *mut DetectEncodingInfo, pnscores: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DetectInputCodepage(::core::mem::transmute_copy(&dwflag), ::core::mem::transmute_copy(&dwprefwincodepage), ::core::mem::transmute_copy(&psrcstr), ::core::mem::transmute_copy(&pcsrcsize), ::core::mem::transmute_copy(&lpencoding), ::core::mem::transmute_copy(&pnscores)).into()
        }
        unsafe extern "system" fn ValidateCodePage<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uicodepage: u32, hwnd: super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ValidateCodePage(::core::mem::transmute_copy(&uicodepage), ::core::mem::transmute_copy(&hwnd)).into()
        }
        unsafe extern "system" fn GetCodePageDescription<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uicodepage: u32, lcid: u32, lpwidecharstr: super::Foundation::PWSTR, cchwidechar: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCodePageDescription(::core::mem::transmute_copy(&uicodepage), ::core::mem::transmute_copy(&lcid), ::core::mem::transmute_copy(&lpwidecharstr), ::core::mem::transmute_copy(&cchwidechar)).into()
        }
        unsafe extern "system" fn IsCodePageInstallable<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uicodepage: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsCodePageInstallable(::core::mem::transmute_copy(&uicodepage)).into()
        }
        unsafe extern "system" fn SetMimeDBSource<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsource: MIMECONTF) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMimeDBSource(::core::mem::transmute_copy(&dwsource)).into()
        }
        unsafe extern "system" fn GetNumberOfScripts<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnscripts: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNumberOfScripts() {
                ::core::result::Result::Ok(ok__) => {
                    *pnscripts = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumScripts<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, langid: u16, ppenumscript: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumScripts(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&langid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumscript = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValidateCodePageEx<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uicodepage: u32, hwnd: super::Foundation::HWND, dwfiodcontrol: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ValidateCodePageEx(::core::mem::transmute_copy(&uicodepage), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&dwfiodcontrol)).into()
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
    fn DetectOutboundCodePage(&mut self, dwflags: u32, lpwidecharstr: super::Foundation::PWSTR, cchwidechar: u32, puipreferredcodepages: *const u32, npreferredcodepages: u32, puidetectedcodepages: *mut u32, pndetectedcodepages: *mut u32, lpspecialchar: super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn DetectOutboundCodePageInIStream(&mut self, dwflags: u32, pstrin: ::core::option::Option<super::System::Com::IStream>, puipreferredcodepages: *const u32, npreferredcodepages: u32, puidetectedcodepages: *mut u32, pndetectedcodepages: *mut u32, lpspecialchar: super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IMultiLanguage3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMultiLanguage3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMultiLanguage3Vtbl {
        unsafe extern "system" fn DetectOutboundCodePage<Impl: IMultiLanguage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, lpwidecharstr: super::Foundation::PWSTR, cchwidechar: u32, puipreferredcodepages: *const u32, npreferredcodepages: u32, puidetectedcodepages: *mut u32, pndetectedcodepages: *mut u32, lpspecialchar: super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DetectOutboundCodePage(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&lpwidecharstr), ::core::mem::transmute_copy(&cchwidechar), ::core::mem::transmute_copy(&puipreferredcodepages), ::core::mem::transmute_copy(&npreferredcodepages), ::core::mem::transmute_copy(&puidetectedcodepages), ::core::mem::transmute_copy(&pndetectedcodepages), ::core::mem::transmute_copy(&lpspecialchar)).into()
        }
        unsafe extern "system" fn DetectOutboundCodePageInIStream<Impl: IMultiLanguage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pstrin: ::windows::core::RawPtr, puipreferredcodepages: *const u32, npreferredcodepages: u32, puidetectedcodepages: *mut u32, pndetectedcodepages: *mut u32, lpspecialchar: super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DetectOutboundCodePageInIStream(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pstrin), ::core::mem::transmute_copy(&puipreferredcodepages), ::core::mem::transmute_copy(&npreferredcodepages), ::core::mem::transmute_copy(&puidetectedcodepages), ::core::mem::transmute_copy(&pndetectedcodepages), ::core::mem::transmute_copy(&lpspecialchar)).into()
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
    fn Id(&mut self) -> ::windows::core::Result<super::Foundation::PWSTR>;
    fn Heading(&mut self) -> ::windows::core::Result<super::Foundation::PWSTR>;
    fn Description(&mut self) -> ::windows::core::Result<super::Foundation::PWSTR>;
    fn Labels(&mut self) -> ::windows::core::Result<super::System::Com::IEnumString>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IOptionDescriptionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOptionDescriptionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOptionDescriptionVtbl {
        unsafe extern "system" fn Id<Impl: IOptionDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Heading<Impl: IOptionDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Heading() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: IOptionDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Labels<Impl: IOptionDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Labels() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn LanguageTag(&mut self) -> ::windows::core::Result<super::Foundation::PWSTR>;
    fn Check(&mut self, text: super::Foundation::PWSTR) -> ::windows::core::Result<IEnumSpellingError>;
    fn Suggest(&mut self, word: super::Foundation::PWSTR) -> ::windows::core::Result<super::System::Com::IEnumString>;
    fn GetOptionValue(&mut self, optionid: super::Foundation::PWSTR) -> ::windows::core::Result<u8>;
    fn SetOptionValue(&mut self, optionid: super::Foundation::PWSTR, value: u8) -> ::windows::core::Result<()>;
    fn OptionIds(&mut self) -> ::windows::core::Result<super::System::Com::IEnumString>;
    fn Id(&mut self) -> ::windows::core::Result<super::Foundation::PWSTR>;
    fn LocalizedName(&mut self) -> ::windows::core::Result<super::Foundation::PWSTR>;
    fn GetOptionDescription(&mut self, optionid: super::Foundation::PWSTR) -> ::windows::core::Result<IOptionDescription>;
    fn InitializeWordlist(&mut self, wordlisttype: WORDLIST_TYPE, words: ::core::option::Option<super::System::Com::IEnumString>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ISpellCheckProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpellCheckProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpellCheckProviderVtbl {
        unsafe extern "system" fn LanguageTag<Impl: ISpellCheckProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LanguageTag() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Check<Impl: ISpellCheckProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: super::Foundation::PWSTR, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Check(::core::mem::transmute_copy(&text)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Suggest<Impl: ISpellCheckProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, word: super::Foundation::PWSTR, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Suggest(::core::mem::transmute_copy(&word)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOptionValue<Impl: ISpellCheckProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, optionid: super::Foundation::PWSTR, value: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOptionValue(::core::mem::transmute_copy(&optionid)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOptionValue<Impl: ISpellCheckProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, optionid: super::Foundation::PWSTR, value: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOptionValue(::core::mem::transmute_copy(&optionid), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn OptionIds<Impl: ISpellCheckProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OptionIds() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: ISpellCheckProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalizedName<Impl: ISpellCheckProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalizedName() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOptionDescription<Impl: ISpellCheckProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, optionid: super::Foundation::PWSTR, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOptionDescription(::core::mem::transmute_copy(&optionid)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializeWordlist<Impl: ISpellCheckProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wordlisttype: WORDLIST_TYPE, words: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeWordlist(::core::mem::transmute_copy(&wordlisttype), ::core::mem::transmute(&words)).into()
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
    fn SupportedLanguages(&mut self) -> ::windows::core::Result<super::System::Com::IEnumString>;
    fn IsSupported(&mut self, languagetag: super::Foundation::PWSTR) -> ::windows::core::Result<super::Foundation::BOOL>;
    fn CreateSpellCheckProvider(&mut self, languagetag: super::Foundation::PWSTR) -> ::windows::core::Result<ISpellCheckProvider>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ISpellCheckProviderFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpellCheckProviderFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpellCheckProviderFactoryVtbl {
        unsafe extern "system" fn SupportedLanguages<Impl: ISpellCheckProviderFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedLanguages() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSupported<Impl: ISpellCheckProviderFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languagetag: super::Foundation::PWSTR, value: *mut super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSupported(::core::mem::transmute_copy(&languagetag)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSpellCheckProvider<Impl: ISpellCheckProviderFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languagetag: super::Foundation::PWSTR, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSpellCheckProvider(::core::mem::transmute_copy(&languagetag)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn LanguageTag(&mut self) -> ::windows::core::Result<super::Foundation::PWSTR>;
    fn Check(&mut self, text: super::Foundation::PWSTR) -> ::windows::core::Result<IEnumSpellingError>;
    fn Suggest(&mut self, word: super::Foundation::PWSTR) -> ::windows::core::Result<super::System::Com::IEnumString>;
    fn Add(&mut self, word: super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Ignore(&mut self, word: super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn AutoCorrect(&mut self, from: super::Foundation::PWSTR, to: super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetOptionValue(&mut self, optionid: super::Foundation::PWSTR) -> ::windows::core::Result<u8>;
    fn OptionIds(&mut self) -> ::windows::core::Result<super::System::Com::IEnumString>;
    fn Id(&mut self) -> ::windows::core::Result<super::Foundation::PWSTR>;
    fn LocalizedName(&mut self) -> ::windows::core::Result<super::Foundation::PWSTR>;
    fn SpellCheckerChanged(&mut self, handler: ::core::option::Option<ISpellCheckerChangedEventHandler>) -> ::windows::core::Result<u32>;
    fn RemoveSpellCheckerChanged(&mut self, eventcookie: u32) -> ::windows::core::Result<()>;
    fn GetOptionDescription(&mut self, optionid: super::Foundation::PWSTR) -> ::windows::core::Result<IOptionDescription>;
    fn ComprehensiveCheck(&mut self, text: super::Foundation::PWSTR) -> ::windows::core::Result<IEnumSpellingError>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ISpellCheckerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpellCheckerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpellCheckerVtbl {
        unsafe extern "system" fn LanguageTag<Impl: ISpellCheckerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LanguageTag() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Check<Impl: ISpellCheckerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: super::Foundation::PWSTR, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Check(::core::mem::transmute_copy(&text)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Suggest<Impl: ISpellCheckerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, word: super::Foundation::PWSTR, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Suggest(::core::mem::transmute_copy(&word)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: ISpellCheckerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, word: super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute_copy(&word)).into()
        }
        unsafe extern "system" fn Ignore<Impl: ISpellCheckerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, word: super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Ignore(::core::mem::transmute_copy(&word)).into()
        }
        unsafe extern "system" fn AutoCorrect<Impl: ISpellCheckerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, from: super::Foundation::PWSTR, to: super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AutoCorrect(::core::mem::transmute_copy(&from), ::core::mem::transmute_copy(&to)).into()
        }
        unsafe extern "system" fn GetOptionValue<Impl: ISpellCheckerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, optionid: super::Foundation::PWSTR, value: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOptionValue(::core::mem::transmute_copy(&optionid)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OptionIds<Impl: ISpellCheckerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OptionIds() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: ISpellCheckerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalizedName<Impl: ISpellCheckerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalizedName() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpellCheckerChanged<Impl: ISpellCheckerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, eventcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SpellCheckerChanged(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    *eventcookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSpellCheckerChanged<Impl: ISpellCheckerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSpellCheckerChanged(::core::mem::transmute_copy(&eventcookie)).into()
        }
        unsafe extern "system" fn GetOptionDescription<Impl: ISpellCheckerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, optionid: super::Foundation::PWSTR, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOptionDescription(::core::mem::transmute_copy(&optionid)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ComprehensiveCheck<Impl: ISpellCheckerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: super::Foundation::PWSTR, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ComprehensiveCheck(::core::mem::transmute_copy(&text)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn Remove(&mut self, word: super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ISpellChecker2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpellChecker2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpellChecker2Vtbl {
        unsafe extern "system" fn Remove<Impl: ISpellChecker2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, word: super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&word)).into()
        }
        Self { base: ISpellCheckerVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Remove: Remove::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpellChecker2 as ::windows::core::Interface>::IID
    }
}
pub trait ISpellCheckerChangedEventHandlerImpl: Sized {
    fn Invoke(&mut self, sender: ::core::option::Option<ISpellChecker>) -> ::windows::core::Result<()>;
}
impl ISpellCheckerChangedEventHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpellCheckerChangedEventHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpellCheckerChangedEventHandlerVtbl {
        unsafe extern "system" fn Invoke<Impl: ISpellCheckerChangedEventHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Invoke(::core::mem::transmute(&sender)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Invoke: Invoke::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpellCheckerChangedEventHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ISpellCheckerFactoryImpl: Sized {
    fn SupportedLanguages(&mut self) -> ::windows::core::Result<super::System::Com::IEnumString>;
    fn IsSupported(&mut self, languagetag: super::Foundation::PWSTR) -> ::windows::core::Result<super::Foundation::BOOL>;
    fn CreateSpellChecker(&mut self, languagetag: super::Foundation::PWSTR) -> ::windows::core::Result<ISpellChecker>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ISpellCheckerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpellCheckerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpellCheckerFactoryVtbl {
        unsafe extern "system" fn SupportedLanguages<Impl: ISpellCheckerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedLanguages() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSupported<Impl: ISpellCheckerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languagetag: super::Foundation::PWSTR, value: *mut super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSupported(::core::mem::transmute_copy(&languagetag)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSpellChecker<Impl: ISpellCheckerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languagetag: super::Foundation::PWSTR, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSpellChecker(::core::mem::transmute_copy(&languagetag)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn StartIndex(&mut self) -> ::windows::core::Result<u32>;
    fn Length(&mut self) -> ::windows::core::Result<u32>;
    fn CorrectiveAction(&mut self) -> ::windows::core::Result<CORRECTIVE_ACTION>;
    fn Replacement(&mut self) -> ::windows::core::Result<super::Foundation::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISpellingErrorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpellingErrorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpellingErrorVtbl {
        unsafe extern "system" fn StartIndex<Impl: ISpellingErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Length<Impl: ISpellingErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Length() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CorrectiveAction<Impl: ISpellingErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut CORRECTIVE_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CorrectiveAction() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Replacement<Impl: ISpellingErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Replacement() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn RegisterUserDictionary(&mut self, dictionarypath: super::Foundation::PWSTR, languagetag: super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn UnregisterUserDictionary(&mut self, dictionarypath: super::Foundation::PWSTR, languagetag: super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IUserDictionariesRegistrarVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserDictionariesRegistrarImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserDictionariesRegistrarVtbl {
        unsafe extern "system" fn RegisterUserDictionary<Impl: IUserDictionariesRegistrarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dictionarypath: super::Foundation::PWSTR, languagetag: super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterUserDictionary(::core::mem::transmute_copy(&dictionarypath), ::core::mem::transmute_copy(&languagetag)).into()
        }
        unsafe extern "system" fn UnregisterUserDictionary<Impl: IUserDictionariesRegistrarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dictionarypath: super::Foundation::PWSTR, languagetag: super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterUserDictionary(::core::mem::transmute_copy(&dictionarypath), ::core::mem::transmute_copy(&languagetag)).into()
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
