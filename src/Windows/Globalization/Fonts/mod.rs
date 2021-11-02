#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ILanguageFont(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILanguageFont {
    type Vtable = ILanguageFont_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2972605498, 46957, 17819, [190, 235, 144, 17, 81, 205, 119, 209]);
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Text")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::UI::Text::FontWeight) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Text"))] usize,
    #[cfg(feature = "UI_Text")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::UI::Text::FontStretch) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Text"))] usize,
    #[cfg(feature = "UI_Text")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::UI::Text::FontStyle) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Text"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ILanguageFontGroup(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILanguageFontGroup {
    type Vtable = ILanguageFontGroup_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4080697283, 14940, 19178, [185, 255, 179, 159, 178, 66, 247, 246]);
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ILanguageFontGroupFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILanguageFontGroupFactory {
    type Vtable = ILanguageFontGroupFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4239305831, 20087, 18887, [184, 86, 221, 233, 52, 252, 115, 91]);
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, languagetag: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Globalization_Fonts`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct LanguageFont(::windows::runtime::IInspectable);
impl LanguageFont {
    #[doc = "*Required features: `Globalization_Fonts`*"]
    pub fn FontFamily(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "UI_Text")]
    #[doc = "*Required features: `Globalization_Fonts`, `UI_Text`*"]
    pub fn FontWeight(&self) -> ::windows::runtime::Result<super::super::UI::Text::FontWeight> {
        let this = self;
        unsafe {
            let mut result__: super::super::UI::Text::FontWeight = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Text::FontWeight>(result__)
        }
    }
    #[cfg(feature = "UI_Text")]
    #[doc = "*Required features: `Globalization_Fonts`, `UI_Text`*"]
    pub fn FontStretch(&self) -> ::windows::runtime::Result<super::super::UI::Text::FontStretch> {
        let this = self;
        unsafe {
            let mut result__: super::super::UI::Text::FontStretch = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Text::FontStretch>(result__)
        }
    }
    #[cfg(feature = "UI_Text")]
    #[doc = "*Required features: `Globalization_Fonts`, `UI_Text`*"]
    pub fn FontStyle(&self) -> ::windows::runtime::Result<super::super::UI::Text::FontStyle> {
        let this = self;
        unsafe {
            let mut result__: super::super::UI::Text::FontStyle = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Text::FontStyle>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_Fonts`*"]
    pub fn ScaleFactor(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LanguageFont {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Globalization.Fonts.LanguageFont;{b12e5c3a-b76d-459b-beeb-901151cd77d1})");
}
unsafe impl ::windows::runtime::Interface for LanguageFont {
    type Vtable = ILanguageFont_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2972605498, 46957, 17819, [190, 235, 144, 17, 81, 205, 119, 209]);
}
impl ::windows::runtime::RuntimeName for LanguageFont {
    const NAME: &'static str = "Windows.Globalization.Fonts.LanguageFont";
}
impl ::std::convert::From<LanguageFont> for ::windows::runtime::IUnknown {
    fn from(value: LanguageFont) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&LanguageFont> for ::windows::runtime::IUnknown {
    fn from(value: &LanguageFont) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for LanguageFont {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &LanguageFont {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<LanguageFont> for ::windows::runtime::IInspectable {
    fn from(value: LanguageFont) -> Self {
        value.0
    }
}
impl ::std::convert::From<&LanguageFont> for ::windows::runtime::IInspectable {
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
unsafe impl ::std::marker::Send for LanguageFont {}
unsafe impl ::std::marker::Sync for LanguageFont {}
#[doc = "*Required features: `Globalization_Fonts`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct LanguageFontGroup(::windows::runtime::IInspectable);
impl LanguageFontGroup {
    #[doc = "*Required features: `Globalization_Fonts`*"]
    pub fn UITextFont(&self) -> ::windows::runtime::Result<LanguageFont> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LanguageFont>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_Fonts`*"]
    pub fn UIHeadingFont(&self) -> ::windows::runtime::Result<LanguageFont> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LanguageFont>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_Fonts`*"]
    pub fn UITitleFont(&self) -> ::windows::runtime::Result<LanguageFont> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LanguageFont>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_Fonts`*"]
    pub fn UICaptionFont(&self) -> ::windows::runtime::Result<LanguageFont> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LanguageFont>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_Fonts`*"]
    pub fn UINotificationHeadingFont(&self) -> ::windows::runtime::Result<LanguageFont> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LanguageFont>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_Fonts`*"]
    pub fn TraditionalDocumentFont(&self) -> ::windows::runtime::Result<LanguageFont> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LanguageFont>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_Fonts`*"]
    pub fn ModernDocumentFont(&self) -> ::windows::runtime::Result<LanguageFont> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LanguageFont>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_Fonts`*"]
    pub fn DocumentHeadingFont(&self) -> ::windows::runtime::Result<LanguageFont> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LanguageFont>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_Fonts`*"]
    pub fn FixedWidthTextFont(&self) -> ::windows::runtime::Result<LanguageFont> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LanguageFont>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_Fonts`*"]
    pub fn DocumentAlternate1Font(&self) -> ::windows::runtime::Result<LanguageFont> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LanguageFont>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_Fonts`*"]
    pub fn DocumentAlternate2Font(&self) -> ::windows::runtime::Result<LanguageFont> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LanguageFont>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_Fonts`*"]
    pub fn CreateLanguageFontGroup<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(languagetag: Param0) -> ::windows::runtime::Result<LanguageFontGroup> {
        Self::ILanguageFontGroupFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), languagetag.into_param().abi(), &mut result__).from_abi::<LanguageFontGroup>(result__)
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4080697283, 14940, 19178, [185, 255, 179, 159, 178, 66, 247, 246]);
}
impl ::windows::runtime::RuntimeName for LanguageFontGroup {
    const NAME: &'static str = "Windows.Globalization.Fonts.LanguageFontGroup";
}
impl ::std::convert::From<LanguageFontGroup> for ::windows::runtime::IUnknown {
    fn from(value: LanguageFontGroup) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&LanguageFontGroup> for ::windows::runtime::IUnknown {
    fn from(value: &LanguageFontGroup) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for LanguageFontGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &LanguageFontGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<LanguageFontGroup> for ::windows::runtime::IInspectable {
    fn from(value: LanguageFontGroup) -> Self {
        value.0
    }
}
impl ::std::convert::From<&LanguageFontGroup> for ::windows::runtime::IInspectable {
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
unsafe impl ::std::marker::Send for LanguageFontGroup {}
unsafe impl ::std::marker::Sync for LanguageFontGroup {}
