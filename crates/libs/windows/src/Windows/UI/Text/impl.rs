pub trait ITextCharacterFormat_Impl: Sized {
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
impl ITextCharacterFormat_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>() -> ITextCharacterFormat_Vtbl {
        unsafe extern "system" fn AllCaps<Identity: ::windows::core::IUnknownImpl, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AllCaps() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllCaps<Identity: ::windows::core::IUnknownImpl, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAllCaps(value).into()
        }
        unsafe extern "system" fn BackgroundColor<Identity: ::windows::core::IUnknownImpl, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BackgroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackgroundColor<Identity: ::windows::core::IUnknownImpl, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBackgroundColor(&*(&value as *const <super::Color as ::windows::core::Abi>::Abi as *const <super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Bold<Identity: ::windows::core::IUnknownImpl, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Bold() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBold<Identity: ::windows::core::IUnknownImpl, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBold(value).into()
        }
        unsafe extern "system" fn FontStretch<Identity: ::windows::core::IUnknownImpl, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FontStretch) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FontStretch() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFontStretch<Identity: ::windows::core::IUnknownImpl, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FontStretch) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFontStretch(value).into()
        }
        unsafe extern "system" fn FontStyle<Identity: ::windows::core::IUnknownImpl, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FontStyle) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FontStyle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFontStyle<Identity: ::windows::core::IUnknownImpl, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FontStyle) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFontStyle(value).into()
        }
        unsafe extern "system" fn ForegroundColor<Identity: ::windows::core::IUnknownImpl, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ForegroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForegroundColor<Identity: ::windows::core::IUnknownImpl, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetForegroundColor(&*(&value as *const <super::Color as ::windows::core::Abi>::Abi as *const <super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Hidden<Identity: ::windows::core::IUnknownImpl, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Hidden() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHidden<Identity: ::windows::core::IUnknownImpl, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetHidden(value).into()
        }
        unsafe extern "system" fn Italic<Identity: ::windows::core::IUnknownImpl, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Italic() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetItalic<Identity: ::windows::core::IUnknownImpl, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetItalic(value).into()
        }
        unsafe extern "system" fn Kerning<Identity: ::windows::core::IUnknownImpl, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Kerning() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKerning<Identity: ::windows::core::IUnknownImpl, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetKerning(value).into()
        }
        unsafe extern "system" fn LanguageTag<Identity: ::windows::core::IUnknownImpl, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LanguageTag() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLanguageTag<Identity: ::windows::core::IUnknownImpl, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLanguageTag(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LinkType<Identity: ::windows::core::IUnknownImpl, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut LinkType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LinkType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Outline<Identity: ::windows::core::IUnknownImpl, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Outline() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutline<Identity: ::windows::core::IUnknownImpl, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOutline(value).into()
        }
        unsafe extern "system" fn Position<Identity: ::windows::core::IUnknownImpl, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPosition<Identity: ::windows::core::IUnknownImpl, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPosition(value).into()
        }
        unsafe extern "system" fn ProtectedText<Identity: ::windows::core::IUnknownImpl, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ProtectedText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProtectedText<Identity: ::windows::core::IUnknownImpl, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProtectedText(value).into()
        }
        unsafe extern "system" fn Size<Identity: ::windows::core::IUnknownImpl, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Size() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSize<Identity: ::windows::core::IUnknownImpl, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSize(value).into()
        }
        unsafe extern "system" fn SmallCaps<Identity: ::windows::core::IUnknownImpl, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SmallCaps() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSmallCaps<Identity: ::windows::core::IUnknownImpl, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSmallCaps(value).into()
        }
        unsafe extern "system" fn Spacing<Identity: ::windows::core::IUnknownImpl, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Spacing() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpacing<Identity: ::windows::core::IUnknownImpl, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSpacing(value).into()
        }
        unsafe extern "system" fn Strikethrough<Identity: ::windows::core::IUnknownImpl, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Strikethrough() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrikethrough<Identity: ::windows::core::IUnknownImpl, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStrikethrough(value).into()
        }
        unsafe extern "system" fn Subscript<Identity: ::windows::core::IUnknownImpl, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Subscript() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubscript<Identity: ::windows::core::IUnknownImpl, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSubscript(value).into()
        }
        unsafe extern "system" fn Superscript<Identity: ::windows::core::IUnknownImpl, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Superscript() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSuperscript<Identity: ::windows::core::IUnknownImpl, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSuperscript(value).into()
        }
        unsafe extern "system" fn TextScript<Identity: ::windows::core::IUnknownImpl, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TextScript) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TextScript() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTextScript<Identity: ::windows::core::IUnknownImpl, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: TextScript) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTextScript(value).into()
        }
        unsafe extern "system" fn Underline<Identity: ::windows::core::IUnknownImpl, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UnderlineType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Underline() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnderline<Identity: ::windows::core::IUnknownImpl, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: UnderlineType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetUnderline(value).into()
        }
        unsafe extern "system" fn Weight<Identity: ::windows::core::IUnknownImpl, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Weight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWeight<Identity: ::windows::core::IUnknownImpl, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetWeight(value).into()
        }
        unsafe extern "system" fn SetClone<Identity: ::windows::core::IUnknownImpl, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetClone(&*(&value as *const <ITextCharacterFormat as ::windows::core::Abi>::Abi as *const <ITextCharacterFormat as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetClone<Identity: ::windows::core::IUnknownImpl, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetClone() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqual<Identity: ::windows::core::IUnknownImpl, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITextCharacterFormat, OFFSET>(),
            AllCaps: AllCaps::<Identity, Impl, OFFSET>,
            SetAllCaps: SetAllCaps::<Identity, Impl, OFFSET>,
            BackgroundColor: BackgroundColor::<Identity, Impl, OFFSET>,
            SetBackgroundColor: SetBackgroundColor::<Identity, Impl, OFFSET>,
            Bold: Bold::<Identity, Impl, OFFSET>,
            SetBold: SetBold::<Identity, Impl, OFFSET>,
            FontStretch: FontStretch::<Identity, Impl, OFFSET>,
            SetFontStretch: SetFontStretch::<Identity, Impl, OFFSET>,
            FontStyle: FontStyle::<Identity, Impl, OFFSET>,
            SetFontStyle: SetFontStyle::<Identity, Impl, OFFSET>,
            ForegroundColor: ForegroundColor::<Identity, Impl, OFFSET>,
            SetForegroundColor: SetForegroundColor::<Identity, Impl, OFFSET>,
            Hidden: Hidden::<Identity, Impl, OFFSET>,
            SetHidden: SetHidden::<Identity, Impl, OFFSET>,
            Italic: Italic::<Identity, Impl, OFFSET>,
            SetItalic: SetItalic::<Identity, Impl, OFFSET>,
            Kerning: Kerning::<Identity, Impl, OFFSET>,
            SetKerning: SetKerning::<Identity, Impl, OFFSET>,
            LanguageTag: LanguageTag::<Identity, Impl, OFFSET>,
            SetLanguageTag: SetLanguageTag::<Identity, Impl, OFFSET>,
            LinkType: LinkType::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            Outline: Outline::<Identity, Impl, OFFSET>,
            SetOutline: SetOutline::<Identity, Impl, OFFSET>,
            Position: Position::<Identity, Impl, OFFSET>,
            SetPosition: SetPosition::<Identity, Impl, OFFSET>,
            ProtectedText: ProtectedText::<Identity, Impl, OFFSET>,
            SetProtectedText: SetProtectedText::<Identity, Impl, OFFSET>,
            Size: Size::<Identity, Impl, OFFSET>,
            SetSize: SetSize::<Identity, Impl, OFFSET>,
            SmallCaps: SmallCaps::<Identity, Impl, OFFSET>,
            SetSmallCaps: SetSmallCaps::<Identity, Impl, OFFSET>,
            Spacing: Spacing::<Identity, Impl, OFFSET>,
            SetSpacing: SetSpacing::<Identity, Impl, OFFSET>,
            Strikethrough: Strikethrough::<Identity, Impl, OFFSET>,
            SetStrikethrough: SetStrikethrough::<Identity, Impl, OFFSET>,
            Subscript: Subscript::<Identity, Impl, OFFSET>,
            SetSubscript: SetSubscript::<Identity, Impl, OFFSET>,
            Superscript: Superscript::<Identity, Impl, OFFSET>,
            SetSuperscript: SetSuperscript::<Identity, Impl, OFFSET>,
            TextScript: TextScript::<Identity, Impl, OFFSET>,
            SetTextScript: SetTextScript::<Identity, Impl, OFFSET>,
            Underline: Underline::<Identity, Impl, OFFSET>,
            SetUnderline: SetUnderline::<Identity, Impl, OFFSET>,
            Weight: Weight::<Identity, Impl, OFFSET>,
            SetWeight: SetWeight::<Identity, Impl, OFFSET>,
            SetClone: SetClone::<Identity, Impl, OFFSET>,
            GetClone: GetClone::<Identity, Impl, OFFSET>,
            IsEqual: IsEqual::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextCharacterFormat as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
pub trait ITextDocument_Impl: Sized {
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
impl ITextDocument_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument_Impl, const OFFSET: isize>() -> ITextDocument_Vtbl {
        unsafe extern "system" fn CaretType<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CaretType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CaretType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCaretType<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CaretType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCaretType(value).into()
        }
        unsafe extern "system" fn DefaultTabStop<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DefaultTabStop() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultTabStop<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDefaultTabStop(value).into()
        }
        unsafe extern "system" fn Selection<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Selection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UndoLimit<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UndoLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUndoLimit<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetUndoLimit(value).into()
        }
        unsafe extern "system" fn CanCopy<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CanCopy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanPaste<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CanPaste() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanRedo<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CanRedo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanUndo<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CanUndo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplyDisplayUpdates<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ApplyDisplayUpdates() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BatchDisplayUpdates<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BatchDisplayUpdates() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginUndoGroup<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BeginUndoGroup().into()
        }
        unsafe extern "system" fn EndUndoGroup<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EndUndoGroup().into()
        }
        unsafe extern "system" fn GetDefaultCharacterFormat<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDefaultCharacterFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultParagraphFormat<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDefaultParagraphFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRange<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startposition: i32, endposition: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRange(startposition, endposition) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRangeFromPoint<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, point: super::super::Foundation::Point, options: PointOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRangeFromPoint(&*(&point as *const <super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType), options) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetText<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: TextGetOptions, value: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetText(options, ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn LoadFromStream<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: TextSetOptions, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LoadFromStream(options, &*(&value as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Redo<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Redo().into()
        }
        unsafe extern "system" fn SaveToStream<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: TextGetOptions, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SaveToStream(options, &*(&value as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetDefaultCharacterFormat<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDefaultCharacterFormat(&*(&value as *const <ITextCharacterFormat as ::windows::core::Abi>::Abi as *const <ITextCharacterFormat as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetDefaultParagraphFormat<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDefaultParagraphFormat(&*(&value as *const <ITextParagraphFormat as ::windows::core::Abi>::Abi as *const <ITextParagraphFormat as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetText<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: TextSetOptions, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetText(options, &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Undo<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Undo().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITextDocument, OFFSET>(),
            CaretType: CaretType::<Identity, Impl, OFFSET>,
            SetCaretType: SetCaretType::<Identity, Impl, OFFSET>,
            DefaultTabStop: DefaultTabStop::<Identity, Impl, OFFSET>,
            SetDefaultTabStop: SetDefaultTabStop::<Identity, Impl, OFFSET>,
            Selection: Selection::<Identity, Impl, OFFSET>,
            UndoLimit: UndoLimit::<Identity, Impl, OFFSET>,
            SetUndoLimit: SetUndoLimit::<Identity, Impl, OFFSET>,
            CanCopy: CanCopy::<Identity, Impl, OFFSET>,
            CanPaste: CanPaste::<Identity, Impl, OFFSET>,
            CanRedo: CanRedo::<Identity, Impl, OFFSET>,
            CanUndo: CanUndo::<Identity, Impl, OFFSET>,
            ApplyDisplayUpdates: ApplyDisplayUpdates::<Identity, Impl, OFFSET>,
            BatchDisplayUpdates: BatchDisplayUpdates::<Identity, Impl, OFFSET>,
            BeginUndoGroup: BeginUndoGroup::<Identity, Impl, OFFSET>,
            EndUndoGroup: EndUndoGroup::<Identity, Impl, OFFSET>,
            GetDefaultCharacterFormat: GetDefaultCharacterFormat::<Identity, Impl, OFFSET>,
            GetDefaultParagraphFormat: GetDefaultParagraphFormat::<Identity, Impl, OFFSET>,
            GetRange: GetRange::<Identity, Impl, OFFSET>,
            GetRangeFromPoint: GetRangeFromPoint::<Identity, Impl, OFFSET>,
            GetText: GetText::<Identity, Impl, OFFSET>,
            LoadFromStream: LoadFromStream::<Identity, Impl, OFFSET>,
            Redo: Redo::<Identity, Impl, OFFSET>,
            SaveToStream: SaveToStream::<Identity, Impl, OFFSET>,
            SetDefaultCharacterFormat: SetDefaultCharacterFormat::<Identity, Impl, OFFSET>,
            SetDefaultParagraphFormat: SetDefaultParagraphFormat::<Identity, Impl, OFFSET>,
            SetText: SetText::<Identity, Impl, OFFSET>,
            Undo: Undo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextDocument as ::windows::core::Interface>::IID
    }
}
pub trait ITextParagraphFormat_Impl: Sized {
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
impl ITextParagraphFormat_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>() -> ITextParagraphFormat_Vtbl {
        unsafe extern "system" fn Alignment<Identity: ::windows::core::IUnknownImpl, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ParagraphAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Alignment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlignment<Identity: ::windows::core::IUnknownImpl, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ParagraphAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAlignment(value).into()
        }
        unsafe extern "system" fn FirstLineIndent<Identity: ::windows::core::IUnknownImpl, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FirstLineIndent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeepTogether<Identity: ::windows::core::IUnknownImpl, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).KeepTogether() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeepTogether<Identity: ::windows::core::IUnknownImpl, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetKeepTogether(value).into()
        }
        unsafe extern "system" fn KeepWithNext<Identity: ::windows::core::IUnknownImpl, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).KeepWithNext() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeepWithNext<Identity: ::windows::core::IUnknownImpl, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetKeepWithNext(value).into()
        }
        unsafe extern "system" fn LeftIndent<Identity: ::windows::core::IUnknownImpl, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LeftIndent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LineSpacing<Identity: ::windows::core::IUnknownImpl, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LineSpacing() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LineSpacingRule<Identity: ::windows::core::IUnknownImpl, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut LineSpacingRule) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LineSpacingRule() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ListAlignment<Identity: ::windows::core::IUnknownImpl, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MarkerAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ListAlignment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListAlignment<Identity: ::windows::core::IUnknownImpl, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MarkerAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetListAlignment(value).into()
        }
        unsafe extern "system" fn ListLevelIndex<Identity: ::windows::core::IUnknownImpl, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ListLevelIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListLevelIndex<Identity: ::windows::core::IUnknownImpl, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetListLevelIndex(value).into()
        }
        unsafe extern "system" fn ListStart<Identity: ::windows::core::IUnknownImpl, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ListStart() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListStart<Identity: ::windows::core::IUnknownImpl, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetListStart(value).into()
        }
        unsafe extern "system" fn ListStyle<Identity: ::windows::core::IUnknownImpl, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MarkerStyle) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ListStyle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListStyle<Identity: ::windows::core::IUnknownImpl, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MarkerStyle) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetListStyle(value).into()
        }
        unsafe extern "system" fn ListTab<Identity: ::windows::core::IUnknownImpl, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ListTab() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListTab<Identity: ::windows::core::IUnknownImpl, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetListTab(value).into()
        }
        unsafe extern "system" fn ListType<Identity: ::windows::core::IUnknownImpl, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MarkerType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ListType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListType<Identity: ::windows::core::IUnknownImpl, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MarkerType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetListType(value).into()
        }
        unsafe extern "system" fn NoLineNumber<Identity: ::windows::core::IUnknownImpl, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NoLineNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNoLineNumber<Identity: ::windows::core::IUnknownImpl, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetNoLineNumber(value).into()
        }
        unsafe extern "system" fn PageBreakBefore<Identity: ::windows::core::IUnknownImpl, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PageBreakBefore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPageBreakBefore<Identity: ::windows::core::IUnknownImpl, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPageBreakBefore(value).into()
        }
        unsafe extern "system" fn RightIndent<Identity: ::windows::core::IUnknownImpl, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RightIndent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRightIndent<Identity: ::windows::core::IUnknownImpl, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRightIndent(value).into()
        }
        unsafe extern "system" fn RightToLeft<Identity: ::windows::core::IUnknownImpl, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RightToLeft() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRightToLeft<Identity: ::windows::core::IUnknownImpl, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRightToLeft(value).into()
        }
        unsafe extern "system" fn Style<Identity: ::windows::core::IUnknownImpl, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ParagraphStyle) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Style() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStyle<Identity: ::windows::core::IUnknownImpl, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ParagraphStyle) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStyle(value).into()
        }
        unsafe extern "system" fn SpaceAfter<Identity: ::windows::core::IUnknownImpl, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SpaceAfter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpaceAfter<Identity: ::windows::core::IUnknownImpl, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSpaceAfter(value).into()
        }
        unsafe extern "system" fn SpaceBefore<Identity: ::windows::core::IUnknownImpl, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SpaceBefore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpaceBefore<Identity: ::windows::core::IUnknownImpl, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSpaceBefore(value).into()
        }
        unsafe extern "system" fn WidowControl<Identity: ::windows::core::IUnknownImpl, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).WidowControl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWidowControl<Identity: ::windows::core::IUnknownImpl, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetWidowControl(value).into()
        }
        unsafe extern "system" fn TabCount<Identity: ::windows::core::IUnknownImpl, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TabCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddTab<Identity: ::windows::core::IUnknownImpl, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, position: f32, align: TabAlignment, leader: TabLeader) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddTab(position, align, leader).into()
        }
        unsafe extern "system" fn ClearAllTabs<Identity: ::windows::core::IUnknownImpl, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ClearAllTabs().into()
        }
        unsafe extern "system" fn DeleteTab<Identity: ::windows::core::IUnknownImpl, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, position: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteTab(position).into()
        }
        unsafe extern "system" fn GetClone<Identity: ::windows::core::IUnknownImpl, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetClone() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTab<Identity: ::windows::core::IUnknownImpl, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, position: *mut f32, align: *mut TabAlignment, leader: *mut TabLeader) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetTab(index, ::core::mem::transmute_copy(&position), ::core::mem::transmute_copy(&align), ::core::mem::transmute_copy(&leader)).into()
        }
        unsafe extern "system" fn IsEqual<Identity: ::windows::core::IUnknownImpl, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsEqual(&*(&format as *const <ITextParagraphFormat as ::windows::core::Abi>::Abi as *const <ITextParagraphFormat as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClone<Identity: ::windows::core::IUnknownImpl, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetClone(&*(&format as *const <ITextParagraphFormat as ::windows::core::Abi>::Abi as *const <ITextParagraphFormat as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetIndents<Identity: ::windows::core::IUnknownImpl, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, start: f32, left: f32, right: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetIndents(start, left, right).into()
        }
        unsafe extern "system" fn SetLineSpacing<Identity: ::windows::core::IUnknownImpl, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rule: LineSpacingRule, spacing: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLineSpacing(rule, spacing).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITextParagraphFormat, OFFSET>(),
            Alignment: Alignment::<Identity, Impl, OFFSET>,
            SetAlignment: SetAlignment::<Identity, Impl, OFFSET>,
            FirstLineIndent: FirstLineIndent::<Identity, Impl, OFFSET>,
            KeepTogether: KeepTogether::<Identity, Impl, OFFSET>,
            SetKeepTogether: SetKeepTogether::<Identity, Impl, OFFSET>,
            KeepWithNext: KeepWithNext::<Identity, Impl, OFFSET>,
            SetKeepWithNext: SetKeepWithNext::<Identity, Impl, OFFSET>,
            LeftIndent: LeftIndent::<Identity, Impl, OFFSET>,
            LineSpacing: LineSpacing::<Identity, Impl, OFFSET>,
            LineSpacingRule: LineSpacingRule::<Identity, Impl, OFFSET>,
            ListAlignment: ListAlignment::<Identity, Impl, OFFSET>,
            SetListAlignment: SetListAlignment::<Identity, Impl, OFFSET>,
            ListLevelIndex: ListLevelIndex::<Identity, Impl, OFFSET>,
            SetListLevelIndex: SetListLevelIndex::<Identity, Impl, OFFSET>,
            ListStart: ListStart::<Identity, Impl, OFFSET>,
            SetListStart: SetListStart::<Identity, Impl, OFFSET>,
            ListStyle: ListStyle::<Identity, Impl, OFFSET>,
            SetListStyle: SetListStyle::<Identity, Impl, OFFSET>,
            ListTab: ListTab::<Identity, Impl, OFFSET>,
            SetListTab: SetListTab::<Identity, Impl, OFFSET>,
            ListType: ListType::<Identity, Impl, OFFSET>,
            SetListType: SetListType::<Identity, Impl, OFFSET>,
            NoLineNumber: NoLineNumber::<Identity, Impl, OFFSET>,
            SetNoLineNumber: SetNoLineNumber::<Identity, Impl, OFFSET>,
            PageBreakBefore: PageBreakBefore::<Identity, Impl, OFFSET>,
            SetPageBreakBefore: SetPageBreakBefore::<Identity, Impl, OFFSET>,
            RightIndent: RightIndent::<Identity, Impl, OFFSET>,
            SetRightIndent: SetRightIndent::<Identity, Impl, OFFSET>,
            RightToLeft: RightToLeft::<Identity, Impl, OFFSET>,
            SetRightToLeft: SetRightToLeft::<Identity, Impl, OFFSET>,
            Style: Style::<Identity, Impl, OFFSET>,
            SetStyle: SetStyle::<Identity, Impl, OFFSET>,
            SpaceAfter: SpaceAfter::<Identity, Impl, OFFSET>,
            SetSpaceAfter: SetSpaceAfter::<Identity, Impl, OFFSET>,
            SpaceBefore: SpaceBefore::<Identity, Impl, OFFSET>,
            SetSpaceBefore: SetSpaceBefore::<Identity, Impl, OFFSET>,
            WidowControl: WidowControl::<Identity, Impl, OFFSET>,
            SetWidowControl: SetWidowControl::<Identity, Impl, OFFSET>,
            TabCount: TabCount::<Identity, Impl, OFFSET>,
            AddTab: AddTab::<Identity, Impl, OFFSET>,
            ClearAllTabs: ClearAllTabs::<Identity, Impl, OFFSET>,
            DeleteTab: DeleteTab::<Identity, Impl, OFFSET>,
            GetClone: GetClone::<Identity, Impl, OFFSET>,
            GetTab: GetTab::<Identity, Impl, OFFSET>,
            IsEqual: IsEqual::<Identity, Impl, OFFSET>,
            SetClone: SetClone::<Identity, Impl, OFFSET>,
            SetIndents: SetIndents::<Identity, Impl, OFFSET>,
            SetLineSpacing: SetLineSpacing::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextParagraphFormat as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
pub trait ITextRange_Impl: Sized {
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
    fn SetText2(&mut self, options: TextSetOptions, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetTextViaStream(&mut self, options: TextSetOptions, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<()>;
    fn StartOf(&mut self, unit: TextRangeUnit, extend: bool) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ::windows::core::RuntimeName for ITextRange {
    const NAME: &'static str = "Windows.UI.Text.ITextRange";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ITextRange_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>() -> ITextRange_Vtbl {
        unsafe extern "system" fn Character<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Character() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCharacter<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCharacter(value).into()
        }
        unsafe extern "system" fn CharacterFormat<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CharacterFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCharacterFormat<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCharacterFormat(&*(&value as *const <ITextCharacterFormat as ::windows::core::Abi>::Abi as *const <ITextCharacterFormat as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FormattedText<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FormattedText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormattedText<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFormattedText(&*(&value as *const <ITextRange as ::windows::core::Abi>::Abi as *const <ITextRange as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EndPosition<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EndPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEndPosition<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEndPosition(value).into()
        }
        unsafe extern "system" fn Gravity<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut RangeGravity) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Gravity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGravity<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: RangeGravity) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGravity(value).into()
        }
        unsafe extern "system" fn Length<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Length() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Link<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Link() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLink<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLink(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ParagraphFormat<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ParagraphFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParagraphFormat<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetParagraphFormat(&*(&value as *const <ITextParagraphFormat as ::windows::core::Abi>::Abi as *const <ITextParagraphFormat as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StartPosition<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).StartPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartPosition<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStartPosition(value).into()
        }
        unsafe extern "system" fn StoryLength<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).StoryLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Text<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Text() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetText<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CanPaste<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: i32, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CanPaste(format) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChangeCase<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: LetterCase) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ChangeCase(value).into()
        }
        unsafe extern "system" fn Collapse<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Collapse(value).into()
        }
        unsafe extern "system" fn Copy<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Copy().into()
        }
        unsafe extern "system" fn Cut<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Cut().into()
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, count: i32, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Delete(unit, count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndOf<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, extend: bool, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EndOf(unit, extend) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Expand<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Expand(unit) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindText<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, scanlength: i32, options: FindOptions, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FindText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), scanlength, options) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCharacterUtf32<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut u32, offset: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCharacterUtf32(::core::mem::transmute_copy(&value), offset).into()
        }
        unsafe extern "system" fn GetClone<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetClone() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIndex<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetIndex(unit) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPoint<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalalign: HorizontalCharacterAlignment, verticalalign: VerticalCharacterAlignment, options: PointOptions, point: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPoint(horizontalalign, verticalalign, options, ::core::mem::transmute_copy(&point)).into()
        }
        unsafe extern "system" fn GetRect<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: PointOptions, rect: *mut super::super::Foundation::Rect, hit: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRect(options, ::core::mem::transmute_copy(&rect), ::core::mem::transmute_copy(&hit)).into()
        }
        unsafe extern "system" fn GetText<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: TextGetOptions, value: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetText(options, ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetTextViaStream<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: TextGetOptions, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetTextViaStream(options, &*(&value as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InRange<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, range: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InRange(&*(&range as *const <ITextRange as ::windows::core::Abi>::Abi as *const <ITextRange as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertImage<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: i32, height: i32, ascent: i32, verticalalign: VerticalCharacterAlignment, alternatetext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InsertImage(width, height, ascent, verticalalign, &*(&alternatetext as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InStory<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, range: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InStory(&*(&range as *const <ITextRange as ::windows::core::Abi>::Abi as *const <ITextRange as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqual<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, range: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsEqual(&*(&range as *const <ITextRange as ::windows::core::Abi>::Abi as *const <ITextRange as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Move<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, count: i32, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Move(unit, count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveEnd<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, count: i32, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MoveEnd(unit, count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveStart<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, count: i32, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MoveStart(unit, count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Paste<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Paste(format).into()
        }
        unsafe extern "system" fn ScrollIntoView<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PointOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ScrollIntoView(value).into()
        }
        unsafe extern "system" fn MatchSelection<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).MatchSelection().into()
        }
        unsafe extern "system" fn SetIndex<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, index: i32, extend: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetIndex(unit, index, extend).into()
        }
        unsafe extern "system" fn SetPoint<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, point: super::super::Foundation::Point, options: PointOptions, extend: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPoint(&*(&point as *const <super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType), options, extend).into()
        }
        unsafe extern "system" fn SetRange<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startposition: i32, endposition: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRange(startposition, endposition).into()
        }
        unsafe extern "system" fn SetText2<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: TextSetOptions, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetText2(options, &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetTextViaStream<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: TextSetOptions, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTextViaStream(options, &*(&value as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StartOf<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, extend: bool, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITextRange, OFFSET>(),
            Character: Character::<Identity, Impl, OFFSET>,
            SetCharacter: SetCharacter::<Identity, Impl, OFFSET>,
            CharacterFormat: CharacterFormat::<Identity, Impl, OFFSET>,
            SetCharacterFormat: SetCharacterFormat::<Identity, Impl, OFFSET>,
            FormattedText: FormattedText::<Identity, Impl, OFFSET>,
            SetFormattedText: SetFormattedText::<Identity, Impl, OFFSET>,
            EndPosition: EndPosition::<Identity, Impl, OFFSET>,
            SetEndPosition: SetEndPosition::<Identity, Impl, OFFSET>,
            Gravity: Gravity::<Identity, Impl, OFFSET>,
            SetGravity: SetGravity::<Identity, Impl, OFFSET>,
            Length: Length::<Identity, Impl, OFFSET>,
            Link: Link::<Identity, Impl, OFFSET>,
            SetLink: SetLink::<Identity, Impl, OFFSET>,
            ParagraphFormat: ParagraphFormat::<Identity, Impl, OFFSET>,
            SetParagraphFormat: SetParagraphFormat::<Identity, Impl, OFFSET>,
            StartPosition: StartPosition::<Identity, Impl, OFFSET>,
            SetStartPosition: SetStartPosition::<Identity, Impl, OFFSET>,
            StoryLength: StoryLength::<Identity, Impl, OFFSET>,
            Text: Text::<Identity, Impl, OFFSET>,
            SetText: SetText::<Identity, Impl, OFFSET>,
            CanPaste: CanPaste::<Identity, Impl, OFFSET>,
            ChangeCase: ChangeCase::<Identity, Impl, OFFSET>,
            Collapse: Collapse::<Identity, Impl, OFFSET>,
            Copy: Copy::<Identity, Impl, OFFSET>,
            Cut: Cut::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            EndOf: EndOf::<Identity, Impl, OFFSET>,
            Expand: Expand::<Identity, Impl, OFFSET>,
            FindText: FindText::<Identity, Impl, OFFSET>,
            GetCharacterUtf32: GetCharacterUtf32::<Identity, Impl, OFFSET>,
            GetClone: GetClone::<Identity, Impl, OFFSET>,
            GetIndex: GetIndex::<Identity, Impl, OFFSET>,
            GetPoint: GetPoint::<Identity, Impl, OFFSET>,
            GetRect: GetRect::<Identity, Impl, OFFSET>,
            GetText: GetText::<Identity, Impl, OFFSET>,
            GetTextViaStream: GetTextViaStream::<Identity, Impl, OFFSET>,
            InRange: InRange::<Identity, Impl, OFFSET>,
            InsertImage: InsertImage::<Identity, Impl, OFFSET>,
            InStory: InStory::<Identity, Impl, OFFSET>,
            IsEqual: IsEqual::<Identity, Impl, OFFSET>,
            Move: Move::<Identity, Impl, OFFSET>,
            MoveEnd: MoveEnd::<Identity, Impl, OFFSET>,
            MoveStart: MoveStart::<Identity, Impl, OFFSET>,
            Paste: Paste::<Identity, Impl, OFFSET>,
            ScrollIntoView: ScrollIntoView::<Identity, Impl, OFFSET>,
            MatchSelection: MatchSelection::<Identity, Impl, OFFSET>,
            SetIndex: SetIndex::<Identity, Impl, OFFSET>,
            SetPoint: SetPoint::<Identity, Impl, OFFSET>,
            SetRange: SetRange::<Identity, Impl, OFFSET>,
            SetText2: SetText2::<Identity, Impl, OFFSET>,
            SetTextViaStream: SetTextViaStream::<Identity, Impl, OFFSET>,
            StartOf: StartOf::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextRange as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
pub trait ITextSelection_Impl: Sized + ITextRange_Impl {
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
impl ITextSelection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextSelection_Impl, const OFFSET: isize>() -> ITextSelection_Vtbl {
        unsafe extern "system" fn Options<Identity: ::windows::core::IUnknownImpl, Impl: ITextSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SelectionOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Options() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOptions<Identity: ::windows::core::IUnknownImpl, Impl: ITextSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: SelectionOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOptions(value).into()
        }
        unsafe extern "system" fn Type<Identity: ::windows::core::IUnknownImpl, Impl: ITextSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SelectionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndKey<Identity: ::windows::core::IUnknownImpl, Impl: ITextSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, extend: bool, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EndKey(unit, extend) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HomeKey<Identity: ::windows::core::IUnknownImpl, Impl: ITextSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, extend: bool, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HomeKey(unit, extend) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveDown<Identity: ::windows::core::IUnknownImpl, Impl: ITextSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, count: i32, extend: bool, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MoveDown(unit, count, extend) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveLeft<Identity: ::windows::core::IUnknownImpl, Impl: ITextSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, count: i32, extend: bool, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MoveLeft(unit, count, extend) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveRight<Identity: ::windows::core::IUnknownImpl, Impl: ITextSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, count: i32, extend: bool, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MoveRight(unit, count, extend) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveUp<Identity: ::windows::core::IUnknownImpl, Impl: ITextSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, count: i32, extend: bool, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MoveUp(unit, count, extend) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TypeText<Identity: ::windows::core::IUnknownImpl, Impl: ITextSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TypeText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITextSelection, OFFSET>(),
            Options: Options::<Identity, Impl, OFFSET>,
            SetOptions: SetOptions::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
            EndKey: EndKey::<Identity, Impl, OFFSET>,
            HomeKey: HomeKey::<Identity, Impl, OFFSET>,
            MoveDown: MoveDown::<Identity, Impl, OFFSET>,
            MoveLeft: MoveLeft::<Identity, Impl, OFFSET>,
            MoveRight: MoveRight::<Identity, Impl, OFFSET>,
            MoveUp: MoveUp::<Identity, Impl, OFFSET>,
            TypeText: TypeText::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextSelection as ::windows::core::Interface>::IID
    }
}
