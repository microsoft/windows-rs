#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Globalization_NumberFormatting'*"]
#[repr(transparent)]
pub struct CurrencyFormatter(::windows::core::IUnknown);
impl CurrencyFormatter {
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn Currency(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn SetCurrency<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn Mode(&self) -> ::windows::core::Result<CurrencyFormatterMode> {
        let this = &::windows::core::Interface::cast::<ICurrencyFormatter2>(self)?;
        unsafe {
            let mut result__: CurrencyFormatterMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CurrencyFormatterMode>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn SetMode(&self, value: CurrencyFormatterMode) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICurrencyFormatter2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn ApplyRoundingForCurrency(&self, roundingalgorithm: RoundingAlgorithm) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICurrencyFormatter2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), roundingalgorithm).ok() }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn CreateCurrencyFormatterCode<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(currencycode: Param0) -> ::windows::core::Result<CurrencyFormatter> {
        Self::ICurrencyFormatterFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), currencycode.into_param().abi(), &mut result__).from_abi::<CurrencyFormatter>(result__)
        })
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateCurrencyFormatterCodeContext<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(currencycode: Param0, languages: Param1, geographicregion: Param2) -> ::windows::core::Result<CurrencyFormatter> {
        Self::ICurrencyFormatterFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), currencycode.into_param().abi(), languages.into_param().abi(), geographicregion.into_param().abi(), &mut result__).from_abi::<CurrencyFormatter>(result__)
        })
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn FormatInt(&self, value: i64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatter>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn FormatUInt(&self, value: u64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatter>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn FormatDouble(&self, value: f64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatter>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn FormatInt2(&self, value: i64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn FormatUInt2(&self, value: u64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn FormatDouble2(&self, value: f64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Languages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn GeographicRegion(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn IntegerDigits(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn SetIntegerDigits(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn FractionDigits(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn SetFractionDigits(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn IsGrouped(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn SetIsGrouped(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn IsDecimalPointAlwaysDisplayed(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn SetIsDecimalPointAlwaysDisplayed(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn NumeralSystem(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn SetNumeralSystem<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn ResolvedLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn ResolvedGeographicRegion(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ParseInt<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, text: Param0) -> ::windows::core::Result<super::super::Foundation::IReference<i64>> {
        let this = &::windows::core::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IReference<i64>>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ParseUInt<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, text: Param0) -> ::windows::core::Result<super::super::Foundation::IReference<u64>> {
        let this = &::windows::core::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IReference<u64>>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ParseDouble<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, text: Param0) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = &::windows::core::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn NumberRounder(&self) -> ::windows::core::Result<INumberRounder> {
        let this = &::windows::core::Interface::cast::<INumberRounderOption>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<INumberRounder>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn SetNumberRounder<'a, Param0: ::windows::core::IntoParam<'a, INumberRounder>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<INumberRounderOption>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn IsZeroSigned(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ISignedZeroOption>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn SetIsZeroSigned(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISignedZeroOption>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn SignificantDigits(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ISignificantDigitsOption>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn SetSignificantDigits(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISignificantDigitsOption>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc(hidden)]
    pub fn ICurrencyFormatterFactory<R, F: FnOnce(&ICurrencyFormatterFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CurrencyFormatter, ICurrencyFormatterFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for CurrencyFormatter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CurrencyFormatter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CurrencyFormatter {}
impl ::core::fmt::Debug for CurrencyFormatter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CurrencyFormatter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CurrencyFormatter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Globalization.NumberFormatting.CurrencyFormatter;{11730ca5-4b00-41b2-b332-73b12a497d54})");
}
unsafe impl ::windows::core::Interface for CurrencyFormatter {
    type Vtable = ICurrencyFormatterVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x11730ca5_4b00_41b2_b332_73b12a497d54);
}
impl ::windows::core::RuntimeName for CurrencyFormatter {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.CurrencyFormatter";
}
impl ::core::convert::From<CurrencyFormatter> for ::windows::core::IUnknown {
    fn from(value: CurrencyFormatter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CurrencyFormatter> for ::windows::core::IUnknown {
    fn from(value: &CurrencyFormatter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CurrencyFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &CurrencyFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CurrencyFormatter> for ::windows::core::IInspectable {
    fn from(value: CurrencyFormatter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CurrencyFormatter> for ::windows::core::IInspectable {
    fn from(value: &CurrencyFormatter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CurrencyFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &CurrencyFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
#[doc = "*Required features: 'Globalization_NumberFormatting'*"]
#[repr(transparent)]
pub struct CurrencyFormatterMode(pub i32);
impl CurrencyFormatterMode {
    pub const UseSymbol: Self = Self(0i32);
    pub const UseCurrencyCode: Self = Self(1i32);
}
impl ::core::marker::Copy for CurrencyFormatterMode {}
impl ::core::clone::Clone for CurrencyFormatterMode {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CurrencyFormatterMode {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CurrencyFormatterMode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CurrencyFormatterMode {}
impl ::core::fmt::Debug for CurrencyFormatterMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CurrencyFormatterMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CurrencyFormatterMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Globalization.NumberFormatting.CurrencyFormatterMode;i4)");
}
impl ::windows::core::DefaultType for CurrencyFormatterMode {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Globalization_NumberFormatting'*"]
#[repr(transparent)]
pub struct DecimalFormatter(::windows::core::IUnknown);
impl DecimalFormatter {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DecimalFormatter, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateDecimalFormatter<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(languages: Param0, geographicregion: Param1) -> ::windows::core::Result<DecimalFormatter> {
        Self::IDecimalFormatterFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), languages.into_param().abi(), geographicregion.into_param().abi(), &mut result__).from_abi::<DecimalFormatter>(result__)
        })
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn FormatInt(&self, value: i64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn FormatUInt(&self, value: u64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn FormatDouble(&self, value: f64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn FormatInt2(&self, value: i64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn FormatUInt2(&self, value: u64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn FormatDouble2(&self, value: f64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Languages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn GeographicRegion(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn IntegerDigits(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn SetIntegerDigits(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn FractionDigits(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn SetFractionDigits(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn IsGrouped(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn SetIsGrouped(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn IsDecimalPointAlwaysDisplayed(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn SetIsDecimalPointAlwaysDisplayed(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn NumeralSystem(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn SetNumeralSystem<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn ResolvedLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn ResolvedGeographicRegion(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ParseInt<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, text: Param0) -> ::windows::core::Result<super::super::Foundation::IReference<i64>> {
        let this = &::windows::core::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IReference<i64>>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ParseUInt<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, text: Param0) -> ::windows::core::Result<super::super::Foundation::IReference<u64>> {
        let this = &::windows::core::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IReference<u64>>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ParseDouble<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, text: Param0) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = &::windows::core::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn NumberRounder(&self) -> ::windows::core::Result<INumberRounder> {
        let this = &::windows::core::Interface::cast::<INumberRounderOption>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<INumberRounder>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn SetNumberRounder<'a, Param0: ::windows::core::IntoParam<'a, INumberRounder>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<INumberRounderOption>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn IsZeroSigned(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ISignedZeroOption>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn SetIsZeroSigned(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISignedZeroOption>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn SignificantDigits(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ISignificantDigitsOption>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn SetSignificantDigits(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISignificantDigitsOption>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc(hidden)]
    pub fn IDecimalFormatterFactory<R, F: FnOnce(&IDecimalFormatterFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DecimalFormatter, IDecimalFormatterFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DecimalFormatter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DecimalFormatter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DecimalFormatter {}
impl ::core::fmt::Debug for DecimalFormatter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DecimalFormatter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DecimalFormatter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Globalization.NumberFormatting.DecimalFormatter;{a5007c49-7676-4db7-8631-1b6ff265caa9})");
}
unsafe impl ::windows::core::Interface for DecimalFormatter {
    type Vtable = INumberFormatterVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa5007c49_7676_4db7_8631_1b6ff265caa9);
}
impl ::windows::core::RuntimeName for DecimalFormatter {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.DecimalFormatter";
}
impl ::core::convert::From<DecimalFormatter> for ::windows::core::IUnknown {
    fn from(value: DecimalFormatter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DecimalFormatter> for ::windows::core::IUnknown {
    fn from(value: &DecimalFormatter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DecimalFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &DecimalFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DecimalFormatter> for ::windows::core::IInspectable {
    fn from(value: DecimalFormatter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DecimalFormatter> for ::windows::core::IInspectable {
    fn from(value: &DecimalFormatter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DecimalFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &DecimalFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<DecimalFormatter> for INumberFormatter {
    type Error = ::windows::core::Error;
    fn try_from(value: DecimalFormatter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DecimalFormatter> for INumberFormatter {
    type Error = ::windows::core::Error;
    fn try_from(value: &DecimalFormatter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, INumberFormatter> for DecimalFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, INumberFormatter> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, INumberFormatter> for &DecimalFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, INumberFormatter> {
        ::core::convert::TryInto::<INumberFormatter>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
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
#[doc(hidden)]
#[repr(transparent)]
pub struct ICurrencyFormatter(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICurrencyFormatter {
    type Vtable = ICurrencyFormatterVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x11730ca5_4b00_41b2_b332_73b12a497d54);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrencyFormatterVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ICurrencyFormatter2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICurrencyFormatter2 {
    type Vtable = ICurrencyFormatter2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x072c2f1d_e7ba_4197_920e_247c92f7dea6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrencyFormatter2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CurrencyFormatterMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: CurrencyFormatterMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, roundingalgorithm: RoundingAlgorithm) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ICurrencyFormatterFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICurrencyFormatterFactory {
    type Vtable = ICurrencyFormatterFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x86c7537e_b938_4aa2_84b0_2c33dc5b1450);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrencyFormatterFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currencycode: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currencycode: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, languages: ::windows::core::RawPtr, geographicregion: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDecimalFormatterFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDecimalFormatterFactory {
    type Vtable = IDecimalFormatterFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0d018c9a_e393_46b8_b830_7a69c8f89fbb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDecimalFormatterFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languages: ::windows::core::RawPtr, geographicregion: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IIncrementNumberRounder(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IIncrementNumberRounder {
    type Vtable = IIncrementNumberRounderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x70a64ff8_66ab_4155_9da1_739e46764543);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIncrementNumberRounderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RoundingAlgorithm) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: RoundingAlgorithm) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Globalization_NumberFormatting'*"]
#[repr(transparent)]
pub struct INumberFormatter(::windows::core::IUnknown);
impl INumberFormatter {
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn FormatInt(&self, value: i64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn FormatUInt(&self, value: u64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn FormatDouble(&self, value: f64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<INumberFormatter> for ::windows::core::IInspectable {
    fn from(value: INumberFormatter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INumberFormatter> for ::windows::core::IInspectable {
    fn from(value: &INumberFormatter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for INumberFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &INumberFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<INumberFormatter> for ::windows::core::IUnknown {
    fn from(value: INumberFormatter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INumberFormatter> for ::windows::core::IUnknown {
    fn from(value: &INumberFormatter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INumberFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INumberFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INumberFormatter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INumberFormatter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INumberFormatter {}
impl ::core::fmt::Debug for INumberFormatter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INumberFormatter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for INumberFormatter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{a5007c49-7676-4db7-8631-1b6ff265caa9}");
}
unsafe impl ::windows::core::Interface for INumberFormatter {
    type Vtable = INumberFormatterVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa5007c49_7676_4db7_8631_1b6ff265caa9);
}
#[repr(C)]
#[doc(hidden)]
pub struct INumberFormatterVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i64, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u64, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Globalization_NumberFormatting'*"]
#[repr(transparent)]
pub struct INumberFormatter2(::windows::core::IUnknown);
impl INumberFormatter2 {
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn FormatInt(&self, value: i64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn FormatUInt(&self, value: u64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn FormatDouble(&self, value: f64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<INumberFormatter2> for ::windows::core::IInspectable {
    fn from(value: INumberFormatter2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INumberFormatter2> for ::windows::core::IInspectable {
    fn from(value: &INumberFormatter2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for INumberFormatter2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &INumberFormatter2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<INumberFormatter2> for ::windows::core::IUnknown {
    fn from(value: INumberFormatter2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INumberFormatter2> for ::windows::core::IUnknown {
    fn from(value: &INumberFormatter2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INumberFormatter2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INumberFormatter2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INumberFormatter2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INumberFormatter2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INumberFormatter2 {}
impl ::core::fmt::Debug for INumberFormatter2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INumberFormatter2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for INumberFormatter2 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{d4a8c1f0-80d0-4b0d-a89e-882c1e8f8310}");
}
unsafe impl ::windows::core::Interface for INumberFormatter2 {
    type Vtable = INumberFormatter2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4a8c1f0_80d0_4b0d_a89e_882c1e8f8310);
}
#[repr(C)]
#[doc(hidden)]
pub struct INumberFormatter2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i64, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u64, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Globalization_NumberFormatting'*"]
#[repr(transparent)]
pub struct INumberFormatterOptions(::windows::core::IUnknown);
impl INumberFormatterOptions {
    #[doc = "*Required features: 'Globalization_NumberFormatting', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Languages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn GeographicRegion(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn IntegerDigits(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn SetIntegerDigits(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn FractionDigits(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn SetFractionDigits(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn IsGrouped(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn SetIsGrouped(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn IsDecimalPointAlwaysDisplayed(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn SetIsDecimalPointAlwaysDisplayed(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn NumeralSystem(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn SetNumeralSystem<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn ResolvedLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn ResolvedGeographicRegion(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<INumberFormatterOptions> for ::windows::core::IInspectable {
    fn from(value: INumberFormatterOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INumberFormatterOptions> for ::windows::core::IInspectable {
    fn from(value: &INumberFormatterOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for INumberFormatterOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &INumberFormatterOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<INumberFormatterOptions> for ::windows::core::IUnknown {
    fn from(value: INumberFormatterOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INumberFormatterOptions> for ::windows::core::IUnknown {
    fn from(value: &INumberFormatterOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INumberFormatterOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INumberFormatterOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INumberFormatterOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INumberFormatterOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INumberFormatterOptions {}
impl ::core::fmt::Debug for INumberFormatterOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INumberFormatterOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for INumberFormatterOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{80332d21-aee1-4a39-baa2-07ed8c96daf6}");
}
unsafe impl ::windows::core::Interface for INumberFormatterOptions {
    type Vtable = INumberFormatterOptionsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x80332d21_aee1_4a39_baa2_07ed8c96daf6);
}
#[repr(C)]
#[doc(hidden)]
pub struct INumberFormatterOptionsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Globalization_NumberFormatting'*"]
#[repr(transparent)]
pub struct INumberParser(::windows::core::IUnknown);
impl INumberParser {
    #[doc = "*Required features: 'Globalization_NumberFormatting', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ParseInt<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, text: Param0) -> ::windows::core::Result<super::super::Foundation::IReference<i64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IReference<i64>>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ParseUInt<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, text: Param0) -> ::windows::core::Result<super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IReference<u64>>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ParseDouble<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, text: Param0) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
}
impl ::core::convert::From<INumberParser> for ::windows::core::IInspectable {
    fn from(value: INumberParser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INumberParser> for ::windows::core::IInspectable {
    fn from(value: &INumberParser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for INumberParser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &INumberParser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<INumberParser> for ::windows::core::IUnknown {
    fn from(value: INumberParser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INumberParser> for ::windows::core::IUnknown {
    fn from(value: &INumberParser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INumberParser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INumberParser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INumberParser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INumberParser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INumberParser {}
impl ::core::fmt::Debug for INumberParser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INumberParser").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for INumberParser {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{e6659412-4a13-4a53-83a1-392fbe4cff9f}");
}
unsafe impl ::windows::core::Interface for INumberParser {
    type Vtable = INumberParserVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe6659412_4a13_4a53_83a1_392fbe4cff9f);
}
#[repr(C)]
#[doc(hidden)]
pub struct INumberParserVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc = "*Required features: 'Globalization_NumberFormatting'*"]
#[repr(transparent)]
pub struct INumberRounder(::windows::core::IUnknown);
impl INumberRounder {
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn RoundInt32(&self, value: i32) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn RoundUInt32(&self, value: u32) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn RoundInt64(&self, value: i64) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn RoundUInt64(&self, value: u64) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn RoundSingle(&self, value: f32) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn RoundDouble(&self, value: f64) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<f64>(result__)
        }
    }
}
impl ::core::convert::From<INumberRounder> for ::windows::core::IInspectable {
    fn from(value: INumberRounder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INumberRounder> for ::windows::core::IInspectable {
    fn from(value: &INumberRounder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for INumberRounder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &INumberRounder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<INumberRounder> for ::windows::core::IUnknown {
    fn from(value: INumberRounder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INumberRounder> for ::windows::core::IUnknown {
    fn from(value: &INumberRounder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INumberRounder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INumberRounder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INumberRounder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INumberRounder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INumberRounder {}
impl ::core::fmt::Debug for INumberRounder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INumberRounder").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for INumberRounder {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{5473c375-38ed-4631-b80c-ef34fc48b7f5}");
}
unsafe impl ::windows::core::Interface for INumberRounder {
    type Vtable = INumberRounderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5473c375_38ed_4631_b80c_ef34fc48b7f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct INumberRounderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i64, result__: *mut i64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u64, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64, result__: *mut f64) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Globalization_NumberFormatting'*"]
#[repr(transparent)]
pub struct INumberRounderOption(::windows::core::IUnknown);
impl INumberRounderOption {
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn NumberRounder(&self) -> ::windows::core::Result<INumberRounder> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<INumberRounder>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn SetNumberRounder<'a, Param0: ::windows::core::IntoParam<'a, INumberRounder>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::convert::From<INumberRounderOption> for ::windows::core::IInspectable {
    fn from(value: INumberRounderOption) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INumberRounderOption> for ::windows::core::IInspectable {
    fn from(value: &INumberRounderOption) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for INumberRounderOption {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &INumberRounderOption {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<INumberRounderOption> for ::windows::core::IUnknown {
    fn from(value: INumberRounderOption) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INumberRounderOption> for ::windows::core::IUnknown {
    fn from(value: &INumberRounderOption) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INumberRounderOption {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INumberRounderOption {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INumberRounderOption {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INumberRounderOption {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INumberRounderOption {}
impl ::core::fmt::Debug for INumberRounderOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INumberRounderOption").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for INumberRounderOption {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{3b088433-646f-4efe-8d48-66eb2e49e736}");
}
unsafe impl ::windows::core::Interface for INumberRounderOption {
    type Vtable = INumberRounderOptionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b088433_646f_4efe_8d48_66eb2e49e736);
}
#[repr(C)]
#[doc(hidden)]
pub struct INumberRounderOptionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct INumeralSystemTranslator(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for INumeralSystemTranslator {
    type Vtable = INumeralSystemTranslatorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x28f5bc2c_8c23_4234_ad2e_fa5a3a426e9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct INumeralSystemTranslatorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct INumeralSystemTranslatorFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for INumeralSystemTranslatorFactory {
    type Vtable = INumeralSystemTranslatorFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9630c8da_36ef_4d88_a85c_6f0d98d620a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct INumeralSystemTranslatorFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languages: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPercentFormatterFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPercentFormatterFactory {
    type Vtable = IPercentFormatterFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb7828aef_fed4_4018_a6e2_e09961e03765);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPercentFormatterFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languages: ::windows::core::RawPtr, geographicregion: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPermilleFormatterFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPermilleFormatterFactory {
    type Vtable = IPermilleFormatterFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2b37b4ac_e638_4ed5_a998_62f6b06a49ae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPermilleFormatterFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languages: ::windows::core::RawPtr, geographicregion: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc = "*Required features: 'Globalization_NumberFormatting'*"]
#[repr(transparent)]
pub struct ISignedZeroOption(::windows::core::IUnknown);
impl ISignedZeroOption {
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn IsZeroSigned(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn SetIsZeroSigned(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::convert::From<ISignedZeroOption> for ::windows::core::IInspectable {
    fn from(value: ISignedZeroOption) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISignedZeroOption> for ::windows::core::IInspectable {
    fn from(value: &ISignedZeroOption) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ISignedZeroOption {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ISignedZeroOption {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISignedZeroOption> for ::windows::core::IUnknown {
    fn from(value: ISignedZeroOption) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISignedZeroOption> for ::windows::core::IUnknown {
    fn from(value: &ISignedZeroOption) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISignedZeroOption {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISignedZeroOption {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISignedZeroOption {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISignedZeroOption {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISignedZeroOption {}
impl ::core::fmt::Debug for ISignedZeroOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISignedZeroOption").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ISignedZeroOption {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{fd1cdd31-0a3c-49c4-a642-96a1564f4f30}");
}
unsafe impl ::windows::core::Interface for ISignedZeroOption {
    type Vtable = ISignedZeroOptionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd1cdd31_0a3c_49c4_a642_96a1564f4f30);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISignedZeroOptionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISignificantDigitsNumberRounder(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISignificantDigitsNumberRounder {
    type Vtable = ISignificantDigitsNumberRounderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf5941bca_6646_4913_8c76_1b191ff94dfd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISignificantDigitsNumberRounderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RoundingAlgorithm) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: RoundingAlgorithm) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Globalization_NumberFormatting'*"]
#[repr(transparent)]
pub struct ISignificantDigitsOption(::windows::core::IUnknown);
impl ISignificantDigitsOption {
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn SignificantDigits(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn SetSignificantDigits(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::convert::From<ISignificantDigitsOption> for ::windows::core::IInspectable {
    fn from(value: ISignificantDigitsOption) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISignificantDigitsOption> for ::windows::core::IInspectable {
    fn from(value: &ISignificantDigitsOption) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ISignificantDigitsOption {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ISignificantDigitsOption {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISignificantDigitsOption> for ::windows::core::IUnknown {
    fn from(value: ISignificantDigitsOption) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISignificantDigitsOption> for ::windows::core::IUnknown {
    fn from(value: &ISignificantDigitsOption) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISignificantDigitsOption {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISignificantDigitsOption {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISignificantDigitsOption {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISignificantDigitsOption {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISignificantDigitsOption {}
impl ::core::fmt::Debug for ISignificantDigitsOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISignificantDigitsOption").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ISignificantDigitsOption {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{1d4dfcdd-2d43-4ee8-bbf1-c1b26a711a58}");
}
unsafe impl ::windows::core::Interface for ISignificantDigitsOption {
    type Vtable = ISignificantDigitsOptionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d4dfcdd_2d43_4ee8_bbf1_c1b26a711a58);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISignificantDigitsOptionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Globalization_NumberFormatting'*"]
#[repr(transparent)]
pub struct IncrementNumberRounder(::windows::core::IUnknown);
impl IncrementNumberRounder {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<IncrementNumberRounder, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn RoundingAlgorithm(&self) -> ::windows::core::Result<RoundingAlgorithm> {
        let this = &::windows::core::Interface::cast::<IIncrementNumberRounder>(self)?;
        unsafe {
            let mut result__: RoundingAlgorithm = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RoundingAlgorithm>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn SetRoundingAlgorithm(&self, value: RoundingAlgorithm) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IIncrementNumberRounder>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn Increment(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IIncrementNumberRounder>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn SetIncrement(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IIncrementNumberRounder>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn RoundInt32(&self, value: i32) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn RoundUInt32(&self, value: u32) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn RoundInt64(&self, value: i64) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn RoundUInt64(&self, value: u64) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn RoundSingle(&self, value: f32) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn RoundDouble(&self, value: f64) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<f64>(result__)
        }
    }
}
impl ::core::clone::Clone for IncrementNumberRounder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IncrementNumberRounder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IncrementNumberRounder {}
impl ::core::fmt::Debug for IncrementNumberRounder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IncrementNumberRounder").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IncrementNumberRounder {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Globalization.NumberFormatting.IncrementNumberRounder;{5473c375-38ed-4631-b80c-ef34fc48b7f5})");
}
unsafe impl ::windows::core::Interface for IncrementNumberRounder {
    type Vtable = INumberRounderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5473c375_38ed_4631_b80c_ef34fc48b7f5);
}
impl ::windows::core::RuntimeName for IncrementNumberRounder {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.IncrementNumberRounder";
}
impl ::core::convert::From<IncrementNumberRounder> for ::windows::core::IUnknown {
    fn from(value: IncrementNumberRounder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IncrementNumberRounder> for ::windows::core::IUnknown {
    fn from(value: &IncrementNumberRounder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IncrementNumberRounder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IncrementNumberRounder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IncrementNumberRounder> for ::windows::core::IInspectable {
    fn from(value: IncrementNumberRounder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IncrementNumberRounder> for ::windows::core::IInspectable {
    fn from(value: &IncrementNumberRounder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IncrementNumberRounder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IncrementNumberRounder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IncrementNumberRounder> for INumberRounder {
    type Error = ::windows::core::Error;
    fn try_from(value: IncrementNumberRounder) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IncrementNumberRounder> for INumberRounder {
    type Error = ::windows::core::Error;
    fn try_from(value: &IncrementNumberRounder) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, INumberRounder> for IncrementNumberRounder {
    fn into_param(self) -> ::windows::core::Param<'a, INumberRounder> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, INumberRounder> for &IncrementNumberRounder {
    fn into_param(self) -> ::windows::core::Param<'a, INumberRounder> {
        ::core::convert::TryInto::<INumberRounder>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for IncrementNumberRounder {}
unsafe impl ::core::marker::Sync for IncrementNumberRounder {}
#[doc = "*Required features: 'Globalization_NumberFormatting'*"]
#[repr(transparent)]
pub struct NumeralSystemTranslator(::windows::core::IUnknown);
impl NumeralSystemTranslator {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<NumeralSystemTranslator, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Languages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn ResolvedLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn NumeralSystem(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn SetNumeralSystem<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn TranslateNumerals<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(languages: Param0) -> ::windows::core::Result<NumeralSystemTranslator> {
        Self::INumeralSystemTranslatorFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), languages.into_param().abi(), &mut result__).from_abi::<NumeralSystemTranslator>(result__)
        })
    }
    #[doc(hidden)]
    pub fn INumeralSystemTranslatorFactory<R, F: FnOnce(&INumeralSystemTranslatorFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<NumeralSystemTranslator, INumeralSystemTranslatorFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for NumeralSystemTranslator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for NumeralSystemTranslator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NumeralSystemTranslator {}
impl ::core::fmt::Debug for NumeralSystemTranslator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NumeralSystemTranslator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for NumeralSystemTranslator {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Globalization.NumberFormatting.NumeralSystemTranslator;{28f5bc2c-8c23-4234-ad2e-fa5a3a426e9b})");
}
unsafe impl ::windows::core::Interface for NumeralSystemTranslator {
    type Vtable = INumeralSystemTranslatorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x28f5bc2c_8c23_4234_ad2e_fa5a3a426e9b);
}
impl ::windows::core::RuntimeName for NumeralSystemTranslator {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.NumeralSystemTranslator";
}
impl ::core::convert::From<NumeralSystemTranslator> for ::windows::core::IUnknown {
    fn from(value: NumeralSystemTranslator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NumeralSystemTranslator> for ::windows::core::IUnknown {
    fn from(value: &NumeralSystemTranslator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for NumeralSystemTranslator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &NumeralSystemTranslator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<NumeralSystemTranslator> for ::windows::core::IInspectable {
    fn from(value: NumeralSystemTranslator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NumeralSystemTranslator> for ::windows::core::IInspectable {
    fn from(value: &NumeralSystemTranslator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for NumeralSystemTranslator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &NumeralSystemTranslator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for NumeralSystemTranslator {}
unsafe impl ::core::marker::Sync for NumeralSystemTranslator {}
#[doc = "*Required features: 'Globalization_NumberFormatting'*"]
#[repr(transparent)]
pub struct PercentFormatter(::windows::core::IUnknown);
impl PercentFormatter {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PercentFormatter, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn FormatInt(&self, value: i64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn FormatUInt(&self, value: u64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn FormatDouble(&self, value: f64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn FormatInt2(&self, value: i64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn FormatUInt2(&self, value: u64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn FormatDouble2(&self, value: f64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Languages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn GeographicRegion(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn IntegerDigits(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn SetIntegerDigits(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn FractionDigits(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn SetFractionDigits(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn IsGrouped(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn SetIsGrouped(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn IsDecimalPointAlwaysDisplayed(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn SetIsDecimalPointAlwaysDisplayed(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn NumeralSystem(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn SetNumeralSystem<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn ResolvedLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn ResolvedGeographicRegion(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ParseInt<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, text: Param0) -> ::windows::core::Result<super::super::Foundation::IReference<i64>> {
        let this = &::windows::core::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IReference<i64>>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ParseUInt<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, text: Param0) -> ::windows::core::Result<super::super::Foundation::IReference<u64>> {
        let this = &::windows::core::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IReference<u64>>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ParseDouble<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, text: Param0) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = &::windows::core::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn NumberRounder(&self) -> ::windows::core::Result<INumberRounder> {
        let this = &::windows::core::Interface::cast::<INumberRounderOption>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<INumberRounder>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn SetNumberRounder<'a, Param0: ::windows::core::IntoParam<'a, INumberRounder>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<INumberRounderOption>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreatePercentFormatter<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(languages: Param0, geographicregion: Param1) -> ::windows::core::Result<PercentFormatter> {
        Self::IPercentFormatterFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), languages.into_param().abi(), geographicregion.into_param().abi(), &mut result__).from_abi::<PercentFormatter>(result__)
        })
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn IsZeroSigned(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ISignedZeroOption>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn SetIsZeroSigned(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISignedZeroOption>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn SignificantDigits(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ISignificantDigitsOption>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn SetSignificantDigits(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISignificantDigitsOption>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc(hidden)]
    pub fn IPercentFormatterFactory<R, F: FnOnce(&IPercentFormatterFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PercentFormatter, IPercentFormatterFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PercentFormatter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PercentFormatter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PercentFormatter {}
impl ::core::fmt::Debug for PercentFormatter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PercentFormatter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PercentFormatter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Globalization.NumberFormatting.PercentFormatter;{a5007c49-7676-4db7-8631-1b6ff265caa9})");
}
unsafe impl ::windows::core::Interface for PercentFormatter {
    type Vtable = INumberFormatterVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa5007c49_7676_4db7_8631_1b6ff265caa9);
}
impl ::windows::core::RuntimeName for PercentFormatter {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.PercentFormatter";
}
impl ::core::convert::From<PercentFormatter> for ::windows::core::IUnknown {
    fn from(value: PercentFormatter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PercentFormatter> for ::windows::core::IUnknown {
    fn from(value: &PercentFormatter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PercentFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PercentFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PercentFormatter> for ::windows::core::IInspectable {
    fn from(value: PercentFormatter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PercentFormatter> for ::windows::core::IInspectable {
    fn from(value: &PercentFormatter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PercentFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PercentFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PercentFormatter> for INumberFormatter {
    type Error = ::windows::core::Error;
    fn try_from(value: PercentFormatter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PercentFormatter> for INumberFormatter {
    type Error = ::windows::core::Error;
    fn try_from(value: &PercentFormatter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, INumberFormatter> for PercentFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, INumberFormatter> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, INumberFormatter> for &PercentFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, INumberFormatter> {
        ::core::convert::TryInto::<INumberFormatter>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
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
#[doc = "*Required features: 'Globalization_NumberFormatting'*"]
#[repr(transparent)]
pub struct PermilleFormatter(::windows::core::IUnknown);
impl PermilleFormatter {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PermilleFormatter, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn FormatInt(&self, value: i64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn FormatUInt(&self, value: u64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn FormatDouble(&self, value: f64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn FormatInt2(&self, value: i64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn FormatUInt2(&self, value: u64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn FormatDouble2(&self, value: f64) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Languages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn GeographicRegion(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn IntegerDigits(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn SetIntegerDigits(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn FractionDigits(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn SetFractionDigits(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn IsGrouped(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn SetIsGrouped(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn IsDecimalPointAlwaysDisplayed(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn SetIsDecimalPointAlwaysDisplayed(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn NumeralSystem(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn SetNumeralSystem<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn ResolvedLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn ResolvedGeographicRegion(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ParseInt<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, text: Param0) -> ::windows::core::Result<super::super::Foundation::IReference<i64>> {
        let this = &::windows::core::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IReference<i64>>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ParseUInt<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, text: Param0) -> ::windows::core::Result<super::super::Foundation::IReference<u64>> {
        let this = &::windows::core::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IReference<u64>>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ParseDouble<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, text: Param0) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = &::windows::core::Interface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn NumberRounder(&self) -> ::windows::core::Result<INumberRounder> {
        let this = &::windows::core::Interface::cast::<INumberRounderOption>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<INumberRounder>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn SetNumberRounder<'a, Param0: ::windows::core::IntoParam<'a, INumberRounder>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<INumberRounderOption>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreatePermilleFormatter<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(languages: Param0, geographicregion: Param1) -> ::windows::core::Result<PermilleFormatter> {
        Self::IPermilleFormatterFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), languages.into_param().abi(), geographicregion.into_param().abi(), &mut result__).from_abi::<PermilleFormatter>(result__)
        })
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn IsZeroSigned(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ISignedZeroOption>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn SetIsZeroSigned(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISignedZeroOption>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn SignificantDigits(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ISignificantDigitsOption>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn SetSignificantDigits(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISignificantDigitsOption>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc(hidden)]
    pub fn IPermilleFormatterFactory<R, F: FnOnce(&IPermilleFormatterFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PermilleFormatter, IPermilleFormatterFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PermilleFormatter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PermilleFormatter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PermilleFormatter {}
impl ::core::fmt::Debug for PermilleFormatter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PermilleFormatter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PermilleFormatter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Globalization.NumberFormatting.PermilleFormatter;{a5007c49-7676-4db7-8631-1b6ff265caa9})");
}
unsafe impl ::windows::core::Interface for PermilleFormatter {
    type Vtable = INumberFormatterVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa5007c49_7676_4db7_8631_1b6ff265caa9);
}
impl ::windows::core::RuntimeName for PermilleFormatter {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.PermilleFormatter";
}
impl ::core::convert::From<PermilleFormatter> for ::windows::core::IUnknown {
    fn from(value: PermilleFormatter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PermilleFormatter> for ::windows::core::IUnknown {
    fn from(value: &PermilleFormatter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PermilleFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PermilleFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PermilleFormatter> for ::windows::core::IInspectable {
    fn from(value: PermilleFormatter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PermilleFormatter> for ::windows::core::IInspectable {
    fn from(value: &PermilleFormatter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PermilleFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PermilleFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PermilleFormatter> for INumberFormatter {
    type Error = ::windows::core::Error;
    fn try_from(value: PermilleFormatter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PermilleFormatter> for INumberFormatter {
    type Error = ::windows::core::Error;
    fn try_from(value: &PermilleFormatter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, INumberFormatter> for PermilleFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, INumberFormatter> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, INumberFormatter> for &PermilleFormatter {
    fn into_param(self) -> ::windows::core::Param<'a, INumberFormatter> {
        ::core::convert::TryInto::<INumberFormatter>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
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
#[doc = "*Required features: 'Globalization_NumberFormatting'*"]
#[repr(transparent)]
pub struct RoundingAlgorithm(pub i32);
impl RoundingAlgorithm {
    pub const None: Self = Self(0i32);
    pub const RoundDown: Self = Self(1i32);
    pub const RoundUp: Self = Self(2i32);
    pub const RoundTowardsZero: Self = Self(3i32);
    pub const RoundAwayFromZero: Self = Self(4i32);
    pub const RoundHalfDown: Self = Self(5i32);
    pub const RoundHalfUp: Self = Self(6i32);
    pub const RoundHalfTowardsZero: Self = Self(7i32);
    pub const RoundHalfAwayFromZero: Self = Self(8i32);
    pub const RoundHalfToEven: Self = Self(9i32);
    pub const RoundHalfToOdd: Self = Self(10i32);
}
impl ::core::marker::Copy for RoundingAlgorithm {}
impl ::core::clone::Clone for RoundingAlgorithm {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RoundingAlgorithm {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RoundingAlgorithm {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RoundingAlgorithm {}
impl ::core::fmt::Debug for RoundingAlgorithm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RoundingAlgorithm").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RoundingAlgorithm {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Globalization.NumberFormatting.RoundingAlgorithm;i4)");
}
impl ::windows::core::DefaultType for RoundingAlgorithm {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Globalization_NumberFormatting'*"]
#[repr(transparent)]
pub struct SignificantDigitsNumberRounder(::windows::core::IUnknown);
impl SignificantDigitsNumberRounder {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SignificantDigitsNumberRounder, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn RoundInt32(&self, value: i32) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn RoundUInt32(&self, value: u32) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn RoundInt64(&self, value: i64) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn RoundUInt64(&self, value: u64) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn RoundSingle(&self, value: f32) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn RoundDouble(&self, value: f64) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn RoundingAlgorithm(&self) -> ::windows::core::Result<RoundingAlgorithm> {
        let this = &::windows::core::Interface::cast::<ISignificantDigitsNumberRounder>(self)?;
        unsafe {
            let mut result__: RoundingAlgorithm = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RoundingAlgorithm>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn SetRoundingAlgorithm(&self, value: RoundingAlgorithm) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISignificantDigitsNumberRounder>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn SignificantDigits(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<ISignificantDigitsNumberRounder>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Globalization_NumberFormatting'*"]
    pub fn SetSignificantDigits(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISignificantDigitsNumberRounder>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for SignificantDigitsNumberRounder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SignificantDigitsNumberRounder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SignificantDigitsNumberRounder {}
impl ::core::fmt::Debug for SignificantDigitsNumberRounder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SignificantDigitsNumberRounder").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SignificantDigitsNumberRounder {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Globalization.NumberFormatting.SignificantDigitsNumberRounder;{5473c375-38ed-4631-b80c-ef34fc48b7f5})");
}
unsafe impl ::windows::core::Interface for SignificantDigitsNumberRounder {
    type Vtable = INumberRounderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5473c375_38ed_4631_b80c_ef34fc48b7f5);
}
impl ::windows::core::RuntimeName for SignificantDigitsNumberRounder {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.SignificantDigitsNumberRounder";
}
impl ::core::convert::From<SignificantDigitsNumberRounder> for ::windows::core::IUnknown {
    fn from(value: SignificantDigitsNumberRounder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SignificantDigitsNumberRounder> for ::windows::core::IUnknown {
    fn from(value: &SignificantDigitsNumberRounder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SignificantDigitsNumberRounder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SignificantDigitsNumberRounder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SignificantDigitsNumberRounder> for ::windows::core::IInspectable {
    fn from(value: SignificantDigitsNumberRounder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SignificantDigitsNumberRounder> for ::windows::core::IInspectable {
    fn from(value: &SignificantDigitsNumberRounder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SignificantDigitsNumberRounder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SignificantDigitsNumberRounder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SignificantDigitsNumberRounder> for INumberRounder {
    type Error = ::windows::core::Error;
    fn try_from(value: SignificantDigitsNumberRounder) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SignificantDigitsNumberRounder> for INumberRounder {
    type Error = ::windows::core::Error;
    fn try_from(value: &SignificantDigitsNumberRounder) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, INumberRounder> for SignificantDigitsNumberRounder {
    fn into_param(self) -> ::windows::core::Param<'a, INumberRounder> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, INumberRounder> for &SignificantDigitsNumberRounder {
    fn into_param(self) -> ::windows::core::Param<'a, INumberRounder> {
        ::core::convert::TryInto::<INumberRounder>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SignificantDigitsNumberRounder {}
unsafe impl ::core::marker::Sync for SignificantDigitsNumberRounder {}
