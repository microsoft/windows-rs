#[cfg(all(feature = "UI_Text", feature = "implement_exclusive"))]
pub trait ILanguageFontImpl: Sized {
    fn FontFamily(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FontWeight(&self) -> ::windows::core::Result<super::super::UI::Text::FontWeight>;
    fn FontStretch(&self) -> ::windows::core::Result<super::super::UI::Text::FontStretch>;
    fn FontStyle(&self) -> ::windows::core::Result<super::super::UI::Text::FontStyle>;
    fn ScaleFactor(&self) -> ::windows::core::Result<f64>;
}
#[cfg(all(feature = "UI_Text", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILanguageFont {
    const NAME: &'static str = "Windows.Globalization.Fonts.ILanguageFont";
}
#[cfg(all(feature = "UI_Text", feature = "implement_exclusive"))]
impl ILanguageFontVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILanguageFontImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILanguageFontVtbl {
        unsafe extern "system" fn FontFamily<Impl: ILanguageFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FontFamily() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FontWeight<Impl: ILanguageFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Text::FontWeight) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FontWeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FontStretch<Impl: ILanguageFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Text::FontStretch) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FontStretch() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FontStyle<Impl: ILanguageFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Text::FontStyle) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FontStyle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScaleFactor<Impl: ILanguageFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScaleFactor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILanguageFont>, ::windows::core::GetTrustLevel, FontFamily::<Impl, IMPL_OFFSET>, FontWeight::<Impl, IMPL_OFFSET>, FontStretch::<Impl, IMPL_OFFSET>, FontStyle::<Impl, IMPL_OFFSET>, ScaleFactor::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILanguageFont as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILanguageFontGroupImpl: Sized {
    fn UITextFont(&self) -> ::windows::core::Result<LanguageFont>;
    fn UIHeadingFont(&self) -> ::windows::core::Result<LanguageFont>;
    fn UITitleFont(&self) -> ::windows::core::Result<LanguageFont>;
    fn UICaptionFont(&self) -> ::windows::core::Result<LanguageFont>;
    fn UINotificationHeadingFont(&self) -> ::windows::core::Result<LanguageFont>;
    fn TraditionalDocumentFont(&self) -> ::windows::core::Result<LanguageFont>;
    fn ModernDocumentFont(&self) -> ::windows::core::Result<LanguageFont>;
    fn DocumentHeadingFont(&self) -> ::windows::core::Result<LanguageFont>;
    fn FixedWidthTextFont(&self) -> ::windows::core::Result<LanguageFont>;
    fn DocumentAlternate1Font(&self) -> ::windows::core::Result<LanguageFont>;
    fn DocumentAlternate2Font(&self) -> ::windows::core::Result<LanguageFont>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILanguageFontGroup {
    const NAME: &'static str = "Windows.Globalization.Fonts.ILanguageFontGroup";
}
#[cfg(feature = "implement_exclusive")]
impl ILanguageFontGroupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILanguageFontGroupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILanguageFontGroupVtbl {
        unsafe extern "system" fn UITextFont<Impl: ILanguageFontGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UITextFont() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UIHeadingFont<Impl: ILanguageFontGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UIHeadingFont() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UITitleFont<Impl: ILanguageFontGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UITitleFont() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UICaptionFont<Impl: ILanguageFontGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UICaptionFont() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UINotificationHeadingFont<Impl: ILanguageFontGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UINotificationHeadingFont() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TraditionalDocumentFont<Impl: ILanguageFontGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TraditionalDocumentFont() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModernDocumentFont<Impl: ILanguageFontGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ModernDocumentFont() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DocumentHeadingFont<Impl: ILanguageFontGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DocumentHeadingFont() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FixedWidthTextFont<Impl: ILanguageFontGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FixedWidthTextFont() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DocumentAlternate1Font<Impl: ILanguageFontGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DocumentAlternate1Font() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DocumentAlternate2Font<Impl: ILanguageFontGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DocumentAlternate2Font() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ILanguageFontGroup>,
            ::windows::core::GetTrustLevel,
            UITextFont::<Impl, IMPL_OFFSET>,
            UIHeadingFont::<Impl, IMPL_OFFSET>,
            UITitleFont::<Impl, IMPL_OFFSET>,
            UICaptionFont::<Impl, IMPL_OFFSET>,
            UINotificationHeadingFont::<Impl, IMPL_OFFSET>,
            TraditionalDocumentFont::<Impl, IMPL_OFFSET>,
            ModernDocumentFont::<Impl, IMPL_OFFSET>,
            DocumentHeadingFont::<Impl, IMPL_OFFSET>,
            FixedWidthTextFont::<Impl, IMPL_OFFSET>,
            DocumentAlternate1Font::<Impl, IMPL_OFFSET>,
            DocumentAlternate2Font::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILanguageFontGroup as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILanguageFontGroupFactoryImpl: Sized {
    fn CreateLanguageFontGroup(&self, languagetag: &::windows::core::HSTRING) -> ::windows::core::Result<LanguageFontGroup>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILanguageFontGroupFactory {
    const NAME: &'static str = "Windows.Globalization.Fonts.ILanguageFontGroupFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ILanguageFontGroupFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILanguageFontGroupFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILanguageFontGroupFactoryVtbl {
        unsafe extern "system" fn CreateLanguageFontGroup<Impl: ILanguageFontGroupFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languagetag: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateLanguageFontGroup(&*(&languagetag as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILanguageFontGroupFactory>, ::windows::core::GetTrustLevel, CreateLanguageFontGroup::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILanguageFontGroupFactory as ::windows::core::Interface>::IID
    }
}
