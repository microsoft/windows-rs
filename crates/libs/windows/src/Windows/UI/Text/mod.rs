#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "UI_Text_Core")]
pub mod Core;
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CaretType(pub i32);
impl CaretType {
    pub const Normal: Self = Self(0i32);
    pub const Null: Self = Self(1i32);
}
impl ::core::marker::Copy for CaretType {}
impl ::core::clone::Clone for CaretType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CaretType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CaretType {
    type Abi = Self;
}
impl ::core::fmt::Debug for CaretType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CaretType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CaretType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Text.CaretType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
pub struct ContentLinkInfo(::windows::core::IUnknown);
impl ContentLinkInfo {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ContentLinkInfo, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetId(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetId)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn DisplayText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayText)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetDisplayText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDisplayText)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SecondaryText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SecondaryText)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetSecondaryText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSecondaryText)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Uri)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetUri)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn LinkContentKind(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LinkContentKind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetLinkContentKind<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLinkContentKind)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for ContentLinkInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContentLinkInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContentLinkInfo {}
impl ::core::fmt::Debug for ContentLinkInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContentLinkInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContentLinkInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Text.ContentLinkInfo;{1ed52525-1c5f-48cb-b335-78b50a2ee642})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ContentLinkInfo {
    type Vtable = IContentLinkInfo_Vtbl;
    const IID: ::windows::core::GUID = <IContentLinkInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContentLinkInfo {
    const NAME: &'static str = "Windows.UI.Text.ContentLinkInfo";
}
impl ::core::convert::From<ContentLinkInfo> for ::windows::core::IUnknown {
    fn from(value: ContentLinkInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContentLinkInfo> for ::windows::core::IUnknown {
    fn from(value: &ContentLinkInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContentLinkInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ContentLinkInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContentLinkInfo> for ::windows::core::IInspectable {
    fn from(value: ContentLinkInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContentLinkInfo> for ::windows::core::IInspectable {
    fn from(value: &ContentLinkInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContentLinkInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ContentLinkInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ContentLinkInfo {}
unsafe impl ::core::marker::Sync for ContentLinkInfo {}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FindOptions(pub u32);
impl FindOptions {
    pub const None: Self = Self(0u32);
    pub const Word: Self = Self(2u32);
    pub const Case: Self = Self(4u32);
}
impl ::core::marker::Copy for FindOptions {}
impl ::core::clone::Clone for FindOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FindOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FindOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for FindOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FindOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for FindOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for FindOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for FindOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for FindOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for FindOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for FindOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Text.FindOptions;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FontStretch(pub i32);
impl FontStretch {
    pub const Undefined: Self = Self(0i32);
    pub const UltraCondensed: Self = Self(1i32);
    pub const ExtraCondensed: Self = Self(2i32);
    pub const Condensed: Self = Self(3i32);
    pub const SemiCondensed: Self = Self(4i32);
    pub const Normal: Self = Self(5i32);
    pub const SemiExpanded: Self = Self(6i32);
    pub const Expanded: Self = Self(7i32);
    pub const ExtraExpanded: Self = Self(8i32);
    pub const UltraExpanded: Self = Self(9i32);
}
impl ::core::marker::Copy for FontStretch {}
impl ::core::clone::Clone for FontStretch {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FontStretch {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FontStretch {
    type Abi = Self;
}
impl ::core::fmt::Debug for FontStretch {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FontStretch").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FontStretch {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Text.FontStretch;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FontStyle(pub i32);
impl FontStyle {
    pub const Normal: Self = Self(0i32);
    pub const Oblique: Self = Self(1i32);
    pub const Italic: Self = Self(2i32);
}
impl ::core::marker::Copy for FontStyle {}
impl ::core::clone::Clone for FontStyle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FontStyle {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FontStyle {
    type Abi = Self;
}
impl ::core::fmt::Debug for FontStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FontStyle").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FontStyle {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Text.FontStyle;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
#[doc = "*Required features: `\"UI_Text\"`*"]
pub struct FontWeight {
    pub Weight: u16,
}
impl ::core::marker::Copy for FontWeight {}
impl ::core::clone::Clone for FontWeight {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FontWeight {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FontWeight").field("Weight", &self.Weight).finish()
    }
}
unsafe impl ::windows::core::Abi for FontWeight {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for FontWeight {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.UI.Text.FontWeight;u2)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for FontWeight {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FontWeight>()) == 0 }
    }
}
impl ::core::cmp::Eq for FontWeight {}
impl ::core::default::Default for FontWeight {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
pub struct FontWeights(::windows::core::IUnknown);
impl FontWeights {
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Black() -> ::windows::core::Result<FontWeight> {
        Self::IFontWeightsStatics(|this| unsafe {
            let mut result__: FontWeight = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Black)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FontWeight>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Bold() -> ::windows::core::Result<FontWeight> {
        Self::IFontWeightsStatics(|this| unsafe {
            let mut result__: FontWeight = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Bold)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FontWeight>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn ExtraBlack() -> ::windows::core::Result<FontWeight> {
        Self::IFontWeightsStatics(|this| unsafe {
            let mut result__: FontWeight = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtraBlack)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FontWeight>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn ExtraBold() -> ::windows::core::Result<FontWeight> {
        Self::IFontWeightsStatics(|this| unsafe {
            let mut result__: FontWeight = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtraBold)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FontWeight>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn ExtraLight() -> ::windows::core::Result<FontWeight> {
        Self::IFontWeightsStatics(|this| unsafe {
            let mut result__: FontWeight = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtraLight)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FontWeight>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Light() -> ::windows::core::Result<FontWeight> {
        Self::IFontWeightsStatics(|this| unsafe {
            let mut result__: FontWeight = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Light)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FontWeight>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Medium() -> ::windows::core::Result<FontWeight> {
        Self::IFontWeightsStatics(|this| unsafe {
            let mut result__: FontWeight = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Medium)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FontWeight>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Normal() -> ::windows::core::Result<FontWeight> {
        Self::IFontWeightsStatics(|this| unsafe {
            let mut result__: FontWeight = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Normal)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FontWeight>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SemiBold() -> ::windows::core::Result<FontWeight> {
        Self::IFontWeightsStatics(|this| unsafe {
            let mut result__: FontWeight = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SemiBold)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FontWeight>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SemiLight() -> ::windows::core::Result<FontWeight> {
        Self::IFontWeightsStatics(|this| unsafe {
            let mut result__: FontWeight = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SemiLight)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FontWeight>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Thin() -> ::windows::core::Result<FontWeight> {
        Self::IFontWeightsStatics(|this| unsafe {
            let mut result__: FontWeight = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Thin)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FontWeight>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IFontWeightsStatics<R, F: FnOnce(&IFontWeightsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FontWeights, IFontWeightsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for FontWeights {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FontWeights {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FontWeights {}
impl ::core::fmt::Debug for FontWeights {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FontWeights").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FontWeights {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Text.FontWeights;{7880a444-01ab-4997-8517-df822a0c45f1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for FontWeights {
    type Vtable = IFontWeights_Vtbl;
    const IID: ::windows::core::GUID = <IFontWeights as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FontWeights {
    const NAME: &'static str = "Windows.UI.Text.FontWeights";
}
impl ::core::convert::From<FontWeights> for ::windows::core::IUnknown {
    fn from(value: FontWeights) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FontWeights> for ::windows::core::IUnknown {
    fn from(value: &FontWeights) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FontWeights {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FontWeights {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FontWeights> for ::windows::core::IInspectable {
    fn from(value: FontWeights) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FontWeights> for ::windows::core::IInspectable {
    fn from(value: &FontWeights) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FontWeights {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FontWeights {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for FontWeights {}
unsafe impl ::core::marker::Sync for FontWeights {}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FormatEffect(pub i32);
impl FormatEffect {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const Toggle: Self = Self(2i32);
    pub const Undefined: Self = Self(3i32);
}
impl ::core::marker::Copy for FormatEffect {}
impl ::core::clone::Clone for FormatEffect {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FormatEffect {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FormatEffect {
    type Abi = Self;
}
impl ::core::fmt::Debug for FormatEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FormatEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FormatEffect {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Text.FormatEffect;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HorizontalCharacterAlignment(pub i32);
impl HorizontalCharacterAlignment {
    pub const Left: Self = Self(0i32);
    pub const Right: Self = Self(1i32);
    pub const Center: Self = Self(2i32);
}
impl ::core::marker::Copy for HorizontalCharacterAlignment {}
impl ::core::clone::Clone for HorizontalCharacterAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HorizontalCharacterAlignment {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HorizontalCharacterAlignment {
    type Abi = Self;
}
impl ::core::fmt::Debug for HorizontalCharacterAlignment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HorizontalCharacterAlignment").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HorizontalCharacterAlignment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Text.HorizontalCharacterAlignment;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContentLinkInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContentLinkInfo {
    type Vtable = IContentLinkInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ed52525_1c5f_48cb_b335_78b50a2ee642);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentLinkInfo_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub DisplayText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetDisplayText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SecondaryText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetSecondaryText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    #[cfg(feature = "Foundation")]
    pub SetUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetUri: usize,
    pub LinkContentKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetLinkContentKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFontWeights(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFontWeights {
    type Vtable = IFontWeights_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7880a444_01ab_4997_8517_df822a0c45f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFontWeights_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFontWeightsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFontWeightsStatics {
    type Vtable = IFontWeightsStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3b579d5_1ba9_48eb_9dad_c095e8c23ba3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFontWeightsStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Black: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FontWeight) -> ::windows::core::HRESULT,
    pub Bold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FontWeight) -> ::windows::core::HRESULT,
    pub ExtraBlack: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FontWeight) -> ::windows::core::HRESULT,
    pub ExtraBold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FontWeight) -> ::windows::core::HRESULT,
    pub ExtraLight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FontWeight) -> ::windows::core::HRESULT,
    pub Light: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FontWeight) -> ::windows::core::HRESULT,
    pub Medium: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FontWeight) -> ::windows::core::HRESULT,
    pub Normal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FontWeight) -> ::windows::core::HRESULT,
    pub SemiBold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FontWeight) -> ::windows::core::HRESULT,
    pub SemiLight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FontWeight) -> ::windows::core::HRESULT,
    pub Thin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FontWeight) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRichEditTextRange(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRichEditTextRange {
    type Vtable = IRichEditTextRange_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x374e3515_ba8a_4a6e_8c59_0dde3d0cf5cd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRichEditTextRange_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ContentLinkInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetContentLinkInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
pub struct ITextCharacterFormat(::windows::core::IUnknown);
impl ITextCharacterFormat {
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn AllCaps(&self) -> ::windows::core::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__: FormatEffect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AllCaps)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FormatEffect>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetAllCaps(&self, value: FormatEffect) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAllCaps)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn BackgroundColor(&self) -> ::windows::core::Result<super::Color> {
        let this = self;
        unsafe {
            let mut result__: super::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BackgroundColor)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Color>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetBackgroundColor<'a, Param0: ::windows::core::IntoParam<'a, super::Color>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBackgroundColor)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Bold(&self) -> ::windows::core::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__: FormatEffect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Bold)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FormatEffect>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetBold(&self, value: FormatEffect) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBold)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn FontStretch(&self) -> ::windows::core::Result<FontStretch> {
        let this = self;
        unsafe {
            let mut result__: FontStretch = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontStretch)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FontStretch>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetFontStretch(&self, value: FontStretch) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetFontStretch)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn FontStyle(&self) -> ::windows::core::Result<FontStyle> {
        let this = self;
        unsafe {
            let mut result__: FontStyle = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FontStyle)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FontStyle>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetFontStyle(&self, value: FontStyle) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetFontStyle)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn ForegroundColor(&self) -> ::windows::core::Result<super::Color> {
        let this = self;
        unsafe {
            let mut result__: super::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ForegroundColor)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Color>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetForegroundColor<'a, Param0: ::windows::core::IntoParam<'a, super::Color>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetForegroundColor)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Hidden(&self) -> ::windows::core::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__: FormatEffect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Hidden)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FormatEffect>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetHidden(&self, value: FormatEffect) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHidden)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Italic(&self) -> ::windows::core::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__: FormatEffect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Italic)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FormatEffect>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetItalic(&self, value: FormatEffect) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetItalic)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Kerning(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Kerning)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetKerning(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetKerning)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn LanguageTag(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LanguageTag)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetLanguageTag<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLanguageTag)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn LinkType(&self) -> ::windows::core::Result<LinkType> {
        let this = self;
        unsafe {
            let mut result__: LinkType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LinkType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LinkType>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetName)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Outline(&self) -> ::windows::core::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__: FormatEffect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Outline)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FormatEffect>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetOutline(&self, value: FormatEffect) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetOutline)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Position(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Position)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetPosition(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPosition)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn ProtectedText(&self) -> ::windows::core::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__: FormatEffect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProtectedText)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FormatEffect>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetProtectedText(&self, value: FormatEffect) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetProtectedText)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Size(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Size)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetSize(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSize)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SmallCaps(&self) -> ::windows::core::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__: FormatEffect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SmallCaps)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FormatEffect>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetSmallCaps(&self, value: FormatEffect) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSmallCaps)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Spacing(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Spacing)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetSpacing(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSpacing)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Strikethrough(&self) -> ::windows::core::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__: FormatEffect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Strikethrough)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FormatEffect>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetStrikethrough(&self, value: FormatEffect) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStrikethrough)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Subscript(&self) -> ::windows::core::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__: FormatEffect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Subscript)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FormatEffect>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetSubscript(&self, value: FormatEffect) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSubscript)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Superscript(&self) -> ::windows::core::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__: FormatEffect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Superscript)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FormatEffect>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetSuperscript(&self, value: FormatEffect) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSuperscript)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn TextScript(&self) -> ::windows::core::Result<TextScript> {
        let this = self;
        unsafe {
            let mut result__: TextScript = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TextScript)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TextScript>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetTextScript(&self, value: TextScript) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTextScript)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Underline(&self) -> ::windows::core::Result<UnderlineType> {
        let this = self;
        unsafe {
            let mut result__: UnderlineType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Underline)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UnderlineType>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetUnderline(&self, value: UnderlineType) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetUnderline)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Weight(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Weight)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetWeight(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetWeight)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetClone<'a, Param0: ::windows::core::IntoParam<'a, ITextCharacterFormat>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetClone)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn GetClone(&self) -> ::windows::core::Result<ITextCharacterFormat> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetClone)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ITextCharacterFormat>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn IsEqual<'a, Param0: ::windows::core::IntoParam<'a, ITextCharacterFormat>>(&self, format: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsEqual)(::core::mem::transmute_copy(this), format.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::convert::From<ITextCharacterFormat> for ::windows::core::IUnknown {
    fn from(value: ITextCharacterFormat) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextCharacterFormat> for ::windows::core::IUnknown {
    fn from(value: &ITextCharacterFormat) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITextCharacterFormat {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITextCharacterFormat {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITextCharacterFormat> for ::windows::core::IInspectable {
    fn from(value: ITextCharacterFormat) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextCharacterFormat> for ::windows::core::IInspectable {
    fn from(value: &ITextCharacterFormat) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ITextCharacterFormat {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ITextCharacterFormat {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITextCharacterFormat {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITextCharacterFormat {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextCharacterFormat {}
impl ::core::fmt::Debug for ITextCharacterFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextCharacterFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ITextCharacterFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{5adef3db-05fb-442d-8065-642afea02ced}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ITextCharacterFormat {
    type Vtable = ITextCharacterFormat_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5adef3db_05fb_442d_8065_642afea02ced);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextCharacterFormat_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AllCaps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows::core::HRESULT,
    pub SetAllCaps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows::core::HRESULT,
    pub BackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Color) -> ::windows::core::HRESULT,
    pub SetBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Color) -> ::windows::core::HRESULT,
    pub Bold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows::core::HRESULT,
    pub SetBold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows::core::HRESULT,
    pub FontStretch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FontStretch) -> ::windows::core::HRESULT,
    pub SetFontStretch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: FontStretch) -> ::windows::core::HRESULT,
    pub FontStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FontStyle) -> ::windows::core::HRESULT,
    pub SetFontStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: FontStyle) -> ::windows::core::HRESULT,
    pub ForegroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Color) -> ::windows::core::HRESULT,
    pub SetForegroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Color) -> ::windows::core::HRESULT,
    pub Hidden: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows::core::HRESULT,
    pub SetHidden: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows::core::HRESULT,
    pub Italic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows::core::HRESULT,
    pub SetItalic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows::core::HRESULT,
    pub Kerning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub SetKerning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    pub LanguageTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetLanguageTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub LinkType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LinkType) -> ::windows::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Outline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows::core::HRESULT,
    pub SetOutline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows::core::HRESULT,
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub SetPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    pub ProtectedText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows::core::HRESULT,
    pub SetProtectedText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows::core::HRESULT,
    pub Size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub SetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    pub SmallCaps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows::core::HRESULT,
    pub SetSmallCaps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows::core::HRESULT,
    pub Spacing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub SetSpacing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    pub Strikethrough: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows::core::HRESULT,
    pub SetStrikethrough: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows::core::HRESULT,
    pub Subscript: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows::core::HRESULT,
    pub SetSubscript: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows::core::HRESULT,
    pub Superscript: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows::core::HRESULT,
    pub SetSuperscript: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows::core::HRESULT,
    pub TextScript: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TextScript) -> ::windows::core::HRESULT,
    pub SetTextScript: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TextScript) -> ::windows::core::HRESULT,
    pub Underline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UnderlineType) -> ::windows::core::HRESULT,
    pub SetUnderline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: UnderlineType) -> ::windows::core::HRESULT,
    pub Weight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetWeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub SetClone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetClone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub IsEqual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextConstantsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextConstantsStatics {
    type Vtable = ITextConstantsStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x779e7c33_189d_4bfa_97c8_10db135d976e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextConstantsStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AutoColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Color) -> ::windows::core::HRESULT,
    pub MinUnitCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub MaxUnitCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub UndefinedColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Color) -> ::windows::core::HRESULT,
    pub UndefinedFloatValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub UndefinedInt32Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub UndefinedFontStretch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FontStretch) -> ::windows::core::HRESULT,
    pub UndefinedFontStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FontStyle) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
pub struct ITextDocument(::windows::core::IUnknown);
impl ITextDocument {
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn CaretType(&self) -> ::windows::core::Result<CaretType> {
        let this = self;
        unsafe {
            let mut result__: CaretType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CaretType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CaretType>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetCaretType(&self, value: CaretType) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCaretType)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn DefaultTabStop(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DefaultTabStop)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetDefaultTabStop(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDefaultTabStop)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Selection(&self) -> ::windows::core::Result<ITextSelection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Selection)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ITextSelection>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn UndoLimit(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UndoLimit)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetUndoLimit(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetUndoLimit)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn CanCopy(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanCopy)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn CanPaste(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanPaste)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn CanRedo(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanRedo)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn CanUndo(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanUndo)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn ApplyDisplayUpdates(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ApplyDisplayUpdates)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn BatchDisplayUpdates(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BatchDisplayUpdates)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn BeginUndoGroup(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).BeginUndoGroup)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn EndUndoGroup(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).EndUndoGroup)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn GetDefaultCharacterFormat(&self) -> ::windows::core::Result<ITextCharacterFormat> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDefaultCharacterFormat)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ITextCharacterFormat>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn GetDefaultParagraphFormat(&self) -> ::windows::core::Result<ITextParagraphFormat> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDefaultParagraphFormat)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ITextParagraphFormat>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn GetRange(&self, startposition: i32, endposition: i32) -> ::windows::core::Result<ITextRange> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetRange)(::core::mem::transmute_copy(this), startposition, endposition, &mut result__).from_abi::<ITextRange>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetRangeFromPoint<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Point>>(&self, point: Param0, options: PointOptions) -> ::windows::core::Result<ITextRange> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetRangeFromPoint)(::core::mem::transmute_copy(this), point.into_param().abi(), options, &mut result__).from_abi::<ITextRange>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn GetText(&self, options: TextGetOptions, value: &mut ::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).GetText)(::core::mem::transmute_copy(this), options, value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn LoadFromStream<'a, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>>(&self, options: TextSetOptions, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).LoadFromStream)(::core::mem::transmute_copy(this), options, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Redo(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Redo)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SaveToStream<'a, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>>(&self, options: TextGetOptions, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SaveToStream)(::core::mem::transmute_copy(this), options, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetDefaultCharacterFormat<'a, Param0: ::windows::core::IntoParam<'a, ITextCharacterFormat>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDefaultCharacterFormat)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetDefaultParagraphFormat<'a, Param0: ::windows::core::IntoParam<'a, ITextParagraphFormat>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDefaultParagraphFormat)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetText<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, options: TextSetOptions, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetText)(::core::mem::transmute_copy(this), options, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Undo(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Undo)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::convert::From<ITextDocument> for ::windows::core::IUnknown {
    fn from(value: ITextDocument) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextDocument> for ::windows::core::IUnknown {
    fn from(value: &ITextDocument) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITextDocument {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITextDocument {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITextDocument> for ::windows::core::IInspectable {
    fn from(value: ITextDocument) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextDocument> for ::windows::core::IInspectable {
    fn from(value: &ITextDocument) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ITextDocument {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ITextDocument {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITextDocument {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITextDocument {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextDocument {}
impl ::core::fmt::Debug for ITextDocument {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextDocument").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ITextDocument {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{beee4ddb-90b2-408c-a2f6-0a0ac31e33e4}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ITextDocument {
    type Vtable = ITextDocument_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbeee4ddb_90b2_408c_a2f6_0a0ac31e33e4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextDocument_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CaretType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CaretType) -> ::windows::core::HRESULT,
    pub SetCaretType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: CaretType) -> ::windows::core::HRESULT,
    pub DefaultTabStop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub SetDefaultTabStop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    pub Selection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub UndoLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetUndoLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub CanCopy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub CanPaste: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub CanRedo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub CanUndo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ApplyDisplayUpdates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub BatchDisplayUpdates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub BeginUndoGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EndUndoGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDefaultCharacterFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetDefaultParagraphFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startposition: i32, endposition: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetRangeFromPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, point: super::super::Foundation::Point, options: PointOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetRangeFromPoint: usize,
    pub GetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: TextGetOptions, value: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub LoadFromStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: TextSetOptions, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    LoadFromStream: usize,
    pub Redo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SaveToStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: TextGetOptions, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SaveToStream: usize,
    pub SetDefaultCharacterFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetDefaultParagraphFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: TextSetOptions, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Undo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextDocument2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextDocument2 {
    type Vtable = ITextDocument2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2311112_8c89_49c9_9118_f057cbb814ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextDocument2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AlignmentIncludesTrailingWhitespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAlignmentIncludesTrailingWhitespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IgnoreTrailingCharacterSpacing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIgnoreTrailingCharacterSpacing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextDocument3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextDocument3 {
    type Vtable = ITextDocument3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75ab03a1_a6f8_441d_aa18_0a851d6e5e3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextDocument3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ClearUndoRedoHistory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextDocument4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextDocument4 {
    type Vtable = ITextDocument4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x619c20f2_cb3b_4521_981f_2865b1b93f04);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextDocument4_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SetMath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetMath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetMathMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: RichEditMathMode) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
pub struct ITextParagraphFormat(::windows::core::IUnknown);
impl ITextParagraphFormat {
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Alignment(&self) -> ::windows::core::Result<ParagraphAlignment> {
        let this = self;
        unsafe {
            let mut result__: ParagraphAlignment = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Alignment)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ParagraphAlignment>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetAlignment(&self, value: ParagraphAlignment) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAlignment)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn FirstLineIndent(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FirstLineIndent)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn KeepTogether(&self) -> ::windows::core::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__: FormatEffect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeepTogether)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FormatEffect>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetKeepTogether(&self, value: FormatEffect) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetKeepTogether)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn KeepWithNext(&self) -> ::windows::core::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__: FormatEffect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeepWithNext)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FormatEffect>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetKeepWithNext(&self, value: FormatEffect) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetKeepWithNext)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn LeftIndent(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LeftIndent)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn LineSpacing(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LineSpacing)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn LineSpacingRule(&self) -> ::windows::core::Result<LineSpacingRule> {
        let this = self;
        unsafe {
            let mut result__: LineSpacingRule = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LineSpacingRule)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LineSpacingRule>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn ListAlignment(&self) -> ::windows::core::Result<MarkerAlignment> {
        let this = self;
        unsafe {
            let mut result__: MarkerAlignment = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ListAlignment)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MarkerAlignment>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetListAlignment(&self, value: MarkerAlignment) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetListAlignment)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn ListLevelIndex(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ListLevelIndex)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetListLevelIndex(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetListLevelIndex)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn ListStart(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ListStart)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetListStart(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetListStart)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn ListStyle(&self) -> ::windows::core::Result<MarkerStyle> {
        let this = self;
        unsafe {
            let mut result__: MarkerStyle = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ListStyle)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MarkerStyle>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetListStyle(&self, value: MarkerStyle) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetListStyle)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn ListTab(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ListTab)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetListTab(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetListTab)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn ListType(&self) -> ::windows::core::Result<MarkerType> {
        let this = self;
        unsafe {
            let mut result__: MarkerType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ListType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MarkerType>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetListType(&self, value: MarkerType) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetListType)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn NoLineNumber(&self) -> ::windows::core::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__: FormatEffect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NoLineNumber)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FormatEffect>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetNoLineNumber(&self, value: FormatEffect) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetNoLineNumber)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn PageBreakBefore(&self) -> ::windows::core::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__: FormatEffect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PageBreakBefore)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FormatEffect>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetPageBreakBefore(&self, value: FormatEffect) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPageBreakBefore)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn RightIndent(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RightIndent)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetRightIndent(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRightIndent)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn RightToLeft(&self) -> ::windows::core::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__: FormatEffect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RightToLeft)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FormatEffect>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetRightToLeft(&self, value: FormatEffect) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRightToLeft)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Style(&self) -> ::windows::core::Result<ParagraphStyle> {
        let this = self;
        unsafe {
            let mut result__: ParagraphStyle = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Style)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ParagraphStyle>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetStyle(&self, value: ParagraphStyle) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStyle)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SpaceAfter(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SpaceAfter)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetSpaceAfter(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSpaceAfter)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SpaceBefore(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SpaceBefore)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetSpaceBefore(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSpaceBefore)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn WidowControl(&self) -> ::windows::core::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__: FormatEffect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WidowControl)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FormatEffect>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetWidowControl(&self, value: FormatEffect) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetWidowControl)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn TabCount(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TabCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn AddTab(&self, position: f32, align: TabAlignment, leader: TabLeader) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddTab)(::core::mem::transmute_copy(this), position, align, leader).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn ClearAllTabs(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ClearAllTabs)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn DeleteTab(&self, position: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).DeleteTab)(::core::mem::transmute_copy(this), position).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn GetClone(&self) -> ::windows::core::Result<ITextParagraphFormat> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetClone)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ITextParagraphFormat>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn GetTab(&self, index: i32, position: &mut f32, align: &mut TabAlignment, leader: &mut TabLeader) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).GetTab)(::core::mem::transmute_copy(this), index, position, align, leader).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn IsEqual<'a, Param0: ::windows::core::IntoParam<'a, ITextParagraphFormat>>(&self, format: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsEqual)(::core::mem::transmute_copy(this), format.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetClone<'a, Param0: ::windows::core::IntoParam<'a, ITextParagraphFormat>>(&self, format: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetClone)(::core::mem::transmute_copy(this), format.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetIndents(&self, start: f32, left: f32, right: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIndents)(::core::mem::transmute_copy(this), start, left, right).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetLineSpacing(&self, rule: LineSpacingRule, spacing: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLineSpacing)(::core::mem::transmute_copy(this), rule, spacing).ok() }
    }
}
impl ::core::convert::From<ITextParagraphFormat> for ::windows::core::IUnknown {
    fn from(value: ITextParagraphFormat) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextParagraphFormat> for ::windows::core::IUnknown {
    fn from(value: &ITextParagraphFormat) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITextParagraphFormat {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITextParagraphFormat {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITextParagraphFormat> for ::windows::core::IInspectable {
    fn from(value: ITextParagraphFormat) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextParagraphFormat> for ::windows::core::IInspectable {
    fn from(value: &ITextParagraphFormat) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ITextParagraphFormat {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ITextParagraphFormat {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITextParagraphFormat {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITextParagraphFormat {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextParagraphFormat {}
impl ::core::fmt::Debug for ITextParagraphFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextParagraphFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ITextParagraphFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{2cf8cfa6-4676-498a-93f5-bbdbfc0bd883}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ITextParagraphFormat {
    type Vtable = ITextParagraphFormat_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2cf8cfa6_4676_498a_93f5_bbdbfc0bd883);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextParagraphFormat_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Alignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ParagraphAlignment) -> ::windows::core::HRESULT,
    pub SetAlignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ParagraphAlignment) -> ::windows::core::HRESULT,
    pub FirstLineIndent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub KeepTogether: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows::core::HRESULT,
    pub SetKeepTogether: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows::core::HRESULT,
    pub KeepWithNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows::core::HRESULT,
    pub SetKeepWithNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows::core::HRESULT,
    pub LeftIndent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub LineSpacing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub LineSpacingRule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LineSpacingRule) -> ::windows::core::HRESULT,
    pub ListAlignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MarkerAlignment) -> ::windows::core::HRESULT,
    pub SetListAlignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MarkerAlignment) -> ::windows::core::HRESULT,
    pub ListLevelIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetListLevelIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub ListStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetListStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub ListStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MarkerStyle) -> ::windows::core::HRESULT,
    pub SetListStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MarkerStyle) -> ::windows::core::HRESULT,
    pub ListTab: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub SetListTab: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    pub ListType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MarkerType) -> ::windows::core::HRESULT,
    pub SetListType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MarkerType) -> ::windows::core::HRESULT,
    pub NoLineNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows::core::HRESULT,
    pub SetNoLineNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows::core::HRESULT,
    pub PageBreakBefore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows::core::HRESULT,
    pub SetPageBreakBefore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows::core::HRESULT,
    pub RightIndent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub SetRightIndent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    pub RightToLeft: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows::core::HRESULT,
    pub SetRightToLeft: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows::core::HRESULT,
    pub Style: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ParagraphStyle) -> ::windows::core::HRESULT,
    pub SetStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ParagraphStyle) -> ::windows::core::HRESULT,
    pub SpaceAfter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub SetSpaceAfter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    pub SpaceBefore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub SetSpaceBefore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    pub WidowControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows::core::HRESULT,
    pub SetWidowControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows::core::HRESULT,
    pub TabCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub AddTab: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, position: f32, align: TabAlignment, leader: TabLeader) -> ::windows::core::HRESULT,
    pub ClearAllTabs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DeleteTab: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, position: f32) -> ::windows::core::HRESULT,
    pub GetClone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetTab: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, position: *mut f32, align: *mut TabAlignment, leader: *mut TabLeader) -> ::windows::core::HRESULT,
    pub IsEqual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetClone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetIndents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, start: f32, left: f32, right: f32) -> ::windows::core::HRESULT,
    pub SetLineSpacing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rule: LineSpacingRule, spacing: f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
pub struct ITextRange(::windows::core::IUnknown);
impl ITextRange {
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Character(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Character)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetCharacter(&self, value: u16) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCharacter)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn CharacterFormat(&self) -> ::windows::core::Result<ITextCharacterFormat> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CharacterFormat)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ITextCharacterFormat>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetCharacterFormat<'a, Param0: ::windows::core::IntoParam<'a, ITextCharacterFormat>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCharacterFormat)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn FormattedText(&self) -> ::windows::core::Result<ITextRange> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FormattedText)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ITextRange>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetFormattedText<'a, Param0: ::windows::core::IntoParam<'a, ITextRange>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetFormattedText)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn EndPosition(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EndPosition)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetEndPosition(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetEndPosition)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Gravity(&self) -> ::windows::core::Result<RangeGravity> {
        let this = self;
        unsafe {
            let mut result__: RangeGravity = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Gravity)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RangeGravity>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetGravity(&self, value: RangeGravity) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetGravity)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Length(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Length)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Link(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Link)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetLink<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLink)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn ParagraphFormat(&self) -> ::windows::core::Result<ITextParagraphFormat> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ParagraphFormat)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ITextParagraphFormat>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetParagraphFormat<'a, Param0: ::windows::core::IntoParam<'a, ITextParagraphFormat>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetParagraphFormat)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn StartPosition(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartPosition)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetStartPosition(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStartPosition)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn StoryLength(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StoryLength)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Text)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetText)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn CanPaste(&self, format: i32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanPaste)(::core::mem::transmute_copy(this), format, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn ChangeCase(&self, value: LetterCase) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ChangeCase)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Collapse(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Collapse)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Copy(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Copy)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Cut(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Cut)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Delete(&self, unit: TextRangeUnit, count: i32) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Delete)(::core::mem::transmute_copy(this), unit, count, &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn EndOf(&self, unit: TextRangeUnit, extend: bool) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EndOf)(::core::mem::transmute_copy(this), unit, extend, &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Expand(&self, unit: TextRangeUnit) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Expand)(::core::mem::transmute_copy(this), unit, &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn FindText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0, scanlength: i32, options: FindOptions) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindText)(::core::mem::transmute_copy(this), value.into_param().abi(), scanlength, options, &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn GetCharacterUtf32(&self, value: &mut u32, offset: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).GetCharacterUtf32)(::core::mem::transmute_copy(this), value, offset).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn GetClone(&self) -> ::windows::core::Result<ITextRange> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetClone)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ITextRange>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn GetIndex(&self, unit: TextRangeUnit) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetIndex)(::core::mem::transmute_copy(this), unit, &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetPoint(&self, horizontalalign: HorizontalCharacterAlignment, verticalalign: VerticalCharacterAlignment, options: PointOptions, point: &mut super::super::Foundation::Point) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).GetPoint)(::core::mem::transmute_copy(this), horizontalalign, verticalalign, options, point).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetRect(&self, options: PointOptions, rect: &mut super::super::Foundation::Rect, hit: &mut i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).GetRect)(::core::mem::transmute_copy(this), options, rect, hit).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn GetText(&self, options: TextGetOptions, value: &mut ::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).GetText)(::core::mem::transmute_copy(this), options, value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetTextViaStream<'a, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>>(&self, options: TextGetOptions, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).GetTextViaStream)(::core::mem::transmute_copy(this), options, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn InRange<'a, Param0: ::windows::core::IntoParam<'a, ITextRange>>(&self, range: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InRange)(::core::mem::transmute_copy(this), range.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn InsertImage<'a, Param4: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param5: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>>(&self, width: i32, height: i32, ascent: i32, verticalalign: VerticalCharacterAlignment, alternatetext: Param4, value: Param5) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).InsertImage)(::core::mem::transmute_copy(this), width, height, ascent, verticalalign, alternatetext.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn InStory<'a, Param0: ::windows::core::IntoParam<'a, ITextRange>>(&self, range: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InStory)(::core::mem::transmute_copy(this), range.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn IsEqual<'a, Param0: ::windows::core::IntoParam<'a, ITextRange>>(&self, range: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsEqual)(::core::mem::transmute_copy(this), range.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Move(&self, unit: TextRangeUnit, count: i32) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Move)(::core::mem::transmute_copy(this), unit, count, &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn MoveEnd(&self, unit: TextRangeUnit, count: i32) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MoveEnd)(::core::mem::transmute_copy(this), unit, count, &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn MoveStart(&self, unit: TextRangeUnit, count: i32) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MoveStart)(::core::mem::transmute_copy(this), unit, count, &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Paste(&self, format: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Paste)(::core::mem::transmute_copy(this), format).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn ScrollIntoView(&self, value: PointOptions) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ScrollIntoView)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn MatchSelection(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).MatchSelection)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetIndex(&self, unit: TextRangeUnit, index: i32, extend: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIndex)(::core::mem::transmute_copy(this), unit, index, extend).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPoint<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Point>>(&self, point: Param0, options: PointOptions, extend: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPoint)(::core::mem::transmute_copy(this), point.into_param().abi(), options, extend).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetRange(&self, startposition: i32, endposition: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRange)(::core::mem::transmute_copy(this), startposition, endposition).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetText2<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, options: TextSetOptions, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetText2)(::core::mem::transmute_copy(this), options, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetTextViaStream<'a, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>>(&self, options: TextSetOptions, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTextViaStream)(::core::mem::transmute_copy(this), options, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn StartOf(&self, unit: TextRangeUnit, extend: bool) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartOf)(::core::mem::transmute_copy(this), unit, extend, &mut result__).from_abi::<i32>(result__)
        }
    }
}
impl ::core::convert::From<ITextRange> for ::windows::core::IUnknown {
    fn from(value: ITextRange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextRange> for ::windows::core::IUnknown {
    fn from(value: &ITextRange) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITextRange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITextRange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITextRange> for ::windows::core::IInspectable {
    fn from(value: ITextRange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextRange> for ::windows::core::IInspectable {
    fn from(value: &ITextRange) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ITextRange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ITextRange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITextRange {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITextRange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextRange {}
impl ::core::fmt::Debug for ITextRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextRange").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ITextRange {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{5b9e4e57-c072-42a0-8945-af503ee54768}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ITextRange {
    type Vtable = ITextRange_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b9e4e57_c072_42a0_8945_af503ee54768);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextRange_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Character: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub SetCharacter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u16) -> ::windows::core::HRESULT,
    pub CharacterFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetCharacterFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub FormattedText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetFormattedText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub EndPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetEndPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub Gravity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RangeGravity) -> ::windows::core::HRESULT,
    pub SetGravity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: RangeGravity) -> ::windows::core::HRESULT,
    pub Length: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub Link: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ParagraphFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetParagraphFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub StartPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetStartPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub StoryLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub CanPaste: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: i32, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ChangeCase: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: LetterCase) -> ::windows::core::HRESULT,
    pub Collapse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub Copy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Cut: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, count: i32, result__: *mut i32) -> ::windows::core::HRESULT,
    pub EndOf: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, extend: bool, result__: *mut i32) -> ::windows::core::HRESULT,
    pub Expand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, result__: *mut i32) -> ::windows::core::HRESULT,
    pub FindText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, scanlength: i32, options: FindOptions, result__: *mut i32) -> ::windows::core::HRESULT,
    pub GetCharacterUtf32: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut u32, offset: i32) -> ::windows::core::HRESULT,
    pub GetClone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, horizontalalign: HorizontalCharacterAlignment, verticalalign: VerticalCharacterAlignment, options: PointOptions, point: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetPoint: usize,
    #[cfg(feature = "Foundation")]
    pub GetRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: PointOptions, rect: *mut super::super::Foundation::Rect, hit: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetRect: usize,
    pub GetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: TextGetOptions, value: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetTextViaStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: TextGetOptions, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetTextViaStream: usize,
    pub InRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, range: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub InsertImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: i32, height: i32, ascent: i32, verticalalign: VerticalCharacterAlignment, alternatetext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    InsertImage: usize,
    pub InStory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, range: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsEqual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, range: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Move: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, count: i32, result__: *mut i32) -> ::windows::core::HRESULT,
    pub MoveEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, count: i32, result__: *mut i32) -> ::windows::core::HRESULT,
    pub MoveStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, count: i32, result__: *mut i32) -> ::windows::core::HRESULT,
    pub Paste: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: i32) -> ::windows::core::HRESULT,
    pub ScrollIntoView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PointOptions) -> ::windows::core::HRESULT,
    pub MatchSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, index: i32, extend: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, point: super::super::Foundation::Point, options: PointOptions, extend: bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPoint: usize,
    pub SetRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startposition: i32, endposition: i32) -> ::windows::core::HRESULT,
    pub SetText2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: TextSetOptions, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SetTextViaStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: TextSetOptions, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetTextViaStream: usize,
    pub StartOf: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, extend: bool, result__: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
