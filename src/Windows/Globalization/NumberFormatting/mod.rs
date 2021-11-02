#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Globalization_NumberFormatting`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct CurrencyFormatter(::windows::runtime::IInspectable);
impl CurrencyFormatter {
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn Currency(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn SetCurrency<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn Mode(&self) -> ::windows::runtime::Result<CurrencyFormatterMode> {
        let this = &::windows::runtime::Interface::cast::<ICurrencyFormatter2>(self)?;
        unsafe {
            let mut result__: CurrencyFormatterMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CurrencyFormatterMode>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn SetMode(&self, value: CurrencyFormatterMode) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICurrencyFormatter2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn ApplyRoundingForCurrency(&self, roundingalgorithm: RoundingAlgorithm) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICurrencyFormatter2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), roundingalgorithm).ok() }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn FormatInt(&self, value: i64) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatter>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn FormatUInt(&self, value: u64) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatter>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn FormatDouble(&self, value: f64) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatter>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn FormatInt2(&self, value: i64) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn FormatUInt2(&self, value: u64) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn FormatDouble2(&self, value: f64) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Globalization_NumberFormatting`, `Foundation_Collections`*"]
    pub fn Languages(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn GeographicRegion(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn IntegerDigits(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn SetIntegerDigits(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn FractionDigits(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn SetFractionDigits(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn IsGrouped(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn SetIsGrouped(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn IsDecimalPointAlwaysDisplayed(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn SetIsDecimalPointAlwaysDisplayed(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn NumeralSystem(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn SetNumeralSystem<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn ResolvedLanguage(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn ResolvedGeographicRegion(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Globalization_NumberFormatting`, `Foundation`*"]
    pub fn ParseInt<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, text: Param0) -> ::windows::runtime::Result<super::super::Foundation::IReference<i64>> {
        let this = &::windows::runtime::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IReference<i64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Globalization_NumberFormatting`, `Foundation`*"]
    pub fn ParseUInt<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, text: Param0) -> ::windows::runtime::Result<super::super::Foundation::IReference<u64>> {
        let this = &::windows::runtime::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IReference<u64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Globalization_NumberFormatting`, `Foundation`*"]
    pub fn ParseDouble<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, text: Param0) -> ::windows::runtime::Result<super::super::Foundation::IReference<f64>> {
        let this = &::windows::runtime::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn NumberRounder(&self) -> ::windows::runtime::Result<INumberRounder> {
        let this = &::windows::runtime::Interface::cast::<INumberRounderOption>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<INumberRounder>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn SetNumberRounder<'a, Param0: ::windows::runtime::IntoParam<'a, INumberRounder>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<INumberRounderOption>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn IsZeroSigned(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ISignedZeroOption>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn SetIsZeroSigned(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISignedZeroOption>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn SignificantDigits(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ISignificantDigitsOption>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn SetSignificantDigits(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISignificantDigitsOption>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn CreateCurrencyFormatterCode<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(currencycode: Param0) -> ::windows::runtime::Result<CurrencyFormatter> {
        Self::ICurrencyFormatterFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), currencycode.into_param().abi(), &mut result__).from_abi::<CurrencyFormatter>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Globalization_NumberFormatting`, `Foundation_Collections`*"]
    pub fn CreateCurrencyFormatterCodeContext<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(currencycode: Param0, languages: Param1, geographicregion: Param2) -> ::windows::runtime::Result<CurrencyFormatter> {
        Self::ICurrencyFormatterFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), currencycode.into_param().abi(), languages.into_param().abi(), geographicregion.into_param().abi(), &mut result__).from_abi::<CurrencyFormatter>(result__)
        })
    }
    pub fn ICurrencyFormatterFactory<R, F: FnOnce(&ICurrencyFormatterFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<CurrencyFormatter, ICurrencyFormatterFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CurrencyFormatter {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Globalization.NumberFormatting.CurrencyFormatter;{11730ca5-4b00-41b2-b332-73b12a497d54})");
}
unsafe impl ::windows::runtime::Interface for CurrencyFormatter {
    type Vtable = ICurrencyFormatter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(292752549, 19200, 16818, [179, 50, 115, 177, 42, 73, 125, 84]);
}
impl ::windows::runtime::RuntimeName for CurrencyFormatter {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.CurrencyFormatter";
}
impl ::std::convert::From<CurrencyFormatter> for ::windows::runtime::IUnknown {
    fn from(value: CurrencyFormatter) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CurrencyFormatter> for ::windows::runtime::IUnknown {
    fn from(value: &CurrencyFormatter) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CurrencyFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &CurrencyFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<CurrencyFormatter> for ::windows::runtime::IInspectable {
    fn from(value: CurrencyFormatter) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CurrencyFormatter> for ::windows::runtime::IInspectable {
    fn from(value: &CurrencyFormatter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CurrencyFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CurrencyFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<CurrencyFormatter> for INumberFormatter {
    type Error = ::windows::runtime::Error;
    fn try_from(value: CurrencyFormatter) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&CurrencyFormatter> for INumberFormatter {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &CurrencyFormatter) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INumberFormatter> for CurrencyFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, INumberFormatter> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INumberFormatter> for &CurrencyFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, INumberFormatter> {
        ::std::convert::TryInto::<INumberFormatter>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<CurrencyFormatter> for INumberFormatter2 {
    type Error = ::windows::runtime::Error;
    fn try_from(value: CurrencyFormatter) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&CurrencyFormatter> for INumberFormatter2 {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &CurrencyFormatter) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INumberFormatter2> for CurrencyFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, INumberFormatter2> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INumberFormatter2> for &CurrencyFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, INumberFormatter2> {
        ::std::convert::TryInto::<INumberFormatter2>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<CurrencyFormatter> for INumberFormatterOptions {
    type Error = ::windows::runtime::Error;
    fn try_from(value: CurrencyFormatter) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&CurrencyFormatter> for INumberFormatterOptions {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &CurrencyFormatter) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INumberFormatterOptions> for CurrencyFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, INumberFormatterOptions> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INumberFormatterOptions> for &CurrencyFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, INumberFormatterOptions> {
        ::std::convert::TryInto::<INumberFormatterOptions>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<CurrencyFormatter> for INumberParser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: CurrencyFormatter) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&CurrencyFormatter> for INumberParser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &CurrencyFormatter) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INumberParser> for CurrencyFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, INumberParser> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INumberParser> for &CurrencyFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, INumberParser> {
        ::std::convert::TryInto::<INumberParser>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<CurrencyFormatter> for INumberRounderOption {
    type Error = ::windows::runtime::Error;
    fn try_from(value: CurrencyFormatter) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&CurrencyFormatter> for INumberRounderOption {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &CurrencyFormatter) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INumberRounderOption> for CurrencyFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, INumberRounderOption> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INumberRounderOption> for &CurrencyFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, INumberRounderOption> {
        ::std::convert::TryInto::<INumberRounderOption>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<CurrencyFormatter> for ISignedZeroOption {
    type Error = ::windows::runtime::Error;
    fn try_from(value: CurrencyFormatter) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&CurrencyFormatter> for ISignedZeroOption {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &CurrencyFormatter) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISignedZeroOption> for CurrencyFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISignedZeroOption> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISignedZeroOption> for &CurrencyFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISignedZeroOption> {
        ::std::convert::TryInto::<ISignedZeroOption>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<CurrencyFormatter> for ISignificantDigitsOption {
    type Error = ::windows::runtime::Error;
    fn try_from(value: CurrencyFormatter) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&CurrencyFormatter> for ISignificantDigitsOption {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &CurrencyFormatter) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISignificantDigitsOption> for CurrencyFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISignificantDigitsOption> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISignificantDigitsOption> for &CurrencyFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISignificantDigitsOption> {
        ::std::convert::TryInto::<ISignificantDigitsOption>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for CurrencyFormatter {}
unsafe impl ::std::marker::Sync for CurrencyFormatter {}
#[doc = "*Required features: `Globalization_NumberFormatting`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CurrencyFormatterMode(pub i32);
impl CurrencyFormatterMode {
    pub const UseSymbol: CurrencyFormatterMode = CurrencyFormatterMode(0i32);
    pub const UseCurrencyCode: CurrencyFormatterMode = CurrencyFormatterMode(1i32);
}
impl ::std::convert::From<i32> for CurrencyFormatterMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CurrencyFormatterMode {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CurrencyFormatterMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Globalization.NumberFormatting.CurrencyFormatterMode;i4)");
}
#[doc = "*Required features: `Globalization_NumberFormatting`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct DecimalFormatter(::windows::runtime::IInspectable);
impl DecimalFormatter {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<DecimalFormatter, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn FormatInt(&self, value: i64) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn FormatUInt(&self, value: u64) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn FormatDouble(&self, value: f64) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn FormatInt2(&self, value: i64) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn FormatUInt2(&self, value: u64) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn FormatDouble2(&self, value: f64) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Globalization_NumberFormatting`, `Foundation_Collections`*"]
    pub fn Languages(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn GeographicRegion(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn IntegerDigits(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn SetIntegerDigits(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn FractionDigits(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn SetFractionDigits(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn IsGrouped(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn SetIsGrouped(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn IsDecimalPointAlwaysDisplayed(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn SetIsDecimalPointAlwaysDisplayed(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn NumeralSystem(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn SetNumeralSystem<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn ResolvedLanguage(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn ResolvedGeographicRegion(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Globalization_NumberFormatting`, `Foundation`*"]
    pub fn ParseInt<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, text: Param0) -> ::windows::runtime::Result<super::super::Foundation::IReference<i64>> {
        let this = &::windows::runtime::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IReference<i64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Globalization_NumberFormatting`, `Foundation`*"]
    pub fn ParseUInt<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, text: Param0) -> ::windows::runtime::Result<super::super::Foundation::IReference<u64>> {
        let this = &::windows::runtime::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IReference<u64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Globalization_NumberFormatting`, `Foundation`*"]
    pub fn ParseDouble<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, text: Param0) -> ::windows::runtime::Result<super::super::Foundation::IReference<f64>> {
        let this = &::windows::runtime::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn NumberRounder(&self) -> ::windows::runtime::Result<INumberRounder> {
        let this = &::windows::runtime::Interface::cast::<INumberRounderOption>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<INumberRounder>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn SetNumberRounder<'a, Param0: ::windows::runtime::IntoParam<'a, INumberRounder>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<INumberRounderOption>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn IsZeroSigned(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ISignedZeroOption>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn SetIsZeroSigned(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISignedZeroOption>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn SignificantDigits(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ISignificantDigitsOption>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn SetSignificantDigits(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISignificantDigitsOption>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Globalization_NumberFormatting`, `Foundation_Collections`*"]
    pub fn CreateDecimalFormatter<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(languages: Param0, geographicregion: Param1) -> ::windows::runtime::Result<DecimalFormatter> {
        Self::IDecimalFormatterFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), languages.into_param().abi(), geographicregion.into_param().abi(), &mut result__).from_abi::<DecimalFormatter>(result__)
        })
    }
    pub fn IDecimalFormatterFactory<R, F: FnOnce(&IDecimalFormatterFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<DecimalFormatter, IDecimalFormatterFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DecimalFormatter {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Globalization.NumberFormatting.DecimalFormatter;{a5007c49-7676-4db7-8631-1b6ff265caa9})");
}
unsafe impl ::windows::runtime::Interface for DecimalFormatter {
    type Vtable = INumberFormatter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2768272457, 30326, 19895, [134, 49, 27, 111, 242, 101, 202, 169]);
}
impl ::windows::runtime::RuntimeName for DecimalFormatter {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.DecimalFormatter";
}
impl ::std::convert::From<DecimalFormatter> for ::windows::runtime::IUnknown {
    fn from(value: DecimalFormatter) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&DecimalFormatter> for ::windows::runtime::IUnknown {
    fn from(value: &DecimalFormatter) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DecimalFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &DecimalFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<DecimalFormatter> for ::windows::runtime::IInspectable {
    fn from(value: DecimalFormatter) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DecimalFormatter> for ::windows::runtime::IInspectable {
    fn from(value: &DecimalFormatter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DecimalFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DecimalFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<DecimalFormatter> for INumberFormatter {
    fn from(value: DecimalFormatter) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&DecimalFormatter> for INumberFormatter {
    fn from(value: &DecimalFormatter) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INumberFormatter> for DecimalFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, INumberFormatter> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<INumberFormatter>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INumberFormatter> for &DecimalFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, INumberFormatter> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<INumberFormatter>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::TryFrom<DecimalFormatter> for INumberFormatter2 {
    type Error = ::windows::runtime::Error;
    fn try_from(value: DecimalFormatter) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&DecimalFormatter> for INumberFormatter2 {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &DecimalFormatter) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INumberFormatter2> for DecimalFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, INumberFormatter2> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INumberFormatter2> for &DecimalFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, INumberFormatter2> {
        ::std::convert::TryInto::<INumberFormatter2>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<DecimalFormatter> for INumberFormatterOptions {
    type Error = ::windows::runtime::Error;
    fn try_from(value: DecimalFormatter) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&DecimalFormatter> for INumberFormatterOptions {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &DecimalFormatter) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INumberFormatterOptions> for DecimalFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, INumberFormatterOptions> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INumberFormatterOptions> for &DecimalFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, INumberFormatterOptions> {
        ::std::convert::TryInto::<INumberFormatterOptions>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<DecimalFormatter> for INumberParser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: DecimalFormatter) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&DecimalFormatter> for INumberParser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &DecimalFormatter) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INumberParser> for DecimalFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, INumberParser> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INumberParser> for &DecimalFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, INumberParser> {
        ::std::convert::TryInto::<INumberParser>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<DecimalFormatter> for INumberRounderOption {
    type Error = ::windows::runtime::Error;
    fn try_from(value: DecimalFormatter) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&DecimalFormatter> for INumberRounderOption {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &DecimalFormatter) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INumberRounderOption> for DecimalFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, INumberRounderOption> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INumberRounderOption> for &DecimalFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, INumberRounderOption> {
        ::std::convert::TryInto::<INumberRounderOption>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<DecimalFormatter> for ISignedZeroOption {
    type Error = ::windows::runtime::Error;
    fn try_from(value: DecimalFormatter) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&DecimalFormatter> for ISignedZeroOption {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &DecimalFormatter) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISignedZeroOption> for DecimalFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISignedZeroOption> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISignedZeroOption> for &DecimalFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISignedZeroOption> {
        ::std::convert::TryInto::<ISignedZeroOption>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<DecimalFormatter> for ISignificantDigitsOption {
    type Error = ::windows::runtime::Error;
    fn try_from(value: DecimalFormatter) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&DecimalFormatter> for ISignificantDigitsOption {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &DecimalFormatter) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISignificantDigitsOption> for DecimalFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISignificantDigitsOption> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISignificantDigitsOption> for &DecimalFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISignificantDigitsOption> {
        ::std::convert::TryInto::<ISignificantDigitsOption>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for DecimalFormatter {}
unsafe impl ::std::marker::Sync for DecimalFormatter {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ICurrencyFormatter(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICurrencyFormatter {
    type Vtable = ICurrencyFormatter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(292752549, 19200, 16818, [179, 50, 115, 177, 42, 73, 125, 84]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrencyFormatter_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ICurrencyFormatter2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICurrencyFormatter2 {
    type Vtable = ICurrencyFormatter2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(120336157, 59322, 16791, [146, 14, 36, 124, 146, 247, 222, 166]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrencyFormatter2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut CurrencyFormatterMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: CurrencyFormatterMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, roundingalgorithm: RoundingAlgorithm) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ICurrencyFormatterFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICurrencyFormatterFactory {
    type Vtable = ICurrencyFormatterFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2261209982, 47416, 19106, [132, 176, 44, 51, 220, 91, 20, 80]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrencyFormatterFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, currencycode: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, currencycode: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, languages: ::windows::runtime::RawPtr, geographicregion: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IDecimalFormatterFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDecimalFormatterFactory {
    type Vtable = IDecimalFormatterFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(218205338, 58259, 18104, [184, 48, 122, 105, 200, 248, 159, 187]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDecimalFormatterFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, languages: ::windows::runtime::RawPtr, geographicregion: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IIncrementNumberRounder(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IIncrementNumberRounder {
    type Vtable = IIncrementNumberRounder_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1889947640, 26283, 16725, [157, 161, 115, 158, 70, 118, 69, 67]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIncrementNumberRounder_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut RoundingAlgorithm) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: RoundingAlgorithm) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Globalization_NumberFormatting`*"]
pub struct INumberFormatter(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INumberFormatter {
    type Vtable = INumberFormatter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2768272457, 30326, 19895, [134, 49, 27, 111, 242, 101, 202, 169]);
}
impl INumberFormatter {
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn FormatInt(&self, value: i64) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn FormatUInt(&self, value: u64) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn FormatDouble(&self, value: f64) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for INumberFormatter {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{a5007c49-7676-4db7-8631-1b6ff265caa9}");
}
#[repr(C)]
#[doc(hidden)]
pub struct INumberFormatter_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i64, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u64, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Globalization_NumberFormatting`*"]
pub struct INumberFormatter2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INumberFormatter2 {
    type Vtable = INumberFormatter2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3567829488, 32976, 19213, [168, 158, 136, 44, 30, 143, 131, 16]);
}
impl INumberFormatter2 {
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn FormatInt(&self, value: i64) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn FormatUInt(&self, value: u64) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn FormatDouble(&self, value: f64) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for INumberFormatter2 {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{d4a8c1f0-80d0-4b0d-a89e-882c1e8f8310}");
}
#[repr(C)]
#[doc(hidden)]
pub struct INumberFormatter2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i64, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u64, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Globalization_NumberFormatting`*"]
pub struct INumberFormatterOptions(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INumberFormatterOptions {
    type Vtable = INumberFormatterOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2150837537, 44769, 19001, [186, 162, 7, 237, 140, 150, 218, 246]);
}
impl INumberFormatterOptions {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Globalization_NumberFormatting`, `Foundation_Collections`*"]
    pub fn Languages(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn GeographicRegion(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn IntegerDigits(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn SetIntegerDigits(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn FractionDigits(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn SetFractionDigits(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn IsGrouped(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn SetIsGrouped(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn IsDecimalPointAlwaysDisplayed(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn SetIsDecimalPointAlwaysDisplayed(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn NumeralSystem(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn SetNumeralSystem<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn ResolvedLanguage(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn ResolvedGeographicRegion(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for INumberFormatterOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{80332d21-aee1-4a39-baa2-07ed8c96daf6}");
}
#[repr(C)]
#[doc(hidden)]
pub struct INumberFormatterOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Globalization_NumberFormatting`*"]
pub struct INumberParser(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INumberParser {
    type Vtable = INumberParser_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3865416722, 18963, 19027, [131, 161, 57, 47, 190, 76, 255, 159]);
}
impl INumberParser {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Globalization_NumberFormatting`, `Foundation`*"]
    pub fn ParseInt<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, text: Param0) -> ::windows::runtime::Result<super::super::Foundation::IReference<i64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IReference<i64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Globalization_NumberFormatting`, `Foundation`*"]
    pub fn ParseUInt<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, text: Param0) -> ::windows::runtime::Result<super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IReference<u64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Globalization_NumberFormatting`, `Foundation`*"]
    pub fn ParseDouble<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, text: Param0) -> ::windows::runtime::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for INumberParser {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{e6659412-4a13-4a53-83a1-392fbe4cff9f}");
}
#[repr(C)]
#[doc(hidden)]
pub struct INumberParser_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, text: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, text: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, text: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Globalization_NumberFormatting`*"]
pub struct INumberRounder(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INumberRounder {
    type Vtable = INumberRounder_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1416872821, 14573, 17969, [184, 12, 239, 52, 252, 72, 183, 245]);
}
impl INumberRounder {
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn RoundInt32(&self, value: i32) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn RoundUInt32(&self, value: u32) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn RoundInt64(&self, value: i64) -> ::windows::runtime::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn RoundUInt64(&self, value: u64) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn RoundSingle(&self, value: f32) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn RoundDouble(&self, value: f64) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<f64>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for INumberRounder {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{5473c375-38ed-4631-b80c-ef34fc48b7f5}");
}
#[repr(C)]
#[doc(hidden)]
pub struct INumberRounder_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i64, result__: *mut i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u64, result__: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64, result__: *mut f64) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Globalization_NumberFormatting`*"]
pub struct INumberRounderOption(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INumberRounderOption {
    type Vtable = INumberRounderOption_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(990413875, 25711, 20222, [141, 72, 102, 235, 46, 73, 231, 54]);
}
impl INumberRounderOption {
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn NumberRounder(&self) -> ::windows::runtime::Result<INumberRounder> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<INumberRounder>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn SetNumberRounder<'a, Param0: ::windows::runtime::IntoParam<'a, INumberRounder>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for INumberRounderOption {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{3b088433-646f-4efe-8d48-66eb2e49e736}");
}
#[repr(C)]
#[doc(hidden)]
pub struct INumberRounderOption_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct INumeralSystemTranslator(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INumeralSystemTranslator {
    type Vtable = INumeralSystemTranslator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(687193132, 35875, 16948, [173, 46, 250, 90, 58, 66, 110, 155]);
}
#[repr(C)]
#[doc(hidden)]
pub struct INumeralSystemTranslator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct INumeralSystemTranslatorFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INumeralSystemTranslatorFactory {
    type Vtable = INumeralSystemTranslatorFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2519779546, 14063, 19848, [168, 92, 111, 13, 152, 214, 32, 166]);
}
#[repr(C)]
#[doc(hidden)]
pub struct INumeralSystemTranslatorFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, languages: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPercentFormatterFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPercentFormatterFactory {
    type Vtable = IPercentFormatterFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3078785775, 65236, 16408, [166, 226, 224, 153, 97, 224, 55, 101]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPercentFormatterFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, languages: ::windows::runtime::RawPtr, geographicregion: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPermilleFormatterFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPermilleFormatterFactory {
    type Vtable = IPermilleFormatterFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(725071020, 58936, 20181, [169, 152, 98, 246, 176, 106, 73, 174]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPermilleFormatterFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, languages: ::windows::runtime::RawPtr, geographicregion: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Globalization_NumberFormatting`*"]
pub struct ISignedZeroOption(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISignedZeroOption {
    type Vtable = ISignedZeroOption_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4246527281, 2620, 18884, [166, 66, 150, 161, 86, 79, 79, 48]);
}
impl ISignedZeroOption {
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn IsZeroSigned(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn SetIsZeroSigned(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ISignedZeroOption {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{fd1cdd31-0a3c-49c4-a642-96a1564f4f30}");
}
#[repr(C)]
#[doc(hidden)]
pub struct ISignedZeroOption_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ISignificantDigitsNumberRounder(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISignificantDigitsNumberRounder {
    type Vtable = ISignificantDigitsNumberRounder_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4120124362, 26182, 18707, [140, 118, 27, 25, 31, 249, 77, 253]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISignificantDigitsNumberRounder_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut RoundingAlgorithm) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: RoundingAlgorithm) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Globalization_NumberFormatting`*"]
pub struct ISignificantDigitsOption(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISignificantDigitsOption {
    type Vtable = ISignificantDigitsOption_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(491650269, 11587, 20200, [187, 241, 193, 178, 106, 113, 26, 88]);
}
impl ISignificantDigitsOption {
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn SignificantDigits(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn SetSignificantDigits(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ISignificantDigitsOption {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{1d4dfcdd-2d43-4ee8-bbf1-c1b26a711a58}");
}
#[repr(C)]
#[doc(hidden)]
pub struct ISignificantDigitsOption_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Globalization_NumberFormatting`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IncrementNumberRounder(::windows::runtime::IInspectable);
impl IncrementNumberRounder {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<IncrementNumberRounder, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn RoundInt32(&self, value: i32) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn RoundUInt32(&self, value: u32) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn RoundInt64(&self, value: i64) -> ::windows::runtime::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn RoundUInt64(&self, value: u64) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn RoundSingle(&self, value: f32) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn RoundDouble(&self, value: f64) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn RoundingAlgorithm(&self) -> ::windows::runtime::Result<RoundingAlgorithm> {
        let this = &::windows::runtime::Interface::cast::<IIncrementNumberRounder>(self)?;
        unsafe {
            let mut result__: RoundingAlgorithm = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<RoundingAlgorithm>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn SetRoundingAlgorithm(&self, value: RoundingAlgorithm) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IIncrementNumberRounder>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn Increment(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<IIncrementNumberRounder>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn SetIncrement(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IIncrementNumberRounder>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IncrementNumberRounder {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Globalization.NumberFormatting.IncrementNumberRounder;{5473c375-38ed-4631-b80c-ef34fc48b7f5})");
}
unsafe impl ::windows::runtime::Interface for IncrementNumberRounder {
    type Vtable = INumberRounder_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1416872821, 14573, 17969, [184, 12, 239, 52, 252, 72, 183, 245]);
}
impl ::windows::runtime::RuntimeName for IncrementNumberRounder {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.IncrementNumberRounder";
}
impl ::std::convert::From<IncrementNumberRounder> for ::windows::runtime::IUnknown {
    fn from(value: IncrementNumberRounder) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IncrementNumberRounder> for ::windows::runtime::IUnknown {
    fn from(value: &IncrementNumberRounder) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IncrementNumberRounder {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IncrementNumberRounder {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IncrementNumberRounder> for ::windows::runtime::IInspectable {
    fn from(value: IncrementNumberRounder) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IncrementNumberRounder> for ::windows::runtime::IInspectable {
    fn from(value: &IncrementNumberRounder) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IncrementNumberRounder {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IncrementNumberRounder {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IncrementNumberRounder> for INumberRounder {
    fn from(value: IncrementNumberRounder) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IncrementNumberRounder> for INumberRounder {
    fn from(value: &IncrementNumberRounder) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INumberRounder> for IncrementNumberRounder {
    fn into_param(self) -> ::windows::runtime::Param<'a, INumberRounder> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<INumberRounder>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INumberRounder> for &IncrementNumberRounder {
    fn into_param(self) -> ::windows::runtime::Param<'a, INumberRounder> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<INumberRounder>::into(::std::clone::Clone::clone(self)))
    }
}
unsafe impl ::std::marker::Send for IncrementNumberRounder {}
unsafe impl ::std::marker::Sync for IncrementNumberRounder {}
#[doc = "*Required features: `Globalization_NumberFormatting`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct NumeralSystemTranslator(::windows::runtime::IInspectable);
impl NumeralSystemTranslator {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<NumeralSystemTranslator, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Globalization_NumberFormatting`, `Foundation_Collections`*"]
    pub fn Languages(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn ResolvedLanguage(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn NumeralSystem(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn SetNumeralSystem<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn TranslateNumerals<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Globalization_NumberFormatting`, `Foundation_Collections`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(languages: Param0) -> ::windows::runtime::Result<NumeralSystemTranslator> {
        Self::INumeralSystemTranslatorFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), languages.into_param().abi(), &mut result__).from_abi::<NumeralSystemTranslator>(result__)
        })
    }
    pub fn INumeralSystemTranslatorFactory<R, F: FnOnce(&INumeralSystemTranslatorFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<NumeralSystemTranslator, INumeralSystemTranslatorFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for NumeralSystemTranslator {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Globalization.NumberFormatting.NumeralSystemTranslator;{28f5bc2c-8c23-4234-ad2e-fa5a3a426e9b})");
}
unsafe impl ::windows::runtime::Interface for NumeralSystemTranslator {
    type Vtable = INumeralSystemTranslator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(687193132, 35875, 16948, [173, 46, 250, 90, 58, 66, 110, 155]);
}
impl ::windows::runtime::RuntimeName for NumeralSystemTranslator {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.NumeralSystemTranslator";
}
impl ::std::convert::From<NumeralSystemTranslator> for ::windows::runtime::IUnknown {
    fn from(value: NumeralSystemTranslator) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&NumeralSystemTranslator> for ::windows::runtime::IUnknown {
    fn from(value: &NumeralSystemTranslator) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for NumeralSystemTranslator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &NumeralSystemTranslator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<NumeralSystemTranslator> for ::windows::runtime::IInspectable {
    fn from(value: NumeralSystemTranslator) -> Self {
        value.0
    }
}
impl ::std::convert::From<&NumeralSystemTranslator> for ::windows::runtime::IInspectable {
    fn from(value: &NumeralSystemTranslator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for NumeralSystemTranslator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a NumeralSystemTranslator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for NumeralSystemTranslator {}
unsafe impl ::std::marker::Sync for NumeralSystemTranslator {}
#[doc = "*Required features: `Globalization_NumberFormatting`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PercentFormatter(::windows::runtime::IInspectable);
impl PercentFormatter {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PercentFormatter, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn FormatInt(&self, value: i64) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn FormatUInt(&self, value: u64) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn FormatDouble(&self, value: f64) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn FormatInt2(&self, value: i64) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn FormatUInt2(&self, value: u64) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn FormatDouble2(&self, value: f64) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Globalization_NumberFormatting`, `Foundation_Collections`*"]
    pub fn Languages(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn GeographicRegion(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn IntegerDigits(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn SetIntegerDigits(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn FractionDigits(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn SetFractionDigits(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn IsGrouped(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn SetIsGrouped(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn IsDecimalPointAlwaysDisplayed(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn SetIsDecimalPointAlwaysDisplayed(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn NumeralSystem(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn SetNumeralSystem<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn ResolvedLanguage(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn ResolvedGeographicRegion(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Globalization_NumberFormatting`, `Foundation`*"]
    pub fn ParseInt<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, text: Param0) -> ::windows::runtime::Result<super::super::Foundation::IReference<i64>> {
        let this = &::windows::runtime::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IReference<i64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Globalization_NumberFormatting`, `Foundation`*"]
    pub fn ParseUInt<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, text: Param0) -> ::windows::runtime::Result<super::super::Foundation::IReference<u64>> {
        let this = &::windows::runtime::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IReference<u64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Globalization_NumberFormatting`, `Foundation`*"]
    pub fn ParseDouble<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, text: Param0) -> ::windows::runtime::Result<super::super::Foundation::IReference<f64>> {
        let this = &::windows::runtime::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn NumberRounder(&self) -> ::windows::runtime::Result<INumberRounder> {
        let this = &::windows::runtime::Interface::cast::<INumberRounderOption>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<INumberRounder>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn SetNumberRounder<'a, Param0: ::windows::runtime::IntoParam<'a, INumberRounder>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<INumberRounderOption>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn IsZeroSigned(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ISignedZeroOption>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn SetIsZeroSigned(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISignedZeroOption>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn SignificantDigits(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ISignificantDigitsOption>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn SetSignificantDigits(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISignificantDigitsOption>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Globalization_NumberFormatting`, `Foundation_Collections`*"]
    pub fn CreatePercentFormatter<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(languages: Param0, geographicregion: Param1) -> ::windows::runtime::Result<PercentFormatter> {
        Self::IPercentFormatterFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), languages.into_param().abi(), geographicregion.into_param().abi(), &mut result__).from_abi::<PercentFormatter>(result__)
        })
    }
    pub fn IPercentFormatterFactory<R, F: FnOnce(&IPercentFormatterFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PercentFormatter, IPercentFormatterFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PercentFormatter {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Globalization.NumberFormatting.PercentFormatter;{a5007c49-7676-4db7-8631-1b6ff265caa9})");
}
unsafe impl ::windows::runtime::Interface for PercentFormatter {
    type Vtable = INumberFormatter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2768272457, 30326, 19895, [134, 49, 27, 111, 242, 101, 202, 169]);
}
impl ::windows::runtime::RuntimeName for PercentFormatter {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.PercentFormatter";
}
impl ::std::convert::From<PercentFormatter> for ::windows::runtime::IUnknown {
    fn from(value: PercentFormatter) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PercentFormatter> for ::windows::runtime::IUnknown {
    fn from(value: &PercentFormatter) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PercentFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PercentFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<PercentFormatter> for ::windows::runtime::IInspectable {
    fn from(value: PercentFormatter) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PercentFormatter> for ::windows::runtime::IInspectable {
    fn from(value: &PercentFormatter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PercentFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PercentFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<PercentFormatter> for INumberFormatter {
    fn from(value: PercentFormatter) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PercentFormatter> for INumberFormatter {
    fn from(value: &PercentFormatter) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INumberFormatter> for PercentFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, INumberFormatter> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<INumberFormatter>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INumberFormatter> for &PercentFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, INumberFormatter> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<INumberFormatter>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::TryFrom<PercentFormatter> for INumberFormatter2 {
    type Error = ::windows::runtime::Error;
    fn try_from(value: PercentFormatter) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&PercentFormatter> for INumberFormatter2 {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &PercentFormatter) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INumberFormatter2> for PercentFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, INumberFormatter2> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INumberFormatter2> for &PercentFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, INumberFormatter2> {
        ::std::convert::TryInto::<INumberFormatter2>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<PercentFormatter> for INumberFormatterOptions {
    type Error = ::windows::runtime::Error;
    fn try_from(value: PercentFormatter) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&PercentFormatter> for INumberFormatterOptions {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &PercentFormatter) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INumberFormatterOptions> for PercentFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, INumberFormatterOptions> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INumberFormatterOptions> for &PercentFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, INumberFormatterOptions> {
        ::std::convert::TryInto::<INumberFormatterOptions>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<PercentFormatter> for INumberParser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: PercentFormatter) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&PercentFormatter> for INumberParser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &PercentFormatter) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INumberParser> for PercentFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, INumberParser> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INumberParser> for &PercentFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, INumberParser> {
        ::std::convert::TryInto::<INumberParser>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<PercentFormatter> for INumberRounderOption {
    type Error = ::windows::runtime::Error;
    fn try_from(value: PercentFormatter) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&PercentFormatter> for INumberRounderOption {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &PercentFormatter) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INumberRounderOption> for PercentFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, INumberRounderOption> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INumberRounderOption> for &PercentFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, INumberRounderOption> {
        ::std::convert::TryInto::<INumberRounderOption>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<PercentFormatter> for ISignedZeroOption {
    type Error = ::windows::runtime::Error;
    fn try_from(value: PercentFormatter) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&PercentFormatter> for ISignedZeroOption {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &PercentFormatter) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISignedZeroOption> for PercentFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISignedZeroOption> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISignedZeroOption> for &PercentFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISignedZeroOption> {
        ::std::convert::TryInto::<ISignedZeroOption>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<PercentFormatter> for ISignificantDigitsOption {
    type Error = ::windows::runtime::Error;
    fn try_from(value: PercentFormatter) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&PercentFormatter> for ISignificantDigitsOption {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &PercentFormatter) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISignificantDigitsOption> for PercentFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISignificantDigitsOption> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISignificantDigitsOption> for &PercentFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISignificantDigitsOption> {
        ::std::convert::TryInto::<ISignificantDigitsOption>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for PercentFormatter {}
unsafe impl ::std::marker::Sync for PercentFormatter {}
#[doc = "*Required features: `Globalization_NumberFormatting`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PermilleFormatter(::windows::runtime::IInspectable);
impl PermilleFormatter {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PermilleFormatter, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn FormatInt(&self, value: i64) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn FormatUInt(&self, value: u64) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn FormatDouble(&self, value: f64) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn FormatInt2(&self, value: i64) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn FormatUInt2(&self, value: u64) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn FormatDouble2(&self, value: f64) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Globalization_NumberFormatting`, `Foundation_Collections`*"]
    pub fn Languages(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn GeographicRegion(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn IntegerDigits(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn SetIntegerDigits(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn FractionDigits(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn SetFractionDigits(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn IsGrouped(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn SetIsGrouped(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn IsDecimalPointAlwaysDisplayed(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn SetIsDecimalPointAlwaysDisplayed(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn NumeralSystem(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn SetNumeralSystem<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn ResolvedLanguage(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn ResolvedGeographicRegion(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Globalization_NumberFormatting`, `Foundation`*"]
    pub fn ParseInt<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, text: Param0) -> ::windows::runtime::Result<super::super::Foundation::IReference<i64>> {
        let this = &::windows::runtime::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IReference<i64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Globalization_NumberFormatting`, `Foundation`*"]
    pub fn ParseUInt<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, text: Param0) -> ::windows::runtime::Result<super::super::Foundation::IReference<u64>> {
        let this = &::windows::runtime::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IReference<u64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Globalization_NumberFormatting`, `Foundation`*"]
    pub fn ParseDouble<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, text: Param0) -> ::windows::runtime::Result<super::super::Foundation::IReference<f64>> {
        let this = &::windows::runtime::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn NumberRounder(&self) -> ::windows::runtime::Result<INumberRounder> {
        let this = &::windows::runtime::Interface::cast::<INumberRounderOption>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<INumberRounder>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn SetNumberRounder<'a, Param0: ::windows::runtime::IntoParam<'a, INumberRounder>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<INumberRounderOption>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn IsZeroSigned(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ISignedZeroOption>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn SetIsZeroSigned(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISignedZeroOption>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn SignificantDigits(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ISignificantDigitsOption>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn SetSignificantDigits(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISignificantDigitsOption>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Globalization_NumberFormatting`, `Foundation_Collections`*"]
    pub fn CreatePermilleFormatter<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(languages: Param0, geographicregion: Param1) -> ::windows::runtime::Result<PermilleFormatter> {
        Self::IPermilleFormatterFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), languages.into_param().abi(), geographicregion.into_param().abi(), &mut result__).from_abi::<PermilleFormatter>(result__)
        })
    }
    pub fn IPermilleFormatterFactory<R, F: FnOnce(&IPermilleFormatterFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PermilleFormatter, IPermilleFormatterFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PermilleFormatter {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Globalization.NumberFormatting.PermilleFormatter;{a5007c49-7676-4db7-8631-1b6ff265caa9})");
}
unsafe impl ::windows::runtime::Interface for PermilleFormatter {
    type Vtable = INumberFormatter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2768272457, 30326, 19895, [134, 49, 27, 111, 242, 101, 202, 169]);
}
impl ::windows::runtime::RuntimeName for PermilleFormatter {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.PermilleFormatter";
}
impl ::std::convert::From<PermilleFormatter> for ::windows::runtime::IUnknown {
    fn from(value: PermilleFormatter) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PermilleFormatter> for ::windows::runtime::IUnknown {
    fn from(value: &PermilleFormatter) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PermilleFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PermilleFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<PermilleFormatter> for ::windows::runtime::IInspectable {
    fn from(value: PermilleFormatter) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PermilleFormatter> for ::windows::runtime::IInspectable {
    fn from(value: &PermilleFormatter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PermilleFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PermilleFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<PermilleFormatter> for INumberFormatter {
    fn from(value: PermilleFormatter) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PermilleFormatter> for INumberFormatter {
    fn from(value: &PermilleFormatter) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INumberFormatter> for PermilleFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, INumberFormatter> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<INumberFormatter>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INumberFormatter> for &PermilleFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, INumberFormatter> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<INumberFormatter>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::TryFrom<PermilleFormatter> for INumberFormatter2 {
    type Error = ::windows::runtime::Error;
    fn try_from(value: PermilleFormatter) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&PermilleFormatter> for INumberFormatter2 {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &PermilleFormatter) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INumberFormatter2> for PermilleFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, INumberFormatter2> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INumberFormatter2> for &PermilleFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, INumberFormatter2> {
        ::std::convert::TryInto::<INumberFormatter2>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<PermilleFormatter> for INumberFormatterOptions {
    type Error = ::windows::runtime::Error;
    fn try_from(value: PermilleFormatter) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&PermilleFormatter> for INumberFormatterOptions {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &PermilleFormatter) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INumberFormatterOptions> for PermilleFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, INumberFormatterOptions> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INumberFormatterOptions> for &PermilleFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, INumberFormatterOptions> {
        ::std::convert::TryInto::<INumberFormatterOptions>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<PermilleFormatter> for INumberParser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: PermilleFormatter) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&PermilleFormatter> for INumberParser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &PermilleFormatter) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INumberParser> for PermilleFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, INumberParser> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INumberParser> for &PermilleFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, INumberParser> {
        ::std::convert::TryInto::<INumberParser>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<PermilleFormatter> for INumberRounderOption {
    type Error = ::windows::runtime::Error;
    fn try_from(value: PermilleFormatter) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&PermilleFormatter> for INumberRounderOption {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &PermilleFormatter) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INumberRounderOption> for PermilleFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, INumberRounderOption> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INumberRounderOption> for &PermilleFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, INumberRounderOption> {
        ::std::convert::TryInto::<INumberRounderOption>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<PermilleFormatter> for ISignedZeroOption {
    type Error = ::windows::runtime::Error;
    fn try_from(value: PermilleFormatter) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&PermilleFormatter> for ISignedZeroOption {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &PermilleFormatter) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISignedZeroOption> for PermilleFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISignedZeroOption> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISignedZeroOption> for &PermilleFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISignedZeroOption> {
        ::std::convert::TryInto::<ISignedZeroOption>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<PermilleFormatter> for ISignificantDigitsOption {
    type Error = ::windows::runtime::Error;
    fn try_from(value: PermilleFormatter) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&PermilleFormatter> for ISignificantDigitsOption {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &PermilleFormatter) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISignificantDigitsOption> for PermilleFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISignificantDigitsOption> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISignificantDigitsOption> for &PermilleFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISignificantDigitsOption> {
        ::std::convert::TryInto::<ISignificantDigitsOption>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for PermilleFormatter {}
unsafe impl ::std::marker::Sync for PermilleFormatter {}
#[doc = "*Required features: `Globalization_NumberFormatting`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct RoundingAlgorithm(pub i32);
impl RoundingAlgorithm {
    pub const None: RoundingAlgorithm = RoundingAlgorithm(0i32);
    pub const RoundDown: RoundingAlgorithm = RoundingAlgorithm(1i32);
    pub const RoundUp: RoundingAlgorithm = RoundingAlgorithm(2i32);
    pub const RoundTowardsZero: RoundingAlgorithm = RoundingAlgorithm(3i32);
    pub const RoundAwayFromZero: RoundingAlgorithm = RoundingAlgorithm(4i32);
    pub const RoundHalfDown: RoundingAlgorithm = RoundingAlgorithm(5i32);
    pub const RoundHalfUp: RoundingAlgorithm = RoundingAlgorithm(6i32);
    pub const RoundHalfTowardsZero: RoundingAlgorithm = RoundingAlgorithm(7i32);
    pub const RoundHalfAwayFromZero: RoundingAlgorithm = RoundingAlgorithm(8i32);
    pub const RoundHalfToEven: RoundingAlgorithm = RoundingAlgorithm(9i32);
    pub const RoundHalfToOdd: RoundingAlgorithm = RoundingAlgorithm(10i32);
}
impl ::std::convert::From<i32> for RoundingAlgorithm {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RoundingAlgorithm {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for RoundingAlgorithm {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Globalization.NumberFormatting.RoundingAlgorithm;i4)");
}
#[doc = "*Required features: `Globalization_NumberFormatting`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SignificantDigitsNumberRounder(::windows::runtime::IInspectable);
impl SignificantDigitsNumberRounder {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SignificantDigitsNumberRounder, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn RoundInt32(&self, value: i32) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn RoundUInt32(&self, value: u32) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn RoundInt64(&self, value: i64) -> ::windows::runtime::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn RoundUInt64(&self, value: u64) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn RoundSingle(&self, value: f32) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn RoundDouble(&self, value: f64) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn RoundingAlgorithm(&self) -> ::windows::runtime::Result<RoundingAlgorithm> {
        let this = &::windows::runtime::Interface::cast::<ISignificantDigitsNumberRounder>(self)?;
        unsafe {
            let mut result__: RoundingAlgorithm = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<RoundingAlgorithm>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn SetRoundingAlgorithm(&self, value: RoundingAlgorithm) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISignificantDigitsNumberRounder>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn SignificantDigits(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<ISignificantDigitsNumberRounder>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_NumberFormatting`*"]
    pub fn SetSignificantDigits(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISignificantDigitsNumberRounder>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SignificantDigitsNumberRounder {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Globalization.NumberFormatting.SignificantDigitsNumberRounder;{5473c375-38ed-4631-b80c-ef34fc48b7f5})");
}
unsafe impl ::windows::runtime::Interface for SignificantDigitsNumberRounder {
    type Vtable = INumberRounder_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1416872821, 14573, 17969, [184, 12, 239, 52, 252, 72, 183, 245]);
}
impl ::windows::runtime::RuntimeName for SignificantDigitsNumberRounder {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.SignificantDigitsNumberRounder";
}
impl ::std::convert::From<SignificantDigitsNumberRounder> for ::windows::runtime::IUnknown {
    fn from(value: SignificantDigitsNumberRounder) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SignificantDigitsNumberRounder> for ::windows::runtime::IUnknown {
    fn from(value: &SignificantDigitsNumberRounder) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SignificantDigitsNumberRounder {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SignificantDigitsNumberRounder {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SignificantDigitsNumberRounder> for ::windows::runtime::IInspectable {
    fn from(value: SignificantDigitsNumberRounder) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SignificantDigitsNumberRounder> for ::windows::runtime::IInspectable {
    fn from(value: &SignificantDigitsNumberRounder) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SignificantDigitsNumberRounder {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SignificantDigitsNumberRounder {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<SignificantDigitsNumberRounder> for INumberRounder {
    fn from(value: SignificantDigitsNumberRounder) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SignificantDigitsNumberRounder> for INumberRounder {
    fn from(value: &SignificantDigitsNumberRounder) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INumberRounder> for SignificantDigitsNumberRounder {
    fn into_param(self) -> ::windows::runtime::Param<'a, INumberRounder> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<INumberRounder>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INumberRounder> for &SignificantDigitsNumberRounder {
    fn into_param(self) -> ::windows::runtime::Param<'a, INumberRounder> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<INumberRounder>::into(::std::clone::Clone::clone(self)))
    }
}
unsafe impl ::std::marker::Send for SignificantDigitsNumberRounder {}
unsafe impl ::std::marker::Sync for SignificantDigitsNumberRounder {}
