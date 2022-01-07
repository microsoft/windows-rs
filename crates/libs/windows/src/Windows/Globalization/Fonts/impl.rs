#[cfg(feature = "implement_exclusive")]
pub trait ILanguageFontImpl: Sized {
    fn FontFamily(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FontWeight(&self) -> ::windows::core::Result<super::super::UI::Text::FontWeight>;
    fn FontStretch(&self) -> ::windows::core::Result<super::super::UI::Text::FontStretch>;
    fn FontStyle(&self) -> ::windows::core::Result<super::super::UI::Text::FontStyle>;
    fn ScaleFactor(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILanguageFont {
    const NAME: &'static str = "Windows.Globalization.Fonts.ILanguageFont";
}
#[cfg(feature = "implement_exclusive")]
impl ILanguageFontVtbl {
    pub const fn new<Impl: ILanguageFontImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILanguageFontVtbl {
        unsafe extern "system" fn FontFamily<Impl: ILanguageFontImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FontFamily() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FontWeight<Impl: ILanguageFontImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Text::FontWeight) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FontWeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FontStretch<Impl: ILanguageFontImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Text::FontStretch) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FontStretch() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FontStyle<Impl: ILanguageFontImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Text::FontStyle) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FontStyle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScaleFactor<Impl: ILanguageFontImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScaleFactor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILanguageFont>, base.5, FontFamily::<Impl, OFFSET>, FontWeight::<Impl, OFFSET>, FontStretch::<Impl, OFFSET>, FontStyle::<Impl, OFFSET>, ScaleFactor::<Impl, OFFSET>)
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
    pub const fn new<Impl: ILanguageFontGroupImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILanguageFontGroupVtbl {
        unsafe extern "system" fn UITextFont<Impl: ILanguageFontGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UITextFont() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UIHeadingFont<Impl: ILanguageFontGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UIHeadingFont() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UITitleFont<Impl: ILanguageFontGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UITitleFont() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UICaptionFont<Impl: ILanguageFontGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UICaptionFont() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UINotificationHeadingFont<Impl: ILanguageFontGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UINotificationHeadingFont() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TraditionalDocumentFont<Impl: ILanguageFontGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TraditionalDocumentFont() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModernDocumentFont<Impl: ILanguageFontGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ModernDocumentFont() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DocumentHeadingFont<Impl: ILanguageFontGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DocumentHeadingFont() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FixedWidthTextFont<Impl: ILanguageFontGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FixedWidthTextFont() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DocumentAlternate1Font<Impl: ILanguageFontGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DocumentAlternate1Font() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DocumentAlternate2Font<Impl: ILanguageFontGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
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
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ILanguageFontGroup>,
            base.5,
            UITextFont::<Impl, OFFSET>,
            UIHeadingFont::<Impl, OFFSET>,
            UITitleFont::<Impl, OFFSET>,
            UICaptionFont::<Impl, OFFSET>,
            UINotificationHeadingFont::<Impl, OFFSET>,
            TraditionalDocumentFont::<Impl, OFFSET>,
            ModernDocumentFont::<Impl, OFFSET>,
            DocumentHeadingFont::<Impl, OFFSET>,
            FixedWidthTextFont::<Impl, OFFSET>,
            DocumentAlternate1Font::<Impl, OFFSET>,
            DocumentAlternate2Font::<Impl, OFFSET>,
        )
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
    pub const fn new<Impl: ILanguageFontGroupFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILanguageFontGroupFactoryVtbl {
        unsafe extern "system" fn CreateLanguageFontGroup<Impl: ILanguageFontGroupFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, languagetag: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateLanguageFontGroup(&*(&languagetag as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILanguageFontGroupFactory>, base.5, CreateLanguageFontGroup::<Impl, OFFSET>)
    }
}
