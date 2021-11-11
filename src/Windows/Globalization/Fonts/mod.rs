#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct ILanguageFont(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILanguageFont {
    type Vtable = ILanguageFont_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb12e5c3a_b76d_459b_beeb_901151cd77d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanguageFont_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Text")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::UI::Text::FontWeight) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Text"))] usize,
    #[cfg(feature = "UI_Text")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::UI::Text::FontStretch) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Text"))] usize,
    #[cfg(feature = "UI_Text")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::UI::Text::FontStyle) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Text"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILanguageFontGroup(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILanguageFontGroup {
    type Vtable = ILanguageFontGroup_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf33a7fc3_3a5c_4aea_b9ff_b39fb242f7f6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanguageFontGroup_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILanguageFontGroupFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILanguageFontGroupFactory {
    type Vtable = ILanguageFontGroupFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xfcaeac67_4e77_49c7_b856_dde934fc735b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanguageFontGroupFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, languagetag: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Globalization_Fonts`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LanguageFont(pub ::windows::runtime::IInspectable);
impl LanguageFont {
    #[doc = "*Required features: `Globalization_Fonts`*"]
    pub fn FontFamily(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "UI_Text")]
    #[doc = "*Required features: `Globalization_Fonts`, `UI_Text`*"]
    pub fn FontWeight(&self) -> ::windows::runtime::Result<super::super::UI::Text::FontWeight> {
        let this = self;
        unsafe {
            let mut result__: super::super::UI::Text::FontWeight = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Text::FontWeight>(result__)
        }
    }
    #[cfg(feature = "UI_Text")]
    #[doc = "*Required features: `Globalization_Fonts`, `UI_Text`*"]
    pub fn FontStretch(&self) -> ::windows::runtime::Result<super::super::UI::Text::FontStretch> {
        let this = self;
        unsafe {
            let mut result__: super::super::UI::Text::FontStretch = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Text::FontStretch>(result__)
        }
    }
    #[cfg(feature = "UI_Text")]
    #[doc = "*Required features: `Globalization_Fonts`, `UI_Text`*"]
    pub fn FontStyle(&self) -> ::windows::runtime::Result<super::super::UI::Text::FontStyle> {
        let this = self;
        unsafe {
            let mut result__: super::super::UI::Text::FontStyle = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Text::FontStyle>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_Fonts`*"]
    pub fn ScaleFactor(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LanguageFont {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Globalization.Fonts.LanguageFont;{b12e5c3a-b76d-459b-beeb-901151cd77d1})");
}
unsafe impl ::windows::runtime::Interface for LanguageFont {
    type Vtable = ILanguageFont_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb12e5c3a_b76d_459b_beeb_901151cd77d1);
}
impl ::windows::runtime::RuntimeName for LanguageFont {
    const NAME: &'static str = "Windows.Globalization.Fonts.LanguageFont";
}
impl ::core::convert::From<LanguageFont> for ::windows::runtime::IUnknown {
    fn from(value: LanguageFont) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LanguageFont> for ::windows::runtime::IUnknown {
    fn from(value: &LanguageFont) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for LanguageFont {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a LanguageFont {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LanguageFont> for ::windows::runtime::IInspectable {
    fn from(value: LanguageFont) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LanguageFont> for ::windows::runtime::IInspectable {
    fn from(value: &LanguageFont) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for LanguageFont {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a LanguageFont {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for LanguageFont {}
unsafe impl ::core::marker::Sync for LanguageFont {}
#[doc = "*Required features: `Globalization_Fonts`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LanguageFontGroup(pub ::windows::runtime::IInspectable);
impl LanguageFontGroup {
    #[doc = "*Required features: `Globalization_Fonts`*"]
    pub fn UITextFont(&self) -> ::windows::runtime::Result<LanguageFont> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LanguageFont>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_Fonts`*"]
    pub fn UIHeadingFont(&self) -> ::windows::runtime::Result<LanguageFont> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LanguageFont>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_Fonts`*"]
    pub fn UITitleFont(&self) -> ::windows::runtime::Result<LanguageFont> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LanguageFont>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_Fonts`*"]
    pub fn UICaptionFont(&self) -> ::windows::runtime::Result<LanguageFont> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LanguageFont>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_Fonts`*"]
    pub fn UINotificationHeadingFont(&self) -> ::windows::runtime::Result<LanguageFont> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LanguageFont>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_Fonts`*"]
    pub fn TraditionalDocumentFont(&self) -> ::windows::runtime::Result<LanguageFont> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LanguageFont>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_Fonts`*"]
    pub fn ModernDocumentFont(&self) -> ::windows::runtime::Result<LanguageFont> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LanguageFont>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_Fonts`*"]
    pub fn DocumentHeadingFont(&self) -> ::windows::runtime::Result<LanguageFont> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LanguageFont>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_Fonts`*"]
    pub fn FixedWidthTextFont(&self) -> ::windows::runtime::Result<LanguageFont> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LanguageFont>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_Fonts`*"]
    pub fn DocumentAlternate1Font(&self) -> ::windows::runtime::Result<LanguageFont> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LanguageFont>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_Fonts`*"]
    pub fn DocumentAlternate2Font(&self) -> ::windows::runtime::Result<LanguageFont> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LanguageFont>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_Fonts`*"]
    pub fn CreateLanguageFontGroup<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(languagetag: Param0) -> ::windows::runtime::Result<LanguageFontGroup> {
        Self::ILanguageFontGroupFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), languagetag.into_param().abi(), &mut result__).from_abi::<LanguageFontGroup>(result__)
        })
    }
    pub fn ILanguageFontGroupFactory<R, F: FnOnce(&ILanguageFontGroupFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<LanguageFontGroup, ILanguageFontGroupFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LanguageFontGroup {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Globalization.Fonts.LanguageFontGroup;{f33a7fc3-3a5c-4aea-b9ff-b39fb242f7f6})");
}
unsafe impl ::windows::runtime::Interface for LanguageFontGroup {
    type Vtable = ILanguageFontGroup_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf33a7fc3_3a5c_4aea_b9ff_b39fb242f7f6);
}
impl ::windows::runtime::RuntimeName for LanguageFontGroup {
    const NAME: &'static str = "Windows.Globalization.Fonts.LanguageFontGroup";
}
impl ::core::convert::From<LanguageFontGroup> for ::windows::runtime::IUnknown {
    fn from(value: LanguageFontGroup) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LanguageFontGroup> for ::windows::runtime::IUnknown {
    fn from(value: &LanguageFontGroup) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for LanguageFontGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a LanguageFontGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LanguageFontGroup> for ::windows::runtime::IInspectable {
    fn from(value: LanguageFontGroup) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LanguageFontGroup> for ::windows::runtime::IInspectable {
    fn from(value: &LanguageFontGroup) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for LanguageFontGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a LanguageFontGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for LanguageFontGroup {}
unsafe impl ::core::marker::Sync for LanguageFontGroup {}
