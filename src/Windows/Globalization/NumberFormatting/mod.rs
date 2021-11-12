#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CurrencyFormatter(pub ::windows::core::IInspectable);
impl CurrencyFormatter {
    pub fn Currency(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetCurrency<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Mode(&self) -> ::windows::core::Result<CurrencyFormatterMode> {
        let this = &::windows::core::Interface::cast::<ICurrencyFormatter2>(self)?;
        unsafe {
            let mut result__: CurrencyFormatterMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CurrencyFormatterMode>(result__)
        }
    }
    pub fn SetMode(&self, value: CurrencyFormatterMode) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICurrencyFormatter2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ApplyRoundingForCurrency(&self, roundingalgorithm: RoundingAlgorithm) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICurrencyFormatter2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), roundingalgorithm).ok() }
    }
    pub fn FormatInt(&self, value: i64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatter>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn FormatUInt(&self, value: u64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatter>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn FormatDouble(&self, value: f64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatter>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn FormatInt2(&self, value: i64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn FormatUInt2(&self, value: u64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn FormatDouble2(&self, value: f64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Languages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    pub fn GeographicRegion(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn IntegerDigits(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn SetIntegerDigits(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn FractionDigits(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn SetFractionDigits(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsGrouped(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsGrouped(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsDecimalPointAlwaysDisplayed(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsDecimalPointAlwaysDisplayed(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn NumeralSystem(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetNumeralSystem<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ResolvedLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ResolvedGeographicRegion(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ParseInt<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, text: Param0) -> ::windows::core::Result<super::super::Foundation::IReference<i64>> {
        let this = &::windows::core::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IReference<i64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ParseUInt<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, text: Param0) -> ::windows::core::Result<super::super::Foundation::IReference<u64>> {
        let this = &::windows::core::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IReference<u64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ParseDouble<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, text: Param0) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = &::windows::core::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    pub fn NumberRounder(&self) -> ::windows::core::Result<INumberRounder> {
        let this = &::windows::core::Interface::cast::<INumberRounderOption>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<INumberRounder>(result__)
        }
    }
    pub fn SetNumberRounder<'a, Param0: ::windows::core::IntoParam<'a, INumberRounder>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<INumberRounderOption>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn IsZeroSigned(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ISignedZeroOption>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsZeroSigned(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISignedZeroOption>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn SignificantDigits(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ISignificantDigitsOption>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn SetSignificantDigits(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISignificantDigitsOption>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn CreateCurrencyFormatterCode<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(currencycode: Param0) -> ::windows::core::Result<CurrencyFormatter> {
        Self::ICurrencyFormatterFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), currencycode.into_param().abi(), &mut result__).from_abi::<CurrencyFormatter>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateCurrencyFormatterCodeContext<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(currencycode: Param0, languages: Param1, geographicregion: Param2) -> ::windows::core::Result<CurrencyFormatter> {
        Self::ICurrencyFormatterFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), currencycode.into_param().abi(), languages.into_param().abi(), geographicregion.into_param().abi(), &mut result__).from_abi::<CurrencyFormatter>(result__)
        })
    }
    pub fn ICurrencyFormatterFactory<R, F: FnOnce(&ICurrencyFormatterFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CurrencyFormatter, ICurrencyFormatterFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for CurrencyFormatter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Globalization.NumberFormatting.CurrencyFormatter;{11730ca5-4b00-41b2-b332-73b12a497d54})");
}
unsafe impl ::windows::core::Interface for CurrencyFormatter {
    type Vtable = ICurrencyFormatter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x11730ca5_4b00_41b2_b332_73b12a497d54);
}
impl ::windows::core::RuntimeName for CurrencyFormatter {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.CurrencyFormatter";
}
impl ::core::convert::From<CurrencyFormatter> for ::windows::core::IUnknown {
    fn from(value: CurrencyFormatter) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CurrencyFormatter> for ::windows::core::IUnknown {
    fn from(value: &CurrencyFormatter) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CurrencyFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CurrencyFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CurrencyFormatter> for ::windows::core::IInspectable {
    fn from(value: CurrencyFormatter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CurrencyFormatter> for ::windows::core::IInspectable {
    fn from(value: &CurrencyFormatter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CurrencyFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CurrencyFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<CurrencyFormatter> for INumberFormatter {
    type Error = ::windows::core::Error;
    fn try_from(value: CurrencyFormatter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CurrencyFormatter> for INumberFormatter {
    type Error = ::windows::core::Error;
    fn try_from(value: &CurrencyFormatter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, INumberFormatter> for CurrencyFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, INumberFormatter> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, INumberFormatter> for &CurrencyFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, INumberFormatter> {
        ::core::convert::TryInto::<INumberFormatter>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<CurrencyFormatter> for INumberFormatter2 {
    type Error = ::windows::core::Error;
    fn try_from(value: CurrencyFormatter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CurrencyFormatter> for INumberFormatter2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &CurrencyFormatter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, INumberFormatter2> for CurrencyFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, INumberFormatter2> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, INumberFormatter2> for &CurrencyFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, INumberFormatter2> {
        ::core::convert::TryInto::<INumberFormatter2>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<CurrencyFormatter> for INumberFormatterOptions {
    type Error = ::windows::core::Error;
    fn try_from(value: CurrencyFormatter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CurrencyFormatter> for INumberFormatterOptions {
    type Error = ::windows::core::Error;
    fn try_from(value: &CurrencyFormatter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, INumberFormatterOptions> for CurrencyFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, INumberFormatterOptions> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, INumberFormatterOptions> for &CurrencyFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, INumberFormatterOptions> {
        ::core::convert::TryInto::<INumberFormatterOptions>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<CurrencyFormatter> for INumberParser {
    type Error = ::windows::core::Error;
    fn try_from(value: CurrencyFormatter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CurrencyFormatter> for INumberParser {
    type Error = ::windows::core::Error;
    fn try_from(value: &CurrencyFormatter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, INumberParser> for CurrencyFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, INumberParser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, INumberParser> for &CurrencyFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, INumberParser> {
        ::core::convert::TryInto::<INumberParser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<CurrencyFormatter> for INumberRounderOption {
    type Error = ::windows::core::Error;
    fn try_from(value: CurrencyFormatter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CurrencyFormatter> for INumberRounderOption {
    type Error = ::windows::core::Error;
    fn try_from(value: &CurrencyFormatter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, INumberRounderOption> for CurrencyFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, INumberRounderOption> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, INumberRounderOption> for &CurrencyFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, INumberRounderOption> {
        ::core::convert::TryInto::<INumberRounderOption>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<CurrencyFormatter> for ISignedZeroOption {
    type Error = ::windows::core::Error;
    fn try_from(value: CurrencyFormatter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CurrencyFormatter> for ISignedZeroOption {
    type Error = ::windows::core::Error;
    fn try_from(value: &CurrencyFormatter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISignedZeroOption> for CurrencyFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ISignedZeroOption> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISignedZeroOption> for &CurrencyFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ISignedZeroOption> {
        ::core::convert::TryInto::<ISignedZeroOption>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<CurrencyFormatter> for ISignificantDigitsOption {
    type Error = ::windows::core::Error;
    fn try_from(value: CurrencyFormatter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CurrencyFormatter> for ISignificantDigitsOption {
    type Error = ::windows::core::Error;
    fn try_from(value: &CurrencyFormatter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISignificantDigitsOption> for CurrencyFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ISignificantDigitsOption> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISignificantDigitsOption> for &CurrencyFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ISignificantDigitsOption> {
        ::core::convert::TryInto::<ISignificantDigitsOption>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for CurrencyFormatter {}
unsafe impl ::core::marker::Sync for CurrencyFormatter {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CurrencyFormatterMode(pub i32);
impl CurrencyFormatterMode {
    pub const UseSymbol: CurrencyFormatterMode = CurrencyFormatterMode(0i32);
    pub const UseCurrencyCode: CurrencyFormatterMode = CurrencyFormatterMode(1i32);
}
impl ::core::convert::From<i32> for CurrencyFormatterMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CurrencyFormatterMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CurrencyFormatterMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Globalization.NumberFormatting.CurrencyFormatterMode;i4)");
}
impl ::windows::core::DefaultType for CurrencyFormatterMode {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DecimalFormatter(pub ::windows::core::IInspectable);
impl DecimalFormatter {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DecimalFormatter, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn FormatInt(&self, value: i64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn FormatUInt(&self, value: u64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn FormatDouble(&self, value: f64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn FormatInt2(&self, value: i64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn FormatUInt2(&self, value: u64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn FormatDouble2(&self, value: f64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Languages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    pub fn GeographicRegion(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn IntegerDigits(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn SetIntegerDigits(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn FractionDigits(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn SetFractionDigits(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsGrouped(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsGrouped(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsDecimalPointAlwaysDisplayed(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsDecimalPointAlwaysDisplayed(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn NumeralSystem(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetNumeralSystem<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ResolvedLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ResolvedGeographicRegion(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ParseInt<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, text: Param0) -> ::windows::core::Result<super::super::Foundation::IReference<i64>> {
        let this = &::windows::core::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IReference<i64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ParseUInt<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, text: Param0) -> ::windows::core::Result<super::super::Foundation::IReference<u64>> {
        let this = &::windows::core::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IReference<u64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ParseDouble<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, text: Param0) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = &::windows::core::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    pub fn NumberRounder(&self) -> ::windows::core::Result<INumberRounder> {
        let this = &::windows::core::Interface::cast::<INumberRounderOption>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<INumberRounder>(result__)
        }
    }
    pub fn SetNumberRounder<'a, Param0: ::windows::core::IntoParam<'a, INumberRounder>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<INumberRounderOption>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn IsZeroSigned(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ISignedZeroOption>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsZeroSigned(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISignedZeroOption>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn SignificantDigits(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ISignificantDigitsOption>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn SetSignificantDigits(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISignificantDigitsOption>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateDecimalFormatter<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(languages: Param0, geographicregion: Param1) -> ::windows::core::Result<DecimalFormatter> {
        Self::IDecimalFormatterFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), languages.into_param().abi(), geographicregion.into_param().abi(), &mut result__).from_abi::<DecimalFormatter>(result__)
        })
    }
    pub fn IDecimalFormatterFactory<R, F: FnOnce(&IDecimalFormatterFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DecimalFormatter, IDecimalFormatterFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for DecimalFormatter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Globalization.NumberFormatting.DecimalFormatter;{a5007c49-7676-4db7-8631-1b6ff265caa9})");
}
unsafe impl ::windows::core::Interface for DecimalFormatter {
    type Vtable = INumberFormatter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa5007c49_7676_4db7_8631_1b6ff265caa9);
}
impl ::windows::core::RuntimeName for DecimalFormatter {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.DecimalFormatter";
}
impl ::core::convert::From<DecimalFormatter> for ::windows::core::IUnknown {
    fn from(value: DecimalFormatter) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DecimalFormatter> for ::windows::core::IUnknown {
    fn from(value: &DecimalFormatter) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DecimalFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DecimalFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DecimalFormatter> for ::windows::core::IInspectable {
    fn from(value: DecimalFormatter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DecimalFormatter> for ::windows::core::IInspectable {
    fn from(value: &DecimalFormatter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DecimalFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DecimalFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<DecimalFormatter> for INumberFormatter {
    fn from(value: DecimalFormatter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DecimalFormatter> for INumberFormatter {
    fn from(value: &DecimalFormatter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, INumberFormatter> for DecimalFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, INumberFormatter> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, INumberFormatter> for &DecimalFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, INumberFormatter> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<DecimalFormatter> for INumberFormatter2 {
    type Error = ::windows::core::Error;
    fn try_from(value: DecimalFormatter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DecimalFormatter> for INumberFormatter2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &DecimalFormatter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, INumberFormatter2> for DecimalFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, INumberFormatter2> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, INumberFormatter2> for &DecimalFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, INumberFormatter2> {
        ::core::convert::TryInto::<INumberFormatter2>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<DecimalFormatter> for INumberFormatterOptions {
    type Error = ::windows::core::Error;
    fn try_from(value: DecimalFormatter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DecimalFormatter> for INumberFormatterOptions {
    type Error = ::windows::core::Error;
    fn try_from(value: &DecimalFormatter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, INumberFormatterOptions> for DecimalFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, INumberFormatterOptions> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, INumberFormatterOptions> for &DecimalFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, INumberFormatterOptions> {
        ::core::convert::TryInto::<INumberFormatterOptions>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<DecimalFormatter> for INumberParser {
    type Error = ::windows::core::Error;
    fn try_from(value: DecimalFormatter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DecimalFormatter> for INumberParser {
    type Error = ::windows::core::Error;
    fn try_from(value: &DecimalFormatter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, INumberParser> for DecimalFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, INumberParser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, INumberParser> for &DecimalFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, INumberParser> {
        ::core::convert::TryInto::<INumberParser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<DecimalFormatter> for INumberRounderOption {
    type Error = ::windows::core::Error;
    fn try_from(value: DecimalFormatter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DecimalFormatter> for INumberRounderOption {
    type Error = ::windows::core::Error;
    fn try_from(value: &DecimalFormatter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, INumberRounderOption> for DecimalFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, INumberRounderOption> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, INumberRounderOption> for &DecimalFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, INumberRounderOption> {
        ::core::convert::TryInto::<INumberRounderOption>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<DecimalFormatter> for ISignedZeroOption {
    type Error = ::windows::core::Error;
    fn try_from(value: DecimalFormatter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DecimalFormatter> for ISignedZeroOption {
    type Error = ::windows::core::Error;
    fn try_from(value: &DecimalFormatter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISignedZeroOption> for DecimalFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ISignedZeroOption> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISignedZeroOption> for &DecimalFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ISignedZeroOption> {
        ::core::convert::TryInto::<ISignedZeroOption>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<DecimalFormatter> for ISignificantDigitsOption {
    type Error = ::windows::core::Error;
    fn try_from(value: DecimalFormatter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DecimalFormatter> for ISignificantDigitsOption {
    type Error = ::windows::core::Error;
    fn try_from(value: &DecimalFormatter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISignificantDigitsOption> for DecimalFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ISignificantDigitsOption> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISignificantDigitsOption> for &DecimalFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ISignificantDigitsOption> {
        ::core::convert::TryInto::<ISignificantDigitsOption>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for DecimalFormatter {}
unsafe impl ::core::marker::Sync for DecimalFormatter {}
#[repr(transparent)]
#[doc(hidden)]
pub struct ICurrencyFormatter(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICurrencyFormatter {
    type Vtable = ICurrencyFormatter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x11730ca5_4b00_41b2_b332_73b12a497d54);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrencyFormatter_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICurrencyFormatter2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICurrencyFormatter2 {
    type Vtable = ICurrencyFormatter2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x072c2f1d_e7ba_4197_920e_247c92f7dea6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrencyFormatter2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut CurrencyFormatterMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: CurrencyFormatterMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, roundingalgorithm: RoundingAlgorithm) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICurrencyFormatterFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICurrencyFormatterFactory {
    type Vtable = ICurrencyFormatterFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x86c7537e_b938_4aa2_84b0_2c33dc5b1450);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrencyFormatterFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, currencycode: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, currencycode: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, languages: ::windows::core::RawPtr, geographicregion: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDecimalFormatterFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDecimalFormatterFactory {
    type Vtable = IDecimalFormatterFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0d018c9a_e393_46b8_b830_7a69c8f89fbb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDecimalFormatterFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, languages: ::windows::core::RawPtr, geographicregion: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IIncrementNumberRounder(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IIncrementNumberRounder {
    type Vtable = IIncrementNumberRounder_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x70a64ff8_66ab_4155_9da1_739e46764543);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIncrementNumberRounder_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut RoundingAlgorithm) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: RoundingAlgorithm) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INumberFormatter(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for INumberFormatter {
    type Vtable = INumberFormatter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa5007c49_7676_4db7_8631_1b6ff265caa9);
}
impl INumberFormatter {
    pub fn FormatInt(&self, value: i64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn FormatUInt(&self, value: u64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn FormatDouble(&self, value: f64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for INumberFormatter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{a5007c49-7676-4db7-8631-1b6ff265caa9}");
}
impl ::core::convert::From<INumberFormatter> for ::windows::core::IUnknown {
    fn from(value: INumberFormatter) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&INumberFormatter> for ::windows::core::IUnknown {
    fn from(value: &INumberFormatter) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INumberFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INumberFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<INumberFormatter> for ::windows::core::IInspectable {
    fn from(value: INumberFormatter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INumberFormatter> for ::windows::core::IInspectable {
    fn from(value: &INumberFormatter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for INumberFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a INumberFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INumberFormatter_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: i64, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u64, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INumberFormatter2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for INumberFormatter2 {
    type Vtable = INumberFormatter2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4a8c1f0_80d0_4b0d_a89e_882c1e8f8310);
}
impl INumberFormatter2 {
    pub fn FormatInt(&self, value: i64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn FormatUInt(&self, value: u64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn FormatDouble(&self, value: f64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for INumberFormatter2 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{d4a8c1f0-80d0-4b0d-a89e-882c1e8f8310}");
}
impl ::core::convert::From<INumberFormatter2> for ::windows::core::IUnknown {
    fn from(value: INumberFormatter2) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&INumberFormatter2> for ::windows::core::IUnknown {
    fn from(value: &INumberFormatter2) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INumberFormatter2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INumberFormatter2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<INumberFormatter2> for ::windows::core::IInspectable {
    fn from(value: INumberFormatter2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INumberFormatter2> for ::windows::core::IInspectable {
    fn from(value: &INumberFormatter2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for INumberFormatter2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a INumberFormatter2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INumberFormatter2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: i64, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u64, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INumberFormatterOptions(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for INumberFormatterOptions {
    type Vtable = INumberFormatterOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x80332d21_aee1_4a39_baa2_07ed8c96daf6);
}
impl INumberFormatterOptions {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Languages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    pub fn GeographicRegion(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn IntegerDigits(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn SetIntegerDigits(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn FractionDigits(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn SetFractionDigits(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsGrouped(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsGrouped(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsDecimalPointAlwaysDisplayed(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsDecimalPointAlwaysDisplayed(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn NumeralSystem(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetNumeralSystem<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ResolvedLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ResolvedGeographicRegion(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for INumberFormatterOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{80332d21-aee1-4a39-baa2-07ed8c96daf6}");
}
impl ::core::convert::From<INumberFormatterOptions> for ::windows::core::IUnknown {
    fn from(value: INumberFormatterOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&INumberFormatterOptions> for ::windows::core::IUnknown {
    fn from(value: &INumberFormatterOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INumberFormatterOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INumberFormatterOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<INumberFormatterOptions> for ::windows::core::IInspectable {
    fn from(value: INumberFormatterOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INumberFormatterOptions> for ::windows::core::IInspectable {
    fn from(value: &INumberFormatterOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for INumberFormatterOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a INumberFormatterOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INumberFormatterOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INumberParser(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for INumberParser {
    type Vtable = INumberParser_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe6659412_4a13_4a53_83a1_392fbe4cff9f);
}
impl INumberParser {
    #[cfg(feature = "Foundation")]
    pub fn ParseInt<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, text: Param0) -> ::windows::core::Result<super::super::Foundation::IReference<i64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IReference<i64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ParseUInt<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, text: Param0) -> ::windows::core::Result<super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IReference<u64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ParseDouble<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, text: Param0) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for INumberParser {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{e6659412-4a13-4a53-83a1-392fbe4cff9f}");
}
impl ::core::convert::From<INumberParser> for ::windows::core::IUnknown {
    fn from(value: INumberParser) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&INumberParser> for ::windows::core::IUnknown {
    fn from(value: &INumberParser) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INumberParser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INumberParser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<INumberParser> for ::windows::core::IInspectable {
    fn from(value: INumberParser) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INumberParser> for ::windows::core::IInspectable {
    fn from(value: &INumberParser) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for INumberParser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a INumberParser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INumberParser_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INumberRounder(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for INumberRounder {
    type Vtable = INumberRounder_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5473c375_38ed_4631_b80c_ef34fc48b7f5);
}
impl INumberRounder {
    pub fn RoundInt32(&self, value: i32) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn RoundUInt32(&self, value: u32) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn RoundInt64(&self, value: i64) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<i64>(result__)
        }
    }
    pub fn RoundUInt64(&self, value: u64) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn RoundSingle(&self, value: f32) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn RoundDouble(&self, value: f64) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<f64>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for INumberRounder {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{5473c375-38ed-4631-b80c-ef34fc48b7f5}");
}
impl ::core::convert::From<INumberRounder> for ::windows::core::IUnknown {
    fn from(value: INumberRounder) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&INumberRounder> for ::windows::core::IUnknown {
    fn from(value: &INumberRounder) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INumberRounder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INumberRounder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<INumberRounder> for ::windows::core::IInspectable {
    fn from(value: INumberRounder) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INumberRounder> for ::windows::core::IInspectable {
    fn from(value: &INumberRounder) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for INumberRounder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a INumberRounder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INumberRounder_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: i32, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: i64, result__: *mut i64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u64, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f32, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64, result__: *mut f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INumberRounderOption(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for INumberRounderOption {
    type Vtable = INumberRounderOption_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b088433_646f_4efe_8d48_66eb2e49e736);
}
impl INumberRounderOption {
    pub fn NumberRounder(&self) -> ::windows::core::Result<INumberRounder> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<INumberRounder>(result__)
        }
    }
    pub fn SetNumberRounder<'a, Param0: ::windows::core::IntoParam<'a, INumberRounder>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for INumberRounderOption {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{3b088433-646f-4efe-8d48-66eb2e49e736}");
}
impl ::core::convert::From<INumberRounderOption> for ::windows::core::IUnknown {
    fn from(value: INumberRounderOption) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&INumberRounderOption> for ::windows::core::IUnknown {
    fn from(value: &INumberRounderOption) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INumberRounderOption {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INumberRounderOption {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<INumberRounderOption> for ::windows::core::IInspectable {
    fn from(value: INumberRounderOption) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INumberRounderOption> for ::windows::core::IInspectable {
    fn from(value: &INumberRounderOption) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for INumberRounderOption {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a INumberRounderOption {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INumberRounderOption_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct INumeralSystemTranslator(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for INumeralSystemTranslator {
    type Vtable = INumeralSystemTranslator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x28f5bc2c_8c23_4234_ad2e_fa5a3a426e9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct INumeralSystemTranslator_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct INumeralSystemTranslatorFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for INumeralSystemTranslatorFactory {
    type Vtable = INumeralSystemTranslatorFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9630c8da_36ef_4d88_a85c_6f0d98d620a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct INumeralSystemTranslatorFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, languages: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPercentFormatterFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPercentFormatterFactory {
    type Vtable = IPercentFormatterFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb7828aef_fed4_4018_a6e2_e09961e03765);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPercentFormatterFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, languages: ::windows::core::RawPtr, geographicregion: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPermilleFormatterFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPermilleFormatterFactory {
    type Vtable = IPermilleFormatterFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2b37b4ac_e638_4ed5_a998_62f6b06a49ae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPermilleFormatterFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, languages: ::windows::core::RawPtr, geographicregion: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISignedZeroOption(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISignedZeroOption {
    type Vtable = ISignedZeroOption_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd1cdd31_0a3c_49c4_a642_96a1564f4f30);
}
impl ISignedZeroOption {
    pub fn IsZeroSigned(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsZeroSigned(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for ISignedZeroOption {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{fd1cdd31-0a3c-49c4-a642-96a1564f4f30}");
}
impl ::core::convert::From<ISignedZeroOption> for ::windows::core::IUnknown {
    fn from(value: ISignedZeroOption) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ISignedZeroOption> for ::windows::core::IUnknown {
    fn from(value: &ISignedZeroOption) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISignedZeroOption {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISignedZeroOption {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ISignedZeroOption> for ::windows::core::IInspectable {
    fn from(value: ISignedZeroOption) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISignedZeroOption> for ::windows::core::IInspectable {
    fn from(value: &ISignedZeroOption) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ISignedZeroOption {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ISignedZeroOption {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISignedZeroOption_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISignificantDigitsNumberRounder(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISignificantDigitsNumberRounder {
    type Vtable = ISignificantDigitsNumberRounder_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf5941bca_6646_4913_8c76_1b191ff94dfd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISignificantDigitsNumberRounder_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut RoundingAlgorithm) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: RoundingAlgorithm) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISignificantDigitsOption(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISignificantDigitsOption {
    type Vtable = ISignificantDigitsOption_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d4dfcdd_2d43_4ee8_bbf1_c1b26a711a58);
}
impl ISignificantDigitsOption {
    pub fn SignificantDigits(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn SetSignificantDigits(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for ISignificantDigitsOption {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{1d4dfcdd-2d43-4ee8-bbf1-c1b26a711a58}");
}
impl ::core::convert::From<ISignificantDigitsOption> for ::windows::core::IUnknown {
    fn from(value: ISignificantDigitsOption) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ISignificantDigitsOption> for ::windows::core::IUnknown {
    fn from(value: &ISignificantDigitsOption) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISignificantDigitsOption {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISignificantDigitsOption {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ISignificantDigitsOption> for ::windows::core::IInspectable {
    fn from(value: ISignificantDigitsOption) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISignificantDigitsOption> for ::windows::core::IInspectable {
    fn from(value: &ISignificantDigitsOption) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ISignificantDigitsOption {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ISignificantDigitsOption {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISignificantDigitsOption_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IncrementNumberRounder(pub ::windows::core::IInspectable);
impl IncrementNumberRounder {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<IncrementNumberRounder, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn RoundInt32(&self, value: i32) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn RoundUInt32(&self, value: u32) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn RoundInt64(&self, value: i64) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<i64>(result__)
        }
    }
    pub fn RoundUInt64(&self, value: u64) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn RoundSingle(&self, value: f32) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn RoundDouble(&self, value: f64) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn RoundingAlgorithm(&self) -> ::windows::core::Result<RoundingAlgorithm> {
        let this = &::windows::core::Interface::cast::<IIncrementNumberRounder>(self)?;
        unsafe {
            let mut result__: RoundingAlgorithm = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RoundingAlgorithm>(result__)
        }
    }
    pub fn SetRoundingAlgorithm(&self, value: RoundingAlgorithm) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IIncrementNumberRounder>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Increment(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IIncrementNumberRounder>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetIncrement(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IIncrementNumberRounder>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for IncrementNumberRounder {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Globalization.NumberFormatting.IncrementNumberRounder;{5473c375-38ed-4631-b80c-ef34fc48b7f5})");
}
unsafe impl ::windows::core::Interface for IncrementNumberRounder {
    type Vtable = INumberRounder_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5473c375_38ed_4631_b80c_ef34fc48b7f5);
}
impl ::windows::core::RuntimeName for IncrementNumberRounder {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.IncrementNumberRounder";
}
impl ::core::convert::From<IncrementNumberRounder> for ::windows::core::IUnknown {
    fn from(value: IncrementNumberRounder) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IncrementNumberRounder> for ::windows::core::IUnknown {
    fn from(value: &IncrementNumberRounder) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IncrementNumberRounder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IncrementNumberRounder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IncrementNumberRounder> for ::windows::core::IInspectable {
    fn from(value: IncrementNumberRounder) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IncrementNumberRounder> for ::windows::core::IInspectable {
    fn from(value: &IncrementNumberRounder) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IncrementNumberRounder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IncrementNumberRounder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IncrementNumberRounder> for INumberRounder {
    fn from(value: IncrementNumberRounder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IncrementNumberRounder> for INumberRounder {
    fn from(value: &IncrementNumberRounder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, INumberRounder> for IncrementNumberRounder {
    fn into_param(self) -> ::windows::core::Param<'a, INumberRounder> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, INumberRounder> for &IncrementNumberRounder {
    fn into_param(self) -> ::windows::core::Param<'a, INumberRounder> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for IncrementNumberRounder {}
unsafe impl ::core::marker::Sync for IncrementNumberRounder {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct NumeralSystemTranslator(pub ::windows::core::IInspectable);
impl NumeralSystemTranslator {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<NumeralSystemTranslator, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Languages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    pub fn ResolvedLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn NumeralSystem(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetNumeralSystem<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn TranslateNumerals<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(languages: Param0) -> ::windows::core::Result<NumeralSystemTranslator> {
        Self::INumeralSystemTranslatorFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), languages.into_param().abi(), &mut result__).from_abi::<NumeralSystemTranslator>(result__)
        })
    }
    pub fn INumeralSystemTranslatorFactory<R, F: FnOnce(&INumeralSystemTranslatorFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<NumeralSystemTranslator, INumeralSystemTranslatorFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for NumeralSystemTranslator {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Globalization.NumberFormatting.NumeralSystemTranslator;{28f5bc2c-8c23-4234-ad2e-fa5a3a426e9b})");
}
unsafe impl ::windows::core::Interface for NumeralSystemTranslator {
    type Vtable = INumeralSystemTranslator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x28f5bc2c_8c23_4234_ad2e_fa5a3a426e9b);
}
impl ::windows::core::RuntimeName for NumeralSystemTranslator {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.NumeralSystemTranslator";
}
impl ::core::convert::From<NumeralSystemTranslator> for ::windows::core::IUnknown {
    fn from(value: NumeralSystemTranslator) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&NumeralSystemTranslator> for ::windows::core::IUnknown {
    fn from(value: &NumeralSystemTranslator) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for NumeralSystemTranslator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a NumeralSystemTranslator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<NumeralSystemTranslator> for ::windows::core::IInspectable {
    fn from(value: NumeralSystemTranslator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&NumeralSystemTranslator> for ::windows::core::IInspectable {
    fn from(value: &NumeralSystemTranslator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for NumeralSystemTranslator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a NumeralSystemTranslator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for NumeralSystemTranslator {}
unsafe impl ::core::marker::Sync for NumeralSystemTranslator {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PercentFormatter(pub ::windows::core::IInspectable);
impl PercentFormatter {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PercentFormatter, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn FormatInt(&self, value: i64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn FormatUInt(&self, value: u64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn FormatDouble(&self, value: f64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn FormatInt2(&self, value: i64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn FormatUInt2(&self, value: u64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn FormatDouble2(&self, value: f64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Languages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    pub fn GeographicRegion(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn IntegerDigits(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn SetIntegerDigits(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn FractionDigits(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn SetFractionDigits(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsGrouped(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsGrouped(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsDecimalPointAlwaysDisplayed(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsDecimalPointAlwaysDisplayed(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn NumeralSystem(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetNumeralSystem<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ResolvedLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ResolvedGeographicRegion(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ParseInt<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, text: Param0) -> ::windows::core::Result<super::super::Foundation::IReference<i64>> {
        let this = &::windows::core::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IReference<i64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ParseUInt<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, text: Param0) -> ::windows::core::Result<super::super::Foundation::IReference<u64>> {
        let this = &::windows::core::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IReference<u64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ParseDouble<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, text: Param0) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = &::windows::core::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    pub fn NumberRounder(&self) -> ::windows::core::Result<INumberRounder> {
        let this = &::windows::core::Interface::cast::<INumberRounderOption>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<INumberRounder>(result__)
        }
    }
    pub fn SetNumberRounder<'a, Param0: ::windows::core::IntoParam<'a, INumberRounder>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<INumberRounderOption>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn IsZeroSigned(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ISignedZeroOption>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsZeroSigned(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISignedZeroOption>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn SignificantDigits(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ISignificantDigitsOption>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn SetSignificantDigits(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISignificantDigitsOption>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreatePercentFormatter<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(languages: Param0, geographicregion: Param1) -> ::windows::core::Result<PercentFormatter> {
        Self::IPercentFormatterFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), languages.into_param().abi(), geographicregion.into_param().abi(), &mut result__).from_abi::<PercentFormatter>(result__)
        })
    }
    pub fn IPercentFormatterFactory<R, F: FnOnce(&IPercentFormatterFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PercentFormatter, IPercentFormatterFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for PercentFormatter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Globalization.NumberFormatting.PercentFormatter;{a5007c49-7676-4db7-8631-1b6ff265caa9})");
}
unsafe impl ::windows::core::Interface for PercentFormatter {
    type Vtable = INumberFormatter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa5007c49_7676_4db7_8631_1b6ff265caa9);
}
impl ::windows::core::RuntimeName for PercentFormatter {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.PercentFormatter";
}
impl ::core::convert::From<PercentFormatter> for ::windows::core::IUnknown {
    fn from(value: PercentFormatter) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PercentFormatter> for ::windows::core::IUnknown {
    fn from(value: &PercentFormatter) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PercentFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PercentFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PercentFormatter> for ::windows::core::IInspectable {
    fn from(value: PercentFormatter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PercentFormatter> for ::windows::core::IInspectable {
    fn from(value: &PercentFormatter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PercentFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PercentFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<PercentFormatter> for INumberFormatter {
    fn from(value: PercentFormatter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PercentFormatter> for INumberFormatter {
    fn from(value: &PercentFormatter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, INumberFormatter> for PercentFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, INumberFormatter> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, INumberFormatter> for &PercentFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, INumberFormatter> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PercentFormatter> for INumberFormatter2 {
    type Error = ::windows::core::Error;
    fn try_from(value: PercentFormatter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PercentFormatter> for INumberFormatter2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &PercentFormatter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, INumberFormatter2> for PercentFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, INumberFormatter2> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, INumberFormatter2> for &PercentFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, INumberFormatter2> {
        ::core::convert::TryInto::<INumberFormatter2>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<PercentFormatter> for INumberFormatterOptions {
    type Error = ::windows::core::Error;
    fn try_from(value: PercentFormatter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PercentFormatter> for INumberFormatterOptions {
    type Error = ::windows::core::Error;
    fn try_from(value: &PercentFormatter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, INumberFormatterOptions> for PercentFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, INumberFormatterOptions> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, INumberFormatterOptions> for &PercentFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, INumberFormatterOptions> {
        ::core::convert::TryInto::<INumberFormatterOptions>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<PercentFormatter> for INumberParser {
    type Error = ::windows::core::Error;
    fn try_from(value: PercentFormatter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PercentFormatter> for INumberParser {
    type Error = ::windows::core::Error;
    fn try_from(value: &PercentFormatter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, INumberParser> for PercentFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, INumberParser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, INumberParser> for &PercentFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, INumberParser> {
        ::core::convert::TryInto::<INumberParser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<PercentFormatter> for INumberRounderOption {
    type Error = ::windows::core::Error;
    fn try_from(value: PercentFormatter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PercentFormatter> for INumberRounderOption {
    type Error = ::windows::core::Error;
    fn try_from(value: &PercentFormatter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, INumberRounderOption> for PercentFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, INumberRounderOption> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, INumberRounderOption> for &PercentFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, INumberRounderOption> {
        ::core::convert::TryInto::<INumberRounderOption>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<PercentFormatter> for ISignedZeroOption {
    type Error = ::windows::core::Error;
    fn try_from(value: PercentFormatter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PercentFormatter> for ISignedZeroOption {
    type Error = ::windows::core::Error;
    fn try_from(value: &PercentFormatter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISignedZeroOption> for PercentFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ISignedZeroOption> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISignedZeroOption> for &PercentFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ISignedZeroOption> {
        ::core::convert::TryInto::<ISignedZeroOption>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<PercentFormatter> for ISignificantDigitsOption {
    type Error = ::windows::core::Error;
    fn try_from(value: PercentFormatter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PercentFormatter> for ISignificantDigitsOption {
    type Error = ::windows::core::Error;
    fn try_from(value: &PercentFormatter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISignificantDigitsOption> for PercentFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ISignificantDigitsOption> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISignificantDigitsOption> for &PercentFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ISignificantDigitsOption> {
        ::core::convert::TryInto::<ISignificantDigitsOption>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for PercentFormatter {}
unsafe impl ::core::marker::Sync for PercentFormatter {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PermilleFormatter(pub ::windows::core::IInspectable);
impl PermilleFormatter {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PermilleFormatter, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn FormatInt(&self, value: i64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn FormatUInt(&self, value: u64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn FormatDouble(&self, value: f64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn FormatInt2(&self, value: i64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn FormatUInt2(&self, value: u64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn FormatDouble2(&self, value: f64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Languages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    pub fn GeographicRegion(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn IntegerDigits(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn SetIntegerDigits(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn FractionDigits(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn SetFractionDigits(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsGrouped(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsGrouped(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsDecimalPointAlwaysDisplayed(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsDecimalPointAlwaysDisplayed(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn NumeralSystem(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetNumeralSystem<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ResolvedLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ResolvedGeographicRegion(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ParseInt<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, text: Param0) -> ::windows::core::Result<super::super::Foundation::IReference<i64>> {
        let this = &::windows::core::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IReference<i64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ParseUInt<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, text: Param0) -> ::windows::core::Result<super::super::Foundation::IReference<u64>> {
        let this = &::windows::core::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IReference<u64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ParseDouble<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, text: Param0) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = &::windows::core::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    pub fn NumberRounder(&self) -> ::windows::core::Result<INumberRounder> {
        let this = &::windows::core::Interface::cast::<INumberRounderOption>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<INumberRounder>(result__)
        }
    }
    pub fn SetNumberRounder<'a, Param0: ::windows::core::IntoParam<'a, INumberRounder>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<INumberRounderOption>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn IsZeroSigned(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ISignedZeroOption>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsZeroSigned(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISignedZeroOption>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn SignificantDigits(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ISignificantDigitsOption>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn SetSignificantDigits(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISignificantDigitsOption>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreatePermilleFormatter<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(languages: Param0, geographicregion: Param1) -> ::windows::core::Result<PermilleFormatter> {
        Self::IPermilleFormatterFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), languages.into_param().abi(), geographicregion.into_param().abi(), &mut result__).from_abi::<PermilleFormatter>(result__)
        })
    }
    pub fn IPermilleFormatterFactory<R, F: FnOnce(&IPermilleFormatterFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PermilleFormatter, IPermilleFormatterFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for PermilleFormatter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Globalization.NumberFormatting.PermilleFormatter;{a5007c49-7676-4db7-8631-1b6ff265caa9})");
}
unsafe impl ::windows::core::Interface for PermilleFormatter {
    type Vtable = INumberFormatter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa5007c49_7676_4db7_8631_1b6ff265caa9);
}
impl ::windows::core::RuntimeName for PermilleFormatter {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.PermilleFormatter";
}
impl ::core::convert::From<PermilleFormatter> for ::windows::core::IUnknown {
    fn from(value: PermilleFormatter) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PermilleFormatter> for ::windows::core::IUnknown {
    fn from(value: &PermilleFormatter) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PermilleFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PermilleFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PermilleFormatter> for ::windows::core::IInspectable {
    fn from(value: PermilleFormatter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PermilleFormatter> for ::windows::core::IInspectable {
    fn from(value: &PermilleFormatter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PermilleFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PermilleFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<PermilleFormatter> for INumberFormatter {
    fn from(value: PermilleFormatter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PermilleFormatter> for INumberFormatter {
    fn from(value: &PermilleFormatter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, INumberFormatter> for PermilleFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, INumberFormatter> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, INumberFormatter> for &PermilleFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, INumberFormatter> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PermilleFormatter> for INumberFormatter2 {
    type Error = ::windows::core::Error;
    fn try_from(value: PermilleFormatter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PermilleFormatter> for INumberFormatter2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &PermilleFormatter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, INumberFormatter2> for PermilleFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, INumberFormatter2> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, INumberFormatter2> for &PermilleFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, INumberFormatter2> {
        ::core::convert::TryInto::<INumberFormatter2>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<PermilleFormatter> for INumberFormatterOptions {
    type Error = ::windows::core::Error;
    fn try_from(value: PermilleFormatter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PermilleFormatter> for INumberFormatterOptions {
    type Error = ::windows::core::Error;
    fn try_from(value: &PermilleFormatter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, INumberFormatterOptions> for PermilleFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, INumberFormatterOptions> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, INumberFormatterOptions> for &PermilleFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, INumberFormatterOptions> {
        ::core::convert::TryInto::<INumberFormatterOptions>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<PermilleFormatter> for INumberParser {
    type Error = ::windows::core::Error;
    fn try_from(value: PermilleFormatter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PermilleFormatter> for INumberParser {
    type Error = ::windows::core::Error;
    fn try_from(value: &PermilleFormatter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, INumberParser> for PermilleFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, INumberParser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, INumberParser> for &PermilleFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, INumberParser> {
        ::core::convert::TryInto::<INumberParser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<PermilleFormatter> for INumberRounderOption {
    type Error = ::windows::core::Error;
    fn try_from(value: PermilleFormatter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PermilleFormatter> for INumberRounderOption {
    type Error = ::windows::core::Error;
    fn try_from(value: &PermilleFormatter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, INumberRounderOption> for PermilleFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, INumberRounderOption> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, INumberRounderOption> for &PermilleFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, INumberRounderOption> {
        ::core::convert::TryInto::<INumberRounderOption>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<PermilleFormatter> for ISignedZeroOption {
    type Error = ::windows::core::Error;
    fn try_from(value: PermilleFormatter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PermilleFormatter> for ISignedZeroOption {
    type Error = ::windows::core::Error;
    fn try_from(value: &PermilleFormatter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISignedZeroOption> for PermilleFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ISignedZeroOption> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISignedZeroOption> for &PermilleFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ISignedZeroOption> {
        ::core::convert::TryInto::<ISignedZeroOption>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<PermilleFormatter> for ISignificantDigitsOption {
    type Error = ::windows::core::Error;
    fn try_from(value: PermilleFormatter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PermilleFormatter> for ISignificantDigitsOption {
    type Error = ::windows::core::Error;
    fn try_from(value: &PermilleFormatter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISignificantDigitsOption> for PermilleFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ISignificantDigitsOption> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISignificantDigitsOption> for &PermilleFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ISignificantDigitsOption> {
        ::core::convert::TryInto::<ISignificantDigitsOption>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for PermilleFormatter {}
unsafe impl ::core::marker::Sync for PermilleFormatter {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
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
impl ::core::convert::From<i32> for RoundingAlgorithm {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RoundingAlgorithm {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for RoundingAlgorithm {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Globalization.NumberFormatting.RoundingAlgorithm;i4)");
}
impl ::windows::core::DefaultType for RoundingAlgorithm {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SignificantDigitsNumberRounder(pub ::windows::core::IInspectable);
impl SignificantDigitsNumberRounder {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SignificantDigitsNumberRounder, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn RoundInt32(&self, value: i32) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn RoundUInt32(&self, value: u32) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn RoundInt64(&self, value: i64) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<i64>(result__)
        }
    }
    pub fn RoundUInt64(&self, value: u64) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn RoundSingle(&self, value: f32) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn RoundDouble(&self, value: f64) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn RoundingAlgorithm(&self) -> ::windows::core::Result<RoundingAlgorithm> {
        let this = &::windows::core::Interface::cast::<ISignificantDigitsNumberRounder>(self)?;
        unsafe {
            let mut result__: RoundingAlgorithm = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RoundingAlgorithm>(result__)
        }
    }
    pub fn SetRoundingAlgorithm(&self, value: RoundingAlgorithm) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISignificantDigitsNumberRounder>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn SignificantDigits(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<ISignificantDigitsNumberRounder>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SetSignificantDigits(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISignificantDigitsNumberRounder>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for SignificantDigitsNumberRounder {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Globalization.NumberFormatting.SignificantDigitsNumberRounder;{5473c375-38ed-4631-b80c-ef34fc48b7f5})");
}
unsafe impl ::windows::core::Interface for SignificantDigitsNumberRounder {
    type Vtable = INumberRounder_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5473c375_38ed_4631_b80c_ef34fc48b7f5);
}
impl ::windows::core::RuntimeName for SignificantDigitsNumberRounder {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.SignificantDigitsNumberRounder";
}
impl ::core::convert::From<SignificantDigitsNumberRounder> for ::windows::core::IUnknown {
    fn from(value: SignificantDigitsNumberRounder) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SignificantDigitsNumberRounder> for ::windows::core::IUnknown {
    fn from(value: &SignificantDigitsNumberRounder) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SignificantDigitsNumberRounder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SignificantDigitsNumberRounder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SignificantDigitsNumberRounder> for ::windows::core::IInspectable {
    fn from(value: SignificantDigitsNumberRounder) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SignificantDigitsNumberRounder> for ::windows::core::IInspectable {
    fn from(value: &SignificantDigitsNumberRounder) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SignificantDigitsNumberRounder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SignificantDigitsNumberRounder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<SignificantDigitsNumberRounder> for INumberRounder {
    fn from(value: SignificantDigitsNumberRounder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SignificantDigitsNumberRounder> for INumberRounder {
    fn from(value: &SignificantDigitsNumberRounder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, INumberRounder> for SignificantDigitsNumberRounder {
    fn into_param(self) -> ::windows::core::Param<'a, INumberRounder> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, INumberRounder> for &SignificantDigitsNumberRounder {
    fn into_param(self) -> ::windows::core::Param<'a, INumberRounder> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SignificantDigitsNumberRounder {}
unsafe impl ::core::marker::Sync for SignificantDigitsNumberRounder {}