pub struct ITextSelection(::windows::core::IUnknown);
impl ITextSelection {
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Options(&self) -> ::windows::core::Result<SelectionOptions> {
        let this = self;
        unsafe {
            let mut result__: SelectionOptions = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Options)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SelectionOptions>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetOptions(&self, value: SelectionOptions) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetOptions)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Type(&self) -> ::windows::core::Result<SelectionType> {
        let this = self;
        unsafe {
            let mut result__: SelectionType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Type)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SelectionType>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn EndKey(&self, unit: TextRangeUnit, extend: bool) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EndKey)(::core::mem::transmute_copy(this), unit, extend, &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn HomeKey(&self, unit: TextRangeUnit, extend: bool) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HomeKey)(::core::mem::transmute_copy(this), unit, extend, &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn MoveDown(&self, unit: TextRangeUnit, count: i32, extend: bool) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MoveDown)(::core::mem::transmute_copy(this), unit, count, extend, &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn MoveLeft(&self, unit: TextRangeUnit, count: i32, extend: bool) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MoveLeft)(::core::mem::transmute_copy(this), unit, count, extend, &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn MoveRight(&self, unit: TextRangeUnit, count: i32, extend: bool) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MoveRight)(::core::mem::transmute_copy(this), unit, count, extend, &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn MoveUp(&self, unit: TextRangeUnit, count: i32, extend: bool) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MoveUp)(::core::mem::transmute_copy(this), unit, count, extend, &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn TypeText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).TypeText)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Character(&self) -> ::windows::core::Result<u16> {
        let this = &::windows::core::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Character)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetCharacter(&self, value: u16) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextRange>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetCharacter)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn CharacterFormat(&self) -> ::windows::core::Result<ITextCharacterFormat> {
        let this = &::windows::core::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CharacterFormat)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ITextCharacterFormat>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetCharacterFormat<'a, Param0: ::windows::core::IntoParam<'a, ITextCharacterFormat>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextRange>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetCharacterFormat)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn FormattedText(&self) -> ::windows::core::Result<ITextRange> {
        let this = &::windows::core::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FormattedText)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ITextRange>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetFormattedText<'a, Param0: ::windows::core::IntoParam<'a, ITextRange>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextRange>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetFormattedText)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn EndPosition(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EndPosition)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetEndPosition(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextRange>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetEndPosition)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Gravity(&self) -> ::windows::core::Result<RangeGravity> {
        let this = &::windows::core::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: RangeGravity = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Gravity)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RangeGravity>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetGravity(&self, value: RangeGravity) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextRange>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetGravity)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Length(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Length)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Link(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Link)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetLink<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextRange>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetLink)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn ParagraphFormat(&self) -> ::windows::core::Result<ITextParagraphFormat> {
        let this = &::windows::core::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ParagraphFormat)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ITextParagraphFormat>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetParagraphFormat<'a, Param0: ::windows::core::IntoParam<'a, ITextParagraphFormat>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextRange>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetParagraphFormat)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn StartPosition(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartPosition)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetStartPosition(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextRange>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetStartPosition)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn StoryLength(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StoryLength)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Text)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextRange>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetText)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn CanPaste(&self, format: i32) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanPaste)(::core::mem::transmute_copy(this), format, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn ChangeCase(&self, value: LetterCase) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextRange>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).ChangeCase)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Collapse(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextRange>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Collapse)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Copy(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextRange>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Copy)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Cut(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextRange>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Cut)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Delete(&self, unit: TextRangeUnit, count: i32) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Delete)(::core::mem::transmute_copy(this), unit, count, &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn EndOf(&self, unit: TextRangeUnit, extend: bool) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EndOf)(::core::mem::transmute_copy(this), unit, extend, &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Expand(&self, unit: TextRangeUnit) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Expand)(::core::mem::transmute_copy(this), unit, &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn FindText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0, scanlength: i32, options: FindOptions) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindText)(::core::mem::transmute_copy(this), value.into_param().abi(), scanlength, options, &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn GetCharacterUtf32(&self, value: &mut u32, offset: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextRange>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).GetCharacterUtf32)(::core::mem::transmute_copy(this), value, offset).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn GetClone(&self) -> ::windows::core::Result<ITextRange> {
        let this = &::windows::core::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetClone)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ITextRange>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn GetIndex(&self, unit: TextRangeUnit) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetIndex)(::core::mem::transmute_copy(this), unit, &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetPoint(&self, horizontalalign: HorizontalCharacterAlignment, verticalalign: VerticalCharacterAlignment, options: PointOptions, point: &mut super::super::Foundation::Point) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextRange>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).GetPoint)(::core::mem::transmute_copy(this), horizontalalign, verticalalign, options, point).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetRect(&self, options: PointOptions, rect: &mut super::super::Foundation::Rect, hit: &mut i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextRange>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).GetRect)(::core::mem::transmute_copy(this), options, rect, hit).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn GetText(&self, options: TextGetOptions, value: &mut ::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextRange>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).GetText)(::core::mem::transmute_copy(this), options, value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetTextViaStream<'a, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>>(&self, options: TextGetOptions, value: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextRange>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).GetTextViaStream)(::core::mem::transmute_copy(this), options, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn InRange<'a, Param0: ::windows::core::IntoParam<'a, ITextRange>>(&self, range: Param0) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InRange)(::core::mem::transmute_copy(this), range.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn InsertImage<'a, Param4: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param5: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>>(&self, width: i32, height: i32, ascent: i32, verticalalign: VerticalCharacterAlignment, alternatetext: Param4, value: Param5) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextRange>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).InsertImage)(::core::mem::transmute_copy(this), width, height, ascent, verticalalign, alternatetext.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn InStory<'a, Param0: ::windows::core::IntoParam<'a, ITextRange>>(&self, range: Param0) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InStory)(::core::mem::transmute_copy(this), range.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn IsEqual<'a, Param0: ::windows::core::IntoParam<'a, ITextRange>>(&self, range: Param0) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsEqual)(::core::mem::transmute_copy(this), range.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Move(&self, unit: TextRangeUnit, count: i32) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Move)(::core::mem::transmute_copy(this), unit, count, &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn MoveEnd(&self, unit: TextRangeUnit, count: i32) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MoveEnd)(::core::mem::transmute_copy(this), unit, count, &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn MoveStart(&self, unit: TextRangeUnit, count: i32) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MoveStart)(::core::mem::transmute_copy(this), unit, count, &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Paste(&self, format: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextRange>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Paste)(::core::mem::transmute_copy(this), format).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn ScrollIntoView(&self, value: PointOptions) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextRange>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).ScrollIntoView)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn MatchSelection(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextRange>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).MatchSelection)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetIndex(&self, unit: TextRangeUnit, index: i32, extend: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextRange>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetIndex)(::core::mem::transmute_copy(this), unit, index, extend).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPoint<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Point>>(&self, point: Param0, options: PointOptions, extend: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextRange>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPoint)(::core::mem::transmute_copy(this), point.into_param().abi(), options, extend).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetRange(&self, startposition: i32, endposition: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextRange>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetRange)(::core::mem::transmute_copy(this), startposition, endposition).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetText2<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, options: TextSetOptions, value: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextRange>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetText2)(::core::mem::transmute_copy(this), options, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetTextViaStream<'a, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>>(&self, options: TextSetOptions, value: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextRange>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetTextViaStream)(::core::mem::transmute_copy(this), options, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn StartOf(&self, unit: TextRangeUnit, extend: bool) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartOf)(::core::mem::transmute_copy(this), unit, extend, &mut result__).from_abi::<i32>(result__)
        }
    }
}
impl ::core::convert::From<ITextSelection> for ::windows::core::IUnknown {
    fn from(value: ITextSelection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextSelection> for ::windows::core::IUnknown {
    fn from(value: &ITextSelection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITextSelection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITextSelection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITextSelection> for ::windows::core::IInspectable {
    fn from(value: ITextSelection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextSelection> for ::windows::core::IInspectable {
    fn from(value: &ITextSelection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ITextSelection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ITextSelection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ITextSelection> for ITextRange {
    type Error = ::windows::core::Error;
    fn try_from(value: ITextSelection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ITextSelection> for ITextRange {
    type Error = ::windows::core::Error;
    fn try_from(value: &ITextSelection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITextRange> for ITextSelection {
    fn into_param(self) -> ::windows::core::Param<'a, ITextRange> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITextRange> for &ITextSelection {
    fn into_param(self) -> ::windows::core::Param<'a, ITextRange> {
        ::core::convert::TryInto::<ITextRange>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for ITextSelection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITextSelection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextSelection {}
impl ::core::fmt::Debug for ITextSelection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextSelection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ITextSelection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{a6d36724-f28f-430a-b2cf-c343671ec0e9}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ITextSelection {
    type Vtable = ITextSelection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6d36724_f28f_430a_b2cf_c343671ec0e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextSelection_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Options: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SelectionOptions) -> ::windows::core::HRESULT,
    pub SetOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SelectionOptions) -> ::windows::core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SelectionType) -> ::windows::core::HRESULT,
    pub EndKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, extend: bool, result__: *mut i32) -> ::windows::core::HRESULT,
    pub HomeKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, extend: bool, result__: *mut i32) -> ::windows::core::HRESULT,
    pub MoveDown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, count: i32, extend: bool, result__: *mut i32) -> ::windows::core::HRESULT,
    pub MoveLeft: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, count: i32, extend: bool, result__: *mut i32) -> ::windows::core::HRESULT,
    pub MoveRight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, count: i32, extend: bool, result__: *mut i32) -> ::windows::core::HRESULT,
    pub MoveUp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, count: i32, extend: bool, result__: *mut i32) -> ::windows::core::HRESULT,
    pub TypeText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LetterCase(pub i32);
impl LetterCase {
    pub const Lower: Self = Self(0i32);
    pub const Upper: Self = Self(1i32);
}
impl ::core::marker::Copy for LetterCase {}
impl ::core::clone::Clone for LetterCase {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LetterCase {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for LetterCase {
    type Abi = Self;
}
impl ::core::fmt::Debug for LetterCase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LetterCase").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LetterCase {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Text.LetterCase;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LineSpacingRule(pub i32);
impl LineSpacingRule {
    pub const Undefined: Self = Self(0i32);
    pub const Single: Self = Self(1i32);
    pub const OneAndHalf: Self = Self(2i32);
    pub const Double: Self = Self(3i32);
    pub const AtLeast: Self = Self(4i32);
    pub const Exactly: Self = Self(5i32);
    pub const Multiple: Self = Self(6i32);
    pub const Percent: Self = Self(7i32);
}
impl ::core::marker::Copy for LineSpacingRule {}
impl ::core::clone::Clone for LineSpacingRule {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LineSpacingRule {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for LineSpacingRule {
    type Abi = Self;
}
impl ::core::fmt::Debug for LineSpacingRule {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineSpacingRule").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LineSpacingRule {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Text.LineSpacingRule;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LinkType(pub i32);
impl LinkType {
    pub const Undefined: Self = Self(0i32);
    pub const NotALink: Self = Self(1i32);
    pub const ClientLink: Self = Self(2i32);
    pub const FriendlyLinkName: Self = Self(3i32);
    pub const FriendlyLinkAddress: Self = Self(4i32);
    pub const AutoLink: Self = Self(5i32);
    pub const AutoLinkEmail: Self = Self(6i32);
    pub const AutoLinkPhone: Self = Self(7i32);
    pub const AutoLinkPath: Self = Self(8i32);
}
impl ::core::marker::Copy for LinkType {}
impl ::core::clone::Clone for LinkType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LinkType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for LinkType {
    type Abi = Self;
}
impl ::core::fmt::Debug for LinkType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LinkType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LinkType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Text.LinkType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MarkerAlignment(pub i32);
impl MarkerAlignment {
    pub const Undefined: Self = Self(0i32);
    pub const Left: Self = Self(1i32);
    pub const Center: Self = Self(2i32);
    pub const Right: Self = Self(3i32);
}
impl ::core::marker::Copy for MarkerAlignment {}
impl ::core::clone::Clone for MarkerAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MarkerAlignment {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MarkerAlignment {
    type Abi = Self;
}
impl ::core::fmt::Debug for MarkerAlignment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MarkerAlignment").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MarkerAlignment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Text.MarkerAlignment;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MarkerStyle(pub i32);
impl MarkerStyle {
    pub const Undefined: Self = Self(0i32);
    pub const Parenthesis: Self = Self(1i32);
    pub const Parentheses: Self = Self(2i32);
    pub const Period: Self = Self(3i32);
    pub const Plain: Self = Self(4i32);
    pub const Minus: Self = Self(5i32);
    pub const NoNumber: Self = Self(6i32);
}
impl ::core::marker::Copy for MarkerStyle {}
impl ::core::clone::Clone for MarkerStyle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MarkerStyle {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MarkerStyle {
    type Abi = Self;
}
impl ::core::fmt::Debug for MarkerStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MarkerStyle").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MarkerStyle {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Text.MarkerStyle;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MarkerType(pub i32);
impl MarkerType {
    pub const Undefined: Self = Self(0i32);
    pub const None: Self = Self(1i32);
    pub const Bullet: Self = Self(2i32);
    pub const Arabic: Self = Self(3i32);
    pub const LowercaseEnglishLetter: Self = Self(4i32);
    pub const UppercaseEnglishLetter: Self = Self(5i32);
    pub const LowercaseRoman: Self = Self(6i32);
    pub const UppercaseRoman: Self = Self(7i32);
    pub const UnicodeSequence: Self = Self(8i32);
    pub const CircledNumber: Self = Self(9i32);
    pub const BlackCircleWingding: Self = Self(10i32);
    pub const WhiteCircleWingding: Self = Self(11i32);
    pub const ArabicWide: Self = Self(12i32);
    pub const SimplifiedChinese: Self = Self(13i32);
    pub const TraditionalChinese: Self = Self(14i32);
    pub const JapanSimplifiedChinese: Self = Self(15i32);
    pub const JapanKorea: Self = Self(16i32);
    pub const ArabicDictionary: Self = Self(17i32);
    pub const ArabicAbjad: Self = Self(18i32);
    pub const Hebrew: Self = Self(19i32);
    pub const ThaiAlphabetic: Self = Self(20i32);
    pub const ThaiNumeric: Self = Self(21i32);
    pub const DevanagariVowel: Self = Self(22i32);
    pub const DevanagariConsonant: Self = Self(23i32);
    pub const DevanagariNumeric: Self = Self(24i32);
}
impl ::core::marker::Copy for MarkerType {}
impl ::core::clone::Clone for MarkerType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MarkerType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MarkerType {
    type Abi = Self;
}
impl ::core::fmt::Debug for MarkerType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MarkerType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MarkerType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Text.MarkerType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ParagraphAlignment(pub i32);
impl ParagraphAlignment {
    pub const Undefined: Self = Self(0i32);
    pub const Left: Self = Self(1i32);
    pub const Center: Self = Self(2i32);
    pub const Right: Self = Self(3i32);
    pub const Justify: Self = Self(4i32);
}
impl ::core::marker::Copy for ParagraphAlignment {}
impl ::core::clone::Clone for ParagraphAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ParagraphAlignment {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ParagraphAlignment {
    type Abi = Self;
}
impl ::core::fmt::Debug for ParagraphAlignment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ParagraphAlignment").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ParagraphAlignment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Text.ParagraphAlignment;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ParagraphStyle(pub i32);
impl ParagraphStyle {
    pub const Undefined: Self = Self(0i32);
    pub const None: Self = Self(1i32);
    pub const Normal: Self = Self(2i32);
    pub const Heading1: Self = Self(3i32);
    pub const Heading2: Self = Self(4i32);
    pub const Heading3: Self = Self(5i32);
    pub const Heading4: Self = Self(6i32);
    pub const Heading5: Self = Self(7i32);
    pub const Heading6: Self = Self(8i32);
    pub const Heading7: Self = Self(9i32);
    pub const Heading8: Self = Self(10i32);
    pub const Heading9: Self = Self(11i32);
}
impl ::core::marker::Copy for ParagraphStyle {}
impl ::core::clone::Clone for ParagraphStyle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ParagraphStyle {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ParagraphStyle {
    type Abi = Self;
}
impl ::core::fmt::Debug for ParagraphStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ParagraphStyle").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ParagraphStyle {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Text.ParagraphStyle;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PointOptions(pub u32);
impl PointOptions {
    pub const None: Self = Self(0u32);
    pub const IncludeInset: Self = Self(1u32);
    pub const Start: Self = Self(32u32);
    pub const ClientCoordinates: Self = Self(256u32);
    pub const AllowOffClient: Self = Self(512u32);
    pub const Transform: Self = Self(1024u32);
    pub const NoHorizontalScroll: Self = Self(65536u32);
    pub const NoVerticalScroll: Self = Self(262144u32);
}
impl ::core::marker::Copy for PointOptions {}
impl ::core::clone::Clone for PointOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PointOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PointOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for PointOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PointOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PointOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PointOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PointOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PointOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PointOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for PointOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Text.PointOptions;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RangeGravity(pub i32);
impl RangeGravity {
    pub const UIBehavior: Self = Self(0i32);
    pub const Backward: Self = Self(1i32);
    pub const Forward: Self = Self(2i32);
    pub const Inward: Self = Self(3i32);
    pub const Outward: Self = Self(4i32);
}
impl ::core::marker::Copy for RangeGravity {}
impl ::core::clone::Clone for RangeGravity {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RangeGravity {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RangeGravity {
    type Abi = Self;
}
impl ::core::fmt::Debug for RangeGravity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RangeGravity").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RangeGravity {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Text.RangeGravity;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RichEditMathMode(pub i32);
impl RichEditMathMode {
    pub const NoMath: Self = Self(0i32);
    pub const MathOnly: Self = Self(1i32);
}
impl ::core::marker::Copy for RichEditMathMode {}
impl ::core::clone::Clone for RichEditMathMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RichEditMathMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RichEditMathMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for RichEditMathMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RichEditMathMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RichEditMathMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Text.RichEditMathMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
pub struct RichEditTextDocument(::windows::core::IUnknown);
impl RichEditTextDocument {
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn CaretType(&self) -> ::windows::core::Result<CaretType> {
        let this = self;
        unsafe {
            let mut result__: CaretType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CaretType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CaretType>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetCaretType(&self, value: CaretType) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCaretType)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn DefaultTabStop(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DefaultTabStop)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetDefaultTabStop(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDefaultTabStop)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Selection(&self) -> ::windows::core::Result<ITextSelection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Selection)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ITextSelection>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn UndoLimit(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UndoLimit)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetUndoLimit(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetUndoLimit)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn CanCopy(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanCopy)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn CanPaste(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanPaste)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn CanRedo(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanRedo)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn CanUndo(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanUndo)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn ApplyDisplayUpdates(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ApplyDisplayUpdates)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn BatchDisplayUpdates(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BatchDisplayUpdates)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn BeginUndoGroup(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).BeginUndoGroup)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn EndUndoGroup(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).EndUndoGroup)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn GetDefaultCharacterFormat(&self) -> ::windows::core::Result<ITextCharacterFormat> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDefaultCharacterFormat)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ITextCharacterFormat>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn GetDefaultParagraphFormat(&self) -> ::windows::core::Result<ITextParagraphFormat> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDefaultParagraphFormat)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ITextParagraphFormat>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn GetRange(&self, startposition: i32, endposition: i32) -> ::windows::core::Result<ITextRange> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetRange)(::core::mem::transmute_copy(this), startposition, endposition, &mut result__).from_abi::<ITextRange>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetRangeFromPoint<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Point>>(&self, point: Param0, options: PointOptions) -> ::windows::core::Result<ITextRange> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetRangeFromPoint)(::core::mem::transmute_copy(this), point.into_param().abi(), options, &mut result__).from_abi::<ITextRange>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn GetText(&self, options: TextGetOptions, value: &mut ::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).GetText)(::core::mem::transmute_copy(this), options, value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn LoadFromStream<'a, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>>(&self, options: TextSetOptions, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).LoadFromStream)(::core::mem::transmute_copy(this), options, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Redo(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Redo)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SaveToStream<'a, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>>(&self, options: TextGetOptions, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SaveToStream)(::core::mem::transmute_copy(this), options, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetDefaultCharacterFormat<'a, Param0: ::windows::core::IntoParam<'a, ITextCharacterFormat>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDefaultCharacterFormat)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetDefaultParagraphFormat<'a, Param0: ::windows::core::IntoParam<'a, ITextParagraphFormat>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDefaultParagraphFormat)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetText<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, options: TextSetOptions, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetText)(::core::mem::transmute_copy(this), options, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Undo(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Undo)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn AlignmentIncludesTrailingWhitespace(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextDocument2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AlignmentIncludesTrailingWhitespace)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetAlignmentIncludesTrailingWhitespace(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextDocument2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetAlignmentIncludesTrailingWhitespace)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn IgnoreTrailingCharacterSpacing(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextDocument2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IgnoreTrailingCharacterSpacing)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetIgnoreTrailingCharacterSpacing(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextDocument2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetIgnoreTrailingCharacterSpacing)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn ClearUndoRedoHistory(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextDocument3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).ClearUndoRedoHistory)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetMath<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextDocument4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetMath)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn GetMath(&self, value: &mut ::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextDocument4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).GetMath)(::core::mem::transmute_copy(this), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetMathMode(&self, mode: RichEditMathMode) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextDocument4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetMathMode)(::core::mem::transmute_copy(this), mode).ok() }
    }
}
impl ::core::clone::Clone for RichEditTextDocument {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RichEditTextDocument {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RichEditTextDocument {}
impl ::core::fmt::Debug for RichEditTextDocument {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RichEditTextDocument").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RichEditTextDocument {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Text.RichEditTextDocument;{beee4ddb-90b2-408c-a2f6-0a0ac31e33e4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for RichEditTextDocument {
    type Vtable = ITextDocument_Vtbl;
    const IID: ::windows::core::GUID = <ITextDocument as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RichEditTextDocument {
    const NAME: &'static str = "Windows.UI.Text.RichEditTextDocument";
}
impl ::core::convert::From<RichEditTextDocument> for ::windows::core::IUnknown {
    fn from(value: RichEditTextDocument) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RichEditTextDocument> for ::windows::core::IUnknown {
    fn from(value: &RichEditTextDocument) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RichEditTextDocument {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RichEditTextDocument {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RichEditTextDocument> for ::windows::core::IInspectable {
    fn from(value: RichEditTextDocument) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RichEditTextDocument> for ::windows::core::IInspectable {
    fn from(value: &RichEditTextDocument) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RichEditTextDocument {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RichEditTextDocument {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<RichEditTextDocument> for ITextDocument {
    type Error = ::windows::core::Error;
    fn try_from(value: RichEditTextDocument) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&RichEditTextDocument> for ITextDocument {
    type Error = ::windows::core::Error;
    fn try_from(value: &RichEditTextDocument) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITextDocument> for RichEditTextDocument {
    fn into_param(self) -> ::windows::core::Param<'a, ITextDocument> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITextDocument> for &RichEditTextDocument {
    fn into_param(self) -> ::windows::core::Param<'a, ITextDocument> {
        ::core::convert::TryInto::<ITextDocument>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for RichEditTextDocument {}
unsafe impl ::core::marker::Sync for RichEditTextDocument {}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
pub struct RichEditTextRange(::windows::core::IUnknown);
impl RichEditTextRange {
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn ContentLinkInfo(&self) -> ::windows::core::Result<ContentLinkInfo> {
        let this = &::windows::core::Interface::cast::<IRichEditTextRange>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContentLinkInfo)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ContentLinkInfo>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetContentLinkInfo<'a, Param0: ::windows::core::IntoParam<'a, ContentLinkInfo>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IRichEditTextRange>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetContentLinkInfo)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Character(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Character)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetCharacter(&self, value: u16) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCharacter)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn CharacterFormat(&self) -> ::windows::core::Result<ITextCharacterFormat> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CharacterFormat)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ITextCharacterFormat>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetCharacterFormat<'a, Param0: ::windows::core::IntoParam<'a, ITextCharacterFormat>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCharacterFormat)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn FormattedText(&self) -> ::windows::core::Result<ITextRange> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FormattedText)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ITextRange>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetFormattedText<'a, Param0: ::windows::core::IntoParam<'a, ITextRange>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetFormattedText)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn EndPosition(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EndPosition)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetEndPosition(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetEndPosition)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Gravity(&self) -> ::windows::core::Result<RangeGravity> {
        let this = self;
        unsafe {
            let mut result__: RangeGravity = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Gravity)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RangeGravity>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetGravity(&self, value: RangeGravity) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetGravity)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Length(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Length)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Link(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Link)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetLink<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLink)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn ParagraphFormat(&self) -> ::windows::core::Result<ITextParagraphFormat> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ParagraphFormat)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ITextParagraphFormat>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetParagraphFormat<'a, Param0: ::windows::core::IntoParam<'a, ITextParagraphFormat>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetParagraphFormat)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn StartPosition(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartPosition)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetStartPosition(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStartPosition)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn StoryLength(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StoryLength)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Text)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetText)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn CanPaste(&self, format: i32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanPaste)(::core::mem::transmute_copy(this), format, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn ChangeCase(&self, value: LetterCase) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ChangeCase)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Collapse(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Collapse)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Copy(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Copy)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Cut(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Cut)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Delete(&self, unit: TextRangeUnit, count: i32) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Delete)(::core::mem::transmute_copy(this), unit, count, &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn EndOf(&self, unit: TextRangeUnit, extend: bool) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EndOf)(::core::mem::transmute_copy(this), unit, extend, &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Expand(&self, unit: TextRangeUnit) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Expand)(::core::mem::transmute_copy(this), unit, &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn FindText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0, scanlength: i32, options: FindOptions) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindText)(::core::mem::transmute_copy(this), value.into_param().abi(), scanlength, options, &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn GetCharacterUtf32(&self, value: &mut u32, offset: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).GetCharacterUtf32)(::core::mem::transmute_copy(this), value, offset).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn GetClone(&self) -> ::windows::core::Result<ITextRange> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetClone)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ITextRange>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn GetIndex(&self, unit: TextRangeUnit) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetIndex)(::core::mem::transmute_copy(this), unit, &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetPoint(&self, horizontalalign: HorizontalCharacterAlignment, verticalalign: VerticalCharacterAlignment, options: PointOptions, point: &mut super::super::Foundation::Point) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).GetPoint)(::core::mem::transmute_copy(this), horizontalalign, verticalalign, options, point).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetRect(&self, options: PointOptions, rect: &mut super::super::Foundation::Rect, hit: &mut i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).GetRect)(::core::mem::transmute_copy(this), options, rect, hit).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn GetText(&self, options: TextGetOptions, value: &mut ::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).GetText)(::core::mem::transmute_copy(this), options, value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetTextViaStream<'a, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>>(&self, options: TextGetOptions, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).GetTextViaStream)(::core::mem::transmute_copy(this), options, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn InRange<'a, Param0: ::windows::core::IntoParam<'a, ITextRange>>(&self, range: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InRange)(::core::mem::transmute_copy(this), range.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn InsertImage<'a, Param4: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param5: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>>(&self, width: i32, height: i32, ascent: i32, verticalalign: VerticalCharacterAlignment, alternatetext: Param4, value: Param5) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).InsertImage)(::core::mem::transmute_copy(this), width, height, ascent, verticalalign, alternatetext.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn InStory<'a, Param0: ::windows::core::IntoParam<'a, ITextRange>>(&self, range: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InStory)(::core::mem::transmute_copy(this), range.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn IsEqual<'a, Param0: ::windows::core::IntoParam<'a, ITextRange>>(&self, range: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsEqual)(::core::mem::transmute_copy(this), range.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Move(&self, unit: TextRangeUnit, count: i32) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Move)(::core::mem::transmute_copy(this), unit, count, &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn MoveEnd(&self, unit: TextRangeUnit, count: i32) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MoveEnd)(::core::mem::transmute_copy(this), unit, count, &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn MoveStart(&self, unit: TextRangeUnit, count: i32) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MoveStart)(::core::mem::transmute_copy(this), unit, count, &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn Paste(&self, format: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Paste)(::core::mem::transmute_copy(this), format).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn ScrollIntoView(&self, value: PointOptions) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ScrollIntoView)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn MatchSelection(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).MatchSelection)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetIndex(&self, unit: TextRangeUnit, index: i32, extend: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIndex)(::core::mem::transmute_copy(this), unit, index, extend).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPoint<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Point>>(&self, point: Param0, options: PointOptions, extend: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPoint)(::core::mem::transmute_copy(this), point.into_param().abi(), options, extend).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetRange(&self, startposition: i32, endposition: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRange)(::core::mem::transmute_copy(this), startposition, endposition).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn SetText2<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, options: TextSetOptions, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetText2)(::core::mem::transmute_copy(this), options, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetTextViaStream<'a, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>>(&self, options: TextSetOptions, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTextViaStream)(::core::mem::transmute_copy(this), options, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn StartOf(&self, unit: TextRangeUnit, extend: bool) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartOf)(::core::mem::transmute_copy(this), unit, extend, &mut result__).from_abi::<i32>(result__)
        }
    }
}
impl ::core::clone::Clone for RichEditTextRange {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RichEditTextRange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RichEditTextRange {}
impl ::core::fmt::Debug for RichEditTextRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RichEditTextRange").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RichEditTextRange {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Text.RichEditTextRange;{5b9e4e57-c072-42a0-8945-af503ee54768})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for RichEditTextRange {
    type Vtable = ITextRange_Vtbl;
    const IID: ::windows::core::GUID = <ITextRange as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RichEditTextRange {
    const NAME: &'static str = "Windows.UI.Text.RichEditTextRange";
}
impl ::core::convert::From<RichEditTextRange> for ::windows::core::IUnknown {
    fn from(value: RichEditTextRange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RichEditTextRange> for ::windows::core::IUnknown {
    fn from(value: &RichEditTextRange) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RichEditTextRange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RichEditTextRange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RichEditTextRange> for ::windows::core::IInspectable {
    fn from(value: RichEditTextRange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RichEditTextRange> for ::windows::core::IInspectable {
    fn from(value: &RichEditTextRange) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RichEditTextRange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RichEditTextRange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<RichEditTextRange> for ITextRange {
    type Error = ::windows::core::Error;
    fn try_from(value: RichEditTextRange) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&RichEditTextRange> for ITextRange {
    type Error = ::windows::core::Error;
    fn try_from(value: &RichEditTextRange) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITextRange> for RichEditTextRange {
    fn into_param(self) -> ::windows::core::Param<'a, ITextRange> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITextRange> for &RichEditTextRange {
    fn into_param(self) -> ::windows::core::Param<'a, ITextRange> {
        ::core::convert::TryInto::<ITextRange>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for RichEditTextRange {}
unsafe impl ::core::marker::Sync for RichEditTextRange {}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SelectionOptions(pub u32);
impl SelectionOptions {
    pub const StartActive: Self = Self(1u32);
    pub const AtEndOfLine: Self = Self(2u32);
    pub const Overtype: Self = Self(4u32);
    pub const Active: Self = Self(8u32);
    pub const Replace: Self = Self(16u32);
}
impl ::core::marker::Copy for SelectionOptions {}
impl ::core::clone::Clone for SelectionOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SelectionOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SelectionOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for SelectionOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SelectionOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for SelectionOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SelectionOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SelectionOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SelectionOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SelectionOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for SelectionOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Text.SelectionOptions;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SelectionType(pub i32);
impl SelectionType {
    pub const None: Self = Self(0i32);
    pub const InsertionPoint: Self = Self(1i32);
    pub const Normal: Self = Self(2i32);
    pub const InlineShape: Self = Self(7i32);
    pub const Shape: Self = Self(8i32);
}
impl ::core::marker::Copy for SelectionType {}
impl ::core::clone::Clone for SelectionType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SelectionType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SelectionType {
    type Abi = Self;
}
impl ::core::fmt::Debug for SelectionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SelectionType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SelectionType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Text.SelectionType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TabAlignment(pub i32);
impl TabAlignment {
    pub const Left: Self = Self(0i32);
    pub const Center: Self = Self(1i32);
    pub const Right: Self = Self(2i32);
    pub const Decimal: Self = Self(3i32);
    pub const Bar: Self = Self(4i32);
}
impl ::core::marker::Copy for TabAlignment {}
impl ::core::clone::Clone for TabAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TabAlignment {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TabAlignment {
    type Abi = Self;
}
impl ::core::fmt::Debug for TabAlignment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TabAlignment").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TabAlignment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Text.TabAlignment;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TabLeader(pub i32);
impl TabLeader {
    pub const Spaces: Self = Self(0i32);
    pub const Dots: Self = Self(1i32);
    pub const Dashes: Self = Self(2i32);
    pub const Lines: Self = Self(3i32);
    pub const ThickLines: Self = Self(4i32);
    pub const Equals: Self = Self(5i32);
}
impl ::core::marker::Copy for TabLeader {}
impl ::core::clone::Clone for TabLeader {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TabLeader {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TabLeader {
    type Abi = Self;
}
impl ::core::fmt::Debug for TabLeader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TabLeader").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TabLeader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Text.TabLeader;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Text\"`*"]
pub struct TextConstants {}
impl TextConstants {
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn AutoColor() -> ::windows::core::Result<super::Color> {
        Self::ITextConstantsStatics(|this| unsafe {
            let mut result__: super::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AutoColor)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn MinUnitCount() -> ::windows::core::Result<i32> {
        Self::ITextConstantsStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MinUnitCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn MaxUnitCount() -> ::windows::core::Result<i32> {
        Self::ITextConstantsStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MaxUnitCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn UndefinedColor() -> ::windows::core::Result<super::Color> {
        Self::ITextConstantsStatics(|this| unsafe {
            let mut result__: super::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UndefinedColor)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Color>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn UndefinedFloatValue() -> ::windows::core::Result<f32> {
        Self::ITextConstantsStatics(|this| unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UndefinedFloatValue)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn UndefinedInt32Value() -> ::windows::core::Result<i32> {
        Self::ITextConstantsStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UndefinedInt32Value)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn UndefinedFontStretch() -> ::windows::core::Result<FontStretch> {
        Self::ITextConstantsStatics(|this| unsafe {
            let mut result__: FontStretch = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UndefinedFontStretch)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FontStretch>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Text\"`*"]
    pub fn UndefinedFontStyle() -> ::windows::core::Result<FontStyle> {
        Self::ITextConstantsStatics(|this| unsafe {
            let mut result__: FontStyle = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UndefinedFontStyle)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FontStyle>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITextConstantsStatics<R, F: FnOnce(&ITextConstantsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TextConstants, ITextConstantsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for TextConstants {
    const NAME: &'static str = "Windows.UI.Text.TextConstants";
}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TextDecorations(pub u32);
impl TextDecorations {
    pub const None: Self = Self(0u32);
    pub const Underline: Self = Self(1u32);
    pub const Strikethrough: Self = Self(2u32);
}
impl ::core::marker::Copy for TextDecorations {}
impl ::core::clone::Clone for TextDecorations {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TextDecorations {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TextDecorations {
    type Abi = Self;
}
impl ::core::fmt::Debug for TextDecorations {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TextDecorations").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for TextDecorations {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for TextDecorations {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for TextDecorations {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for TextDecorations {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for TextDecorations {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for TextDecorations {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Text.TextDecorations;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TextGetOptions(pub u32);
impl TextGetOptions {
    pub const None: Self = Self(0u32);
    pub const AdjustCrlf: Self = Self(1u32);
    pub const UseCrlf: Self = Self(2u32);
    pub const UseObjectText: Self = Self(4u32);
    pub const AllowFinalEop: Self = Self(8u32);
    pub const NoHidden: Self = Self(32u32);
    pub const IncludeNumbering: Self = Self(64u32);
    pub const FormatRtf: Self = Self(8192u32);
    pub const UseLf: Self = Self(16777216u32);
}
impl ::core::marker::Copy for TextGetOptions {}
impl ::core::clone::Clone for TextGetOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TextGetOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TextGetOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for TextGetOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TextGetOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for TextGetOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for TextGetOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for TextGetOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for TextGetOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for TextGetOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for TextGetOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Text.TextGetOptions;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TextRangeUnit(pub i32);
impl TextRangeUnit {
    pub const Character: Self = Self(0i32);
    pub const Word: Self = Self(1i32);
    pub const Sentence: Self = Self(2i32);
    pub const Paragraph: Self = Self(3i32);
    pub const Line: Self = Self(4i32);
    pub const Story: Self = Self(5i32);
    pub const Screen: Self = Self(6i32);
    pub const Section: Self = Self(7i32);
    pub const Window: Self = Self(8i32);
    pub const CharacterFormat: Self = Self(9i32);
    pub const ParagraphFormat: Self = Self(10i32);
    pub const Object: Self = Self(11i32);
    pub const HardParagraph: Self = Self(12i32);
    pub const Cluster: Self = Self(13i32);
    pub const Bold: Self = Self(14i32);
    pub const Italic: Self = Self(15i32);
    pub const Underline: Self = Self(16i32);
    pub const Strikethrough: Self = Self(17i32);
    pub const ProtectedText: Self = Self(18i32);
    pub const Link: Self = Self(19i32);
    pub const SmallCaps: Self = Self(20i32);
    pub const AllCaps: Self = Self(21i32);
    pub const Hidden: Self = Self(22i32);
    pub const Outline: Self = Self(23i32);
    pub const Shadow: Self = Self(24i32);
    pub const Imprint: Self = Self(25i32);
    pub const Disabled: Self = Self(26i32);
    pub const Revised: Self = Self(27i32);
    pub const Subscript: Self = Self(28i32);
    pub const Superscript: Self = Self(29i32);
    pub const FontBound: Self = Self(30i32);
    pub const LinkProtected: Self = Self(31i32);
    pub const ContentLink: Self = Self(32i32);
}
impl ::core::marker::Copy for TextRangeUnit {}
impl ::core::clone::Clone for TextRangeUnit {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TextRangeUnit {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TextRangeUnit {
    type Abi = Self;
}
impl ::core::fmt::Debug for TextRangeUnit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TextRangeUnit").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TextRangeUnit {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Text.TextRangeUnit;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TextScript(pub i32);
impl TextScript {
    pub const Undefined: Self = Self(0i32);
    pub const Ansi: Self = Self(1i32);
    pub const EastEurope: Self = Self(2i32);
    pub const Cyrillic: Self = Self(3i32);
    pub const Greek: Self = Self(4i32);
    pub const Turkish: Self = Self(5i32);
    pub const Hebrew: Self = Self(6i32);
    pub const Arabic: Self = Self(7i32);
    pub const Baltic: Self = Self(8i32);
    pub const Vietnamese: Self = Self(9i32);
    pub const Default: Self = Self(10i32);
    pub const Symbol: Self = Self(11i32);
    pub const Thai: Self = Self(12i32);
    pub const ShiftJis: Self = Self(13i32);
    pub const GB2312: Self = Self(14i32);
    pub const Hangul: Self = Self(15i32);
    pub const Big5: Self = Self(16i32);
    pub const PC437: Self = Self(17i32);
    pub const Oem: Self = Self(18i32);
    pub const Mac: Self = Self(19i32);
    pub const Armenian: Self = Self(20i32);
    pub const Syriac: Self = Self(21i32);
    pub const Thaana: Self = Self(22i32);
    pub const Devanagari: Self = Self(23i32);
    pub const Bengali: Self = Self(24i32);
    pub const Gurmukhi: Self = Self(25i32);
    pub const Gujarati: Self = Self(26i32);
    pub const Oriya: Self = Self(27i32);
    pub const Tamil: Self = Self(28i32);
    pub const Telugu: Self = Self(29i32);
    pub const Kannada: Self = Self(30i32);
    pub const Malayalam: Self = Self(31i32);
    pub const Sinhala: Self = Self(32i32);
    pub const Lao: Self = Self(33i32);
    pub const Tibetan: Self = Self(34i32);
    pub const Myanmar: Self = Self(35i32);
    pub const Georgian: Self = Self(36i32);
    pub const Jamo: Self = Self(37i32);
    pub const Ethiopic: Self = Self(38i32);
    pub const Cherokee: Self = Self(39i32);
    pub const Aboriginal: Self = Self(40i32);
    pub const Ogham: Self = Self(41i32);
    pub const Runic: Self = Self(42i32);
    pub const Khmer: Self = Self(43i32);
    pub const Mongolian: Self = Self(44i32);
    pub const Braille: Self = Self(45i32);
    pub const Yi: Self = Self(46i32);
    pub const Limbu: Self = Self(47i32);
    pub const TaiLe: Self = Self(48i32);
    pub const NewTaiLue: Self = Self(49i32);
    pub const SylotiNagri: Self = Self(50i32);
    pub const Kharoshthi: Self = Self(51i32);
    pub const Kayahli: Self = Self(52i32);
    pub const UnicodeSymbol: Self = Self(53i32);
    pub const Emoji: Self = Self(54i32);
    pub const Glagolitic: Self = Self(55i32);
    pub const Lisu: Self = Self(56i32);
    pub const Vai: Self = Self(57i32);
    pub const NKo: Self = Self(58i32);
    pub const Osmanya: Self = Self(59i32);
    pub const PhagsPa: Self = Self(60i32);
    pub const Gothic: Self = Self(61i32);
    pub const Deseret: Self = Self(62i32);
    pub const Tifinagh: Self = Self(63i32);
}
impl ::core::marker::Copy for TextScript {}
impl ::core::clone::Clone for TextScript {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TextScript {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TextScript {
    type Abi = Self;
}
impl ::core::fmt::Debug for TextScript {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TextScript").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TextScript {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Text.TextScript;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TextSetOptions(pub u32);
impl TextSetOptions {
    pub const None: Self = Self(0u32);
    pub const UnicodeBidi: Self = Self(1u32);
    pub const Unlink: Self = Self(8u32);
    pub const Unhide: Self = Self(16u32);
    pub const CheckTextLimit: Self = Self(32u32);
    pub const FormatRtf: Self = Self(8192u32);
    pub const ApplyRtfDocumentDefaults: Self = Self(16384u32);
}
impl ::core::marker::Copy for TextSetOptions {}
impl ::core::clone::Clone for TextSetOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TextSetOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TextSetOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for TextSetOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TextSetOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for TextSetOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for TextSetOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for TextSetOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for TextSetOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for TextSetOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for TextSetOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Text.TextSetOptions;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UnderlineType(pub i32);
impl UnderlineType {
    pub const Undefined: Self = Self(0i32);
    pub const None: Self = Self(1i32);
    pub const Single: Self = Self(2i32);
    pub const Words: Self = Self(3i32);
    pub const Double: Self = Self(4i32);
    pub const Dotted: Self = Self(5i32);
    pub const Dash: Self = Self(6i32);
    pub const DashDot: Self = Self(7i32);
    pub const DashDotDot: Self = Self(8i32);
    pub const Wave: Self = Self(9i32);
    pub const Thick: Self = Self(10i32);
    pub const Thin: Self = Self(11i32);
    pub const DoubleWave: Self = Self(12i32);
    pub const HeavyWave: Self = Self(13i32);
    pub const LongDash: Self = Self(14i32);
    pub const ThickDash: Self = Self(15i32);
    pub const ThickDashDot: Self = Self(16i32);
    pub const ThickDashDotDot: Self = Self(17i32);
    pub const ThickDotted: Self = Self(18i32);
    pub const ThickLongDash: Self = Self(19i32);
}
impl ::core::marker::Copy for UnderlineType {}
impl ::core::clone::Clone for UnderlineType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UnderlineType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UnderlineType {
    type Abi = Self;
}
impl ::core::fmt::Debug for UnderlineType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UnderlineType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UnderlineType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Text.UnderlineType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct VerticalCharacterAlignment(pub i32);
impl VerticalCharacterAlignment {
    pub const Top: Self = Self(0i32);
    pub const Baseline: Self = Self(1i32);
    pub const Bottom: Self = Self(2i32);
}
impl ::core::marker::Copy for VerticalCharacterAlignment {}
impl ::core::clone::Clone for VerticalCharacterAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VerticalCharacterAlignment {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VerticalCharacterAlignment {
    type Abi = Self;
}
impl ::core::fmt::Debug for VerticalCharacterAlignment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VerticalCharacterAlignment").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VerticalCharacterAlignment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Text.VerticalCharacterAlignment;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
