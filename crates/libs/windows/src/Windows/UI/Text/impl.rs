#[doc = "*Required features: `\"UI_Text\"`, `\"implement\"`*"]
pub trait ITextCharacterFormat_Impl: Sized {
    fn AllCaps(&self) -> ::windows_core::Result<FormatEffect>;
    fn SetAllCaps(&self, value: FormatEffect) -> ::windows_core::Result<()>;
    fn BackgroundColor(&self) -> ::windows_core::Result<super::Color>;
    fn SetBackgroundColor(&self, value: &super::Color) -> ::windows_core::Result<()>;
    fn Bold(&self) -> ::windows_core::Result<FormatEffect>;
    fn SetBold(&self, value: FormatEffect) -> ::windows_core::Result<()>;
    fn FontStretch(&self) -> ::windows_core::Result<FontStretch>;
    fn SetFontStretch(&self, value: FontStretch) -> ::windows_core::Result<()>;
    fn FontStyle(&self) -> ::windows_core::Result<FontStyle>;
    fn SetFontStyle(&self, value: FontStyle) -> ::windows_core::Result<()>;
    fn ForegroundColor(&self) -> ::windows_core::Result<super::Color>;
    fn SetForegroundColor(&self, value: &super::Color) -> ::windows_core::Result<()>;
    fn Hidden(&self) -> ::windows_core::Result<FormatEffect>;
    fn SetHidden(&self, value: FormatEffect) -> ::windows_core::Result<()>;
    fn Italic(&self) -> ::windows_core::Result<FormatEffect>;
    fn SetItalic(&self, value: FormatEffect) -> ::windows_core::Result<()>;
    fn Kerning(&self) -> ::windows_core::Result<f32>;
    fn SetKerning(&self, value: f32) -> ::windows_core::Result<()>;
    fn LanguageTag(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn SetLanguageTag(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn LinkType(&self) -> ::windows_core::Result<LinkType>;
    fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn SetName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn Outline(&self) -> ::windows_core::Result<FormatEffect>;
    fn SetOutline(&self, value: FormatEffect) -> ::windows_core::Result<()>;
    fn Position(&self) -> ::windows_core::Result<f32>;
    fn SetPosition(&self, value: f32) -> ::windows_core::Result<()>;
    fn ProtectedText(&self) -> ::windows_core::Result<FormatEffect>;
    fn SetProtectedText(&self, value: FormatEffect) -> ::windows_core::Result<()>;
    fn Size(&self) -> ::windows_core::Result<f32>;
    fn SetSize(&self, value: f32) -> ::windows_core::Result<()>;
    fn SmallCaps(&self) -> ::windows_core::Result<FormatEffect>;
    fn SetSmallCaps(&self, value: FormatEffect) -> ::windows_core::Result<()>;
    fn Spacing(&self) -> ::windows_core::Result<f32>;
    fn SetSpacing(&self, value: f32) -> ::windows_core::Result<()>;
    fn Strikethrough(&self) -> ::windows_core::Result<FormatEffect>;
    fn SetStrikethrough(&self, value: FormatEffect) -> ::windows_core::Result<()>;
    fn Subscript(&self) -> ::windows_core::Result<FormatEffect>;
    fn SetSubscript(&self, value: FormatEffect) -> ::windows_core::Result<()>;
    fn Superscript(&self) -> ::windows_core::Result<FormatEffect>;
    fn SetSuperscript(&self, value: FormatEffect) -> ::windows_core::Result<()>;
    fn TextScript(&self) -> ::windows_core::Result<TextScript>;
    fn SetTextScript(&self, value: TextScript) -> ::windows_core::Result<()>;
    fn Underline(&self) -> ::windows_core::Result<UnderlineType>;
    fn SetUnderline(&self, value: UnderlineType) -> ::windows_core::Result<()>;
    fn Weight(&self) -> ::windows_core::Result<i32>;
    fn SetWeight(&self, value: i32) -> ::windows_core::Result<()>;
    fn SetClone(&self, value: ::core::option::Option<&ITextCharacterFormat>) -> ::windows_core::Result<()>;
    fn GetClone(&self) -> ::windows_core::Result<ITextCharacterFormat>;
    fn IsEqual(&self, format: ::core::option::Option<&ITextCharacterFormat>) -> ::windows_core::Result<bool>;
}
impl ::windows_core::RuntimeName for ITextCharacterFormat {
    const NAME: &'static str = "Windows.UI.Text.ITextCharacterFormat";
}
impl ITextCharacterFormat_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>() -> ITextCharacterFormat_Vtbl {
        unsafe extern "system" fn AllCaps<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AllCaps() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllCaps<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAllCaps(value).into()
        }
        unsafe extern "system" fn BackgroundColor<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Color) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BackgroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackgroundColor<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Color) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBackgroundColor(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Bold<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Bold() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBold<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBold(value).into()
        }
        unsafe extern "system" fn FontStretch<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FontStretch) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FontStretch() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFontStretch<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FontStretch) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFontStretch(value).into()
        }
        unsafe extern "system" fn FontStyle<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FontStyle) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FontStyle() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFontStyle<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FontStyle) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFontStyle(value).into()
        }
        unsafe extern "system" fn ForegroundColor<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Color) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ForegroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForegroundColor<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Color) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetForegroundColor(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Hidden<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Hidden() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHidden<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHidden(value).into()
        }
        unsafe extern "system" fn Italic<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Italic() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetItalic<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetItalic(value).into()
        }
        unsafe extern "system" fn Kerning<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Kerning() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKerning<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetKerning(value).into()
        }
        unsafe extern "system" fn LanguageTag<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LanguageTag() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLanguageTag<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLanguageTag(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn LinkType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut LinkType) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LinkType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetName(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Outline<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Outline() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutline<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOutline(value).into()
        }
        unsafe extern "system" fn Position<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Position() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPosition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPosition(value).into()
        }
        unsafe extern "system" fn ProtectedText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ProtectedText() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProtectedText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProtectedText(value).into()
        }
        unsafe extern "system" fn Size<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Size() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSize(value).into()
        }
        unsafe extern "system" fn SmallCaps<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SmallCaps() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSmallCaps<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSmallCaps(value).into()
        }
        unsafe extern "system" fn Spacing<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Spacing() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpacing<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSpacing(value).into()
        }
        unsafe extern "system" fn Strikethrough<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Strikethrough() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrikethrough<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStrikethrough(value).into()
        }
        unsafe extern "system" fn Subscript<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Subscript() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubscript<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSubscript(value).into()
        }
        unsafe extern "system" fn Superscript<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Superscript() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSuperscript<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSuperscript(value).into()
        }
        unsafe extern "system" fn TextScript<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TextScript) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TextScript() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTextScript<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: TextScript) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTextScript(value).into()
        }
        unsafe extern "system" fn Underline<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UnderlineType) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Underline() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnderline<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: UnderlineType) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUnderline(value).into()
        }
        unsafe extern "system" fn Weight<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Weight() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWeight<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetWeight(value).into()
        }
        unsafe extern "system" fn SetClone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetClone(::windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn GetClone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetClone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqual<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsEqual(::windows_core::from_raw_borrowed(&format)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, ITextCharacterFormat, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITextCharacterFormat as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"UI_Text\"`, `\"Foundation\"`, `\"Storage_Streams\"`, `\"implement\"`*"]
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
pub trait ITextDocument_Impl: Sized {
    fn CaretType(&self) -> ::windows_core::Result<CaretType>;
    fn SetCaretType(&self, value: CaretType) -> ::windows_core::Result<()>;
    fn DefaultTabStop(&self) -> ::windows_core::Result<f32>;
    fn SetDefaultTabStop(&self, value: f32) -> ::windows_core::Result<()>;
    fn Selection(&self) -> ::windows_core::Result<ITextSelection>;
    fn UndoLimit(&self) -> ::windows_core::Result<u32>;
    fn SetUndoLimit(&self, value: u32) -> ::windows_core::Result<()>;
    fn CanCopy(&self) -> ::windows_core::Result<bool>;
    fn CanPaste(&self) -> ::windows_core::Result<bool>;
    fn CanRedo(&self) -> ::windows_core::Result<bool>;
    fn CanUndo(&self) -> ::windows_core::Result<bool>;
    fn ApplyDisplayUpdates(&self) -> ::windows_core::Result<i32>;
    fn BatchDisplayUpdates(&self) -> ::windows_core::Result<i32>;
    fn BeginUndoGroup(&self) -> ::windows_core::Result<()>;
    fn EndUndoGroup(&self) -> ::windows_core::Result<()>;
    fn GetDefaultCharacterFormat(&self) -> ::windows_core::Result<ITextCharacterFormat>;
    fn GetDefaultParagraphFormat(&self) -> ::windows_core::Result<ITextParagraphFormat>;
    fn GetRange(&self, startposition: i32, endposition: i32) -> ::windows_core::Result<ITextRange>;
    fn GetRangeFromPoint(&self, point: &super::super::Foundation::Point, options: PointOptions) -> ::windows_core::Result<ITextRange>;
    fn GetText(&self, options: TextGetOptions, value: &mut ::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn LoadFromStream(&self, options: TextSetOptions, value: ::core::option::Option<&super::super::Storage::Streams::IRandomAccessStream>) -> ::windows_core::Result<()>;
    fn Redo(&self) -> ::windows_core::Result<()>;
    fn SaveToStream(&self, options: TextGetOptions, value: ::core::option::Option<&super::super::Storage::Streams::IRandomAccessStream>) -> ::windows_core::Result<()>;
    fn SetDefaultCharacterFormat(&self, value: ::core::option::Option<&ITextCharacterFormat>) -> ::windows_core::Result<()>;
    fn SetDefaultParagraphFormat(&self, value: ::core::option::Option<&ITextParagraphFormat>) -> ::windows_core::Result<()>;
    fn SetText(&self, options: TextSetOptions, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn Undo(&self) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ::windows_core::RuntimeName for ITextDocument {
    const NAME: &'static str = "Windows.UI.Text.ITextDocument";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ITextDocument_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: isize>() -> ITextDocument_Vtbl {
        unsafe extern "system" fn CaretType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CaretType) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CaretType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCaretType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CaretType) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCaretType(value).into()
        }
        unsafe extern "system" fn DefaultTabStop<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DefaultTabStop() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultTabStop<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDefaultTabStop(value).into()
        }
        unsafe extern "system" fn Selection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Selection() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UndoLimit<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UndoLimit() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUndoLimit<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUndoLimit(value).into()
        }
        unsafe extern "system" fn CanCopy<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CanCopy() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanPaste<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CanPaste() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanRedo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CanRedo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanUndo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CanUndo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplyDisplayUpdates<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ApplyDisplayUpdates() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BatchDisplayUpdates<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BatchDisplayUpdates() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginUndoGroup<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginUndoGroup().into()
        }
        unsafe extern "system" fn EndUndoGroup<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndUndoGroup().into()
        }
        unsafe extern "system" fn GetDefaultCharacterFormat<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDefaultCharacterFormat() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultParagraphFormat<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDefaultParagraphFormat() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startposition: i32, endposition: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRange(startposition, endposition) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRangeFromPoint<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, point: super::super::Foundation::Point, options: PointOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRangeFromPoint(::core::mem::transmute(&point), options) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: TextGetOptions, value: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetText(options, ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn LoadFromStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: TextSetOptions, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LoadFromStream(options, ::windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn Redo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Redo().into()
        }
        unsafe extern "system" fn SaveToStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: TextGetOptions, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SaveToStream(options, ::windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn SetDefaultCharacterFormat<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDefaultCharacterFormat(::windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn SetDefaultParagraphFormat<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDefaultParagraphFormat(::windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn SetText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: TextSetOptions, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetText(options, ::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Undo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Undo().into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, ITextDocument, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITextDocument as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"UI_Text\"`, `\"implement\"`*"]
pub trait ITextParagraphFormat_Impl: Sized {
    fn Alignment(&self) -> ::windows_core::Result<ParagraphAlignment>;
    fn SetAlignment(&self, value: ParagraphAlignment) -> ::windows_core::Result<()>;
    fn FirstLineIndent(&self) -> ::windows_core::Result<f32>;
    fn KeepTogether(&self) -> ::windows_core::Result<FormatEffect>;
    fn SetKeepTogether(&self, value: FormatEffect) -> ::windows_core::Result<()>;
    fn KeepWithNext(&self) -> ::windows_core::Result<FormatEffect>;
    fn SetKeepWithNext(&self, value: FormatEffect) -> ::windows_core::Result<()>;
    fn LeftIndent(&self) -> ::windows_core::Result<f32>;
    fn LineSpacing(&self) -> ::windows_core::Result<f32>;
    fn LineSpacingRule(&self) -> ::windows_core::Result<LineSpacingRule>;
    fn ListAlignment(&self) -> ::windows_core::Result<MarkerAlignment>;
    fn SetListAlignment(&self, value: MarkerAlignment) -> ::windows_core::Result<()>;
    fn ListLevelIndex(&self) -> ::windows_core::Result<i32>;
    fn SetListLevelIndex(&self, value: i32) -> ::windows_core::Result<()>;
    fn ListStart(&self) -> ::windows_core::Result<i32>;
    fn SetListStart(&self, value: i32) -> ::windows_core::Result<()>;
    fn ListStyle(&self) -> ::windows_core::Result<MarkerStyle>;
    fn SetListStyle(&self, value: MarkerStyle) -> ::windows_core::Result<()>;
    fn ListTab(&self) -> ::windows_core::Result<f32>;
    fn SetListTab(&self, value: f32) -> ::windows_core::Result<()>;
    fn ListType(&self) -> ::windows_core::Result<MarkerType>;
    fn SetListType(&self, value: MarkerType) -> ::windows_core::Result<()>;
    fn NoLineNumber(&self) -> ::windows_core::Result<FormatEffect>;
    fn SetNoLineNumber(&self, value: FormatEffect) -> ::windows_core::Result<()>;
    fn PageBreakBefore(&self) -> ::windows_core::Result<FormatEffect>;
    fn SetPageBreakBefore(&self, value: FormatEffect) -> ::windows_core::Result<()>;
    fn RightIndent(&self) -> ::windows_core::Result<f32>;
    fn SetRightIndent(&self, value: f32) -> ::windows_core::Result<()>;
    fn RightToLeft(&self) -> ::windows_core::Result<FormatEffect>;
    fn SetRightToLeft(&self, value: FormatEffect) -> ::windows_core::Result<()>;
    fn Style(&self) -> ::windows_core::Result<ParagraphStyle>;
    fn SetStyle(&self, value: ParagraphStyle) -> ::windows_core::Result<()>;
    fn SpaceAfter(&self) -> ::windows_core::Result<f32>;
    fn SetSpaceAfter(&self, value: f32) -> ::windows_core::Result<()>;
    fn SpaceBefore(&self) -> ::windows_core::Result<f32>;
    fn SetSpaceBefore(&self, value: f32) -> ::windows_core::Result<()>;
    fn WidowControl(&self) -> ::windows_core::Result<FormatEffect>;
    fn SetWidowControl(&self, value: FormatEffect) -> ::windows_core::Result<()>;
    fn TabCount(&self) -> ::windows_core::Result<i32>;
    fn AddTab(&self, position: f32, align: TabAlignment, leader: TabLeader) -> ::windows_core::Result<()>;
    fn ClearAllTabs(&self) -> ::windows_core::Result<()>;
    fn DeleteTab(&self, position: f32) -> ::windows_core::Result<()>;
    fn GetClone(&self) -> ::windows_core::Result<ITextParagraphFormat>;
    fn GetTab(&self, index: i32, position: &mut f32, align: &mut TabAlignment, leader: &mut TabLeader) -> ::windows_core::Result<()>;
    fn IsEqual(&self, format: ::core::option::Option<&ITextParagraphFormat>) -> ::windows_core::Result<bool>;
    fn SetClone(&self, format: ::core::option::Option<&ITextParagraphFormat>) -> ::windows_core::Result<()>;
    fn SetIndents(&self, start: f32, left: f32, right: f32) -> ::windows_core::Result<()>;
    fn SetLineSpacing(&self, rule: LineSpacingRule, spacing: f32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ITextParagraphFormat {
    const NAME: &'static str = "Windows.UI.Text.ITextParagraphFormat";
}
impl ITextParagraphFormat_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>() -> ITextParagraphFormat_Vtbl {
        unsafe extern "system" fn Alignment<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ParagraphAlignment) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Alignment() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlignment<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ParagraphAlignment) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAlignment(value).into()
        }
        unsafe extern "system" fn FirstLineIndent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FirstLineIndent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeepTogether<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.KeepTogether() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeepTogether<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetKeepTogether(value).into()
        }
        unsafe extern "system" fn KeepWithNext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.KeepWithNext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeepWithNext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetKeepWithNext(value).into()
        }
        unsafe extern "system" fn LeftIndent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LeftIndent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LineSpacing<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LineSpacing() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LineSpacingRule<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut LineSpacingRule) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LineSpacingRule() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ListAlignment<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MarkerAlignment) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ListAlignment() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListAlignment<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MarkerAlignment) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetListAlignment(value).into()
        }
        unsafe extern "system" fn ListLevelIndex<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ListLevelIndex() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListLevelIndex<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetListLevelIndex(value).into()
        }
        unsafe extern "system" fn ListStart<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ListStart() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListStart<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetListStart(value).into()
        }
        unsafe extern "system" fn ListStyle<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MarkerStyle) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ListStyle() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListStyle<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MarkerStyle) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetListStyle(value).into()
        }
        unsafe extern "system" fn ListTab<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ListTab() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListTab<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetListTab(value).into()
        }
        unsafe extern "system" fn ListType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MarkerType) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ListType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MarkerType) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetListType(value).into()
        }
        unsafe extern "system" fn NoLineNumber<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NoLineNumber() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNoLineNumber<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNoLineNumber(value).into()
        }
        unsafe extern "system" fn PageBreakBefore<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PageBreakBefore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPageBreakBefore<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPageBreakBefore(value).into()
        }
        unsafe extern "system" fn RightIndent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RightIndent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRightIndent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRightIndent(value).into()
        }
        unsafe extern "system" fn RightToLeft<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RightToLeft() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRightToLeft<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRightToLeft(value).into()
        }
        unsafe extern "system" fn Style<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ParagraphStyle) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Style() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStyle<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ParagraphStyle) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStyle(value).into()
        }
        unsafe extern "system" fn SpaceAfter<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SpaceAfter() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpaceAfter<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSpaceAfter(value).into()
        }
        unsafe extern "system" fn SpaceBefore<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SpaceBefore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpaceBefore<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSpaceBefore(value).into()
        }
        unsafe extern "system" fn WidowControl<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.WidowControl() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWidowControl<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetWidowControl(value).into()
        }
        unsafe extern "system" fn TabCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TabCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddTab<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, position: f32, align: TabAlignment, leader: TabLeader) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddTab(position, align, leader).into()
        }
        unsafe extern "system" fn ClearAllTabs<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ClearAllTabs().into()
        }
        unsafe extern "system" fn DeleteTab<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, position: f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteTab(position).into()
        }
        unsafe extern "system" fn GetClone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetClone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTab<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, position: *mut f32, align: *mut TabAlignment, leader: *mut TabLeader) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetTab(index, ::core::mem::transmute_copy(&position), ::core::mem::transmute_copy(&align), ::core::mem::transmute_copy(&leader)).into()
        }
        unsafe extern "system" fn IsEqual<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsEqual(::windows_core::from_raw_borrowed(&format)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetClone(::windows_core::from_raw_borrowed(&format)).into()
        }
        unsafe extern "system" fn SetIndents<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, start: f32, left: f32, right: f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIndents(start, left, right).into()
        }
        unsafe extern "system" fn SetLineSpacing<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rule: LineSpacingRule, spacing: f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLineSpacing(rule, spacing).into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, ITextParagraphFormat, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITextParagraphFormat as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"UI_Text\"`, `\"Foundation\"`, `\"Storage_Streams\"`, `\"implement\"`*"]
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
pub trait ITextRange_Impl: Sized {
    fn Character(&self) -> ::windows_core::Result<u16>;
    fn SetCharacter(&self, value: u16) -> ::windows_core::Result<()>;
    fn CharacterFormat(&self) -> ::windows_core::Result<ITextCharacterFormat>;
    fn SetCharacterFormat(&self, value: ::core::option::Option<&ITextCharacterFormat>) -> ::windows_core::Result<()>;
    fn FormattedText(&self) -> ::windows_core::Result<ITextRange>;
    fn SetFormattedText(&self, value: ::core::option::Option<&ITextRange>) -> ::windows_core::Result<()>;
    fn EndPosition(&self) -> ::windows_core::Result<i32>;
    fn SetEndPosition(&self, value: i32) -> ::windows_core::Result<()>;
    fn Gravity(&self) -> ::windows_core::Result<RangeGravity>;
    fn SetGravity(&self, value: RangeGravity) -> ::windows_core::Result<()>;
    fn Length(&self) -> ::windows_core::Result<i32>;
    fn Link(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn SetLink(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn ParagraphFormat(&self) -> ::windows_core::Result<ITextParagraphFormat>;
    fn SetParagraphFormat(&self, value: ::core::option::Option<&ITextParagraphFormat>) -> ::windows_core::Result<()>;
    fn StartPosition(&self) -> ::windows_core::Result<i32>;
    fn SetStartPosition(&self, value: i32) -> ::windows_core::Result<()>;
    fn StoryLength(&self) -> ::windows_core::Result<i32>;
    fn Text(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn SetText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn CanPaste(&self, format: i32) -> ::windows_core::Result<bool>;
    fn ChangeCase(&self, value: LetterCase) -> ::windows_core::Result<()>;
    fn Collapse(&self, value: bool) -> ::windows_core::Result<()>;
    fn Copy(&self) -> ::windows_core::Result<()>;
    fn Cut(&self) -> ::windows_core::Result<()>;
    fn Delete(&self, unit: TextRangeUnit, count: i32) -> ::windows_core::Result<i32>;
    fn EndOf(&self, unit: TextRangeUnit, extend: bool) -> ::windows_core::Result<i32>;
    fn Expand(&self, unit: TextRangeUnit) -> ::windows_core::Result<i32>;
    fn FindText(&self, value: &::windows_core::HSTRING, scanlength: i32, options: FindOptions) -> ::windows_core::Result<i32>;
    fn GetCharacterUtf32(&self, value: &mut u32, offset: i32) -> ::windows_core::Result<()>;
    fn GetClone(&self) -> ::windows_core::Result<ITextRange>;
    fn GetIndex(&self, unit: TextRangeUnit) -> ::windows_core::Result<i32>;
    fn GetPoint(&self, horizontalalign: HorizontalCharacterAlignment, verticalalign: VerticalCharacterAlignment, options: PointOptions, point: &mut super::super::Foundation::Point) -> ::windows_core::Result<()>;
    fn GetRect(&self, options: PointOptions, rect: &mut super::super::Foundation::Rect, hit: &mut i32) -> ::windows_core::Result<()>;
    fn GetText(&self, options: TextGetOptions, value: &mut ::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn GetTextViaStream(&self, options: TextGetOptions, value: ::core::option::Option<&super::super::Storage::Streams::IRandomAccessStream>) -> ::windows_core::Result<()>;
    fn InRange(&self, range: ::core::option::Option<&ITextRange>) -> ::windows_core::Result<bool>;
    fn InsertImage(&self, width: i32, height: i32, ascent: i32, verticalalign: VerticalCharacterAlignment, alternatetext: &::windows_core::HSTRING, value: ::core::option::Option<&super::super::Storage::Streams::IRandomAccessStream>) -> ::windows_core::Result<()>;
    fn InStory(&self, range: ::core::option::Option<&ITextRange>) -> ::windows_core::Result<bool>;
    fn IsEqual(&self, range: ::core::option::Option<&ITextRange>) -> ::windows_core::Result<bool>;
    fn Move(&self, unit: TextRangeUnit, count: i32) -> ::windows_core::Result<i32>;
    fn MoveEnd(&self, unit: TextRangeUnit, count: i32) -> ::windows_core::Result<i32>;
    fn MoveStart(&self, unit: TextRangeUnit, count: i32) -> ::windows_core::Result<i32>;
    fn Paste(&self, format: i32) -> ::windows_core::Result<()>;
    fn ScrollIntoView(&self, value: PointOptions) -> ::windows_core::Result<()>;
    fn MatchSelection(&self) -> ::windows_core::Result<()>;
    fn SetIndex(&self, unit: TextRangeUnit, index: i32, extend: bool) -> ::windows_core::Result<()>;
    fn SetPoint(&self, point: &super::super::Foundation::Point, options: PointOptions, extend: bool) -> ::windows_core::Result<()>;
    fn SetRange(&self, startposition: i32, endposition: i32) -> ::windows_core::Result<()>;
    fn SetText2(&self, options: TextSetOptions, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn SetTextViaStream(&self, options: TextSetOptions, value: ::core::option::Option<&super::super::Storage::Streams::IRandomAccessStream>) -> ::windows_core::Result<()>;
    fn StartOf(&self, unit: TextRangeUnit, extend: bool) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ::windows_core::RuntimeName for ITextRange {
    const NAME: &'static str = "Windows.UI.Text.ITextRange";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ITextRange_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>() -> ITextRange_Vtbl {
        unsafe extern "system" fn Character<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Character() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCharacter<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCharacter(value).into()
        }
        unsafe extern "system" fn CharacterFormat<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CharacterFormat() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCharacterFormat<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCharacterFormat(::windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn FormattedText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FormattedText() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormattedText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFormattedText(::windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn EndPosition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndPosition() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEndPosition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEndPosition(value).into()
        }
        unsafe extern "system" fn Gravity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut RangeGravity) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Gravity() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGravity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: RangeGravity) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGravity(value).into()
        }
        unsafe extern "system" fn Length<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Length() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Link<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Link() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLink<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLink(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn ParagraphFormat<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ParagraphFormat() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParagraphFormat<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetParagraphFormat(::windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn StartPosition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StartPosition() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartPosition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStartPosition(value).into()
        }
        unsafe extern "system" fn StoryLength<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StoryLength() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Text<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Text() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetText(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn CanPaste<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: i32, result__: *mut bool) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CanPaste(format) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChangeCase<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: LetterCase) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ChangeCase(value).into()
        }
        unsafe extern "system" fn Collapse<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Collapse(value).into()
        }
        unsafe extern "system" fn Copy<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Copy().into()
        }
        unsafe extern "system" fn Cut<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Cut().into()
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, count: i32, result__: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Delete(unit, count) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndOf<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, extend: bool, result__: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndOf(unit, extend) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Expand<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, result__: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Expand(unit) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>, scanlength: i32, options: FindOptions, result__: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindText(::core::mem::transmute(&value), scanlength, options) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCharacterUtf32<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut u32, offset: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCharacterUtf32(::core::mem::transmute_copy(&value), offset).into()
        }
        unsafe extern "system" fn GetClone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetClone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIndex<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, result__: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetIndex(unit) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPoint<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalalign: HorizontalCharacterAlignment, verticalalign: VerticalCharacterAlignment, options: PointOptions, point: *mut super::super::Foundation::Point) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPoint(horizontalalign, verticalalign, options, ::core::mem::transmute_copy(&point)).into()
        }
        unsafe extern "system" fn GetRect<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: PointOptions, rect: *mut super::super::Foundation::Rect, hit: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRect(options, ::core::mem::transmute_copy(&rect), ::core::mem::transmute_copy(&hit)).into()
        }
        unsafe extern "system" fn GetText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: TextGetOptions, value: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetText(options, ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetTextViaStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: TextGetOptions, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetTextViaStream(options, ::windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn InRange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, range: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InRange(::windows_core::from_raw_borrowed(&range)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertImage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: i32, height: i32, ascent: i32, verticalalign: VerticalCharacterAlignment, alternatetext: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InsertImage(width, height, ascent, verticalalign, ::core::mem::transmute(&alternatetext), ::windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn InStory<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, range: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InStory(::windows_core::from_raw_borrowed(&range)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqual<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, range: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsEqual(::windows_core::from_raw_borrowed(&range)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Move<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, count: i32, result__: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Move(unit, count) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveEnd<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, count: i32, result__: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveEnd(unit, count) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveStart<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, count: i32, result__: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveStart(unit, count) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Paste<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Paste(format).into()
        }
        unsafe extern "system" fn ScrollIntoView<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PointOptions) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ScrollIntoView(value).into()
        }
        unsafe extern "system" fn MatchSelection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MatchSelection().into()
        }
        unsafe extern "system" fn SetIndex<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, index: i32, extend: bool) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIndex(unit, index, extend).into()
        }
        unsafe extern "system" fn SetPoint<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, point: super::super::Foundation::Point, options: PointOptions, extend: bool) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPoint(::core::mem::transmute(&point), options, extend).into()
        }
        unsafe extern "system" fn SetRange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startposition: i32, endposition: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRange(startposition, endposition).into()
        }
        unsafe extern "system" fn SetText2<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: TextSetOptions, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetText2(options, ::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn SetTextViaStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: TextSetOptions, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTextViaStream(options, ::windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn StartOf<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, extend: bool, result__: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StartOf(unit, extend) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, ITextRange, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITextRange as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"UI_Text\"`, `\"Foundation\"`, `\"Storage_Streams\"`, `\"implement\"`*"]
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
pub trait ITextSelection_Impl: Sized + ITextRange_Impl {
    fn Options(&self) -> ::windows_core::Result<SelectionOptions>;
    fn SetOptions(&self, value: SelectionOptions) -> ::windows_core::Result<()>;
    fn Type(&self) -> ::windows_core::Result<SelectionType>;
    fn EndKey(&self, unit: TextRangeUnit, extend: bool) -> ::windows_core::Result<i32>;
    fn HomeKey(&self, unit: TextRangeUnit, extend: bool) -> ::windows_core::Result<i32>;
    fn MoveDown(&self, unit: TextRangeUnit, count: i32, extend: bool) -> ::windows_core::Result<i32>;
    fn MoveLeft(&self, unit: TextRangeUnit, count: i32, extend: bool) -> ::windows_core::Result<i32>;
    fn MoveRight(&self, unit: TextRangeUnit, count: i32, extend: bool) -> ::windows_core::Result<i32>;
    fn MoveUp(&self, unit: TextRangeUnit, count: i32, extend: bool) -> ::windows_core::Result<i32>;
    fn TypeText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ::windows_core::RuntimeName for ITextSelection {
    const NAME: &'static str = "Windows.UI.Text.ITextSelection";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ITextSelection_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextSelection_Impl, const OFFSET: isize>() -> ITextSelection_Vtbl {
        unsafe extern "system" fn Options<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SelectionOptions) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Options() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOptions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: SelectionOptions) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOptions(value).into()
        }
        unsafe extern "system" fn Type<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SelectionType) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Type() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndKey<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, extend: bool, result__: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndKey(unit, extend) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HomeKey<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, extend: bool, result__: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HomeKey(unit, extend) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveDown<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, count: i32, extend: bool, result__: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveDown(unit, count, extend) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveLeft<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, count: i32, extend: bool, result__: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveLeft(unit, count, extend) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveRight<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, count: i32, extend: bool, result__: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveRight(unit, count, extend) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveUp<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, count: i32, extend: bool, result__: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveUp(unit, count, extend) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TypeText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITextSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TypeText(::core::mem::transmute(&value)).into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, ITextSelection, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITextSelection as ::windows_core::ComInterface>::IID
    }
}
