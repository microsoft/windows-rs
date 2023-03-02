#[doc(hidden)]
#[repr(transparent)]
pub struct ILanguageFont(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILanguageFont {
    type Vtable = ILanguageFont_Vtbl;
}
impl ::core::clone::Clone for ILanguageFont {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILanguageFont {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb12e5c3a_b76d_459b_beeb_901151cd77d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanguageFont_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub FontFamily: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Text")]
    pub FontWeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Text::FontWeight) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    FontWeight: usize,
    #[cfg(feature = "UI_Text")]
    pub FontStretch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Text::FontStretch) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    FontStretch: usize,
    #[cfg(feature = "UI_Text")]
    pub FontStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Text::FontStyle) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    FontStyle: usize,
    pub ScaleFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILanguageFontGroup(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILanguageFontGroup {
    type Vtable = ILanguageFontGroup_Vtbl;
}
impl ::core::clone::Clone for ILanguageFontGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILanguageFontGroup {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf33a7fc3_3a5c_4aea_b9ff_b39fb242f7f6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanguageFontGroup_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub UITextFont: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UIHeadingFont: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UITitleFont: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UICaptionFont: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UINotificationHeadingFont: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TraditionalDocumentFont: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ModernDocumentFont: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DocumentHeadingFont: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FixedWidthTextFont: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DocumentAlternate1Font: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DocumentAlternate2Font: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILanguageFontGroupFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILanguageFontGroupFactory {
    type Vtable = ILanguageFontGroupFactory_Vtbl;
}
impl ::core::clone::Clone for ILanguageFontGroupFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILanguageFontGroupFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfcaeac67_4e77_49c7_b856_dde934fc735b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanguageFontGroupFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateLanguageFontGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languagetag: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Globalization_Fonts\"`*"]
#[repr(transparent)]
pub struct LanguageFont(::windows::core::IUnknown);
impl LanguageFont {
    pub fn FontFamily(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).FontFamily)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    #[cfg(feature = "UI_Text")]
    pub fn FontWeight(&self) -> ::windows::core::Result<super::super::UI::Text::FontWeight> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::UI::Text::FontWeight>();
            (::windows::core::Interface::vtable(this).FontWeight)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    #[cfg(feature = "UI_Text")]
    pub fn FontStretch(&self) -> ::windows::core::Result<super::super::UI::Text::FontStretch> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::UI::Text::FontStretch>();
            (::windows::core::Interface::vtable(this).FontStretch)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    #[cfg(feature = "UI_Text")]
    pub fn FontStyle(&self) -> ::windows::core::Result<super::super::UI::Text::FontStyle> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::UI::Text::FontStyle>();
            (::windows::core::Interface::vtable(this).FontStyle)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ScaleFactor(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).ScaleFactor)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for LanguageFont {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LanguageFont {}
impl ::core::fmt::Debug for LanguageFont {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LanguageFont").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for LanguageFont {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Globalization.Fonts.LanguageFont;{b12e5c3a-b76d-459b-beeb-901151cd77d1})");
}
impl ::core::clone::Clone for LanguageFont {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for LanguageFont {
    type Vtable = ILanguageFont_Vtbl;
}
unsafe impl ::windows::core::ComInterface for LanguageFont {
    const IID: ::windows::core::GUID = <ILanguageFont as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for LanguageFont {
    const NAME: &'static str = "Windows.Globalization.Fonts.LanguageFont";
}
::windows::imp::interface_hierarchy!(LanguageFont, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for LanguageFont {}
unsafe impl ::core::marker::Sync for LanguageFont {}
#[doc = "*Required features: `\"Globalization_Fonts\"`*"]
#[repr(transparent)]
pub struct LanguageFontGroup(::windows::core::IUnknown);
impl LanguageFontGroup {
    pub fn UITextFont(&self) -> ::windows::core::Result<LanguageFont> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<LanguageFont>();
            (::windows::core::Interface::vtable(this).UITextFont)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn UIHeadingFont(&self) -> ::windows::core::Result<LanguageFont> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<LanguageFont>();
            (::windows::core::Interface::vtable(this).UIHeadingFont)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn UITitleFont(&self) -> ::windows::core::Result<LanguageFont> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<LanguageFont>();
            (::windows::core::Interface::vtable(this).UITitleFont)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn UICaptionFont(&self) -> ::windows::core::Result<LanguageFont> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<LanguageFont>();
            (::windows::core::Interface::vtable(this).UICaptionFont)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn UINotificationHeadingFont(&self) -> ::windows::core::Result<LanguageFont> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<LanguageFont>();
            (::windows::core::Interface::vtable(this).UINotificationHeadingFont)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TraditionalDocumentFont(&self) -> ::windows::core::Result<LanguageFont> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<LanguageFont>();
            (::windows::core::Interface::vtable(this).TraditionalDocumentFont)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ModernDocumentFont(&self) -> ::windows::core::Result<LanguageFont> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<LanguageFont>();
            (::windows::core::Interface::vtable(this).ModernDocumentFont)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DocumentHeadingFont(&self) -> ::windows::core::Result<LanguageFont> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<LanguageFont>();
            (::windows::core::Interface::vtable(this).DocumentHeadingFont)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FixedWidthTextFont(&self) -> ::windows::core::Result<LanguageFont> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<LanguageFont>();
            (::windows::core::Interface::vtable(this).FixedWidthTextFont)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DocumentAlternate1Font(&self) -> ::windows::core::Result<LanguageFont> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<LanguageFont>();
            (::windows::core::Interface::vtable(this).DocumentAlternate1Font)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DocumentAlternate2Font(&self) -> ::windows::core::Result<LanguageFont> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<LanguageFont>();
            (::windows::core::Interface::vtable(this).DocumentAlternate2Font)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateLanguageFontGroup(languagetag: &::windows::core::HSTRING) -> ::windows::core::Result<LanguageFontGroup> {
        Self::ILanguageFontGroupFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<LanguageFontGroup>();
            (::windows::core::Interface::vtable(this).CreateLanguageFontGroup)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(languagetag), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILanguageFontGroupFactory<R, F: FnOnce(&ILanguageFontGroupFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<LanguageFontGroup, ILanguageFontGroupFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for LanguageFontGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LanguageFontGroup {}
impl ::core::fmt::Debug for LanguageFontGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LanguageFontGroup").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for LanguageFontGroup {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Globalization.Fonts.LanguageFontGroup;{f33a7fc3-3a5c-4aea-b9ff-b39fb242f7f6})");
}
impl ::core::clone::Clone for LanguageFontGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for LanguageFontGroup {
    type Vtable = ILanguageFontGroup_Vtbl;
}
unsafe impl ::windows::core::ComInterface for LanguageFontGroup {
    const IID: ::windows::core::GUID = <ILanguageFontGroup as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for LanguageFontGroup {
    const NAME: &'static str = "Windows.Globalization.Fonts.LanguageFontGroup";
}
::windows::imp::interface_hierarchy!(LanguageFontGroup, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for LanguageFontGroup {}
unsafe impl ::core::marker::Sync for LanguageFontGroup {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
