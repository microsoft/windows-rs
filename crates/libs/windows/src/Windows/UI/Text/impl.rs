#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IContentLinkInfoImpl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<u32>;
    fn SetId(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn DisplayText(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayText(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SecondaryText(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSecondaryText(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Uri(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetUri(&mut self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn LinkContentKind(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLinkContentKind(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IContentLinkInfo {
    const NAME: &'static str = "Windows.UI.Text.IContentLinkInfo";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IContentLinkInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContentLinkInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContentLinkInfoVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IContentLinkInfo, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            SetId: SetId::<Impl, IMPL_OFFSET>,
            DisplayText: DisplayText::<Impl, IMPL_OFFSET>,
            SetDisplayText: SetDisplayText::<Impl, IMPL_OFFSET>,
            SecondaryText: SecondaryText::<Impl, IMPL_OFFSET>,
            SetSecondaryText: SetSecondaryText::<Impl, IMPL_OFFSET>,
            Uri: Uri::<Impl, IMPL_OFFSET>,
            SetUri: SetUri::<Impl, IMPL_OFFSET>,
            LinkContentKind: LinkContentKind::<Impl, IMPL_OFFSET>,
            SetLinkContentKind: SetLinkContentKind::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContentLinkInfo as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFontWeightsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFontWeightsVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IFontWeights, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFontWeights as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFontWeightsStaticsImpl: Sized {
    fn Black(&mut self) -> ::windows::core::Result<FontWeight>;
    fn Bold(&mut self) -> ::windows::core::Result<FontWeight>;
    fn ExtraBlack(&mut self) -> ::windows::core::Result<FontWeight>;
    fn ExtraBold(&mut self) -> ::windows::core::Result<FontWeight>;
    fn ExtraLight(&mut self) -> ::windows::core::Result<FontWeight>;
    fn Light(&mut self) -> ::windows::core::Result<FontWeight>;
    fn Medium(&mut self) -> ::windows::core::Result<FontWeight>;
    fn Normal(&mut self) -> ::windows::core::Result<FontWeight>;
    fn SemiBold(&mut self) -> ::windows::core::Result<FontWeight>;
    fn SemiLight(&mut self) -> ::windows::core::Result<FontWeight>;
    fn Thin(&mut self) -> ::windows::core::Result<FontWeight>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFontWeightsStatics {
    const NAME: &'static str = "Windows.UI.Text.IFontWeightsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IFontWeightsStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFontWeightsStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFontWeightsStaticsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFontWeightsStatics, BASE_OFFSET>(),
            Black: Black::<Impl, IMPL_OFFSET>,
            Bold: Bold::<Impl, IMPL_OFFSET>,
            ExtraBlack: ExtraBlack::<Impl, IMPL_OFFSET>,
            ExtraBold: ExtraBold::<Impl, IMPL_OFFSET>,
            ExtraLight: ExtraLight::<Impl, IMPL_OFFSET>,
            Light: Light::<Impl, IMPL_OFFSET>,
            Medium: Medium::<Impl, IMPL_OFFSET>,
            Normal: Normal::<Impl, IMPL_OFFSET>,
            SemiBold: SemiBold::<Impl, IMPL_OFFSET>,
            SemiLight: SemiLight::<Impl, IMPL_OFFSET>,
            Thin: Thin::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFontWeightsStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRichEditTextRangeImpl: Sized {
    fn ContentLinkInfo(&mut self) -> ::windows::core::Result<ContentLinkInfo>;
    fn SetContentLinkInfo(&mut self, value: &::core::option::Option<ContentLinkInfo>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRichEditTextRange {
    const NAME: &'static str = "Windows.UI.Text.IRichEditTextRange";
}
#[cfg(feature = "implement_exclusive")]
impl IRichEditTextRangeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRichEditTextRangeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRichEditTextRangeVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRichEditTextRange, BASE_OFFSET>(),
            ContentLinkInfo: ContentLinkInfo::<Impl, IMPL_OFFSET>,
            SetContentLinkInfo: SetContentLinkInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRichEditTextRange as ::windows::core::Interface>::IID
    }
}
pub trait ITextCharacterFormatImpl: Sized {
    fn AllCaps(&mut self) -> ::windows::core::Result<FormatEffect>;
    fn SetAllCaps(&mut self, value: FormatEffect) -> ::windows::core::Result<()>;
    fn BackgroundColor(&mut self) -> ::windows::core::Result<super::Color>;
    fn SetBackgroundColor(&mut self, value: &super::Color) -> ::windows::core::Result<()>;
    fn Bold(&mut self) -> ::windows::core::Result<FormatEffect>;
    fn SetBold(&mut self, value: FormatEffect) -> ::windows::core::Result<()>;
    fn FontStretch(&mut self) -> ::windows::core::Result<FontStretch>;
    fn SetFontStretch(&mut self, value: FontStretch) -> ::windows::core::Result<()>;
    fn FontStyle(&mut self) -> ::windows::core::Result<FontStyle>;
    fn SetFontStyle(&mut self, value: FontStyle) -> ::windows::core::Result<()>;
    fn ForegroundColor(&mut self) -> ::windows::core::Result<super::Color>;
    fn SetForegroundColor(&mut self, value: &super::Color) -> ::windows::core::Result<()>;
    fn Hidden(&mut self) -> ::windows::core::Result<FormatEffect>;
    fn SetHidden(&mut self, value: FormatEffect) -> ::windows::core::Result<()>;
    fn Italic(&mut self) -> ::windows::core::Result<FormatEffect>;
    fn SetItalic(&mut self, value: FormatEffect) -> ::windows::core::Result<()>;
    fn Kerning(&mut self) -> ::windows::core::Result<f32>;
    fn SetKerning(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn LanguageTag(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLanguageTag(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn LinkType(&mut self) -> ::windows::core::Result<LinkType>;
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Outline(&mut self) -> ::windows::core::Result<FormatEffect>;
    fn SetOutline(&mut self, value: FormatEffect) -> ::windows::core::Result<()>;
    fn Position(&mut self) -> ::windows::core::Result<f32>;
    fn SetPosition(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn ProtectedText(&mut self) -> ::windows::core::Result<FormatEffect>;
    fn SetProtectedText(&mut self, value: FormatEffect) -> ::windows::core::Result<()>;
    fn Size(&mut self) -> ::windows::core::Result<f32>;
    fn SetSize(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn SmallCaps(&mut self) -> ::windows::core::Result<FormatEffect>;
    fn SetSmallCaps(&mut self, value: FormatEffect) -> ::windows::core::Result<()>;
    fn Spacing(&mut self) -> ::windows::core::Result<f32>;
    fn SetSpacing(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn Strikethrough(&mut self) -> ::windows::core::Result<FormatEffect>;
    fn SetStrikethrough(&mut self, value: FormatEffect) -> ::windows::core::Result<()>;
    fn Subscript(&mut self) -> ::windows::core::Result<FormatEffect>;
    fn SetSubscript(&mut self, value: FormatEffect) -> ::windows::core::Result<()>;
    fn Superscript(&mut self) -> ::windows::core::Result<FormatEffect>;
    fn SetSuperscript(&mut self, value: FormatEffect) -> ::windows::core::Result<()>;
    fn TextScript(&mut self) -> ::windows::core::Result<TextScript>;
    fn SetTextScript(&mut self, value: TextScript) -> ::windows::core::Result<()>;
    fn Underline(&mut self) -> ::windows::core::Result<UnderlineType>;
    fn SetUnderline(&mut self, value: UnderlineType) -> ::windows::core::Result<()>;
    fn Weight(&mut self) -> ::windows::core::Result<i32>;
    fn SetWeight(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn SetClone(&mut self, value: &::core::option::Option<ITextCharacterFormat>) -> ::windows::core::Result<()>;
    fn GetClone(&mut self) -> ::windows::core::Result<ITextCharacterFormat>;
    fn IsEqual(&mut self, format: &::core::option::Option<ITextCharacterFormat>) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for ITextCharacterFormat {
    const NAME: &'static str = "Windows.UI.Text.ITextCharacterFormat";
}
impl ITextCharacterFormatVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextCharacterFormatImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextCharacterFormatVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITextCharacterFormat, BASE_OFFSET>(),
            AllCaps: AllCaps::<Impl, IMPL_OFFSET>,
            SetAllCaps: SetAllCaps::<Impl, IMPL_OFFSET>,
            BackgroundColor: BackgroundColor::<Impl, IMPL_OFFSET>,
            SetBackgroundColor: SetBackgroundColor::<Impl, IMPL_OFFSET>,
            Bold: Bold::<Impl, IMPL_OFFSET>,
            SetBold: SetBold::<Impl, IMPL_OFFSET>,
            FontStretch: FontStretch::<Impl, IMPL_OFFSET>,
            SetFontStretch: SetFontStretch::<Impl, IMPL_OFFSET>,
            FontStyle: FontStyle::<Impl, IMPL_OFFSET>,
            SetFontStyle: SetFontStyle::<Impl, IMPL_OFFSET>,
            ForegroundColor: ForegroundColor::<Impl, IMPL_OFFSET>,
            SetForegroundColor: SetForegroundColor::<Impl, IMPL_OFFSET>,
            Hidden: Hidden::<Impl, IMPL_OFFSET>,
            SetHidden: SetHidden::<Impl, IMPL_OFFSET>,
            Italic: Italic::<Impl, IMPL_OFFSET>,
            SetItalic: SetItalic::<Impl, IMPL_OFFSET>,
            Kerning: Kerning::<Impl, IMPL_OFFSET>,
            SetKerning: SetKerning::<Impl, IMPL_OFFSET>,
            LanguageTag: LanguageTag::<Impl, IMPL_OFFSET>,
            SetLanguageTag: SetLanguageTag::<Impl, IMPL_OFFSET>,
            LinkType: LinkType::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
            Outline: Outline::<Impl, IMPL_OFFSET>,
            SetOutline: SetOutline::<Impl, IMPL_OFFSET>,
            Position: Position::<Impl, IMPL_OFFSET>,
            SetPosition: SetPosition::<Impl, IMPL_OFFSET>,
            ProtectedText: ProtectedText::<Impl, IMPL_OFFSET>,
            SetProtectedText: SetProtectedText::<Impl, IMPL_OFFSET>,
            Size: Size::<Impl, IMPL_OFFSET>,
            SetSize: SetSize::<Impl, IMPL_OFFSET>,
            SmallCaps: SmallCaps::<Impl, IMPL_OFFSET>,
            SetSmallCaps: SetSmallCaps::<Impl, IMPL_OFFSET>,
            Spacing: Spacing::<Impl, IMPL_OFFSET>,
            SetSpacing: SetSpacing::<Impl, IMPL_OFFSET>,
            Strikethrough: Strikethrough::<Impl, IMPL_OFFSET>,
            SetStrikethrough: SetStrikethrough::<Impl, IMPL_OFFSET>,
            Subscript: Subscript::<Impl, IMPL_OFFSET>,
            SetSubscript: SetSubscript::<Impl, IMPL_OFFSET>,
            Superscript: Superscript::<Impl, IMPL_OFFSET>,
            SetSuperscript: SetSuperscript::<Impl, IMPL_OFFSET>,
            TextScript: TextScript::<Impl, IMPL_OFFSET>,
            SetTextScript: SetTextScript::<Impl, IMPL_OFFSET>,
            Underline: Underline::<Impl, IMPL_OFFSET>,
            SetUnderline: SetUnderline::<Impl, IMPL_OFFSET>,
            Weight: Weight::<Impl, IMPL_OFFSET>,
            SetWeight: SetWeight::<Impl, IMPL_OFFSET>,
            SetClone: SetClone::<Impl, IMPL_OFFSET>,
            GetClone: GetClone::<Impl, IMPL_OFFSET>,
            IsEqual: IsEqual::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextCharacterFormat as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextConstantsStaticsImpl: Sized {
    fn AutoColor(&mut self) -> ::windows::core::Result<super::Color>;
    fn MinUnitCount(&mut self) -> ::windows::core::Result<i32>;
    fn MaxUnitCount(&mut self) -> ::windows::core::Result<i32>;
    fn UndefinedColor(&mut self) -> ::windows::core::Result<super::Color>;
    fn UndefinedFloatValue(&mut self) -> ::windows::core::Result<f32>;
    fn UndefinedInt32Value(&mut self) -> ::windows::core::Result<i32>;
    fn UndefinedFontStretch(&mut self) -> ::windows::core::Result<FontStretch>;
    fn UndefinedFontStyle(&mut self) -> ::windows::core::Result<FontStyle>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITextConstantsStatics {
    const NAME: &'static str = "Windows.UI.Text.ITextConstantsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ITextConstantsStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextConstantsStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextConstantsStaticsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITextConstantsStatics, BASE_OFFSET>(),
            AutoColor: AutoColor::<Impl, IMPL_OFFSET>,
            MinUnitCount: MinUnitCount::<Impl, IMPL_OFFSET>,
            MaxUnitCount: MaxUnitCount::<Impl, IMPL_OFFSET>,
            UndefinedColor: UndefinedColor::<Impl, IMPL_OFFSET>,
            UndefinedFloatValue: UndefinedFloatValue::<Impl, IMPL_OFFSET>,
            UndefinedInt32Value: UndefinedInt32Value::<Impl, IMPL_OFFSET>,
            UndefinedFontStretch: UndefinedFontStretch::<Impl, IMPL_OFFSET>,
            UndefinedFontStyle: UndefinedFontStyle::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextConstantsStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
pub trait ITextDocumentImpl: Sized {
    fn CaretType(&mut self) -> ::windows::core::Result<CaretType>;
    fn SetCaretType(&mut self, value: CaretType) -> ::windows::core::Result<()>;
    fn DefaultTabStop(&mut self) -> ::windows::core::Result<f32>;
    fn SetDefaultTabStop(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn Selection(&mut self) -> ::windows::core::Result<ITextSelection>;
    fn UndoLimit(&mut self) -> ::windows::core::Result<u32>;
    fn SetUndoLimit(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn CanCopy(&mut self) -> ::windows::core::Result<bool>;
    fn CanPaste(&mut self) -> ::windows::core::Result<bool>;
    fn CanRedo(&mut self) -> ::windows::core::Result<bool>;
    fn CanUndo(&mut self) -> ::windows::core::Result<bool>;
    fn ApplyDisplayUpdates(&mut self) -> ::windows::core::Result<i32>;
    fn BatchDisplayUpdates(&mut self) -> ::windows::core::Result<i32>;
    fn BeginUndoGroup(&mut self) -> ::windows::core::Result<()>;
    fn EndUndoGroup(&mut self) -> ::windows::core::Result<()>;
    fn GetDefaultCharacterFormat(&mut self) -> ::windows::core::Result<ITextCharacterFormat>;
    fn GetDefaultParagraphFormat(&mut self) -> ::windows::core::Result<ITextParagraphFormat>;
    fn GetRange(&mut self, startposition: i32, endposition: i32) -> ::windows::core::Result<ITextRange>;
    fn GetRangeFromPoint(&mut self, point: &super::super::Foundation::Point, options: PointOptions) -> ::windows::core::Result<ITextRange>;
    fn GetText(&mut self, options: TextGetOptions, value: &mut ::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn LoadFromStream(&mut self, options: TextSetOptions, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<()>;
    fn Redo(&mut self) -> ::windows::core::Result<()>;
    fn SaveToStream(&mut self, options: TextGetOptions, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<()>;
    fn SetDefaultCharacterFormat(&mut self, value: &::core::option::Option<ITextCharacterFormat>) -> ::windows::core::Result<()>;
    fn SetDefaultParagraphFormat(&mut self, value: &::core::option::Option<ITextParagraphFormat>) -> ::windows::core::Result<()>;
    fn SetText(&mut self, options: TextSetOptions, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Undo(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ::windows::core::RuntimeName for ITextDocument {
    const NAME: &'static str = "Windows.UI.Text.ITextDocument";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ITextDocumentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocumentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextDocumentVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITextDocument, BASE_OFFSET>(),
            CaretType: CaretType::<Impl, IMPL_OFFSET>,
            SetCaretType: SetCaretType::<Impl, IMPL_OFFSET>,
            DefaultTabStop: DefaultTabStop::<Impl, IMPL_OFFSET>,
            SetDefaultTabStop: SetDefaultTabStop::<Impl, IMPL_OFFSET>,
            Selection: Selection::<Impl, IMPL_OFFSET>,
            UndoLimit: UndoLimit::<Impl, IMPL_OFFSET>,
            SetUndoLimit: SetUndoLimit::<Impl, IMPL_OFFSET>,
            CanCopy: CanCopy::<Impl, IMPL_OFFSET>,
            CanPaste: CanPaste::<Impl, IMPL_OFFSET>,
            CanRedo: CanRedo::<Impl, IMPL_OFFSET>,
            CanUndo: CanUndo::<Impl, IMPL_OFFSET>,
            ApplyDisplayUpdates: ApplyDisplayUpdates::<Impl, IMPL_OFFSET>,
            BatchDisplayUpdates: BatchDisplayUpdates::<Impl, IMPL_OFFSET>,
            BeginUndoGroup: BeginUndoGroup::<Impl, IMPL_OFFSET>,
            EndUndoGroup: EndUndoGroup::<Impl, IMPL_OFFSET>,
            GetDefaultCharacterFormat: GetDefaultCharacterFormat::<Impl, IMPL_OFFSET>,
            GetDefaultParagraphFormat: GetDefaultParagraphFormat::<Impl, IMPL_OFFSET>,
            GetRange: GetRange::<Impl, IMPL_OFFSET>,
            GetRangeFromPoint: GetRangeFromPoint::<Impl, IMPL_OFFSET>,
            GetText: GetText::<Impl, IMPL_OFFSET>,
            LoadFromStream: LoadFromStream::<Impl, IMPL_OFFSET>,
            Redo: Redo::<Impl, IMPL_OFFSET>,
            SaveToStream: SaveToStream::<Impl, IMPL_OFFSET>,
            SetDefaultCharacterFormat: SetDefaultCharacterFormat::<Impl, IMPL_OFFSET>,
            SetDefaultParagraphFormat: SetDefaultParagraphFormat::<Impl, IMPL_OFFSET>,
            SetText: SetText::<Impl, IMPL_OFFSET>,
            Undo: Undo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextDocument as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextDocument2Impl: Sized {
    fn AlignmentIncludesTrailingWhitespace(&mut self) -> ::windows::core::Result<bool>;
    fn SetAlignmentIncludesTrailingWhitespace(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IgnoreTrailingCharacterSpacing(&mut self) -> ::windows::core::Result<bool>;
    fn SetIgnoreTrailingCharacterSpacing(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITextDocument2 {
    const NAME: &'static str = "Windows.UI.Text.ITextDocument2";
}
#[cfg(feature = "implement_exclusive")]
impl ITextDocument2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextDocument2Vtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITextDocument2, BASE_OFFSET>(),
            AlignmentIncludesTrailingWhitespace: AlignmentIncludesTrailingWhitespace::<Impl, IMPL_OFFSET>,
            SetAlignmentIncludesTrailingWhitespace: SetAlignmentIncludesTrailingWhitespace::<Impl, IMPL_OFFSET>,
            IgnoreTrailingCharacterSpacing: IgnoreTrailingCharacterSpacing::<Impl, IMPL_OFFSET>,
            SetIgnoreTrailingCharacterSpacing: SetIgnoreTrailingCharacterSpacing::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextDocument2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextDocument3Impl: Sized {
    fn ClearUndoRedoHistory(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITextDocument3 {
    const NAME: &'static str = "Windows.UI.Text.ITextDocument3";
}
#[cfg(feature = "implement_exclusive")]
impl ITextDocument3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextDocument3Vtbl {
        unsafe extern "system" fn ClearUndoRedoHistory<Impl: ITextDocument3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearUndoRedoHistory().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITextDocument3, BASE_OFFSET>(),
            ClearUndoRedoHistory: ClearUndoRedoHistory::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextDocument3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextDocument4Impl: Sized {
    fn SetMath(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn GetMath(&mut self, value: &mut ::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetMathMode(&mut self, mode: RichEditMathMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITextDocument4 {
    const NAME: &'static str = "Windows.UI.Text.ITextDocument4";
}
#[cfg(feature = "implement_exclusive")]
impl ITextDocument4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextDocument4Vtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITextDocument4, BASE_OFFSET>(),
            SetMath: SetMath::<Impl, IMPL_OFFSET>,
            GetMath: GetMath::<Impl, IMPL_OFFSET>,
            SetMathMode: SetMathMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextDocument4 as ::windows::core::Interface>::IID
    }
}
pub trait ITextParagraphFormatImpl: Sized {
    fn Alignment(&mut self) -> ::windows::core::Result<ParagraphAlignment>;
    fn SetAlignment(&mut self, value: ParagraphAlignment) -> ::windows::core::Result<()>;
    fn FirstLineIndent(&mut self) -> ::windows::core::Result<f32>;
    fn KeepTogether(&mut self) -> ::windows::core::Result<FormatEffect>;
    fn SetKeepTogether(&mut self, value: FormatEffect) -> ::windows::core::Result<()>;
    fn KeepWithNext(&mut self) -> ::windows::core::Result<FormatEffect>;
    fn SetKeepWithNext(&mut self, value: FormatEffect) -> ::windows::core::Result<()>;
    fn LeftIndent(&mut self) -> ::windows::core::Result<f32>;
    fn LineSpacing(&mut self) -> ::windows::core::Result<f32>;
    fn LineSpacingRule(&mut self) -> ::windows::core::Result<LineSpacingRule>;
    fn ListAlignment(&mut self) -> ::windows::core::Result<MarkerAlignment>;
    fn SetListAlignment(&mut self, value: MarkerAlignment) -> ::windows::core::Result<()>;
    fn ListLevelIndex(&mut self) -> ::windows::core::Result<i32>;
    fn SetListLevelIndex(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn ListStart(&mut self) -> ::windows::core::Result<i32>;
    fn SetListStart(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn ListStyle(&mut self) -> ::windows::core::Result<MarkerStyle>;
    fn SetListStyle(&mut self, value: MarkerStyle) -> ::windows::core::Result<()>;
    fn ListTab(&mut self) -> ::windows::core::Result<f32>;
    fn SetListTab(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn ListType(&mut self) -> ::windows::core::Result<MarkerType>;
    fn SetListType(&mut self, value: MarkerType) -> ::windows::core::Result<()>;
    fn NoLineNumber(&mut self) -> ::windows::core::Result<FormatEffect>;
    fn SetNoLineNumber(&mut self, value: FormatEffect) -> ::windows::core::Result<()>;
    fn PageBreakBefore(&mut self) -> ::windows::core::Result<FormatEffect>;
    fn SetPageBreakBefore(&mut self, value: FormatEffect) -> ::windows::core::Result<()>;
    fn RightIndent(&mut self) -> ::windows::core::Result<f32>;
    fn SetRightIndent(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn RightToLeft(&mut self) -> ::windows::core::Result<FormatEffect>;
    fn SetRightToLeft(&mut self, value: FormatEffect) -> ::windows::core::Result<()>;
    fn Style(&mut self) -> ::windows::core::Result<ParagraphStyle>;
    fn SetStyle(&mut self, value: ParagraphStyle) -> ::windows::core::Result<()>;
    fn SpaceAfter(&mut self) -> ::windows::core::Result<f32>;
    fn SetSpaceAfter(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn SpaceBefore(&mut self) -> ::windows::core::Result<f32>;
    fn SetSpaceBefore(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn WidowControl(&mut self) -> ::windows::core::Result<FormatEffect>;
    fn SetWidowControl(&mut self, value: FormatEffect) -> ::windows::core::Result<()>;
    fn TabCount(&mut self) -> ::windows::core::Result<i32>;
    fn AddTab(&mut self, position: f32, align: TabAlignment, leader: TabLeader) -> ::windows::core::Result<()>;
    fn ClearAllTabs(&mut self) -> ::windows::core::Result<()>;
    fn DeleteTab(&mut self, position: f32) -> ::windows::core::Result<()>;
    fn GetClone(&mut self) -> ::windows::core::Result<ITextParagraphFormat>;
    fn GetTab(&mut self, index: i32, position: &mut f32, align: &mut TabAlignment, leader: &mut TabLeader) -> ::windows::core::Result<()>;
    fn IsEqual(&mut self, format: &::core::option::Option<ITextParagraphFormat>) -> ::windows::core::Result<bool>;
    fn SetClone(&mut self, format: &::core::option::Option<ITextParagraphFormat>) -> ::windows::core::Result<()>;
    fn SetIndents(&mut self, start: f32, left: f32, right: f32) -> ::windows::core::Result<()>;
    fn SetLineSpacing(&mut self, rule: LineSpacingRule, spacing: f32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ITextParagraphFormat {
    const NAME: &'static str = "Windows.UI.Text.ITextParagraphFormat";
}
impl ITextParagraphFormatVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextParagraphFormatImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextParagraphFormatVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITextParagraphFormat, BASE_OFFSET>(),
            Alignment: Alignment::<Impl, IMPL_OFFSET>,
            SetAlignment: SetAlignment::<Impl, IMPL_OFFSET>,
            FirstLineIndent: FirstLineIndent::<Impl, IMPL_OFFSET>,
            KeepTogether: KeepTogether::<Impl, IMPL_OFFSET>,
            SetKeepTogether: SetKeepTogether::<Impl, IMPL_OFFSET>,
            KeepWithNext: KeepWithNext::<Impl, IMPL_OFFSET>,
            SetKeepWithNext: SetKeepWithNext::<Impl, IMPL_OFFSET>,
            LeftIndent: LeftIndent::<Impl, IMPL_OFFSET>,
            LineSpacing: LineSpacing::<Impl, IMPL_OFFSET>,
            LineSpacingRule: LineSpacingRule::<Impl, IMPL_OFFSET>,
            ListAlignment: ListAlignment::<Impl, IMPL_OFFSET>,
            SetListAlignment: SetListAlignment::<Impl, IMPL_OFFSET>,
            ListLevelIndex: ListLevelIndex::<Impl, IMPL_OFFSET>,
            SetListLevelIndex: SetListLevelIndex::<Impl, IMPL_OFFSET>,
            ListStart: ListStart::<Impl, IMPL_OFFSET>,
            SetListStart: SetListStart::<Impl, IMPL_OFFSET>,
            ListStyle: ListStyle::<Impl, IMPL_OFFSET>,
            SetListStyle: SetListStyle::<Impl, IMPL_OFFSET>,
            ListTab: ListTab::<Impl, IMPL_OFFSET>,
            SetListTab: SetListTab::<Impl, IMPL_OFFSET>,
            ListType: ListType::<Impl, IMPL_OFFSET>,
            SetListType: SetListType::<Impl, IMPL_OFFSET>,
            NoLineNumber: NoLineNumber::<Impl, IMPL_OFFSET>,
            SetNoLineNumber: SetNoLineNumber::<Impl, IMPL_OFFSET>,
            PageBreakBefore: PageBreakBefore::<Impl, IMPL_OFFSET>,
            SetPageBreakBefore: SetPageBreakBefore::<Impl, IMPL_OFFSET>,
            RightIndent: RightIndent::<Impl, IMPL_OFFSET>,
            SetRightIndent: SetRightIndent::<Impl, IMPL_OFFSET>,
            RightToLeft: RightToLeft::<Impl, IMPL_OFFSET>,
            SetRightToLeft: SetRightToLeft::<Impl, IMPL_OFFSET>,
            Style: Style::<Impl, IMPL_OFFSET>,
            SetStyle: SetStyle::<Impl, IMPL_OFFSET>,
            SpaceAfter: SpaceAfter::<Impl, IMPL_OFFSET>,
            SetSpaceAfter: SetSpaceAfter::<Impl, IMPL_OFFSET>,
            SpaceBefore: SpaceBefore::<Impl, IMPL_OFFSET>,
            SetSpaceBefore: SetSpaceBefore::<Impl, IMPL_OFFSET>,
            WidowControl: WidowControl::<Impl, IMPL_OFFSET>,
            SetWidowControl: SetWidowControl::<Impl, IMPL_OFFSET>,
            TabCount: TabCount::<Impl, IMPL_OFFSET>,
            AddTab: AddTab::<Impl, IMPL_OFFSET>,
            ClearAllTabs: ClearAllTabs::<Impl, IMPL_OFFSET>,
            DeleteTab: DeleteTab::<Impl, IMPL_OFFSET>,
            GetClone: GetClone::<Impl, IMPL_OFFSET>,
            GetTab: GetTab::<Impl, IMPL_OFFSET>,
            IsEqual: IsEqual::<Impl, IMPL_OFFSET>,
            SetClone: SetClone::<Impl, IMPL_OFFSET>,
            SetIndents: SetIndents::<Impl, IMPL_OFFSET>,
            SetLineSpacing: SetLineSpacing::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextParagraphFormat as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
pub trait ITextRangeImpl: Sized {
    fn Character(&mut self) -> ::windows::core::Result<u16>;
    fn SetCharacter(&mut self, value: u16) -> ::windows::core::Result<()>;
    fn CharacterFormat(&mut self) -> ::windows::core::Result<ITextCharacterFormat>;
    fn SetCharacterFormat(&mut self, value: &::core::option::Option<ITextCharacterFormat>) -> ::windows::core::Result<()>;
    fn FormattedText(&mut self) -> ::windows::core::Result<ITextRange>;
    fn SetFormattedText(&mut self, value: &::core::option::Option<ITextRange>) -> ::windows::core::Result<()>;
    fn EndPosition(&mut self) -> ::windows::core::Result<i32>;
    fn SetEndPosition(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn Gravity(&mut self) -> ::windows::core::Result<RangeGravity>;
    fn SetGravity(&mut self, value: RangeGravity) -> ::windows::core::Result<()>;
    fn Length(&mut self) -> ::windows::core::Result<i32>;
    fn Link(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLink(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ParagraphFormat(&mut self) -> ::windows::core::Result<ITextParagraphFormat>;
    fn SetParagraphFormat(&mut self, value: &::core::option::Option<ITextParagraphFormat>) -> ::windows::core::Result<()>;
    fn StartPosition(&mut self) -> ::windows::core::Result<i32>;
    fn SetStartPosition(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn StoryLength(&mut self) -> ::windows::core::Result<i32>;
    fn Text(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetText(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn CanPaste(&mut self, format: i32) -> ::windows::core::Result<bool>;
    fn ChangeCase(&mut self, value: LetterCase) -> ::windows::core::Result<()>;
    fn Collapse(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Copy(&mut self) -> ::windows::core::Result<()>;
    fn Cut(&mut self) -> ::windows::core::Result<()>;
    fn Delete(&mut self, unit: TextRangeUnit, count: i32) -> ::windows::core::Result<i32>;
    fn EndOf(&mut self, unit: TextRangeUnit, extend: bool) -> ::windows::core::Result<i32>;
    fn Expand(&mut self, unit: TextRangeUnit) -> ::windows::core::Result<i32>;
    fn FindText(&mut self, value: &::windows::core::HSTRING, scanlength: i32, options: FindOptions) -> ::windows::core::Result<i32>;
    fn GetCharacterUtf32(&mut self, value: &mut u32, offset: i32) -> ::windows::core::Result<()>;
    fn GetClone(&mut self) -> ::windows::core::Result<ITextRange>;
    fn GetIndex(&mut self, unit: TextRangeUnit) -> ::windows::core::Result<i32>;
    fn GetPoint(&mut self, horizontalalign: HorizontalCharacterAlignment, verticalalign: VerticalCharacterAlignment, options: PointOptions, point: &mut super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn GetRect(&mut self, options: PointOptions, rect: &mut super::super::Foundation::Rect, hit: &mut i32) -> ::windows::core::Result<()>;
    fn GetText(&mut self, options: TextGetOptions, value: &mut ::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn GetTextViaStream(&mut self, options: TextGetOptions, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<()>;
    fn InRange(&mut self, range: &::core::option::Option<ITextRange>) -> ::windows::core::Result<bool>;
    fn InsertImage(&mut self, width: i32, height: i32, ascent: i32, verticalalign: VerticalCharacterAlignment, alternatetext: &::windows::core::HSTRING, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<()>;
    fn InStory(&mut self, range: &::core::option::Option<ITextRange>) -> ::windows::core::Result<bool>;
    fn IsEqual(&mut self, range: &::core::option::Option<ITextRange>) -> ::windows::core::Result<bool>;
    fn Move(&mut self, unit: TextRangeUnit, count: i32) -> ::windows::core::Result<i32>;
    fn MoveEnd(&mut self, unit: TextRangeUnit, count: i32) -> ::windows::core::Result<i32>;
    fn MoveStart(&mut self, unit: TextRangeUnit, count: i32) -> ::windows::core::Result<i32>;
    fn Paste(&mut self, format: i32) -> ::windows::core::Result<()>;
    fn ScrollIntoView(&mut self, value: PointOptions) -> ::windows::core::Result<()>;
    fn MatchSelection(&mut self) -> ::windows::core::Result<()>;
    fn SetIndex(&mut self, unit: TextRangeUnit, index: i32, extend: bool) -> ::windows::core::Result<()>;
    fn SetPoint(&mut self, point: &super::super::Foundation::Point, options: PointOptions, extend: bool) -> ::windows::core::Result<()>;
    fn SetRange(&mut self, startposition: i32, endposition: i32) -> ::windows::core::Result<()>;
    fn SetText(&mut self, options: TextSetOptions, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetTextViaStream(&mut self, options: TextSetOptions, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<()>;
    fn StartOf(&mut self, unit: TextRangeUnit, extend: bool) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ::windows::core::RuntimeName for ITextRange {
    const NAME: &'static str = "Windows.UI.Text.ITextRange";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ITextRangeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextRangeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextRangeVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITextRange, BASE_OFFSET>(),
            Character: Character::<Impl, IMPL_OFFSET>,
            SetCharacter: SetCharacter::<Impl, IMPL_OFFSET>,
            CharacterFormat: CharacterFormat::<Impl, IMPL_OFFSET>,
            SetCharacterFormat: SetCharacterFormat::<Impl, IMPL_OFFSET>,
            FormattedText: FormattedText::<Impl, IMPL_OFFSET>,
            SetFormattedText: SetFormattedText::<Impl, IMPL_OFFSET>,
            EndPosition: EndPosition::<Impl, IMPL_OFFSET>,
            SetEndPosition: SetEndPosition::<Impl, IMPL_OFFSET>,
            Gravity: Gravity::<Impl, IMPL_OFFSET>,
            SetGravity: SetGravity::<Impl, IMPL_OFFSET>,
            Length: Length::<Impl, IMPL_OFFSET>,
            Link: Link::<Impl, IMPL_OFFSET>,
            SetLink: SetLink::<Impl, IMPL_OFFSET>,
            ParagraphFormat: ParagraphFormat::<Impl, IMPL_OFFSET>,
            SetParagraphFormat: SetParagraphFormat::<Impl, IMPL_OFFSET>,
            StartPosition: StartPosition::<Impl, IMPL_OFFSET>,
            SetStartPosition: SetStartPosition::<Impl, IMPL_OFFSET>,
            StoryLength: StoryLength::<Impl, IMPL_OFFSET>,
            Text: Text::<Impl, IMPL_OFFSET>,
            SetText: SetText::<Impl, IMPL_OFFSET>,
            CanPaste: CanPaste::<Impl, IMPL_OFFSET>,
            ChangeCase: ChangeCase::<Impl, IMPL_OFFSET>,
            Collapse: Collapse::<Impl, IMPL_OFFSET>,
            Copy: Copy::<Impl, IMPL_OFFSET>,
            Cut: Cut::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
            EndOf: EndOf::<Impl, IMPL_OFFSET>,
            Expand: Expand::<Impl, IMPL_OFFSET>,
            FindText: FindText::<Impl, IMPL_OFFSET>,
            GetCharacterUtf32: GetCharacterUtf32::<Impl, IMPL_OFFSET>,
            GetClone: GetClone::<Impl, IMPL_OFFSET>,
            GetIndex: GetIndex::<Impl, IMPL_OFFSET>,
            GetPoint: GetPoint::<Impl, IMPL_OFFSET>,
            GetRect: GetRect::<Impl, IMPL_OFFSET>,
            GetText: GetText::<Impl, IMPL_OFFSET>,
            GetTextViaStream: GetTextViaStream::<Impl, IMPL_OFFSET>,
            InRange: InRange::<Impl, IMPL_OFFSET>,
            InsertImage: InsertImage::<Impl, IMPL_OFFSET>,
            InStory: InStory::<Impl, IMPL_OFFSET>,
            IsEqual: IsEqual::<Impl, IMPL_OFFSET>,
            Move: Move::<Impl, IMPL_OFFSET>,
            MoveEnd: MoveEnd::<Impl, IMPL_OFFSET>,
            MoveStart: MoveStart::<Impl, IMPL_OFFSET>,
            Paste: Paste::<Impl, IMPL_OFFSET>,
            ScrollIntoView: ScrollIntoView::<Impl, IMPL_OFFSET>,
            MatchSelection: MatchSelection::<Impl, IMPL_OFFSET>,
            SetIndex: SetIndex::<Impl, IMPL_OFFSET>,
            SetPoint: SetPoint::<Impl, IMPL_OFFSET>,
            SetRange: SetRange::<Impl, IMPL_OFFSET>,
            SetText: SetText::<Impl, IMPL_OFFSET>,
            SetTextViaStream: SetTextViaStream::<Impl, IMPL_OFFSET>,
            StartOf: StartOf::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextRange as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
pub trait ITextSelectionImpl: Sized + ITextRangeImpl {
    fn Options(&mut self) -> ::windows::core::Result<SelectionOptions>;
    fn SetOptions(&mut self, value: SelectionOptions) -> ::windows::core::Result<()>;
    fn Type(&mut self) -> ::windows::core::Result<SelectionType>;
    fn EndKey(&mut self, unit: TextRangeUnit, extend: bool) -> ::windows::core::Result<i32>;
    fn HomeKey(&mut self, unit: TextRangeUnit, extend: bool) -> ::windows::core::Result<i32>;
    fn MoveDown(&mut self, unit: TextRangeUnit, count: i32, extend: bool) -> ::windows::core::Result<i32>;
    fn MoveLeft(&mut self, unit: TextRangeUnit, count: i32, extend: bool) -> ::windows::core::Result<i32>;
    fn MoveRight(&mut self, unit: TextRangeUnit, count: i32, extend: bool) -> ::windows::core::Result<i32>;
    fn MoveUp(&mut self, unit: TextRangeUnit, count: i32, extend: bool) -> ::windows::core::Result<i32>;
    fn TypeText(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ::windows::core::RuntimeName for ITextSelection {
    const NAME: &'static str = "Windows.UI.Text.ITextSelection";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ITextSelectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextSelectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextSelectionVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITextSelection, BASE_OFFSET>(),
            Options: Options::<Impl, IMPL_OFFSET>,
            SetOptions: SetOptions::<Impl, IMPL_OFFSET>,
            Type: Type::<Impl, IMPL_OFFSET>,
            EndKey: EndKey::<Impl, IMPL_OFFSET>,
            HomeKey: HomeKey::<Impl, IMPL_OFFSET>,
            MoveDown: MoveDown::<Impl, IMPL_OFFSET>,
            MoveLeft: MoveLeft::<Impl, IMPL_OFFSET>,
            MoveRight: MoveRight::<Impl, IMPL_OFFSET>,
            MoveUp: MoveUp::<Impl, IMPL_OFFSET>,
            TypeText: TypeText::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextSelection as ::windows::core::Interface>::IID
    }
}
