#[cfg(feature = "implement_exclusive")]
pub trait IContentLinkInfoImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<u32>;
    fn SetId(&self, value: u32) -> ::windows::core::Result<()>;
    fn DisplayText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SecondaryText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSecondaryText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn LinkContentKind(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLinkContentKind(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContentLinkInfo {
    const NAME: &'static str = "Windows.UI.Text.IContentLinkInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IContentLinkInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContentLinkInfoImpl, const OFFSET: isize>() -> IContentLinkInfoVtbl {
        unsafe extern "system" fn Id<Impl: IContentLinkInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetId<Impl: IContentLinkInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetId(value).into()
        }
        unsafe extern "system" fn DisplayText<Impl: IContentLinkInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayText<Impl: IContentLinkInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SecondaryText<Impl: IContentLinkInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SecondaryText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecondaryText<Impl: IContentLinkInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSecondaryText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Uri<Impl: IContentLinkInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Uri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUri<Impl: IContentLinkInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUri(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LinkContentKind<Impl: IContentLinkInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LinkContentKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLinkContentKind<Impl: IContentLinkInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLinkContentKind(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IContentLinkInfo>,
            ::windows::core::GetTrustLevel,
            Id::<Impl, OFFSET>,
            SetId::<Impl, OFFSET>,
            DisplayText::<Impl, OFFSET>,
            SetDisplayText::<Impl, OFFSET>,
            SecondaryText::<Impl, OFFSET>,
            SetSecondaryText::<Impl, OFFSET>,
            Uri::<Impl, OFFSET>,
            SetUri::<Impl, OFFSET>,
            LinkContentKind::<Impl, OFFSET>,
            SetLinkContentKind::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFontWeightsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFontWeights {
    const NAME: &'static str = "Windows.UI.Text.IFontWeights";
}
#[cfg(feature = "implement_exclusive")]
impl IFontWeightsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFontWeightsImpl, const OFFSET: isize>() -> IFontWeightsVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFontWeights>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFontWeightsStaticsImpl: Sized {
    fn Black(&self) -> ::windows::core::Result<FontWeight>;
    fn Bold(&self) -> ::windows::core::Result<FontWeight>;
    fn ExtraBlack(&self) -> ::windows::core::Result<FontWeight>;
    fn ExtraBold(&self) -> ::windows::core::Result<FontWeight>;
    fn ExtraLight(&self) -> ::windows::core::Result<FontWeight>;
    fn Light(&self) -> ::windows::core::Result<FontWeight>;
    fn Medium(&self) -> ::windows::core::Result<FontWeight>;
    fn Normal(&self) -> ::windows::core::Result<FontWeight>;
    fn SemiBold(&self) -> ::windows::core::Result<FontWeight>;
    fn SemiLight(&self) -> ::windows::core::Result<FontWeight>;
    fn Thin(&self) -> ::windows::core::Result<FontWeight>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFontWeightsStatics {
    const NAME: &'static str = "Windows.UI.Text.IFontWeightsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IFontWeightsStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFontWeightsStaticsImpl, const OFFSET: isize>() -> IFontWeightsStaticsVtbl {
        unsafe extern "system" fn Black<Impl: IFontWeightsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FontWeight) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Black() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Bold<Impl: IFontWeightsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FontWeight) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Bold() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtraBlack<Impl: IFontWeightsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FontWeight) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtraBlack() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtraBold<Impl: IFontWeightsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FontWeight) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtraBold() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtraLight<Impl: IFontWeightsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FontWeight) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtraLight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Light<Impl: IFontWeightsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FontWeight) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Light() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Medium<Impl: IFontWeightsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FontWeight) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Medium() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Normal<Impl: IFontWeightsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FontWeight) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Normal() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SemiBold<Impl: IFontWeightsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FontWeight) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SemiBold() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SemiLight<Impl: IFontWeightsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FontWeight) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SemiLight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Thin<Impl: IFontWeightsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FontWeight) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Thin() {
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
            ::windows::core::GetRuntimeClassName::<IFontWeightsStatics>,
            ::windows::core::GetTrustLevel,
            Black::<Impl, OFFSET>,
            Bold::<Impl, OFFSET>,
            ExtraBlack::<Impl, OFFSET>,
            ExtraBold::<Impl, OFFSET>,
            ExtraLight::<Impl, OFFSET>,
            Light::<Impl, OFFSET>,
            Medium::<Impl, OFFSET>,
            Normal::<Impl, OFFSET>,
            SemiBold::<Impl, OFFSET>,
            SemiLight::<Impl, OFFSET>,
            Thin::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRichEditTextRangeImpl: Sized {
    fn ContentLinkInfo(&self) -> ::windows::core::Result<ContentLinkInfo>;
    fn SetContentLinkInfo(&self, value: &::core::option::Option<ContentLinkInfo>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRichEditTextRange {
    const NAME: &'static str = "Windows.UI.Text.IRichEditTextRange";
}
#[cfg(feature = "implement_exclusive")]
impl IRichEditTextRangeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRichEditTextRangeImpl, const OFFSET: isize>() -> IRichEditTextRangeVtbl {
        unsafe extern "system" fn ContentLinkInfo<Impl: IRichEditTextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentLinkInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentLinkInfo<Impl: IRichEditTextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentLinkInfo(&*(&value as *const <ContentLinkInfo as ::windows::core::Abi>::Abi as *const <ContentLinkInfo as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRichEditTextRange>, ::windows::core::GetTrustLevel, ContentLinkInfo::<Impl, OFFSET>, SetContentLinkInfo::<Impl, OFFSET>)
    }
}
pub trait ITextCharacterFormatImpl: Sized {
    fn AllCaps(&self) -> ::windows::core::Result<FormatEffect>;
    fn SetAllCaps(&self, value: FormatEffect) -> ::windows::core::Result<()>;
    fn BackgroundColor(&self) -> ::windows::core::Result<super::Color>;
    fn SetBackgroundColor(&self, value: &super::Color) -> ::windows::core::Result<()>;
    fn Bold(&self) -> ::windows::core::Result<FormatEffect>;
    fn SetBold(&self, value: FormatEffect) -> ::windows::core::Result<()>;
    fn FontStretch(&self) -> ::windows::core::Result<FontStretch>;
    fn SetFontStretch(&self, value: FontStretch) -> ::windows::core::Result<()>;
    fn FontStyle(&self) -> ::windows::core::Result<FontStyle>;
    fn SetFontStyle(&self, value: FontStyle) -> ::windows::core::Result<()>;
    fn ForegroundColor(&self) -> ::windows::core::Result<super::Color>;
    fn SetForegroundColor(&self, value: &super::Color) -> ::windows::core::Result<()>;
    fn Hidden(&self) -> ::windows::core::Result<FormatEffect>;
    fn SetHidden(&self, value: FormatEffect) -> ::windows::core::Result<()>;
    fn Italic(&self) -> ::windows::core::Result<FormatEffect>;
    fn SetItalic(&self, value: FormatEffect) -> ::windows::core::Result<()>;
    fn Kerning(&self) -> ::windows::core::Result<f32>;
    fn SetKerning(&self, value: f32) -> ::windows::core::Result<()>;
    fn LanguageTag(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLanguageTag(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn LinkType(&self) -> ::windows::core::Result<LinkType>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Outline(&self) -> ::windows::core::Result<FormatEffect>;
    fn SetOutline(&self, value: FormatEffect) -> ::windows::core::Result<()>;
    fn Position(&self) -> ::windows::core::Result<f32>;
    fn SetPosition(&self, value: f32) -> ::windows::core::Result<()>;
    fn ProtectedText(&self) -> ::windows::core::Result<FormatEffect>;
    fn SetProtectedText(&self, value: FormatEffect) -> ::windows::core::Result<()>;
    fn Size(&self) -> ::windows::core::Result<f32>;
    fn SetSize(&self, value: f32) -> ::windows::core::Result<()>;
    fn SmallCaps(&self) -> ::windows::core::Result<FormatEffect>;
    fn SetSmallCaps(&self, value: FormatEffect) -> ::windows::core::Result<()>;
    fn Spacing(&self) -> ::windows::core::Result<f32>;
    fn SetSpacing(&self, value: f32) -> ::windows::core::Result<()>;
    fn Strikethrough(&self) -> ::windows::core::Result<FormatEffect>;
    fn SetStrikethrough(&self, value: FormatEffect) -> ::windows::core::Result<()>;
    fn Subscript(&self) -> ::windows::core::Result<FormatEffect>;
    fn SetSubscript(&self, value: FormatEffect) -> ::windows::core::Result<()>;
    fn Superscript(&self) -> ::windows::core::Result<FormatEffect>;
    fn SetSuperscript(&self, value: FormatEffect) -> ::windows::core::Result<()>;
    fn TextScript(&self) -> ::windows::core::Result<TextScript>;
    fn SetTextScript(&self, value: TextScript) -> ::windows::core::Result<()>;
    fn Underline(&self) -> ::windows::core::Result<UnderlineType>;
    fn SetUnderline(&self, value: UnderlineType) -> ::windows::core::Result<()>;
    fn Weight(&self) -> ::windows::core::Result<i32>;
    fn SetWeight(&self, value: i32) -> ::windows::core::Result<()>;
    fn SetClone(&self, value: &::core::option::Option<ITextCharacterFormat>) -> ::windows::core::Result<()>;
    fn GetClone(&self) -> ::windows::core::Result<ITextCharacterFormat>;
    fn IsEqual(&self, format: &::core::option::Option<ITextCharacterFormat>) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for ITextCharacterFormat {
    const NAME: &'static str = "Windows.UI.Text.ITextCharacterFormat";
}
impl ITextCharacterFormatVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextCharacterFormatImpl, const OFFSET: isize>() -> ITextCharacterFormatVtbl {
        unsafe extern "system" fn AllCaps<Impl: ITextCharacterFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllCaps() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllCaps<Impl: ITextCharacterFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllCaps(value).into()
        }
        unsafe extern "system" fn BackgroundColor<Impl: ITextCharacterFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BackgroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackgroundColor<Impl: ITextCharacterFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBackgroundColor(&*(&value as *const <super::Color as ::windows::core::Abi>::Abi as *const <super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Bold<Impl: ITextCharacterFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Bold() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBold<Impl: ITextCharacterFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBold(value).into()
        }
        unsafe extern "system" fn FontStretch<Impl: ITextCharacterFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FontStretch) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetFontStretch<Impl: ITextCharacterFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FontStretch) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFontStretch(value).into()
        }
        unsafe extern "system" fn FontStyle<Impl: ITextCharacterFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FontStyle) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetFontStyle<Impl: ITextCharacterFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FontStyle) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFontStyle(value).into()
        }
        unsafe extern "system" fn ForegroundColor<Impl: ITextCharacterFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ForegroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForegroundColor<Impl: ITextCharacterFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetForegroundColor(&*(&value as *const <super::Color as ::windows::core::Abi>::Abi as *const <super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Hidden<Impl: ITextCharacterFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Hidden() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHidden<Impl: ITextCharacterFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHidden(value).into()
        }
        unsafe extern "system" fn Italic<Impl: ITextCharacterFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Italic() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetItalic<Impl: ITextCharacterFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetItalic(value).into()
        }
        unsafe extern "system" fn Kerning<Impl: ITextCharacterFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Kerning() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKerning<Impl: ITextCharacterFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKerning(value).into()
        }
        unsafe extern "system" fn LanguageTag<Impl: ITextCharacterFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LanguageTag() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLanguageTag<Impl: ITextCharacterFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLanguageTag(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LinkType<Impl: ITextCharacterFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut LinkType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LinkType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: ITextCharacterFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: ITextCharacterFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Outline<Impl: ITextCharacterFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Outline() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutline<Impl: ITextCharacterFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutline(value).into()
        }
        unsafe extern "system" fn Position<Impl: ITextCharacterFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPosition<Impl: ITextCharacterFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPosition(value).into()
        }
        unsafe extern "system" fn ProtectedText<Impl: ITextCharacterFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProtectedText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProtectedText<Impl: ITextCharacterFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProtectedText(value).into()
        }
        unsafe extern "system" fn Size<Impl: ITextCharacterFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Size() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSize<Impl: ITextCharacterFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSize(value).into()
        }
        unsafe extern "system" fn SmallCaps<Impl: ITextCharacterFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SmallCaps() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSmallCaps<Impl: ITextCharacterFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSmallCaps(value).into()
        }
        unsafe extern "system" fn Spacing<Impl: ITextCharacterFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Spacing() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpacing<Impl: ITextCharacterFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSpacing(value).into()
        }
        unsafe extern "system" fn Strikethrough<Impl: ITextCharacterFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Strikethrough() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrikethrough<Impl: ITextCharacterFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrikethrough(value).into()
        }
        unsafe extern "system" fn Subscript<Impl: ITextCharacterFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Subscript() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubscript<Impl: ITextCharacterFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSubscript(value).into()
        }
        unsafe extern "system" fn Superscript<Impl: ITextCharacterFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Superscript() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSuperscript<Impl: ITextCharacterFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSuperscript(value).into()
        }
        unsafe extern "system" fn TextScript<Impl: ITextCharacterFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TextScript) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TextScript() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTextScript<Impl: ITextCharacterFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: TextScript) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTextScript(value).into()
        }
        unsafe extern "system" fn Underline<Impl: ITextCharacterFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UnderlineType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Underline() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnderline<Impl: ITextCharacterFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: UnderlineType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUnderline(value).into()
        }
        unsafe extern "system" fn Weight<Impl: ITextCharacterFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Weight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWeight<Impl: ITextCharacterFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWeight(value).into()
        }
        unsafe extern "system" fn SetClone<Impl: ITextCharacterFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClone(&*(&value as *const <ITextCharacterFormat as ::windows::core::Abi>::Abi as *const <ITextCharacterFormat as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetClone<Impl: ITextCharacterFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClone() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqual<Impl: ITextCharacterFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEqual(&*(&format as *const <ITextCharacterFormat as ::windows::core::Abi>::Abi as *const <ITextCharacterFormat as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<ITextCharacterFormat>,
            ::windows::core::GetTrustLevel,
            AllCaps::<Impl, OFFSET>,
            SetAllCaps::<Impl, OFFSET>,
            BackgroundColor::<Impl, OFFSET>,
            SetBackgroundColor::<Impl, OFFSET>,
            Bold::<Impl, OFFSET>,
            SetBold::<Impl, OFFSET>,
            FontStretch::<Impl, OFFSET>,
            SetFontStretch::<Impl, OFFSET>,
            FontStyle::<Impl, OFFSET>,
            SetFontStyle::<Impl, OFFSET>,
            ForegroundColor::<Impl, OFFSET>,
            SetForegroundColor::<Impl, OFFSET>,
            Hidden::<Impl, OFFSET>,
            SetHidden::<Impl, OFFSET>,
            Italic::<Impl, OFFSET>,
            SetItalic::<Impl, OFFSET>,
            Kerning::<Impl, OFFSET>,
            SetKerning::<Impl, OFFSET>,
            LanguageTag::<Impl, OFFSET>,
            SetLanguageTag::<Impl, OFFSET>,
            LinkType::<Impl, OFFSET>,
            Name::<Impl, OFFSET>,
            SetName::<Impl, OFFSET>,
            Outline::<Impl, OFFSET>,
            SetOutline::<Impl, OFFSET>,
            Position::<Impl, OFFSET>,
            SetPosition::<Impl, OFFSET>,
            ProtectedText::<Impl, OFFSET>,
            SetProtectedText::<Impl, OFFSET>,
            Size::<Impl, OFFSET>,
            SetSize::<Impl, OFFSET>,
            SmallCaps::<Impl, OFFSET>,
            SetSmallCaps::<Impl, OFFSET>,
            Spacing::<Impl, OFFSET>,
            SetSpacing::<Impl, OFFSET>,
            Strikethrough::<Impl, OFFSET>,
            SetStrikethrough::<Impl, OFFSET>,
            Subscript::<Impl, OFFSET>,
            SetSubscript::<Impl, OFFSET>,
            Superscript::<Impl, OFFSET>,
            SetSuperscript::<Impl, OFFSET>,
            TextScript::<Impl, OFFSET>,
            SetTextScript::<Impl, OFFSET>,
            Underline::<Impl, OFFSET>,
            SetUnderline::<Impl, OFFSET>,
            Weight::<Impl, OFFSET>,
            SetWeight::<Impl, OFFSET>,
            SetClone::<Impl, OFFSET>,
            GetClone::<Impl, OFFSET>,
            IsEqual::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextConstantsStaticsImpl: Sized {
    fn AutoColor(&self) -> ::windows::core::Result<super::Color>;
    fn MinUnitCount(&self) -> ::windows::core::Result<i32>;
    fn MaxUnitCount(&self) -> ::windows::core::Result<i32>;
    fn UndefinedColor(&self) -> ::windows::core::Result<super::Color>;
    fn UndefinedFloatValue(&self) -> ::windows::core::Result<f32>;
    fn UndefinedInt32Value(&self) -> ::windows::core::Result<i32>;
    fn UndefinedFontStretch(&self) -> ::windows::core::Result<FontStretch>;
    fn UndefinedFontStyle(&self) -> ::windows::core::Result<FontStyle>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITextConstantsStatics {
    const NAME: &'static str = "Windows.UI.Text.ITextConstantsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ITextConstantsStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextConstantsStaticsImpl, const OFFSET: isize>() -> ITextConstantsStaticsVtbl {
        unsafe extern "system" fn AutoColor<Impl: ITextConstantsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinUnitCount<Impl: ITextConstantsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinUnitCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxUnitCount<Impl: ITextConstantsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxUnitCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UndefinedColor<Impl: ITextConstantsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UndefinedColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UndefinedFloatValue<Impl: ITextConstantsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UndefinedFloatValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UndefinedInt32Value<Impl: ITextConstantsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UndefinedInt32Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UndefinedFontStretch<Impl: ITextConstantsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FontStretch) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UndefinedFontStretch() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UndefinedFontStyle<Impl: ITextConstantsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FontStyle) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UndefinedFontStyle() {
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
            ::windows::core::GetRuntimeClassName::<ITextConstantsStatics>,
            ::windows::core::GetTrustLevel,
            AutoColor::<Impl, OFFSET>,
            MinUnitCount::<Impl, OFFSET>,
            MaxUnitCount::<Impl, OFFSET>,
            UndefinedColor::<Impl, OFFSET>,
            UndefinedFloatValue::<Impl, OFFSET>,
            UndefinedInt32Value::<Impl, OFFSET>,
            UndefinedFontStretch::<Impl, OFFSET>,
            UndefinedFontStyle::<Impl, OFFSET>,
        )
    }
}
pub trait ITextDocumentImpl: Sized {
    fn CaretType(&self) -> ::windows::core::Result<CaretType>;
    fn SetCaretType(&self, value: CaretType) -> ::windows::core::Result<()>;
    fn DefaultTabStop(&self) -> ::windows::core::Result<f32>;
    fn SetDefaultTabStop(&self, value: f32) -> ::windows::core::Result<()>;
    fn Selection(&self) -> ::windows::core::Result<ITextSelection>;
    fn UndoLimit(&self) -> ::windows::core::Result<u32>;
    fn SetUndoLimit(&self, value: u32) -> ::windows::core::Result<()>;
    fn CanCopy(&self) -> ::windows::core::Result<bool>;
    fn CanPaste(&self) -> ::windows::core::Result<bool>;
    fn CanRedo(&self) -> ::windows::core::Result<bool>;
    fn CanUndo(&self) -> ::windows::core::Result<bool>;
    fn ApplyDisplayUpdates(&self) -> ::windows::core::Result<i32>;
    fn BatchDisplayUpdates(&self) -> ::windows::core::Result<i32>;
    fn BeginUndoGroup(&self) -> ::windows::core::Result<()>;
    fn EndUndoGroup(&self) -> ::windows::core::Result<()>;
    fn GetDefaultCharacterFormat(&self) -> ::windows::core::Result<ITextCharacterFormat>;
    fn GetDefaultParagraphFormat(&self) -> ::windows::core::Result<ITextParagraphFormat>;
    fn GetRange(&self, startposition: i32, endposition: i32) -> ::windows::core::Result<ITextRange>;
    fn GetRangeFromPoint(&self, point: &super::super::Foundation::Point, options: PointOptions) -> ::windows::core::Result<ITextRange>;
    fn GetText(&self, options: TextGetOptions, value: &mut ::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn LoadFromStream(&self, options: TextSetOptions, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<()>;
    fn Redo(&self) -> ::windows::core::Result<()>;
    fn SaveToStream(&self, options: TextGetOptions, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<()>;
    fn SetDefaultCharacterFormat(&self, value: &::core::option::Option<ITextCharacterFormat>) -> ::windows::core::Result<()>;
    fn SetDefaultParagraphFormat(&self, value: &::core::option::Option<ITextParagraphFormat>) -> ::windows::core::Result<()>;
    fn SetText(&self, options: TextSetOptions, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Undo(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ITextDocument {
    const NAME: &'static str = "Windows.UI.Text.ITextDocument";
}
impl ITextDocumentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocumentImpl, const OFFSET: isize>() -> ITextDocumentVtbl {
        unsafe extern "system" fn CaretType<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CaretType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CaretType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCaretType<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CaretType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCaretType(value).into()
        }
        unsafe extern "system" fn DefaultTabStop<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultTabStop() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultTabStop<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefaultTabStop(value).into()
        }
        unsafe extern "system" fn Selection<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Selection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UndoLimit<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UndoLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUndoLimit<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUndoLimit(value).into()
        }
        unsafe extern "system" fn CanCopy<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanCopy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanPaste<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanPaste() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanRedo<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanRedo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanUndo<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanUndo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplyDisplayUpdates<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplyDisplayUpdates() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BatchDisplayUpdates<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BatchDisplayUpdates() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginUndoGroup<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginUndoGroup().into()
        }
        unsafe extern "system" fn EndUndoGroup<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndUndoGroup().into()
        }
        unsafe extern "system" fn GetDefaultCharacterFormat<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultCharacterFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultParagraphFormat<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultParagraphFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRange<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startposition: i32, endposition: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRange(startposition, endposition) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRangeFromPoint<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, point: super::super::Foundation::Point, options: PointOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRangeFromPoint(&*(&point as *const <super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType), options) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetText<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: TextGetOptions, value: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetText(options, ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn LoadFromStream<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: TextSetOptions, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LoadFromStream(options, &*(&value as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Redo<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Redo().into()
        }
        unsafe extern "system" fn SaveToStream<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: TextGetOptions, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SaveToStream(options, &*(&value as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetDefaultCharacterFormat<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefaultCharacterFormat(&*(&value as *const <ITextCharacterFormat as ::windows::core::Abi>::Abi as *const <ITextCharacterFormat as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetDefaultParagraphFormat<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefaultParagraphFormat(&*(&value as *const <ITextParagraphFormat as ::windows::core::Abi>::Abi as *const <ITextParagraphFormat as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetText<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: TextSetOptions, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetText(options, &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Undo<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Undo().into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ITextDocument>,
            ::windows::core::GetTrustLevel,
            CaretType::<Impl, OFFSET>,
            SetCaretType::<Impl, OFFSET>,
            DefaultTabStop::<Impl, OFFSET>,
            SetDefaultTabStop::<Impl, OFFSET>,
            Selection::<Impl, OFFSET>,
            UndoLimit::<Impl, OFFSET>,
            SetUndoLimit::<Impl, OFFSET>,
            CanCopy::<Impl, OFFSET>,
            CanPaste::<Impl, OFFSET>,
            CanRedo::<Impl, OFFSET>,
            CanUndo::<Impl, OFFSET>,
            ApplyDisplayUpdates::<Impl, OFFSET>,
            BatchDisplayUpdates::<Impl, OFFSET>,
            BeginUndoGroup::<Impl, OFFSET>,
            EndUndoGroup::<Impl, OFFSET>,
            GetDefaultCharacterFormat::<Impl, OFFSET>,
            GetDefaultParagraphFormat::<Impl, OFFSET>,
            GetRange::<Impl, OFFSET>,
            GetRangeFromPoint::<Impl, OFFSET>,
            GetText::<Impl, OFFSET>,
            LoadFromStream::<Impl, OFFSET>,
            Redo::<Impl, OFFSET>,
            SaveToStream::<Impl, OFFSET>,
            SetDefaultCharacterFormat::<Impl, OFFSET>,
            SetDefaultParagraphFormat::<Impl, OFFSET>,
            SetText::<Impl, OFFSET>,
            Undo::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextDocument2Impl: Sized {
    fn AlignmentIncludesTrailingWhitespace(&self) -> ::windows::core::Result<bool>;
    fn SetAlignmentIncludesTrailingWhitespace(&self, value: bool) -> ::windows::core::Result<()>;
    fn IgnoreTrailingCharacterSpacing(&self) -> ::windows::core::Result<bool>;
    fn SetIgnoreTrailingCharacterSpacing(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITextDocument2 {
    const NAME: &'static str = "Windows.UI.Text.ITextDocument2";
}
#[cfg(feature = "implement_exclusive")]
impl ITextDocument2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2Impl, const OFFSET: isize>() -> ITextDocument2Vtbl {
        unsafe extern "system" fn AlignmentIncludesTrailingWhitespace<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlignmentIncludesTrailingWhitespace() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlignmentIncludesTrailingWhitespace<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlignmentIncludesTrailingWhitespace(value).into()
        }
        unsafe extern "system" fn IgnoreTrailingCharacterSpacing<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IgnoreTrailingCharacterSpacing() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIgnoreTrailingCharacterSpacing<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIgnoreTrailingCharacterSpacing(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITextDocument2>, ::windows::core::GetTrustLevel, AlignmentIncludesTrailingWhitespace::<Impl, OFFSET>, SetAlignmentIncludesTrailingWhitespace::<Impl, OFFSET>, IgnoreTrailingCharacterSpacing::<Impl, OFFSET>, SetIgnoreTrailingCharacterSpacing::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextDocument3Impl: Sized {
    fn ClearUndoRedoHistory(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITextDocument3 {
    const NAME: &'static str = "Windows.UI.Text.ITextDocument3";
}
#[cfg(feature = "implement_exclusive")]
impl ITextDocument3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument3Impl, const OFFSET: isize>() -> ITextDocument3Vtbl {
        unsafe extern "system" fn ClearUndoRedoHistory<Impl: ITextDocument3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearUndoRedoHistory().into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITextDocument3>, ::windows::core::GetTrustLevel, ClearUndoRedoHistory::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextDocument4Impl: Sized {
    fn SetMath(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn GetMath(&self, value: &mut ::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetMathMode(&self, mode: RichEditMathMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITextDocument4 {
    const NAME: &'static str = "Windows.UI.Text.ITextDocument4";
}
#[cfg(feature = "implement_exclusive")]
impl ITextDocument4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument4Impl, const OFFSET: isize>() -> ITextDocument4Vtbl {
        unsafe extern "system" fn SetMath<Impl: ITextDocument4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMath(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetMath<Impl: ITextDocument4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMath(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetMathMode<Impl: ITextDocument4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: RichEditMathMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMathMode(mode).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITextDocument4>, ::windows::core::GetTrustLevel, SetMath::<Impl, OFFSET>, GetMath::<Impl, OFFSET>, SetMathMode::<Impl, OFFSET>)
    }
}
pub trait ITextParagraphFormatImpl: Sized {
    fn Alignment(&self) -> ::windows::core::Result<ParagraphAlignment>;
    fn SetAlignment(&self, value: ParagraphAlignment) -> ::windows::core::Result<()>;
    fn FirstLineIndent(&self) -> ::windows::core::Result<f32>;
    fn KeepTogether(&self) -> ::windows::core::Result<FormatEffect>;
    fn SetKeepTogether(&self, value: FormatEffect) -> ::windows::core::Result<()>;
    fn KeepWithNext(&self) -> ::windows::core::Result<FormatEffect>;
    fn SetKeepWithNext(&self, value: FormatEffect) -> ::windows::core::Result<()>;
    fn LeftIndent(&self) -> ::windows::core::Result<f32>;
    fn LineSpacing(&self) -> ::windows::core::Result<f32>;
    fn LineSpacingRule(&self) -> ::windows::core::Result<LineSpacingRule>;
    fn ListAlignment(&self) -> ::windows::core::Result<MarkerAlignment>;
    fn SetListAlignment(&self, value: MarkerAlignment) -> ::windows::core::Result<()>;
    fn ListLevelIndex(&self) -> ::windows::core::Result<i32>;
    fn SetListLevelIndex(&self, value: i32) -> ::windows::core::Result<()>;
    fn ListStart(&self) -> ::windows::core::Result<i32>;
    fn SetListStart(&self, value: i32) -> ::windows::core::Result<()>;
    fn ListStyle(&self) -> ::windows::core::Result<MarkerStyle>;
    fn SetListStyle(&self, value: MarkerStyle) -> ::windows::core::Result<()>;
    fn ListTab(&self) -> ::windows::core::Result<f32>;
    fn SetListTab(&self, value: f32) -> ::windows::core::Result<()>;
    fn ListType(&self) -> ::windows::core::Result<MarkerType>;
    fn SetListType(&self, value: MarkerType) -> ::windows::core::Result<()>;
    fn NoLineNumber(&self) -> ::windows::core::Result<FormatEffect>;
    fn SetNoLineNumber(&self, value: FormatEffect) -> ::windows::core::Result<()>;
    fn PageBreakBefore(&self) -> ::windows::core::Result<FormatEffect>;
    fn SetPageBreakBefore(&self, value: FormatEffect) -> ::windows::core::Result<()>;
    fn RightIndent(&self) -> ::windows::core::Result<f32>;
    fn SetRightIndent(&self, value: f32) -> ::windows::core::Result<()>;
    fn RightToLeft(&self) -> ::windows::core::Result<FormatEffect>;
    fn SetRightToLeft(&self, value: FormatEffect) -> ::windows::core::Result<()>;
    fn Style(&self) -> ::windows::core::Result<ParagraphStyle>;
    fn SetStyle(&self, value: ParagraphStyle) -> ::windows::core::Result<()>;
    fn SpaceAfter(&self) -> ::windows::core::Result<f32>;
    fn SetSpaceAfter(&self, value: f32) -> ::windows::core::Result<()>;
    fn SpaceBefore(&self) -> ::windows::core::Result<f32>;
    fn SetSpaceBefore(&self, value: f32) -> ::windows::core::Result<()>;
    fn WidowControl(&self) -> ::windows::core::Result<FormatEffect>;
    fn SetWidowControl(&self, value: FormatEffect) -> ::windows::core::Result<()>;
    fn TabCount(&self) -> ::windows::core::Result<i32>;
    fn AddTab(&self, position: f32, align: TabAlignment, leader: TabLeader) -> ::windows::core::Result<()>;
    fn ClearAllTabs(&self) -> ::windows::core::Result<()>;
    fn DeleteTab(&self, position: f32) -> ::windows::core::Result<()>;
    fn GetClone(&self) -> ::windows::core::Result<ITextParagraphFormat>;
    fn GetTab(&self, index: i32, position: &mut f32, align: &mut TabAlignment, leader: &mut TabLeader) -> ::windows::core::Result<()>;
    fn IsEqual(&self, format: &::core::option::Option<ITextParagraphFormat>) -> ::windows::core::Result<bool>;
    fn SetClone(&self, format: &::core::option::Option<ITextParagraphFormat>) -> ::windows::core::Result<()>;
    fn SetIndents(&self, start: f32, left: f32, right: f32) -> ::windows::core::Result<()>;
    fn SetLineSpacing(&self, rule: LineSpacingRule, spacing: f32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ITextParagraphFormat {
    const NAME: &'static str = "Windows.UI.Text.ITextParagraphFormat";
}
impl ITextParagraphFormatVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextParagraphFormatImpl, const OFFSET: isize>() -> ITextParagraphFormatVtbl {
        unsafe extern "system" fn Alignment<Impl: ITextParagraphFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ParagraphAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Alignment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlignment<Impl: ITextParagraphFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ParagraphAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlignment(value).into()
        }
        unsafe extern "system" fn FirstLineIndent<Impl: ITextParagraphFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FirstLineIndent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeepTogether<Impl: ITextParagraphFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeepTogether() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeepTogether<Impl: ITextParagraphFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeepTogether(value).into()
        }
        unsafe extern "system" fn KeepWithNext<Impl: ITextParagraphFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeepWithNext() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeepWithNext<Impl: ITextParagraphFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeepWithNext(value).into()
        }
        unsafe extern "system" fn LeftIndent<Impl: ITextParagraphFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LeftIndent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LineSpacing<Impl: ITextParagraphFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LineSpacing() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LineSpacingRule<Impl: ITextParagraphFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut LineSpacingRule) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LineSpacingRule() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ListAlignment<Impl: ITextParagraphFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MarkerAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ListAlignment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListAlignment<Impl: ITextParagraphFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MarkerAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetListAlignment(value).into()
        }
        unsafe extern "system" fn ListLevelIndex<Impl: ITextParagraphFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ListLevelIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListLevelIndex<Impl: ITextParagraphFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetListLevelIndex(value).into()
        }
        unsafe extern "system" fn ListStart<Impl: ITextParagraphFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ListStart() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListStart<Impl: ITextParagraphFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetListStart(value).into()
        }
        unsafe extern "system" fn ListStyle<Impl: ITextParagraphFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MarkerStyle) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ListStyle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListStyle<Impl: ITextParagraphFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MarkerStyle) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetListStyle(value).into()
        }
        unsafe extern "system" fn ListTab<Impl: ITextParagraphFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ListTab() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListTab<Impl: ITextParagraphFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetListTab(value).into()
        }
        unsafe extern "system" fn ListType<Impl: ITextParagraphFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MarkerType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ListType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListType<Impl: ITextParagraphFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MarkerType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetListType(value).into()
        }
        unsafe extern "system" fn NoLineNumber<Impl: ITextParagraphFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NoLineNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNoLineNumber<Impl: ITextParagraphFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNoLineNumber(value).into()
        }
        unsafe extern "system" fn PageBreakBefore<Impl: ITextParagraphFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PageBreakBefore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPageBreakBefore<Impl: ITextParagraphFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPageBreakBefore(value).into()
        }
        unsafe extern "system" fn RightIndent<Impl: ITextParagraphFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RightIndent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRightIndent<Impl: ITextParagraphFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRightIndent(value).into()
        }
        unsafe extern "system" fn RightToLeft<Impl: ITextParagraphFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RightToLeft() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRightToLeft<Impl: ITextParagraphFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRightToLeft(value).into()
        }
        unsafe extern "system" fn Style<Impl: ITextParagraphFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ParagraphStyle) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Style() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStyle<Impl: ITextParagraphFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ParagraphStyle) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStyle(value).into()
        }
        unsafe extern "system" fn SpaceAfter<Impl: ITextParagraphFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SpaceAfter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpaceAfter<Impl: ITextParagraphFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSpaceAfter(value).into()
        }
        unsafe extern "system" fn SpaceBefore<Impl: ITextParagraphFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SpaceBefore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpaceBefore<Impl: ITextParagraphFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSpaceBefore(value).into()
        }
        unsafe extern "system" fn WidowControl<Impl: ITextParagraphFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WidowControl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWidowControl<Impl: ITextParagraphFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWidowControl(value).into()
        }
        unsafe extern "system" fn TabCount<Impl: ITextParagraphFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TabCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddTab<Impl: ITextParagraphFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, position: f32, align: TabAlignment, leader: TabLeader) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddTab(position, align, leader).into()
        }
        unsafe extern "system" fn ClearAllTabs<Impl: ITextParagraphFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearAllTabs().into()
        }
        unsafe extern "system" fn DeleteTab<Impl: ITextParagraphFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, position: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteTab(position).into()
        }
        unsafe extern "system" fn GetClone<Impl: ITextParagraphFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClone() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTab<Impl: ITextParagraphFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, position: *mut f32, align: *mut TabAlignment, leader: *mut TabLeader) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetTab(index, ::core::mem::transmute_copy(&position), ::core::mem::transmute_copy(&align), ::core::mem::transmute_copy(&leader)).into()
        }
        unsafe extern "system" fn IsEqual<Impl: ITextParagraphFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEqual(&*(&format as *const <ITextParagraphFormat as ::windows::core::Abi>::Abi as *const <ITextParagraphFormat as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClone<Impl: ITextParagraphFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClone(&*(&format as *const <ITextParagraphFormat as ::windows::core::Abi>::Abi as *const <ITextParagraphFormat as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetIndents<Impl: ITextParagraphFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, start: f32, left: f32, right: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIndents(start, left, right).into()
        }
        unsafe extern "system" fn SetLineSpacing<Impl: ITextParagraphFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rule: LineSpacingRule, spacing: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLineSpacing(rule, spacing).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ITextParagraphFormat>,
            ::windows::core::GetTrustLevel,
            Alignment::<Impl, OFFSET>,
            SetAlignment::<Impl, OFFSET>,
            FirstLineIndent::<Impl, OFFSET>,
            KeepTogether::<Impl, OFFSET>,
            SetKeepTogether::<Impl, OFFSET>,
            KeepWithNext::<Impl, OFFSET>,
            SetKeepWithNext::<Impl, OFFSET>,
            LeftIndent::<Impl, OFFSET>,
            LineSpacing::<Impl, OFFSET>,
            LineSpacingRule::<Impl, OFFSET>,
            ListAlignment::<Impl, OFFSET>,
            SetListAlignment::<Impl, OFFSET>,
            ListLevelIndex::<Impl, OFFSET>,
            SetListLevelIndex::<Impl, OFFSET>,
            ListStart::<Impl, OFFSET>,
            SetListStart::<Impl, OFFSET>,
            ListStyle::<Impl, OFFSET>,
            SetListStyle::<Impl, OFFSET>,
            ListTab::<Impl, OFFSET>,
            SetListTab::<Impl, OFFSET>,
            ListType::<Impl, OFFSET>,
            SetListType::<Impl, OFFSET>,
            NoLineNumber::<Impl, OFFSET>,
            SetNoLineNumber::<Impl, OFFSET>,
            PageBreakBefore::<Impl, OFFSET>,
            SetPageBreakBefore::<Impl, OFFSET>,
            RightIndent::<Impl, OFFSET>,
            SetRightIndent::<Impl, OFFSET>,
            RightToLeft::<Impl, OFFSET>,
            SetRightToLeft::<Impl, OFFSET>,
            Style::<Impl, OFFSET>,
            SetStyle::<Impl, OFFSET>,
            SpaceAfter::<Impl, OFFSET>,
            SetSpaceAfter::<Impl, OFFSET>,
            SpaceBefore::<Impl, OFFSET>,
            SetSpaceBefore::<Impl, OFFSET>,
            WidowControl::<Impl, OFFSET>,
            SetWidowControl::<Impl, OFFSET>,
            TabCount::<Impl, OFFSET>,
            AddTab::<Impl, OFFSET>,
            ClearAllTabs::<Impl, OFFSET>,
            DeleteTab::<Impl, OFFSET>,
            GetClone::<Impl, OFFSET>,
            GetTab::<Impl, OFFSET>,
            IsEqual::<Impl, OFFSET>,
            SetClone::<Impl, OFFSET>,
            SetIndents::<Impl, OFFSET>,
            SetLineSpacing::<Impl, OFFSET>,
        )
    }
}
pub trait ITextRangeImpl: Sized {
    fn Character(&self) -> ::windows::core::Result<u16>;
    fn SetCharacter(&self, value: u16) -> ::windows::core::Result<()>;
    fn CharacterFormat(&self) -> ::windows::core::Result<ITextCharacterFormat>;
    fn SetCharacterFormat(&self, value: &::core::option::Option<ITextCharacterFormat>) -> ::windows::core::Result<()>;
    fn FormattedText(&self) -> ::windows::core::Result<ITextRange>;
    fn SetFormattedText(&self, value: &::core::option::Option<ITextRange>) -> ::windows::core::Result<()>;
    fn EndPosition(&self) -> ::windows::core::Result<i32>;
    fn SetEndPosition(&self, value: i32) -> ::windows::core::Result<()>;
    fn Gravity(&self) -> ::windows::core::Result<RangeGravity>;
    fn SetGravity(&self, value: RangeGravity) -> ::windows::core::Result<()>;
    fn Length(&self) -> ::windows::core::Result<i32>;
    fn Link(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLink(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ParagraphFormat(&self) -> ::windows::core::Result<ITextParagraphFormat>;
    fn SetParagraphFormat(&self, value: &::core::option::Option<ITextParagraphFormat>) -> ::windows::core::Result<()>;
    fn StartPosition(&self) -> ::windows::core::Result<i32>;
    fn SetStartPosition(&self, value: i32) -> ::windows::core::Result<()>;
    fn StoryLength(&self) -> ::windows::core::Result<i32>;
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn CanPaste(&self, format: i32) -> ::windows::core::Result<bool>;
    fn ChangeCase(&self, value: LetterCase) -> ::windows::core::Result<()>;
    fn Collapse(&self, value: bool) -> ::windows::core::Result<()>;
    fn Copy(&self) -> ::windows::core::Result<()>;
    fn Cut(&self) -> ::windows::core::Result<()>;
    fn Delete(&self, unit: TextRangeUnit, count: i32) -> ::windows::core::Result<i32>;
    fn EndOf(&self, unit: TextRangeUnit, extend: bool) -> ::windows::core::Result<i32>;
    fn Expand(&self, unit: TextRangeUnit) -> ::windows::core::Result<i32>;
    fn FindText(&self, value: &::windows::core::HSTRING, scanlength: i32, options: FindOptions) -> ::windows::core::Result<i32>;
    fn GetCharacterUtf32(&self, value: &mut u32, offset: i32) -> ::windows::core::Result<()>;
    fn GetClone(&self) -> ::windows::core::Result<ITextRange>;
    fn GetIndex(&self, unit: TextRangeUnit) -> ::windows::core::Result<i32>;
    fn GetPoint(&self, horizontalalign: HorizontalCharacterAlignment, verticalalign: VerticalCharacterAlignment, options: PointOptions, point: &mut super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn GetRect(&self, options: PointOptions, rect: &mut super::super::Foundation::Rect, hit: &mut i32) -> ::windows::core::Result<()>;
    fn GetText(&self, options: TextGetOptions, value: &mut ::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn GetTextViaStream(&self, options: TextGetOptions, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<()>;
    fn InRange(&self, range: &::core::option::Option<ITextRange>) -> ::windows::core::Result<bool>;
    fn InsertImage(&self, width: i32, height: i32, ascent: i32, verticalalign: VerticalCharacterAlignment, alternatetext: &::windows::core::HSTRING, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<()>;
    fn InStory(&self, range: &::core::option::Option<ITextRange>) -> ::windows::core::Result<bool>;
    fn IsEqual(&self, range: &::core::option::Option<ITextRange>) -> ::windows::core::Result<bool>;
    fn Move(&self, unit: TextRangeUnit, count: i32) -> ::windows::core::Result<i32>;
    fn MoveEnd(&self, unit: TextRangeUnit, count: i32) -> ::windows::core::Result<i32>;
    fn MoveStart(&self, unit: TextRangeUnit, count: i32) -> ::windows::core::Result<i32>;
    fn Paste(&self, format: i32) -> ::windows::core::Result<()>;
    fn ScrollIntoView(&self, value: PointOptions) -> ::windows::core::Result<()>;
    fn MatchSelection(&self) -> ::windows::core::Result<()>;
    fn SetIndex(&self, unit: TextRangeUnit, index: i32, extend: bool) -> ::windows::core::Result<()>;
    fn SetPoint(&self, point: &super::super::Foundation::Point, options: PointOptions, extend: bool) -> ::windows::core::Result<()>;
    fn SetRange(&self, startposition: i32, endposition: i32) -> ::windows::core::Result<()>;
    fn SetText(&self, options: TextSetOptions, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetTextViaStream(&self, options: TextSetOptions, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<()>;
    fn StartOf(&self, unit: TextRangeUnit, extend: bool) -> ::windows::core::Result<i32>;
}
impl ::windows::core::RuntimeName for ITextRange {
    const NAME: &'static str = "Windows.UI.Text.ITextRange";
}
impl ITextRangeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextRangeImpl, const OFFSET: isize>() -> ITextRangeVtbl {
        unsafe extern "system" fn Character<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Character() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCharacter<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCharacter(value).into()
        }
        unsafe extern "system" fn CharacterFormat<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CharacterFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCharacterFormat<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCharacterFormat(&*(&value as *const <ITextCharacterFormat as ::windows::core::Abi>::Abi as *const <ITextCharacterFormat as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FormattedText<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FormattedText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormattedText<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFormattedText(&*(&value as *const <ITextRange as ::windows::core::Abi>::Abi as *const <ITextRange as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EndPosition<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEndPosition<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEndPosition(value).into()
        }
        unsafe extern "system" fn Gravity<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut RangeGravity) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Gravity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGravity<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: RangeGravity) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGravity(value).into()
        }
        unsafe extern "system" fn Length<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Length() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Link<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Link() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLink<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLink(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ParagraphFormat<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ParagraphFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParagraphFormat<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetParagraphFormat(&*(&value as *const <ITextParagraphFormat as ::windows::core::Abi>::Abi as *const <ITextParagraphFormat as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StartPosition<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartPosition<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStartPosition(value).into()
        }
        unsafe extern "system" fn StoryLength<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StoryLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Text<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Text() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetText<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CanPaste<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: i32, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanPaste(format) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChangeCase<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: LetterCase) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ChangeCase(value).into()
        }
        unsafe extern "system" fn Collapse<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Collapse(value).into()
        }
        unsafe extern "system" fn Copy<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Copy().into()
        }
        unsafe extern "system" fn Cut<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Cut().into()
        }
        unsafe extern "system" fn Delete<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, count: i32, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Delete(unit, count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndOf<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, extend: bool, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndOf(unit, extend) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Expand<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Expand(unit) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindText<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, scanlength: i32, options: FindOptions, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), scanlength, options) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCharacterUtf32<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut u32, offset: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCharacterUtf32(::core::mem::transmute_copy(&value), offset).into()
        }
        unsafe extern "system" fn GetClone<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClone() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIndex<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIndex(unit) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPoint<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalalign: HorizontalCharacterAlignment, verticalalign: VerticalCharacterAlignment, options: PointOptions, point: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPoint(horizontalalign, verticalalign, options, ::core::mem::transmute_copy(&point)).into()
        }
        unsafe extern "system" fn GetRect<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: PointOptions, rect: *mut super::super::Foundation::Rect, hit: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRect(options, ::core::mem::transmute_copy(&rect), ::core::mem::transmute_copy(&hit)).into()
        }
        unsafe extern "system" fn GetText<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: TextGetOptions, value: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetText(options, ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetTextViaStream<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: TextGetOptions, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetTextViaStream(options, &*(&value as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InRange<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, range: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InRange(&*(&range as *const <ITextRange as ::windows::core::Abi>::Abi as *const <ITextRange as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertImage<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: i32, height: i32, ascent: i32, verticalalign: VerticalCharacterAlignment, alternatetext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertImage(width, height, ascent, verticalalign, &*(&alternatetext as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InStory<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, range: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InStory(&*(&range as *const <ITextRange as ::windows::core::Abi>::Abi as *const <ITextRange as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqual<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, range: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEqual(&*(&range as *const <ITextRange as ::windows::core::Abi>::Abi as *const <ITextRange as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Move<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, count: i32, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Move(unit, count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveEnd<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, count: i32, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveEnd(unit, count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveStart<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, count: i32, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveStart(unit, count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Paste<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Paste(format).into()
        }
        unsafe extern "system" fn ScrollIntoView<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PointOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ScrollIntoView(value).into()
        }
        unsafe extern "system" fn MatchSelection<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MatchSelection().into()
        }
        unsafe extern "system" fn SetIndex<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, index: i32, extend: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIndex(unit, index, extend).into()
        }
        unsafe extern "system" fn SetPoint<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, point: super::super::Foundation::Point, options: PointOptions, extend: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPoint(&*(&point as *const <super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType), options, extend).into()
        }
        unsafe extern "system" fn SetRange<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startposition: i32, endposition: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRange(startposition, endposition).into()
        }
        unsafe extern "system" fn SetText<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: TextSetOptions, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetText(options, &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetTextViaStream<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: TextSetOptions, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTextViaStream(options, &*(&value as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StartOf<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, extend: bool, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartOf(unit, extend) {
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
            ::windows::core::GetRuntimeClassName::<ITextRange>,
            ::windows::core::GetTrustLevel,
            Character::<Impl, OFFSET>,
            SetCharacter::<Impl, OFFSET>,
            CharacterFormat::<Impl, OFFSET>,
            SetCharacterFormat::<Impl, OFFSET>,
            FormattedText::<Impl, OFFSET>,
            SetFormattedText::<Impl, OFFSET>,
            EndPosition::<Impl, OFFSET>,
            SetEndPosition::<Impl, OFFSET>,
            Gravity::<Impl, OFFSET>,
            SetGravity::<Impl, OFFSET>,
            Length::<Impl, OFFSET>,
            Link::<Impl, OFFSET>,
            SetLink::<Impl, OFFSET>,
            ParagraphFormat::<Impl, OFFSET>,
            SetParagraphFormat::<Impl, OFFSET>,
            StartPosition::<Impl, OFFSET>,
            SetStartPosition::<Impl, OFFSET>,
            StoryLength::<Impl, OFFSET>,
            Text::<Impl, OFFSET>,
            SetText::<Impl, OFFSET>,
            CanPaste::<Impl, OFFSET>,
            ChangeCase::<Impl, OFFSET>,
            Collapse::<Impl, OFFSET>,
            Copy::<Impl, OFFSET>,
            Cut::<Impl, OFFSET>,
            Delete::<Impl, OFFSET>,
            EndOf::<Impl, OFFSET>,
            Expand::<Impl, OFFSET>,
            FindText::<Impl, OFFSET>,
            GetCharacterUtf32::<Impl, OFFSET>,
            GetClone::<Impl, OFFSET>,
            GetIndex::<Impl, OFFSET>,
            GetPoint::<Impl, OFFSET>,
            GetRect::<Impl, OFFSET>,
            GetText::<Impl, OFFSET>,
            GetTextViaStream::<Impl, OFFSET>,
            InRange::<Impl, OFFSET>,
            InsertImage::<Impl, OFFSET>,
            InStory::<Impl, OFFSET>,
            IsEqual::<Impl, OFFSET>,
            Move::<Impl, OFFSET>,
            MoveEnd::<Impl, OFFSET>,
            MoveStart::<Impl, OFFSET>,
            Paste::<Impl, OFFSET>,
            ScrollIntoView::<Impl, OFFSET>,
            MatchSelection::<Impl, OFFSET>,
            SetIndex::<Impl, OFFSET>,
            SetPoint::<Impl, OFFSET>,
            SetRange::<Impl, OFFSET>,
            SetText::<Impl, OFFSET>,
            SetTextViaStream::<Impl, OFFSET>,
            StartOf::<Impl, OFFSET>,
        )
    }
}
pub trait ITextSelectionImpl: Sized + ITextRangeImpl {
    fn Options(&self) -> ::windows::core::Result<SelectionOptions>;
    fn SetOptions(&self, value: SelectionOptions) -> ::windows::core::Result<()>;
    fn Type(&self) -> ::windows::core::Result<SelectionType>;
    fn EndKey(&self, unit: TextRangeUnit, extend: bool) -> ::windows::core::Result<i32>;
    fn HomeKey(&self, unit: TextRangeUnit, extend: bool) -> ::windows::core::Result<i32>;
    fn MoveDown(&self, unit: TextRangeUnit, count: i32, extend: bool) -> ::windows::core::Result<i32>;
    fn MoveLeft(&self, unit: TextRangeUnit, count: i32, extend: bool) -> ::windows::core::Result<i32>;
    fn MoveRight(&self, unit: TextRangeUnit, count: i32, extend: bool) -> ::windows::core::Result<i32>;
    fn MoveUp(&self, unit: TextRangeUnit, count: i32, extend: bool) -> ::windows::core::Result<i32>;
    fn TypeText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ITextSelection {
    const NAME: &'static str = "Windows.UI.Text.ITextSelection";
}
impl ITextSelectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextSelectionImpl, const OFFSET: isize>() -> ITextSelectionVtbl {
        unsafe extern "system" fn Options<Impl: ITextSelectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SelectionOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Options() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOptions<Impl: ITextSelectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: SelectionOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOptions(value).into()
        }
        unsafe extern "system" fn Type<Impl: ITextSelectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SelectionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndKey<Impl: ITextSelectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, extend: bool, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndKey(unit, extend) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HomeKey<Impl: ITextSelectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, extend: bool, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HomeKey(unit, extend) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveDown<Impl: ITextSelectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, count: i32, extend: bool, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveDown(unit, count, extend) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveLeft<Impl: ITextSelectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, count: i32, extend: bool, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveLeft(unit, count, extend) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveRight<Impl: ITextSelectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, count: i32, extend: bool, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveRight(unit, count, extend) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveUp<Impl: ITextSelectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, count: i32, extend: bool, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveUp(unit, count, extend) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TypeText<Impl: ITextSelectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TypeText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ITextSelection>,
            ::windows::core::GetTrustLevel,
            Options::<Impl, OFFSET>,
            SetOptions::<Impl, OFFSET>,
            Type::<Impl, OFFSET>,
            EndKey::<Impl, OFFSET>,
            HomeKey::<Impl, OFFSET>,
            MoveDown::<Impl, OFFSET>,
            MoveLeft::<Impl, OFFSET>,
            MoveRight::<Impl, OFFSET>,
            MoveUp::<Impl, OFFSET>,
            TypeText::<Impl, OFFSET>,
        )
    }
}
