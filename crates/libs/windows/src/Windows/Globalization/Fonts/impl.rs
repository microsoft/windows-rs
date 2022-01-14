#[cfg(all(feature = "UI_Text", feature = "implement_exclusive"))]
pub trait ILanguageFont_Impl: Sized {
    fn FontFamily(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FontWeight(&mut self) -> ::windows::core::Result<super::super::UI::Text::FontWeight>;
    fn FontStretch(&mut self) -> ::windows::core::Result<super::super::UI::Text::FontStretch>;
    fn FontStyle(&mut self) -> ::windows::core::Result<super::super::UI::Text::FontStyle>;
    fn ScaleFactor(&mut self) -> ::windows::core::Result<f64>;
}
#[cfg(all(feature = "UI_Text", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILanguageFont {
    const NAME: &'static str = "Windows.Globalization.Fonts.ILanguageFont";
}
#[cfg(all(feature = "UI_Text", feature = "implement_exclusive"))]
impl ILanguageFont_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILanguageFont_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILanguageFont_Vtbl {
        unsafe extern "system" fn FontFamily<Impl: ILanguageFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FontWeight<Impl: ILanguageFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Text::FontWeight) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FontStretch<Impl: ILanguageFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Text::FontStretch) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FontStyle<Impl: ILanguageFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Text::FontStyle) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ScaleFactor<Impl: ILanguageFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILanguageFont, BASE_OFFSET>(),
            FontFamily: FontFamily::<Impl, IMPL_OFFSET>,
            FontWeight: FontWeight::<Impl, IMPL_OFFSET>,
            FontStretch: FontStretch::<Impl, IMPL_OFFSET>,
            FontStyle: FontStyle::<Impl, IMPL_OFFSET>,
            ScaleFactor: ScaleFactor::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILanguageFont as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILanguageFontGroup_Impl: Sized {
    fn UITextFont(&mut self) -> ::windows::core::Result<LanguageFont>;
    fn UIHeadingFont(&mut self) -> ::windows::core::Result<LanguageFont>;
    fn UITitleFont(&mut self) -> ::windows::core::Result<LanguageFont>;
    fn UICaptionFont(&mut self) -> ::windows::core::Result<LanguageFont>;
    fn UINotificationHeadingFont(&mut self) -> ::windows::core::Result<LanguageFont>;
    fn TraditionalDocumentFont(&mut self) -> ::windows::core::Result<LanguageFont>;
    fn ModernDocumentFont(&mut self) -> ::windows::core::Result<LanguageFont>;
    fn DocumentHeadingFont(&mut self) -> ::windows::core::Result<LanguageFont>;
    fn FixedWidthTextFont(&mut self) -> ::windows::core::Result<LanguageFont>;
    fn DocumentAlternate1Font(&mut self) -> ::windows::core::Result<LanguageFont>;
    fn DocumentAlternate2Font(&mut self) -> ::windows::core::Result<LanguageFont>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILanguageFontGroup {
    const NAME: &'static str = "Windows.Globalization.Fonts.ILanguageFontGroup";
}
#[cfg(feature = "implement_exclusive")]
impl ILanguageFontGroup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILanguageFontGroup_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILanguageFontGroup_Vtbl {
        unsafe extern "system" fn UITextFont<Impl: ILanguageFontGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UIHeadingFont<Impl: ILanguageFontGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UITitleFont<Impl: ILanguageFontGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UICaptionFont<Impl: ILanguageFontGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UINotificationHeadingFont<Impl: ILanguageFontGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TraditionalDocumentFont<Impl: ILanguageFontGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ModernDocumentFont<Impl: ILanguageFontGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DocumentHeadingFont<Impl: ILanguageFontGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FixedWidthTextFont<Impl: ILanguageFontGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DocumentAlternate1Font<Impl: ILanguageFontGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DocumentAlternate2Font<Impl: ILanguageFontGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILanguageFontGroup, BASE_OFFSET>(),
            UITextFont: UITextFont::<Impl, IMPL_OFFSET>,
            UIHeadingFont: UIHeadingFont::<Impl, IMPL_OFFSET>,
            UITitleFont: UITitleFont::<Impl, IMPL_OFFSET>,
            UICaptionFont: UICaptionFont::<Impl, IMPL_OFFSET>,
            UINotificationHeadingFont: UINotificationHeadingFont::<Impl, IMPL_OFFSET>,
            TraditionalDocumentFont: TraditionalDocumentFont::<Impl, IMPL_OFFSET>,
            ModernDocumentFont: ModernDocumentFont::<Impl, IMPL_OFFSET>,
            DocumentHeadingFont: DocumentHeadingFont::<Impl, IMPL_OFFSET>,
            FixedWidthTextFont: FixedWidthTextFont::<Impl, IMPL_OFFSET>,
            DocumentAlternate1Font: DocumentAlternate1Font::<Impl, IMPL_OFFSET>,
            DocumentAlternate2Font: DocumentAlternate2Font::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILanguageFontGroup as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILanguageFontGroupFactory_Impl: Sized {
    fn CreateLanguageFontGroup(&mut self, languagetag: &::windows::core::HSTRING) -> ::windows::core::Result<LanguageFontGroup>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILanguageFontGroupFactory {
    const NAME: &'static str = "Windows.Globalization.Fonts.ILanguageFontGroupFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ILanguageFontGroupFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILanguageFontGroupFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILanguageFontGroupFactory_Vtbl {
        unsafe extern "system" fn CreateLanguageFontGroup<Impl: ILanguageFontGroupFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languagetag: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILanguageFontGroupFactory, BASE_OFFSET>(),
            CreateLanguageFontGroup: CreateLanguageFontGroup::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILanguageFontGroupFactory as ::windows::core::Interface>::IID
    }
}
