pub trait IComprehensiveSpellCheckProviderImpl: Sized {
    fn ComprehensiveCheck();
}
impl ::windows::core::RuntimeName for IComprehensiveSpellCheckProvider {
    const NAME: &'static str = "Windows.Win32.Globalization.IComprehensiveSpellCheckProvider";
}
impl IComprehensiveSpellCheckProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComprehensiveSpellCheckProviderImpl, const OFFSET: isize>() -> IComprehensiveSpellCheckProviderVtbl {
        unsafe extern "system" fn ComprehensiveCheck<Impl: IComprehensiveSpellCheckProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: super::Foundation::PWSTR, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ComprehensiveCheck(&*(&text as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IComprehensiveSpellCheckProvider>, ::windows::core::GetTrustLevel, ComprehensiveCheck::<Impl, OFFSET>)
    }
}
pub trait IEnumCodePageImpl: Sized {
    fn Clone();
    fn Next();
    fn Reset();
    fn Skip();
}
impl ::windows::core::RuntimeName for IEnumCodePage {
    const NAME: &'static str = "Windows.Win32.Globalization.IEnumCodePage";
}
impl IEnumCodePageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumCodePageImpl, const OFFSET: isize>() -> IEnumCodePageVtbl {
        unsafe extern "system" fn Clone<Impl: IEnumCodePageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumCodePageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut MIMECPINFO, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumCodePageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumCodePageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumCodePage>, ::windows::core::GetTrustLevel, Clone::<Impl, OFFSET>, Next::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Skip::<Impl, OFFSET>)
    }
}
pub trait IEnumRfc1766Impl: Sized {
    fn Clone();
    fn Next();
    fn Reset();
    fn Skip();
}
impl ::windows::core::RuntimeName for IEnumRfc1766 {
    const NAME: &'static str = "Windows.Win32.Globalization.IEnumRfc1766";
}
impl IEnumRfc1766Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumRfc1766Impl, const OFFSET: isize>() -> IEnumRfc1766Vtbl {
        unsafe extern "system" fn Clone<Impl: IEnumRfc1766Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumRfc1766Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut RFC1766INFO, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumRfc1766Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumRfc1766Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumRfc1766>, ::windows::core::GetTrustLevel, Clone::<Impl, OFFSET>, Next::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Skip::<Impl, OFFSET>)
    }
}
pub trait IEnumScriptImpl: Sized {
    fn Clone();
    fn Next();
    fn Reset();
    fn Skip();
}
impl ::windows::core::RuntimeName for IEnumScript {
    const NAME: &'static str = "Windows.Win32.Globalization.IEnumScript";
}
impl IEnumScriptVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumScriptImpl, const OFFSET: isize>() -> IEnumScriptVtbl {
        unsafe extern "system" fn Clone<Impl: IEnumScriptImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumScriptImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut SCRIPTINFO, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumScriptImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumScriptImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumScript>, ::windows::core::GetTrustLevel, Clone::<Impl, OFFSET>, Next::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Skip::<Impl, OFFSET>)
    }
}
pub trait IEnumSpellingErrorImpl: Sized {
    fn Next();
}
impl ::windows::core::RuntimeName for IEnumSpellingError {
    const NAME: &'static str = "Windows.Win32.Globalization.IEnumSpellingError";
}
impl IEnumSpellingErrorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSpellingErrorImpl, const OFFSET: isize>() -> IEnumSpellingErrorVtbl {
        unsafe extern "system" fn Next<Impl: IEnumSpellingErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumSpellingError>, ::windows::core::GetTrustLevel, Next::<Impl, OFFSET>)
    }
}
pub trait IMLangCodePagesImpl: Sized {
    fn GetCharCodePages();
    fn GetStrCodePages();
    fn CodePageToCodePages();
    fn CodePagesToCodePage();
}
impl ::windows::core::RuntimeName for IMLangCodePages {
    const NAME: &'static str = "Windows.Win32.Globalization.IMLangCodePages";
}
impl IMLangCodePagesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMLangCodePagesImpl, const OFFSET: isize>() -> IMLangCodePagesVtbl {
        unsafe extern "system" fn GetCharCodePages<Impl: IMLangCodePagesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, chsrc: u16, pdwcodepages: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCharCodePages(chsrc, ::core::mem::transmute_copy(&pdwcodepages)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStrCodePages<Impl: IMLangCodePagesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsrc: super::Foundation::PWSTR, cchsrc: i32, dwprioritycodepages: u32, pdwcodepages: *mut u32, pcchcodepages: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStrCodePages(&*(&pszsrc as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cchsrc, dwprioritycodepages, ::core::mem::transmute_copy(&pdwcodepages), ::core::mem::transmute_copy(&pcchcodepages)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CodePageToCodePages<Impl: IMLangCodePagesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ucodepage: u32, pdwcodepages: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CodePageToCodePages(ucodepage, ::core::mem::transmute_copy(&pdwcodepages)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CodePagesToCodePage<Impl: IMLangCodePagesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcodepages: u32, udefaultcodepage: u32, pucodepage: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CodePagesToCodePage(dwcodepages, udefaultcodepage, ::core::mem::transmute_copy(&pucodepage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMLangCodePages>, ::windows::core::GetTrustLevel, GetCharCodePages::<Impl, OFFSET>, GetStrCodePages::<Impl, OFFSET>, CodePageToCodePages::<Impl, OFFSET>, CodePagesToCodePage::<Impl, OFFSET>)
    }
}
pub trait IMLangConvertCharsetImpl: Sized {
    fn Initialize();
    fn GetSourceCodePage();
    fn GetDestinationCodePage();
    fn GetProperty();
    fn DoConversion();
    fn DoConversionToUnicode();
    fn DoConversionFromUnicode();
}
impl ::windows::core::RuntimeName for IMLangConvertCharset {
    const NAME: &'static str = "Windows.Win32.Globalization.IMLangConvertCharset";
}
impl IMLangConvertCharsetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMLangConvertCharsetImpl, const OFFSET: isize>() -> IMLangConvertCharsetVtbl {
        unsafe extern "system" fn Initialize<Impl: IMLangConvertCharsetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uisrccodepage: u32, uidstcodepage: u32, dwproperty: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(uisrccodepage, uidstcodepage, dwproperty) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceCodePage<Impl: IMLangConvertCharsetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puisrccodepage: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSourceCodePage(::core::mem::transmute_copy(&puisrccodepage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDestinationCodePage<Impl: IMLangConvertCharsetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puidstcodepage: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDestinationCodePage(::core::mem::transmute_copy(&puidstcodepage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Impl: IMLangConvertCharsetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwproperty: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(::core::mem::transmute_copy(&pdwproperty)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoConversion<Impl: IMLangConvertCharsetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psrcstr: *const u8, pcsrcsize: *mut u32, pdststr: *mut u8, pcdstsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DoConversion(psrcstr, pcsrcsize, ::core::mem::transmute_copy(&pdststr), pcdstsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoConversionToUnicode<Impl: IMLangConvertCharsetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psrcstr: super::Foundation::PSTR, pcsrcsize: *mut u32, pdststr: super::Foundation::PWSTR, pcdstsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DoConversionToUnicode(&*(&psrcstr as *const <super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), pcsrcsize, ::core::mem::transmute_copy(&pdststr), pcdstsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoConversionFromUnicode<Impl: IMLangConvertCharsetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psrcstr: super::Foundation::PWSTR, pcsrcsize: *mut u32, pdststr: super::Foundation::PSTR, pcdstsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DoConversionFromUnicode(&*(&psrcstr as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), pcsrcsize, ::core::mem::transmute_copy(&pdststr), pcdstsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMLangConvertCharset>,
            ::windows::core::GetTrustLevel,
            Initialize::<Impl, OFFSET>,
            GetSourceCodePage::<Impl, OFFSET>,
            GetDestinationCodePage::<Impl, OFFSET>,
            GetProperty::<Impl, OFFSET>,
            DoConversion::<Impl, OFFSET>,
            DoConversionToUnicode::<Impl, OFFSET>,
            DoConversionFromUnicode::<Impl, OFFSET>,
        )
    }
}
pub trait IMLangFontLinkImpl: Sized + IMLangCodePagesImpl {
    fn GetFontCodePages();
    fn MapFont();
    fn ReleaseFont();
    fn ResetFontMapping();
}
impl ::windows::core::RuntimeName for IMLangFontLink {
    const NAME: &'static str = "Windows.Win32.Globalization.IMLangFontLink";
}
impl IMLangFontLinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMLangFontLinkImpl, const OFFSET: isize>() -> IMLangFontLinkVtbl {
        unsafe extern "system" fn GetFontCodePages<Impl: IMLangFontLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: super::Graphics::Gdi::HDC, hfont: super::Graphics::Gdi::HFONT, pdwcodepages: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontCodePages(&*(&hdc as *const <super::Graphics::Gdi::HDC as ::windows::core::Abi>::Abi as *const <super::Graphics::Gdi::HDC as ::windows::core::DefaultType>::DefaultType), &*(&hfont as *const <super::Graphics::Gdi::HFONT as ::windows::core::Abi>::Abi as *const <super::Graphics::Gdi::HFONT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwcodepages)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MapFont<Impl: IMLangFontLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: super::Graphics::Gdi::HDC, dwcodepages: u32, hsrcfont: super::Graphics::Gdi::HFONT, phdestfont: *mut super::Graphics::Gdi::HFONT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MapFont(&*(&hdc as *const <super::Graphics::Gdi::HDC as ::windows::core::Abi>::Abi as *const <super::Graphics::Gdi::HDC as ::windows::core::DefaultType>::DefaultType), dwcodepages, &*(&hsrcfont as *const <super::Graphics::Gdi::HFONT as ::windows::core::Abi>::Abi as *const <super::Graphics::Gdi::HFONT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&phdestfont)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseFont<Impl: IMLangFontLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hfont: super::Graphics::Gdi::HFONT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReleaseFont(&*(&hfont as *const <super::Graphics::Gdi::HFONT as ::windows::core::Abi>::Abi as *const <super::Graphics::Gdi::HFONT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResetFontMapping<Impl: IMLangFontLinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResetFontMapping() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMLangFontLink>, ::windows::core::GetTrustLevel, GetFontCodePages::<Impl, OFFSET>, MapFont::<Impl, OFFSET>, ReleaseFont::<Impl, OFFSET>, ResetFontMapping::<Impl, OFFSET>)
    }
}
pub trait IMLangFontLink2Impl: Sized + IMLangCodePagesImpl {
    fn GetFontCodePages();
    fn ReleaseFont();
    fn ResetFontMapping();
    fn MapFont();
    fn GetFontUnicodeRanges();
    fn GetScriptFontInfo();
    fn CodePageToScriptID();
}
impl ::windows::core::RuntimeName for IMLangFontLink2 {
    const NAME: &'static str = "Windows.Win32.Globalization.IMLangFontLink2";
}
impl IMLangFontLink2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMLangFontLink2Impl, const OFFSET: isize>() -> IMLangFontLink2Vtbl {
        unsafe extern "system" fn GetFontCodePages<Impl: IMLangFontLink2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: super::Graphics::Gdi::HDC, hfont: super::Graphics::Gdi::HFONT, pdwcodepages: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontCodePages(&*(&hdc as *const <super::Graphics::Gdi::HDC as ::windows::core::Abi>::Abi as *const <super::Graphics::Gdi::HDC as ::windows::core::DefaultType>::DefaultType), &*(&hfont as *const <super::Graphics::Gdi::HFONT as ::windows::core::Abi>::Abi as *const <super::Graphics::Gdi::HFONT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwcodepages)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseFont<Impl: IMLangFontLink2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hfont: super::Graphics::Gdi::HFONT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReleaseFont(&*(&hfont as *const <super::Graphics::Gdi::HFONT as ::windows::core::Abi>::Abi as *const <super::Graphics::Gdi::HFONT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResetFontMapping<Impl: IMLangFontLink2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResetFontMapping() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MapFont<Impl: IMLangFontLink2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: super::Graphics::Gdi::HDC, dwcodepages: u32, chsrc: u16, pfont: *mut super::Graphics::Gdi::HFONT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MapFont(&*(&hdc as *const <super::Graphics::Gdi::HDC as ::windows::core::Abi>::Abi as *const <super::Graphics::Gdi::HDC as ::windows::core::DefaultType>::DefaultType), dwcodepages, chsrc, ::core::mem::transmute_copy(&pfont)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontUnicodeRanges<Impl: IMLangFontLink2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: super::Graphics::Gdi::HDC, puiranges: *const u32, puranges: *mut UNICODERANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontUnicodeRanges(&*(&hdc as *const <super::Graphics::Gdi::HDC as ::windows::core::Abi>::Abi as *const <super::Graphics::Gdi::HDC as ::windows::core::DefaultType>::DefaultType), puiranges, ::core::mem::transmute_copy(&puranges)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetScriptFontInfo<Impl: IMLangFontLink2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sid: u8, dwflags: u32, puifonts: *mut u32, pscriptfont: *mut tagSCRIPFONTINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetScriptFontInfo(sid, dwflags, puifonts, ::core::mem::transmute_copy(&pscriptfont)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CodePageToScriptID<Impl: IMLangFontLink2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uicodepage: u32, psid: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CodePageToScriptID(uicodepage, ::core::mem::transmute_copy(&psid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMLangFontLink2>,
            ::windows::core::GetTrustLevel,
            GetFontCodePages::<Impl, OFFSET>,
            ReleaseFont::<Impl, OFFSET>,
            ResetFontMapping::<Impl, OFFSET>,
            MapFont::<Impl, OFFSET>,
            GetFontUnicodeRanges::<Impl, OFFSET>,
            GetScriptFontInfo::<Impl, OFFSET>,
            CodePageToScriptID::<Impl, OFFSET>,
        )
    }
}
pub trait IMLangLineBreakConsoleImpl: Sized {
    fn BreakLineML();
    fn BreakLineW();
    fn BreakLineA();
}
impl ::windows::core::RuntimeName for IMLangLineBreakConsole {
    const NAME: &'static str = "Windows.Win32.Globalization.IMLangLineBreakConsole";
}
impl IMLangLineBreakConsoleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMLangLineBreakConsoleImpl, const OFFSET: isize>() -> IMLangLineBreakConsoleVtbl {
        unsafe extern "system" fn BreakLineML<Impl: IMLangLineBreakConsoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psrcmlstr: ::windows::core::RawPtr, lsrcpos: i32, lsrclen: i32, cmincolumns: i32, cmaxcolumns: i32, pllinelen: *mut i32, plskiplen: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BreakLineML(&*(&psrcmlstr as *const <IMLangString as ::windows::core::Abi>::Abi as *const <IMLangString as ::windows::core::DefaultType>::DefaultType), lsrcpos, lsrclen, cmincolumns, cmaxcolumns, ::core::mem::transmute_copy(&pllinelen), ::core::mem::transmute_copy(&plskiplen)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BreakLineW<Impl: IMLangLineBreakConsoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, locale: u32, pszsrc: super::Foundation::PWSTR, cchsrc: i32, cmaxcolumns: i32, pcchline: *mut i32, pcchskip: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BreakLineW(locale, &*(&pszsrc as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cchsrc, cmaxcolumns, ::core::mem::transmute_copy(&pcchline), ::core::mem::transmute_copy(&pcchskip)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BreakLineA<Impl: IMLangLineBreakConsoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, locale: u32, ucodepage: u32, pszsrc: super::Foundation::PSTR, cchsrc: i32, cmaxcolumns: i32, pcchline: *mut i32, pcchskip: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BreakLineA(locale, ucodepage, &*(&pszsrc as *const <super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), cchsrc, cmaxcolumns, ::core::mem::transmute_copy(&pcchline), ::core::mem::transmute_copy(&pcchskip)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMLangLineBreakConsole>, ::windows::core::GetTrustLevel, BreakLineML::<Impl, OFFSET>, BreakLineW::<Impl, OFFSET>, BreakLineA::<Impl, OFFSET>)
    }
}
pub trait IMLangStringImpl: Sized {
    fn Sync();
    fn GetLength();
    fn SetMLStr();
    fn GetMLStr();
}
impl ::windows::core::RuntimeName for IMLangString {
    const NAME: &'static str = "Windows.Win32.Globalization.IMLangString";
}
impl IMLangStringVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMLangStringImpl, const OFFSET: isize>() -> IMLangStringVtbl {
        unsafe extern "system" fn Sync<Impl: IMLangStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fnoaccess: super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Sync(&*(&fnoaccess as *const <super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLength<Impl: IMLangStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pllen: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLength(::core::mem::transmute_copy(&pllen)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMLStr<Impl: IMLangStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ldestpos: i32, ldestlen: i32, psrcmlstr: *mut ::core::ffi::c_void, lsrcpos: i32, lsrclen: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMLStr(ldestpos, ldestlen, &*(&psrcmlstr as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), lsrcpos, lsrclen) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMLStr<Impl: IMLangStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsrcpos: i32, lsrclen: i32, punkouter: *mut ::core::ffi::c_void, dwclscontext: u32, piid: *const ::windows::core::GUID, ppdestmlstr: *mut *mut ::core::ffi::c_void, pldestpos: *mut i32, pldestlen: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMLStr(lsrcpos, lsrclen, &*(&punkouter as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), dwclscontext, &*(&piid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppdestmlstr), ::core::mem::transmute_copy(&pldestpos), ::core::mem::transmute_copy(&pldestlen)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMLangString>, ::windows::core::GetTrustLevel, Sync::<Impl, OFFSET>, GetLength::<Impl, OFFSET>, SetMLStr::<Impl, OFFSET>, GetMLStr::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IMLangStringAStr {
    const NAME: &'static str = "Windows.Win32.Globalization.IMLangStringAStr";
}
impl IMLangStringAStrVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMLangStringAStrImpl, const OFFSET: isize>() -> IMLangStringAStrVtbl {
        unsafe extern "system" fn SetAStr<Impl: IMLangStringAStrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ldestpos: i32, ldestlen: i32, ucodepage: u32, pszsrc: super::Foundation::PSTR, cchsrc: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAStr(ldestpos, ldestlen, ucodepage, &*(&pszsrc as *const <super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), cchsrc, ::core::mem::transmute_copy(&pcchactual), ::core::mem::transmute_copy(&plactuallen)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrBufA<Impl: IMLangStringAStrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ldestpos: i32, ldestlen: i32, ucodepage: u32, psrcbuf: ::windows::core::RawPtr, pcchactual: *mut i32, plactuallen: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetStrBufA(ldestpos, ldestlen, ucodepage, &*(&psrcbuf as *const <IMLangStringBufA as ::windows::core::Abi>::Abi as *const <IMLangStringBufA as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pcchactual), ::core::mem::transmute_copy(&plactuallen)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAStr<Impl: IMLangStringAStrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsrcpos: i32, lsrclen: i32, ucodepagein: u32, pucodepageout: *mut u32, pszdest: super::Foundation::PSTR, cchdest: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAStr(lsrcpos, lsrclen, ucodepagein, pucodepageout, ::core::mem::transmute_copy(&pszdest), cchdest, ::core::mem::transmute_copy(&pcchactual), ::core::mem::transmute_copy(&plactuallen)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStrBufA<Impl: IMLangStringAStrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsrcpos: i32, lsrcmaxlen: i32, pudestcodepage: *mut u32, ppdestbuf: *mut ::windows::core::RawPtr, pldestlen: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStrBufA(lsrcpos, lsrcmaxlen, ::core::mem::transmute_copy(&pudestcodepage), ::core::mem::transmute_copy(&ppdestbuf), ::core::mem::transmute_copy(&pldestlen)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LockAStr<Impl: IMLangStringAStrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsrcpos: i32, lsrclen: i32, lflags: i32, ucodepagein: u32, cchrequest: i32, pucodepageout: *mut u32, ppszdest: *mut super::Foundation::PSTR, pcchdest: *mut i32, pldestlen: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LockAStr(lsrcpos, lsrclen, lflags, ucodepagein, cchrequest, ::core::mem::transmute_copy(&pucodepageout), ::core::mem::transmute_copy(&ppszdest), ::core::mem::transmute_copy(&pcchdest), ::core::mem::transmute_copy(&pldestlen)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnlockAStr<Impl: IMLangStringAStrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsrc: super::Foundation::PSTR, cchsrc: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnlockAStr(&*(&pszsrc as *const <super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), cchsrc, ::core::mem::transmute_copy(&pcchactual), ::core::mem::transmute_copy(&plactuallen)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocale<Impl: IMLangStringAStrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ldestpos: i32, ldestlen: i32, locale: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLocale(ldestpos, ldestlen, locale) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocale<Impl: IMLangStringAStrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsrcpos: i32, lsrcmaxlen: i32, plocale: *mut u32, pllocalepos: *mut i32, pllocalelen: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLocale(lsrcpos, lsrcmaxlen, ::core::mem::transmute_copy(&plocale), ::core::mem::transmute_copy(&pllocalepos), ::core::mem::transmute_copy(&pllocalelen)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMLangStringAStr>, ::windows::core::GetTrustLevel, SetAStr::<Impl, OFFSET>, SetStrBufA::<Impl, OFFSET>, GetAStr::<Impl, OFFSET>, GetStrBufA::<Impl, OFFSET>, LockAStr::<Impl, OFFSET>, UnlockAStr::<Impl, OFFSET>, SetLocale::<Impl, OFFSET>, GetLocale::<Impl, OFFSET>)
    }
}
pub trait IMLangStringBufAImpl: Sized {
    fn GetStatus();
    fn LockBuf();
    fn UnlockBuf();
    fn Insert();
    fn Delete();
}
impl ::windows::core::RuntimeName for IMLangStringBufA {
    const NAME: &'static str = "Windows.Win32.Globalization.IMLangStringBufA";
}
impl IMLangStringBufAVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMLangStringBufAImpl, const OFFSET: isize>() -> IMLangStringBufAVtbl {
        unsafe extern "system" fn GetStatus<Impl: IMLangStringBufAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plflags: *mut i32, pcchbuf: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus(::core::mem::transmute_copy(&plflags), ::core::mem::transmute_copy(&pcchbuf)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LockBuf<Impl: IMLangStringBufAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchoffset: i32, cchmaxlock: i32, ppszbuf: *mut *mut super::Foundation::CHAR, pcchbuf: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LockBuf(cchoffset, cchmaxlock, ::core::mem::transmute_copy(&ppszbuf), ::core::mem::transmute_copy(&pcchbuf)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnlockBuf<Impl: IMLangStringBufAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszbuf: super::Foundation::PSTR, cchoffset: i32, cchwrite: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnlockBuf(&*(&pszbuf as *const <super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), cchoffset, cchwrite) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Insert<Impl: IMLangStringBufAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchoffset: i32, cchmaxinsert: i32, pcchactual: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Insert(cchoffset, cchmaxinsert, ::core::mem::transmute_copy(&pcchactual)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IMLangStringBufAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchoffset: i32, cchdelete: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Delete(cchoffset, cchdelete) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMLangStringBufA>, ::windows::core::GetTrustLevel, GetStatus::<Impl, OFFSET>, LockBuf::<Impl, OFFSET>, UnlockBuf::<Impl, OFFSET>, Insert::<Impl, OFFSET>, Delete::<Impl, OFFSET>)
    }
}
pub trait IMLangStringBufWImpl: Sized {
    fn GetStatus();
    fn LockBuf();
    fn UnlockBuf();
    fn Insert();
    fn Delete();
}
impl ::windows::core::RuntimeName for IMLangStringBufW {
    const NAME: &'static str = "Windows.Win32.Globalization.IMLangStringBufW";
}
impl IMLangStringBufWVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMLangStringBufWImpl, const OFFSET: isize>() -> IMLangStringBufWVtbl {
        unsafe extern "system" fn GetStatus<Impl: IMLangStringBufWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plflags: *mut i32, pcchbuf: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus(::core::mem::transmute_copy(&plflags), ::core::mem::transmute_copy(&pcchbuf)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LockBuf<Impl: IMLangStringBufWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchoffset: i32, cchmaxlock: i32, ppszbuf: *mut *mut u16, pcchbuf: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LockBuf(cchoffset, cchmaxlock, ::core::mem::transmute_copy(&ppszbuf), ::core::mem::transmute_copy(&pcchbuf)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnlockBuf<Impl: IMLangStringBufWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszbuf: super::Foundation::PWSTR, cchoffset: i32, cchwrite: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnlockBuf(&*(&pszbuf as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cchoffset, cchwrite) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Insert<Impl: IMLangStringBufWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchoffset: i32, cchmaxinsert: i32, pcchactual: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Insert(cchoffset, cchmaxinsert, ::core::mem::transmute_copy(&pcchactual)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IMLangStringBufWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchoffset: i32, cchdelete: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Delete(cchoffset, cchdelete) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMLangStringBufW>, ::windows::core::GetTrustLevel, GetStatus::<Impl, OFFSET>, LockBuf::<Impl, OFFSET>, UnlockBuf::<Impl, OFFSET>, Insert::<Impl, OFFSET>, Delete::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IMLangStringWStr {
    const NAME: &'static str = "Windows.Win32.Globalization.IMLangStringWStr";
}
impl IMLangStringWStrVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMLangStringWStrImpl, const OFFSET: isize>() -> IMLangStringWStrVtbl {
        unsafe extern "system" fn SetWStr<Impl: IMLangStringWStrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ldestpos: i32, ldestlen: i32, pszsrc: super::Foundation::PWSTR, cchsrc: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetWStr(ldestpos, ldestlen, &*(&pszsrc as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cchsrc, ::core::mem::transmute_copy(&pcchactual), ::core::mem::transmute_copy(&plactuallen)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrBufW<Impl: IMLangStringWStrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ldestpos: i32, ldestlen: i32, psrcbuf: ::windows::core::RawPtr, pcchactual: *mut i32, plactuallen: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetStrBufW(ldestpos, ldestlen, &*(&psrcbuf as *const <IMLangStringBufW as ::windows::core::Abi>::Abi as *const <IMLangStringBufW as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pcchactual), ::core::mem::transmute_copy(&plactuallen)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWStr<Impl: IMLangStringWStrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsrcpos: i32, lsrclen: i32, pszdest: super::Foundation::PWSTR, cchdest: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWStr(lsrcpos, lsrclen, ::core::mem::transmute_copy(&pszdest), cchdest, ::core::mem::transmute_copy(&pcchactual), ::core::mem::transmute_copy(&plactuallen)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStrBufW<Impl: IMLangStringWStrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsrcpos: i32, lsrcmaxlen: i32, ppdestbuf: *mut ::windows::core::RawPtr, pldestlen: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStrBufW(lsrcpos, lsrcmaxlen, ::core::mem::transmute_copy(&ppdestbuf), ::core::mem::transmute_copy(&pldestlen)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LockWStr<Impl: IMLangStringWStrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsrcpos: i32, lsrclen: i32, lflags: i32, cchrequest: i32, ppszdest: *mut super::Foundation::PWSTR, pcchdest: *mut i32, pldestlen: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LockWStr(lsrcpos, lsrclen, lflags, cchrequest, ::core::mem::transmute_copy(&ppszdest), ::core::mem::transmute_copy(&pcchdest), ::core::mem::transmute_copy(&pldestlen)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnlockWStr<Impl: IMLangStringWStrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsrc: super::Foundation::PWSTR, cchsrc: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnlockWStr(&*(&pszsrc as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cchsrc, ::core::mem::transmute_copy(&pcchactual), ::core::mem::transmute_copy(&plactuallen)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocale<Impl: IMLangStringWStrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ldestpos: i32, ldestlen: i32, locale: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLocale(ldestpos, ldestlen, locale) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocale<Impl: IMLangStringWStrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsrcpos: i32, lsrcmaxlen: i32, plocale: *mut u32, pllocalepos: *mut i32, pllocalelen: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLocale(lsrcpos, lsrcmaxlen, ::core::mem::transmute_copy(&plocale), ::core::mem::transmute_copy(&pllocalepos), ::core::mem::transmute_copy(&pllocalelen)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMLangStringWStr>, ::windows::core::GetTrustLevel, SetWStr::<Impl, OFFSET>, SetStrBufW::<Impl, OFFSET>, GetWStr::<Impl, OFFSET>, GetStrBufW::<Impl, OFFSET>, LockWStr::<Impl, OFFSET>, UnlockWStr::<Impl, OFFSET>, SetLocale::<Impl, OFFSET>, GetLocale::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IMultiLanguage {
    const NAME: &'static str = "Windows.Win32.Globalization.IMultiLanguage";
}
impl IMultiLanguageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMultiLanguageImpl, const OFFSET: isize>() -> IMultiLanguageVtbl {
        unsafe extern "system" fn GetNumberOfCodePageInfo<Impl: IMultiLanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccodepage: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNumberOfCodePageInfo(::core::mem::transmute_copy(&pccodepage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCodePageInfo<Impl: IMultiLanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uicodepage: u32, pcodepageinfo: *mut MIMECPINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCodePageInfo(uicodepage, ::core::mem::transmute_copy(&pcodepageinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFamilyCodePage<Impl: IMultiLanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uicodepage: u32, puifamilycodepage: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFamilyCodePage(uicodepage, ::core::mem::transmute_copy(&puifamilycodepage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumCodePages<Impl: IMultiLanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfflags: u32, ppenumcodepage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumCodePages(grfflags, ::core::mem::transmute_copy(&ppenumcodepage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCharsetInfo<Impl: IMultiLanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, charset: ::core::mem::ManuallyDrop<super::Foundation::BSTR>, pcharsetinfo: *mut MIMECSETINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCharsetInfo(&*(&charset as *const <super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pcharsetinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsConvertible<Impl: IMultiLanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsrcencoding: u32, dwdstencoding: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsConvertible(dwsrcencoding, dwdstencoding) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertString<Impl: IMultiLanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmode: *mut u32, dwsrcencoding: u32, dwdstencoding: u32, psrcstr: *const u8, pcsrcsize: *mut u32, pdststr: *mut u8, pcdstsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConvertString(pdwmode, dwsrcencoding, dwdstencoding, psrcstr, pcsrcsize, ::core::mem::transmute_copy(&pdststr), pcdstsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertStringToUnicode<Impl: IMultiLanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmode: *mut u32, dwencoding: u32, psrcstr: super::Foundation::PSTR, pcsrcsize: *mut u32, pdststr: super::Foundation::PWSTR, pcdstsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConvertStringToUnicode(pdwmode, dwencoding, &*(&psrcstr as *const <super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), pcsrcsize, ::core::mem::transmute_copy(&pdststr), pcdstsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertStringFromUnicode<Impl: IMultiLanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmode: *mut u32, dwencoding: u32, psrcstr: super::Foundation::PWSTR, pcsrcsize: *mut u32, pdststr: super::Foundation::PSTR, pcdstsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConvertStringFromUnicode(pdwmode, dwencoding, &*(&psrcstr as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), pcsrcsize, ::core::mem::transmute_copy(&pdststr), pcdstsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertStringReset<Impl: IMultiLanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConvertStringReset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRfc1766FromLcid<Impl: IMultiLanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, locale: u32, pbstrrfc1766: *mut super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRfc1766FromLcid(locale, ::core::mem::transmute_copy(&pbstrrfc1766)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLcidFromRfc1766<Impl: IMultiLanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plocale: *mut u32, bstrrfc1766: ::core::mem::ManuallyDrop<super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLcidFromRfc1766(::core::mem::transmute_copy(&plocale), &*(&bstrrfc1766 as *const <super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumRfc1766<Impl: IMultiLanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumrfc1766: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumRfc1766(::core::mem::transmute_copy(&ppenumrfc1766)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRfc1766Info<Impl: IMultiLanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, locale: u32, prfc1766info: *mut RFC1766INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRfc1766Info(locale, ::core::mem::transmute_copy(&prfc1766info)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateConvertCharset<Impl: IMultiLanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uisrccodepage: u32, uidstcodepage: u32, dwproperty: u32, ppmlangconvertcharset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateConvertCharset(uisrccodepage, uidstcodepage, dwproperty, ::core::mem::transmute_copy(&ppmlangconvertcharset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMultiLanguage>,
            ::windows::core::GetTrustLevel,
            GetNumberOfCodePageInfo::<Impl, OFFSET>,
            GetCodePageInfo::<Impl, OFFSET>,
            GetFamilyCodePage::<Impl, OFFSET>,
            EnumCodePages::<Impl, OFFSET>,
            GetCharsetInfo::<Impl, OFFSET>,
            IsConvertible::<Impl, OFFSET>,
            ConvertString::<Impl, OFFSET>,
            ConvertStringToUnicode::<Impl, OFFSET>,
            ConvertStringFromUnicode::<Impl, OFFSET>,
            ConvertStringReset::<Impl, OFFSET>,
            GetRfc1766FromLcid::<Impl, OFFSET>,
            GetLcidFromRfc1766::<Impl, OFFSET>,
            EnumRfc1766::<Impl, OFFSET>,
            GetRfc1766Info::<Impl, OFFSET>,
            CreateConvertCharset::<Impl, OFFSET>,
        )
    }
}
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
impl ::windows::core::RuntimeName for IMultiLanguage2 {
    const NAME: &'static str = "Windows.Win32.Globalization.IMultiLanguage2";
}
impl IMultiLanguage2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMultiLanguage2Impl, const OFFSET: isize>() -> IMultiLanguage2Vtbl {
        unsafe extern "system" fn GetNumberOfCodePageInfo<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccodepage: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNumberOfCodePageInfo(::core::mem::transmute_copy(&pccodepage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCodePageInfo<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uicodepage: u32, langid: u16, pcodepageinfo: *mut MIMECPINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCodePageInfo(uicodepage, langid, ::core::mem::transmute_copy(&pcodepageinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFamilyCodePage<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uicodepage: u32, puifamilycodepage: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFamilyCodePage(uicodepage, ::core::mem::transmute_copy(&puifamilycodepage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumCodePages<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfflags: u32, langid: u16, ppenumcodepage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumCodePages(grfflags, langid, ::core::mem::transmute_copy(&ppenumcodepage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCharsetInfo<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, charset: ::core::mem::ManuallyDrop<super::Foundation::BSTR>, pcharsetinfo: *mut MIMECSETINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCharsetInfo(&*(&charset as *const <super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pcharsetinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsConvertible<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsrcencoding: u32, dwdstencoding: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsConvertible(dwsrcencoding, dwdstencoding) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertString<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmode: *mut u32, dwsrcencoding: u32, dwdstencoding: u32, psrcstr: *const u8, pcsrcsize: *mut u32, pdststr: *mut u8, pcdstsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConvertString(pdwmode, dwsrcencoding, dwdstencoding, psrcstr, pcsrcsize, ::core::mem::transmute_copy(&pdststr), pcdstsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertStringToUnicode<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmode: *mut u32, dwencoding: u32, psrcstr: super::Foundation::PSTR, pcsrcsize: *mut u32, pdststr: super::Foundation::PWSTR, pcdstsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConvertStringToUnicode(pdwmode, dwencoding, &*(&psrcstr as *const <super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), pcsrcsize, ::core::mem::transmute_copy(&pdststr), pcdstsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertStringFromUnicode<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmode: *mut u32, dwencoding: u32, psrcstr: super::Foundation::PWSTR, pcsrcsize: *mut u32, pdststr: super::Foundation::PSTR, pcdstsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConvertStringFromUnicode(pdwmode, dwencoding, &*(&psrcstr as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), pcsrcsize, ::core::mem::transmute_copy(&pdststr), pcdstsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertStringReset<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConvertStringReset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRfc1766FromLcid<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, locale: u32, pbstrrfc1766: *mut super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRfc1766FromLcid(locale, ::core::mem::transmute_copy(&pbstrrfc1766)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLcidFromRfc1766<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plocale: *mut u32, bstrrfc1766: ::core::mem::ManuallyDrop<super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLcidFromRfc1766(::core::mem::transmute_copy(&plocale), &*(&bstrrfc1766 as *const <super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumRfc1766<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: u16, ppenumrfc1766: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumRfc1766(langid, ::core::mem::transmute_copy(&ppenumrfc1766)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRfc1766Info<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, locale: u32, langid: u16, prfc1766info: *mut RFC1766INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRfc1766Info(locale, langid, ::core::mem::transmute_copy(&prfc1766info)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateConvertCharset<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uisrccodepage: u32, uidstcodepage: u32, dwproperty: u32, ppmlangconvertcharset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateConvertCharset(uisrccodepage, uidstcodepage, dwproperty, ::core::mem::transmute_copy(&ppmlangconvertcharset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertStringInIStream<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmode: *mut u32, dwflag: u32, lpfallback: super::Foundation::PWSTR, dwsrcencoding: u32, dwdstencoding: u32, pstmin: ::windows::core::RawPtr, pstmout: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConvertStringInIStream(
                pdwmode,
                dwflag,
                &*(&lpfallback as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                dwsrcencoding,
                dwdstencoding,
                &*(&pstmin as *const <super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
                &*(&pstmout as *const <super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertStringToUnicodeEx<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmode: *mut u32, dwencoding: u32, psrcstr: super::Foundation::PSTR, pcsrcsize: *mut u32, pdststr: super::Foundation::PWSTR, pcdstsize: *mut u32, dwflag: u32, lpfallback: super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConvertStringToUnicodeEx(pdwmode, dwencoding, &*(&psrcstr as *const <super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), pcsrcsize, ::core::mem::transmute_copy(&pdststr), pcdstsize, dwflag, &*(&lpfallback as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertStringFromUnicodeEx<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmode: *mut u32, dwencoding: u32, psrcstr: super::Foundation::PWSTR, pcsrcsize: *mut u32, pdststr: super::Foundation::PSTR, pcdstsize: *mut u32, dwflag: u32, lpfallback: super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConvertStringFromUnicodeEx(pdwmode, dwencoding, &*(&psrcstr as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), pcsrcsize, ::core::mem::transmute_copy(&pdststr), pcdstsize, dwflag, &*(&lpfallback as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DetectCodepageInIStream<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflag: u32, dwprefwincodepage: u32, pstmin: ::windows::core::RawPtr, lpencoding: *mut DetectEncodingInfo, pnscores: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DetectCodepageInIStream(dwflag, dwprefwincodepage, &*(&pstmin as *const <super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&lpencoding), pnscores) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DetectInputCodepage<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflag: u32, dwprefwincodepage: u32, psrcstr: super::Foundation::PSTR, pcsrcsize: *mut i32, lpencoding: *mut DetectEncodingInfo, pnscores: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DetectInputCodepage(dwflag, dwprefwincodepage, &*(&psrcstr as *const <super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), pcsrcsize, ::core::mem::transmute_copy(&lpencoding), pnscores) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValidateCodePage<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uicodepage: u32, hwnd: super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ValidateCodePage(uicodepage, &*(&hwnd as *const <super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCodePageDescription<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uicodepage: u32, lcid: u32, lpwidecharstr: super::Foundation::PWSTR, cchwidechar: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCodePageDescription(uicodepage, lcid, ::core::mem::transmute_copy(&lpwidecharstr), cchwidechar) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCodePageInstallable<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uicodepage: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCodePageInstallable(uicodepage) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMimeDBSource<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsource: MIMECONTF) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMimeDBSource(dwsource) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumberOfScripts<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnscripts: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNumberOfScripts(::core::mem::transmute_copy(&pnscripts)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumScripts<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, langid: u16, ppenumscript: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumScripts(dwflags, langid, ::core::mem::transmute_copy(&ppenumscript)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValidateCodePageEx<Impl: IMultiLanguage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uicodepage: u32, hwnd: super::Foundation::HWND, dwfiodcontrol: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ValidateCodePageEx(uicodepage, &*(&hwnd as *const <super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), dwfiodcontrol) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMultiLanguage2>,
            ::windows::core::GetTrustLevel,
            GetNumberOfCodePageInfo::<Impl, OFFSET>,
            GetCodePageInfo::<Impl, OFFSET>,
            GetFamilyCodePage::<Impl, OFFSET>,
            EnumCodePages::<Impl, OFFSET>,
            GetCharsetInfo::<Impl, OFFSET>,
            IsConvertible::<Impl, OFFSET>,
            ConvertString::<Impl, OFFSET>,
            ConvertStringToUnicode::<Impl, OFFSET>,
            ConvertStringFromUnicode::<Impl, OFFSET>,
            ConvertStringReset::<Impl, OFFSET>,
            GetRfc1766FromLcid::<Impl, OFFSET>,
            GetLcidFromRfc1766::<Impl, OFFSET>,
            EnumRfc1766::<Impl, OFFSET>,
            GetRfc1766Info::<Impl, OFFSET>,
            CreateConvertCharset::<Impl, OFFSET>,
            ConvertStringInIStream::<Impl, OFFSET>,
            ConvertStringToUnicodeEx::<Impl, OFFSET>,
            ConvertStringFromUnicodeEx::<Impl, OFFSET>,
            DetectCodepageInIStream::<Impl, OFFSET>,
            DetectInputCodepage::<Impl, OFFSET>,
            ValidateCodePage::<Impl, OFFSET>,
            GetCodePageDescription::<Impl, OFFSET>,
            IsCodePageInstallable::<Impl, OFFSET>,
            SetMimeDBSource::<Impl, OFFSET>,
            GetNumberOfScripts::<Impl, OFFSET>,
            EnumScripts::<Impl, OFFSET>,
            ValidateCodePageEx::<Impl, OFFSET>,
        )
    }
}
pub trait IMultiLanguage3Impl: Sized + IMultiLanguage2Impl {
    fn DetectOutboundCodePage();
    fn DetectOutboundCodePageInIStream();
}
impl ::windows::core::RuntimeName for IMultiLanguage3 {
    const NAME: &'static str = "Windows.Win32.Globalization.IMultiLanguage3";
}
impl IMultiLanguage3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMultiLanguage3Impl, const OFFSET: isize>() -> IMultiLanguage3Vtbl {
        unsafe extern "system" fn DetectOutboundCodePage<Impl: IMultiLanguage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, lpwidecharstr: super::Foundation::PWSTR, cchwidechar: u32, puipreferredcodepages: *const u32, npreferredcodepages: u32, puidetectedcodepages: *mut u32, pndetectedcodepages: *mut u32, lpspecialchar: super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DetectOutboundCodePage(dwflags, &*(&lpwidecharstr as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cchwidechar, puipreferredcodepages, npreferredcodepages, ::core::mem::transmute_copy(&puidetectedcodepages), pndetectedcodepages, &*(&lpspecialchar as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DetectOutboundCodePageInIStream<Impl: IMultiLanguage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pstrin: ::windows::core::RawPtr, puipreferredcodepages: *const u32, npreferredcodepages: u32, puidetectedcodepages: *mut u32, pndetectedcodepages: *mut u32, lpspecialchar: super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DetectOutboundCodePageInIStream(dwflags, &*(&pstrin as *const <super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), puipreferredcodepages, npreferredcodepages, ::core::mem::transmute_copy(&puidetectedcodepages), pndetectedcodepages, &*(&lpspecialchar as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMultiLanguage3>, ::windows::core::GetTrustLevel, DetectOutboundCodePage::<Impl, OFFSET>, DetectOutboundCodePageInIStream::<Impl, OFFSET>)
    }
}
pub trait IOptionDescriptionImpl: Sized {
    fn Id();
    fn Heading();
    fn Description();
    fn Labels();
}
impl ::windows::core::RuntimeName for IOptionDescription {
    const NAME: &'static str = "Windows.Win32.Globalization.IOptionDescription";
}
impl IOptionDescriptionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOptionDescriptionImpl, const OFFSET: isize>() -> IOptionDescriptionVtbl {
        unsafe extern "system" fn Id<Impl: IOptionDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Heading<Impl: IOptionDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Heading(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: IOptionDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Labels<Impl: IOptionDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Labels(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOptionDescription>, ::windows::core::GetTrustLevel, Id::<Impl, OFFSET>, Heading::<Impl, OFFSET>, Description::<Impl, OFFSET>, Labels::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for ISpellCheckProvider {
    const NAME: &'static str = "Windows.Win32.Globalization.ISpellCheckProvider";
}
impl ISpellCheckProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpellCheckProviderImpl, const OFFSET: isize>() -> ISpellCheckProviderVtbl {
        unsafe extern "system" fn LanguageTag<Impl: ISpellCheckProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LanguageTag(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Check<Impl: ISpellCheckProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: super::Foundation::PWSTR, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Check(&*(&text as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Suggest<Impl: ISpellCheckProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, word: super::Foundation::PWSTR, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Suggest(&*(&word as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOptionValue<Impl: ISpellCheckProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, optionid: super::Foundation::PWSTR, value: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOptionValue(&*(&optionid as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOptionValue<Impl: ISpellCheckProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, optionid: super::Foundation::PWSTR, value: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetOptionValue(&*(&optionid as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OptionIds<Impl: ISpellCheckProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OptionIds(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: ISpellCheckProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalizedName<Impl: ISpellCheckProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalizedName(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOptionDescription<Impl: ISpellCheckProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, optionid: super::Foundation::PWSTR, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOptionDescription(&*(&optionid as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializeWordlist<Impl: ISpellCheckProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wordlisttype: WORDLIST_TYPE, words: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitializeWordlist(wordlisttype, &*(&words as *const <super::System::Com::IEnumString as ::windows::core::Abi>::Abi as *const <super::System::Com::IEnumString as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ISpellCheckProvider>,
            ::windows::core::GetTrustLevel,
            LanguageTag::<Impl, OFFSET>,
            Check::<Impl, OFFSET>,
            Suggest::<Impl, OFFSET>,
            GetOptionValue::<Impl, OFFSET>,
            SetOptionValue::<Impl, OFFSET>,
            OptionIds::<Impl, OFFSET>,
            Id::<Impl, OFFSET>,
            LocalizedName::<Impl, OFFSET>,
            GetOptionDescription::<Impl, OFFSET>,
            InitializeWordlist::<Impl, OFFSET>,
        )
    }
}
pub trait ISpellCheckProviderFactoryImpl: Sized {
    fn SupportedLanguages();
    fn IsSupported();
    fn CreateSpellCheckProvider();
}
impl ::windows::core::RuntimeName for ISpellCheckProviderFactory {
    const NAME: &'static str = "Windows.Win32.Globalization.ISpellCheckProviderFactory";
}
impl ISpellCheckProviderFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpellCheckProviderFactoryImpl, const OFFSET: isize>() -> ISpellCheckProviderFactoryVtbl {
        unsafe extern "system" fn SupportedLanguages<Impl: ISpellCheckProviderFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedLanguages(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSupported<Impl: ISpellCheckProviderFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languagetag: super::Foundation::PWSTR, value: *mut super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSupported(&*(&languagetag as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSpellCheckProvider<Impl: ISpellCheckProviderFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languagetag: super::Foundation::PWSTR, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSpellCheckProvider(&*(&languagetag as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpellCheckProviderFactory>, ::windows::core::GetTrustLevel, SupportedLanguages::<Impl, OFFSET>, IsSupported::<Impl, OFFSET>, CreateSpellCheckProvider::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for ISpellChecker {
    const NAME: &'static str = "Windows.Win32.Globalization.ISpellChecker";
}
impl ISpellCheckerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpellCheckerImpl, const OFFSET: isize>() -> ISpellCheckerVtbl {
        unsafe extern "system" fn LanguageTag<Impl: ISpellCheckerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LanguageTag(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Check<Impl: ISpellCheckerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: super::Foundation::PWSTR, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Check(&*(&text as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Suggest<Impl: ISpellCheckerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, word: super::Foundation::PWSTR, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Suggest(&*(&word as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: ISpellCheckerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, word: super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Add(&*(&word as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ignore<Impl: ISpellCheckerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, word: super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ignore(&*(&word as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoCorrect<Impl: ISpellCheckerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, from: super::Foundation::PWSTR, to: super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoCorrect(&*(&from as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&to as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOptionValue<Impl: ISpellCheckerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, optionid: super::Foundation::PWSTR, value: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOptionValue(&*(&optionid as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OptionIds<Impl: ISpellCheckerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OptionIds(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: ISpellCheckerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalizedName<Impl: ISpellCheckerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalizedName(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpellCheckerChanged<Impl: ISpellCheckerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, eventcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SpellCheckerChanged(&*(&handler as *const <ISpellCheckerChangedEventHandler as ::windows::core::Abi>::Abi as *const <ISpellCheckerChangedEventHandler as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&eventcookie)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSpellCheckerChanged<Impl: ISpellCheckerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveSpellCheckerChanged(eventcookie) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOptionDescription<Impl: ISpellCheckerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, optionid: super::Foundation::PWSTR, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOptionDescription(&*(&optionid as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ComprehensiveCheck<Impl: ISpellCheckerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: super::Foundation::PWSTR, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ComprehensiveCheck(&*(&text as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ISpellChecker>,
            ::windows::core::GetTrustLevel,
            LanguageTag::<Impl, OFFSET>,
            Check::<Impl, OFFSET>,
            Suggest::<Impl, OFFSET>,
            Add::<Impl, OFFSET>,
            Ignore::<Impl, OFFSET>,
            AutoCorrect::<Impl, OFFSET>,
            GetOptionValue::<Impl, OFFSET>,
            OptionIds::<Impl, OFFSET>,
            Id::<Impl, OFFSET>,
            LocalizedName::<Impl, OFFSET>,
            SpellCheckerChanged::<Impl, OFFSET>,
            RemoveSpellCheckerChanged::<Impl, OFFSET>,
            GetOptionDescription::<Impl, OFFSET>,
            ComprehensiveCheck::<Impl, OFFSET>,
        )
    }
}
pub trait ISpellChecker2Impl: Sized + ISpellCheckerImpl {
    fn Remove();
}
impl ::windows::core::RuntimeName for ISpellChecker2 {
    const NAME: &'static str = "Windows.Win32.Globalization.ISpellChecker2";
}
impl ISpellChecker2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpellChecker2Impl, const OFFSET: isize>() -> ISpellChecker2Vtbl {
        unsafe extern "system" fn Remove<Impl: ISpellChecker2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, word: super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Remove(&*(&word as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpellChecker2>, ::windows::core::GetTrustLevel, Remove::<Impl, OFFSET>)
    }
}
pub trait ISpellCheckerChangedEventHandlerImpl: Sized {
    fn Invoke();
}
impl ::windows::core::RuntimeName for ISpellCheckerChangedEventHandler {
    const NAME: &'static str = "Windows.Win32.Globalization.ISpellCheckerChangedEventHandler";
}
impl ISpellCheckerChangedEventHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpellCheckerChangedEventHandlerImpl, const OFFSET: isize>() -> ISpellCheckerChangedEventHandlerVtbl {
        unsafe extern "system" fn Invoke<Impl: ISpellCheckerChangedEventHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Invoke(&*(&sender as *const <ISpellChecker as ::windows::core::Abi>::Abi as *const <ISpellChecker as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpellCheckerChangedEventHandler>, ::windows::core::GetTrustLevel, Invoke::<Impl, OFFSET>)
    }
}
pub trait ISpellCheckerFactoryImpl: Sized {
    fn SupportedLanguages();
    fn IsSupported();
    fn CreateSpellChecker();
}
impl ::windows::core::RuntimeName for ISpellCheckerFactory {
    const NAME: &'static str = "Windows.Win32.Globalization.ISpellCheckerFactory";
}
impl ISpellCheckerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpellCheckerFactoryImpl, const OFFSET: isize>() -> ISpellCheckerFactoryVtbl {
        unsafe extern "system" fn SupportedLanguages<Impl: ISpellCheckerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedLanguages(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSupported<Impl: ISpellCheckerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languagetag: super::Foundation::PWSTR, value: *mut super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSupported(&*(&languagetag as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSpellChecker<Impl: ISpellCheckerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languagetag: super::Foundation::PWSTR, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSpellChecker(&*(&languagetag as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpellCheckerFactory>, ::windows::core::GetTrustLevel, SupportedLanguages::<Impl, OFFSET>, IsSupported::<Impl, OFFSET>, CreateSpellChecker::<Impl, OFFSET>)
    }
}
pub trait ISpellingErrorImpl: Sized {
    fn StartIndex();
    fn Length();
    fn CorrectiveAction();
    fn Replacement();
}
impl ::windows::core::RuntimeName for ISpellingError {
    const NAME: &'static str = "Windows.Win32.Globalization.ISpellingError";
}
impl ISpellingErrorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpellingErrorImpl, const OFFSET: isize>() -> ISpellingErrorVtbl {
        unsafe extern "system" fn StartIndex<Impl: ISpellingErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartIndex(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Length<Impl: ISpellingErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Length(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CorrectiveAction<Impl: ISpellingErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut CORRECTIVE_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CorrectiveAction(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Replacement<Impl: ISpellingErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Replacement(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpellingError>, ::windows::core::GetTrustLevel, StartIndex::<Impl, OFFSET>, Length::<Impl, OFFSET>, CorrectiveAction::<Impl, OFFSET>, Replacement::<Impl, OFFSET>)
    }
}
pub trait IUserDictionariesRegistrarImpl: Sized {
    fn RegisterUserDictionary();
    fn UnregisterUserDictionary();
}
impl ::windows::core::RuntimeName for IUserDictionariesRegistrar {
    const NAME: &'static str = "Windows.Win32.Globalization.IUserDictionariesRegistrar";
}
impl IUserDictionariesRegistrarVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserDictionariesRegistrarImpl, const OFFSET: isize>() -> IUserDictionariesRegistrarVtbl {
        unsafe extern "system" fn RegisterUserDictionary<Impl: IUserDictionariesRegistrarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dictionarypath: super::Foundation::PWSTR, languagetag: super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterUserDictionary(&*(&dictionarypath as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&languagetag as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterUserDictionary<Impl: IUserDictionariesRegistrarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dictionarypath: super::Foundation::PWSTR, languagetag: super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnregisterUserDictionary(&*(&dictionarypath as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&languagetag as *const <super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUserDictionariesRegistrar>, ::windows::core::GetTrustLevel, RegisterUserDictionary::<Impl, OFFSET>, UnregisterUserDictionary::<Impl, OFFSET>)
    }
}
