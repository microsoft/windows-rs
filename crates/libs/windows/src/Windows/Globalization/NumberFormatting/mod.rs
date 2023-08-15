#[doc(hidden)]
#[repr(transparent)]
pub struct ICurrencyFormatter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICurrencyFormatter {
    type Vtable = ICurrencyFormatter_Vtbl;
}
impl ::core::clone::Clone for ICurrencyFormatter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICurrencyFormatter {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x11730ca5_4b00_41b2_b332_73b12a497d54);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrencyFormatter_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Currency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub SetCurrency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetCurrency: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICurrencyFormatter2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICurrencyFormatter2 {
    type Vtable = ICurrencyFormatter2_Vtbl;
}
impl ::core::clone::Clone for ICurrencyFormatter2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICurrencyFormatter2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x072c2f1d_e7ba_4197_920e_247c92f7dea6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrencyFormatter2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Mode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CurrencyFormatterMode) -> ::windows_core::HRESULT,
    pub SetMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: CurrencyFormatterMode) -> ::windows_core::HRESULT,
    pub ApplyRoundingForCurrency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, roundingalgorithm: RoundingAlgorithm) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICurrencyFormatterFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICurrencyFormatterFactory {
    type Vtable = ICurrencyFormatterFactory_Vtbl;
}
impl ::core::clone::Clone for ICurrencyFormatterFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICurrencyFormatterFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x86c7537e_b938_4aa2_84b0_2c33dc5b1450);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrencyFormatterFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateCurrencyFormatterCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currencycode: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateCurrencyFormatterCodeContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currencycode: ::std::mem::MaybeUninit<::windows_core::HSTRING>, languages: *mut ::core::ffi::c_void, geographicregion: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateCurrencyFormatterCodeContext: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDecimalFormatterFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDecimalFormatterFactory {
    type Vtable = IDecimalFormatterFactory_Vtbl;
}
impl ::core::clone::Clone for IDecimalFormatterFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDecimalFormatterFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0d018c9a_e393_46b8_b830_7a69c8f89fbb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDecimalFormatterFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateDecimalFormatter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languages: *mut ::core::ffi::c_void, geographicregion: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateDecimalFormatter: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIncrementNumberRounder(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIncrementNumberRounder {
    type Vtable = IIncrementNumberRounder_Vtbl;
}
impl ::core::clone::Clone for IIncrementNumberRounder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IIncrementNumberRounder {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x70a64ff8_66ab_4155_9da1_739e46764543);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIncrementNumberRounder_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub RoundingAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RoundingAlgorithm) -> ::windows_core::HRESULT,
    pub SetRoundingAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: RoundingAlgorithm) -> ::windows_core::HRESULT,
    pub Increment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetIncrement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Globalization_NumberFormatting\"`*"]
#[repr(transparent)]
pub struct INumberFormatter(::windows_core::IUnknown);
impl INumberFormatter {
    pub fn FormatInt(&self, value: i64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FormatInt)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    pub fn FormatUInt(&self, value: u64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FormatUInt)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    pub fn FormatDouble(&self, value: f64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FormatDouble)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(INumberFormatter, ::windows_core::IUnknown, ::windows_core::IInspectable);
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
impl ::windows_core::RuntimeType for INumberFormatter {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{a5007c49-7676-4db7-8631-1b6ff265caa9}");
}
unsafe impl ::windows_core::Interface for INumberFormatter {
    type Vtable = INumberFormatter_Vtbl;
}
impl ::core::clone::Clone for INumberFormatter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for INumberFormatter {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa5007c49_7676_4db7_8631_1b6ff265caa9);
}
#[repr(C)]
#[doc(hidden)]
pub struct INumberFormatter_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub FormatInt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i64, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FormatUInt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u64, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FormatDouble: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Globalization_NumberFormatting\"`*"]
#[repr(transparent)]
pub struct INumberFormatter2(::windows_core::IUnknown);
impl INumberFormatter2 {
    pub fn FormatInt(&self, value: i64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FormatInt)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    pub fn FormatUInt(&self, value: u64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FormatUInt)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    pub fn FormatDouble(&self, value: f64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FormatDouble)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(INumberFormatter2, ::windows_core::IUnknown, ::windows_core::IInspectable);
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
impl ::windows_core::RuntimeType for INumberFormatter2 {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{d4a8c1f0-80d0-4b0d-a89e-882c1e8f8310}");
}
unsafe impl ::windows_core::Interface for INumberFormatter2 {
    type Vtable = INumberFormatter2_Vtbl;
}
impl ::core::clone::Clone for INumberFormatter2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for INumberFormatter2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd4a8c1f0_80d0_4b0d_a89e_882c1e8f8310);
}
#[repr(C)]
#[doc(hidden)]
pub struct INumberFormatter2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub FormatInt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i64, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FormatUInt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u64, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FormatDouble: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Globalization_NumberFormatting\"`*"]
#[repr(transparent)]
pub struct INumberFormatterOptions(::windows_core::IUnknown);
impl INumberFormatterOptions {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Languages(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Languages)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GeographicRegion(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GeographicRegion)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IntegerDigits(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IntegerDigits)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIntegerDigits(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIntegerDigits)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FractionDigits(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FractionDigits)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetFractionDigits(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFractionDigits)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsGrouped(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsGrouped)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsGrouped(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsGrouped)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDecimalPointAlwaysDisplayed(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsDecimalPointAlwaysDisplayed)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsDecimalPointAlwaysDisplayed(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsDecimalPointAlwaysDisplayed)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn NumeralSystem(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NumeralSystem)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNumeralSystem(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetNumeralSystem)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ResolvedLanguage(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResolvedLanguage)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ResolvedGeographicRegion(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResolvedGeographicRegion)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(INumberFormatterOptions, ::windows_core::IUnknown, ::windows_core::IInspectable);
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
impl ::windows_core::RuntimeType for INumberFormatterOptions {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{80332d21-aee1-4a39-baa2-07ed8c96daf6}");
}
unsafe impl ::windows_core::Interface for INumberFormatterOptions {
    type Vtable = INumberFormatterOptions_Vtbl;
}
impl ::core::clone::Clone for INumberFormatterOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for INumberFormatterOptions {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x80332d21_aee1_4a39_baa2_07ed8c96daf6);
}
#[repr(C)]
#[doc(hidden)]
pub struct INumberFormatterOptions_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Languages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Languages: usize,
    pub GeographicRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IntegerDigits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetIntegerDigits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub FractionDigits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetFractionDigits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub IsGrouped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsGrouped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsDecimalPointAlwaysDisplayed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsDecimalPointAlwaysDisplayed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub NumeralSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetNumeralSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ResolvedLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ResolvedGeographicRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Globalization_NumberFormatting\"`*"]
#[repr(transparent)]
pub struct INumberParser(::windows_core::IUnknown);
impl INumberParser {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ParseInt(&self, text: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IReference<i64>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ParseInt)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(text), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ParseUInt(&self, text: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ParseUInt)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(text), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ParseDouble(&self, text: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ParseDouble)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(text), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(INumberParser, ::windows_core::IUnknown, ::windows_core::IInspectable);
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
impl ::windows_core::RuntimeType for INumberParser {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{e6659412-4a13-4a53-83a1-392fbe4cff9f}");
}
unsafe impl ::windows_core::Interface for INumberParser {
    type Vtable = INumberParser_Vtbl;
}
impl ::core::clone::Clone for INumberParser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for INumberParser {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe6659412_4a13_4a53_83a1_392fbe4cff9f);
}
#[repr(C)]
#[doc(hidden)]
pub struct INumberParser_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ParseInt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ParseInt: usize,
    #[cfg(feature = "Foundation")]
    pub ParseUInt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ParseUInt: usize,
    #[cfg(feature = "Foundation")]
    pub ParseDouble: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ParseDouble: usize,
}
#[doc = "*Required features: `\"Globalization_NumberFormatting\"`*"]
#[repr(transparent)]
pub struct INumberRounder(::windows_core::IUnknown);
impl INumberRounder {
    pub fn RoundInt32(&self, value: i32) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RoundInt32)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    pub fn RoundUInt32(&self, value: u32) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RoundUInt32)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    pub fn RoundInt64(&self, value: i64) -> ::windows_core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RoundInt64)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    pub fn RoundUInt64(&self, value: u64) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RoundUInt64)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    pub fn RoundSingle(&self, value: f32) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RoundSingle)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    pub fn RoundDouble(&self, value: f64) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RoundDouble)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(INumberRounder, ::windows_core::IUnknown, ::windows_core::IInspectable);
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
impl ::windows_core::RuntimeType for INumberRounder {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{5473c375-38ed-4631-b80c-ef34fc48b7f5}");
}
unsafe impl ::windows_core::Interface for INumberRounder {
    type Vtable = INumberRounder_Vtbl;
}
impl ::core::clone::Clone for INumberRounder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for INumberRounder {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5473c375_38ed_4631_b80c_ef34fc48b7f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct INumberRounder_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub RoundInt32: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32, result__: *mut i32) -> ::windows_core::HRESULT,
    pub RoundUInt32: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32, result__: *mut u32) -> ::windows_core::HRESULT,
    pub RoundInt64: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i64, result__: *mut i64) -> ::windows_core::HRESULT,
    pub RoundUInt64: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u64, result__: *mut u64) -> ::windows_core::HRESULT,
    pub RoundSingle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32, result__: *mut f32) -> ::windows_core::HRESULT,
    pub RoundDouble: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64, result__: *mut f64) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Globalization_NumberFormatting\"`*"]
#[repr(transparent)]
pub struct INumberRounderOption(::windows_core::IUnknown);
impl INumberRounderOption {
    pub fn NumberRounder(&self) -> ::windows_core::Result<INumberRounder> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NumberRounder)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNumberRounder<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<INumberRounder>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetNumberRounder)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
}
::windows_core::imp::interface_hierarchy!(INumberRounderOption, ::windows_core::IUnknown, ::windows_core::IInspectable);
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
impl ::windows_core::RuntimeType for INumberRounderOption {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{3b088433-646f-4efe-8d48-66eb2e49e736}");
}
unsafe impl ::windows_core::Interface for INumberRounderOption {
    type Vtable = INumberRounderOption_Vtbl;
}
impl ::core::clone::Clone for INumberRounderOption {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for INumberRounderOption {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3b088433_646f_4efe_8d48_66eb2e49e736);
}
#[repr(C)]
#[doc(hidden)]
pub struct INumberRounderOption_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub NumberRounder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetNumberRounder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INumeralSystemTranslator(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INumeralSystemTranslator {
    type Vtable = INumeralSystemTranslator_Vtbl;
}
impl ::core::clone::Clone for INumeralSystemTranslator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for INumeralSystemTranslator {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x28f5bc2c_8c23_4234_ad2e_fa5a3a426e9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct INumeralSystemTranslator_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Languages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Languages: usize,
    pub ResolvedLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub NumeralSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetNumeralSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TranslateNumerals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INumeralSystemTranslatorFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INumeralSystemTranslatorFactory {
    type Vtable = INumeralSystemTranslatorFactory_Vtbl;
}
impl ::core::clone::Clone for INumeralSystemTranslatorFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for INumeralSystemTranslatorFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9630c8da_36ef_4d88_a85c_6f0d98d620a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct INumeralSystemTranslatorFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languages: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPercentFormatterFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPercentFormatterFactory {
    type Vtable = IPercentFormatterFactory_Vtbl;
}
impl ::core::clone::Clone for IPercentFormatterFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPercentFormatterFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb7828aef_fed4_4018_a6e2_e09961e03765);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPercentFormatterFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreatePercentFormatter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languages: *mut ::core::ffi::c_void, geographicregion: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreatePercentFormatter: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPermilleFormatterFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPermilleFormatterFactory {
    type Vtable = IPermilleFormatterFactory_Vtbl;
}
impl ::core::clone::Clone for IPermilleFormatterFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPermilleFormatterFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2b37b4ac_e638_4ed5_a998_62f6b06a49ae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPermilleFormatterFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreatePermilleFormatter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languages: *mut ::core::ffi::c_void, geographicregion: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreatePermilleFormatter: usize,
}
#[doc = "*Required features: `\"Globalization_NumberFormatting\"`*"]
#[repr(transparent)]
pub struct ISignedZeroOption(::windows_core::IUnknown);
impl ISignedZeroOption {
    pub fn IsZeroSigned(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsZeroSigned)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsZeroSigned(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsZeroSigned)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
::windows_core::imp::interface_hierarchy!(ISignedZeroOption, ::windows_core::IUnknown, ::windows_core::IInspectable);
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
impl ::windows_core::RuntimeType for ISignedZeroOption {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{fd1cdd31-0a3c-49c4-a642-96a1564f4f30}");
}
unsafe impl ::windows_core::Interface for ISignedZeroOption {
    type Vtable = ISignedZeroOption_Vtbl;
}
impl ::core::clone::Clone for ISignedZeroOption {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISignedZeroOption {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfd1cdd31_0a3c_49c4_a642_96a1564f4f30);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISignedZeroOption_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsZeroSigned: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsZeroSigned: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISignificantDigitsNumberRounder(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISignificantDigitsNumberRounder {
    type Vtable = ISignificantDigitsNumberRounder_Vtbl;
}
impl ::core::clone::Clone for ISignificantDigitsNumberRounder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISignificantDigitsNumberRounder {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf5941bca_6646_4913_8c76_1b191ff94dfd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISignificantDigitsNumberRounder_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub RoundingAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RoundingAlgorithm) -> ::windows_core::HRESULT,
    pub SetRoundingAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: RoundingAlgorithm) -> ::windows_core::HRESULT,
    pub SignificantDigits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetSignificantDigits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Globalization_NumberFormatting\"`*"]
#[repr(transparent)]
pub struct ISignificantDigitsOption(::windows_core::IUnknown);
impl ISignificantDigitsOption {
    pub fn SignificantDigits(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SignificantDigits)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSignificantDigits(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSignificantDigits)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
::windows_core::imp::interface_hierarchy!(ISignificantDigitsOption, ::windows_core::IUnknown, ::windows_core::IInspectable);
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
impl ::windows_core::RuntimeType for ISignificantDigitsOption {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{1d4dfcdd-2d43-4ee8-bbf1-c1b26a711a58}");
}
unsafe impl ::windows_core::Interface for ISignificantDigitsOption {
    type Vtable = ISignificantDigitsOption_Vtbl;
}
impl ::core::clone::Clone for ISignificantDigitsOption {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISignificantDigitsOption {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1d4dfcdd_2d43_4ee8_bbf1_c1b26a711a58);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISignificantDigitsOption_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SignificantDigits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetSignificantDigits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Globalization_NumberFormatting\"`*"]
#[repr(transparent)]
pub struct CurrencyFormatter(::windows_core::IUnknown);
impl CurrencyFormatter {
    pub fn Currency(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Currency)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetCurrency(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCurrency)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Mode(&self) -> ::windows_core::Result<CurrencyFormatterMode> {
        let this = &::windows_core::ComInterface::cast::<ICurrencyFormatter2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Mode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMode(&self, value: CurrencyFormatterMode) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICurrencyFormatter2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ApplyRoundingForCurrency(&self, roundingalgorithm: RoundingAlgorithm) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICurrencyFormatter2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ApplyRoundingForCurrency)(::windows_core::Interface::as_raw(this), roundingalgorithm).ok() }
    }
    pub fn CreateCurrencyFormatterCode(currencycode: &::windows_core::HSTRING) -> ::windows_core::Result<CurrencyFormatter> {
        Self::ICurrencyFormatterFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateCurrencyFormatterCode)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(currencycode), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateCurrencyFormatterCodeContext<P0>(currencycode: &::windows_core::HSTRING, languages: P0, geographicregion: &::windows_core::HSTRING) -> ::windows_core::Result<CurrencyFormatter>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        Self::ICurrencyFormatterFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateCurrencyFormatterCodeContext)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(currencycode), languages.try_into_param()?.abi(), ::core::mem::transmute_copy(geographicregion), &mut result__).from_abi(result__)
        })
    }
    pub fn FormatInt(&self, value: i64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatter>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FormatInt)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    pub fn FormatUInt(&self, value: u64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatter>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FormatUInt)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    pub fn FormatDouble(&self, value: f64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatter>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FormatDouble)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    pub fn FormatInt2(&self, value: i64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FormatInt)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    pub fn FormatUInt2(&self, value: u64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FormatUInt)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    pub fn FormatDouble2(&self, value: f64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FormatDouble)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Languages(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Languages)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GeographicRegion(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GeographicRegion)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IntegerDigits(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IntegerDigits)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIntegerDigits(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIntegerDigits)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FractionDigits(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FractionDigits)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetFractionDigits(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetFractionDigits)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsGrouped(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsGrouped)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsGrouped(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsGrouped)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDecimalPointAlwaysDisplayed(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsDecimalPointAlwaysDisplayed)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsDecimalPointAlwaysDisplayed(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsDecimalPointAlwaysDisplayed)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn NumeralSystem(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NumeralSystem)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNumeralSystem(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNumeralSystem)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ResolvedLanguage(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResolvedLanguage)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ResolvedGeographicRegion(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResolvedGeographicRegion)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ParseInt(&self, text: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IReference<i64>> {
        let this = &::windows_core::ComInterface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ParseInt)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(text), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ParseUInt(&self, text: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IReference<u64>> {
        let this = &::windows_core::ComInterface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ParseUInt)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(text), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ParseDouble(&self, text: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = &::windows_core::ComInterface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ParseDouble)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(text), &mut result__).from_abi(result__)
        }
    }
    pub fn NumberRounder(&self) -> ::windows_core::Result<INumberRounder> {
        let this = &::windows_core::ComInterface::cast::<INumberRounderOption>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NumberRounder)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNumberRounder<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<INumberRounder>,
    {
        let this = &::windows_core::ComInterface::cast::<INumberRounderOption>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNumberRounder)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn IsZeroSigned(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ISignedZeroOption>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsZeroSigned)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsZeroSigned(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISignedZeroOption>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsZeroSigned)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SignificantDigits(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<ISignificantDigitsOption>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SignificantDigits)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSignificantDigits(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISignificantDigitsOption>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSignificantDigits)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc(hidden)]
    pub fn ICurrencyFormatterFactory<R, F: FnOnce(&ICurrencyFormatterFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<CurrencyFormatter, ICurrencyFormatterFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for CurrencyFormatter {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Globalization.NumberFormatting.CurrencyFormatter;{11730ca5-4b00-41b2-b332-73b12a497d54})");
}
impl ::core::clone::Clone for CurrencyFormatter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for CurrencyFormatter {
    type Vtable = ICurrencyFormatter_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CurrencyFormatter {
    const IID: ::windows_core::GUID = <ICurrencyFormatter as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CurrencyFormatter {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.CurrencyFormatter";
}
::windows_core::imp::interface_hierarchy!(CurrencyFormatter, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<INumberFormatter> for CurrencyFormatter {}
impl ::windows_core::CanTryInto<INumberFormatter2> for CurrencyFormatter {}
impl ::windows_core::CanTryInto<INumberFormatterOptions> for CurrencyFormatter {}
impl ::windows_core::CanTryInto<INumberParser> for CurrencyFormatter {}
impl ::windows_core::CanTryInto<INumberRounderOption> for CurrencyFormatter {}
impl ::windows_core::CanTryInto<ISignedZeroOption> for CurrencyFormatter {}
impl ::windows_core::CanTryInto<ISignificantDigitsOption> for CurrencyFormatter {}
unsafe impl ::core::marker::Send for CurrencyFormatter {}
unsafe impl ::core::marker::Sync for CurrencyFormatter {}
#[doc = "*Required features: `\"Globalization_NumberFormatting\"`*"]
#[repr(transparent)]
pub struct DecimalFormatter(::windows_core::IUnknown);
impl DecimalFormatter {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<DecimalFormatter, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateDecimalFormatter<P0>(languages: P0, geographicregion: &::windows_core::HSTRING) -> ::windows_core::Result<DecimalFormatter>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        Self::IDecimalFormatterFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateDecimalFormatter)(::windows_core::Interface::as_raw(this), languages.try_into_param()?.abi(), ::core::mem::transmute_copy(geographicregion), &mut result__).from_abi(result__)
        })
    }
    pub fn FormatInt(&self, value: i64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FormatInt)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    pub fn FormatUInt(&self, value: u64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FormatUInt)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    pub fn FormatDouble(&self, value: f64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FormatDouble)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    pub fn FormatInt2(&self, value: i64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FormatInt)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    pub fn FormatUInt2(&self, value: u64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FormatUInt)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    pub fn FormatDouble2(&self, value: f64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FormatDouble)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Languages(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Languages)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GeographicRegion(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GeographicRegion)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IntegerDigits(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IntegerDigits)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIntegerDigits(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIntegerDigits)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FractionDigits(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FractionDigits)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetFractionDigits(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetFractionDigits)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsGrouped(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsGrouped)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsGrouped(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsGrouped)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDecimalPointAlwaysDisplayed(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsDecimalPointAlwaysDisplayed)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsDecimalPointAlwaysDisplayed(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsDecimalPointAlwaysDisplayed)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn NumeralSystem(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NumeralSystem)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNumeralSystem(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNumeralSystem)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ResolvedLanguage(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResolvedLanguage)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ResolvedGeographicRegion(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResolvedGeographicRegion)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ParseInt(&self, text: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IReference<i64>> {
        let this = &::windows_core::ComInterface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ParseInt)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(text), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ParseUInt(&self, text: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IReference<u64>> {
        let this = &::windows_core::ComInterface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ParseUInt)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(text), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ParseDouble(&self, text: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = &::windows_core::ComInterface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ParseDouble)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(text), &mut result__).from_abi(result__)
        }
    }
    pub fn NumberRounder(&self) -> ::windows_core::Result<INumberRounder> {
        let this = &::windows_core::ComInterface::cast::<INumberRounderOption>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NumberRounder)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNumberRounder<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<INumberRounder>,
    {
        let this = &::windows_core::ComInterface::cast::<INumberRounderOption>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNumberRounder)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn IsZeroSigned(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ISignedZeroOption>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsZeroSigned)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsZeroSigned(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISignedZeroOption>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsZeroSigned)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SignificantDigits(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<ISignificantDigitsOption>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SignificantDigits)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSignificantDigits(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISignificantDigitsOption>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSignificantDigits)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc(hidden)]
    pub fn IDecimalFormatterFactory<R, F: FnOnce(&IDecimalFormatterFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<DecimalFormatter, IDecimalFormatterFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for DecimalFormatter {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Globalization.NumberFormatting.DecimalFormatter;{a5007c49-7676-4db7-8631-1b6ff265caa9})");
}
impl ::core::clone::Clone for DecimalFormatter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for DecimalFormatter {
    type Vtable = INumberFormatter_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DecimalFormatter {
    const IID: ::windows_core::GUID = <INumberFormatter as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DecimalFormatter {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.DecimalFormatter";
}
::windows_core::imp::interface_hierarchy!(DecimalFormatter, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<INumberFormatter> for DecimalFormatter {}
impl ::windows_core::CanTryInto<INumberFormatter2> for DecimalFormatter {}
impl ::windows_core::CanTryInto<INumberFormatterOptions> for DecimalFormatter {}
impl ::windows_core::CanTryInto<INumberParser> for DecimalFormatter {}
impl ::windows_core::CanTryInto<INumberRounderOption> for DecimalFormatter {}
impl ::windows_core::CanTryInto<ISignedZeroOption> for DecimalFormatter {}
impl ::windows_core::CanTryInto<ISignificantDigitsOption> for DecimalFormatter {}
unsafe impl ::core::marker::Send for DecimalFormatter {}
unsafe impl ::core::marker::Sync for DecimalFormatter {}
#[doc = "*Required features: `\"Globalization_NumberFormatting\"`*"]
#[repr(transparent)]
pub struct IncrementNumberRounder(::windows_core::IUnknown);
impl IncrementNumberRounder {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<IncrementNumberRounder, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn RoundingAlgorithm(&self) -> ::windows_core::Result<RoundingAlgorithm> {
        let this = &::windows_core::ComInterface::cast::<IIncrementNumberRounder>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RoundingAlgorithm)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRoundingAlgorithm(&self, value: RoundingAlgorithm) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IIncrementNumberRounder>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRoundingAlgorithm)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Increment(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<IIncrementNumberRounder>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Increment)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIncrement(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IIncrementNumberRounder>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIncrement)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RoundInt32(&self, value: i32) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RoundInt32)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    pub fn RoundUInt32(&self, value: u32) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RoundUInt32)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    pub fn RoundInt64(&self, value: i64) -> ::windows_core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RoundInt64)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    pub fn RoundUInt64(&self, value: u64) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RoundUInt64)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    pub fn RoundSingle(&self, value: f32) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RoundSingle)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    pub fn RoundDouble(&self, value: f64) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RoundDouble)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for IncrementNumberRounder {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Globalization.NumberFormatting.IncrementNumberRounder;{5473c375-38ed-4631-b80c-ef34fc48b7f5})");
}
impl ::core::clone::Clone for IncrementNumberRounder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for IncrementNumberRounder {
    type Vtable = INumberRounder_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IncrementNumberRounder {
    const IID: ::windows_core::GUID = <INumberRounder as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for IncrementNumberRounder {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.IncrementNumberRounder";
}
::windows_core::imp::interface_hierarchy!(IncrementNumberRounder, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<INumberRounder> for IncrementNumberRounder {}
unsafe impl ::core::marker::Send for IncrementNumberRounder {}
unsafe impl ::core::marker::Sync for IncrementNumberRounder {}
#[doc = "*Required features: `\"Globalization_NumberFormatting\"`*"]
#[repr(transparent)]
pub struct NumeralSystemTranslator(::windows_core::IUnknown);
impl NumeralSystemTranslator {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<NumeralSystemTranslator, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Languages(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Languages)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ResolvedLanguage(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResolvedLanguage)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NumeralSystem(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NumeralSystem)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNumeralSystem(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetNumeralSystem)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn TranslateNumerals(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TranslateNumerals)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create<P0>(languages: P0) -> ::windows_core::Result<NumeralSystemTranslator>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        Self::INumeralSystemTranslatorFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), languages.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn INumeralSystemTranslatorFactory<R, F: FnOnce(&INumeralSystemTranslatorFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<NumeralSystemTranslator, INumeralSystemTranslatorFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for NumeralSystemTranslator {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Globalization.NumberFormatting.NumeralSystemTranslator;{28f5bc2c-8c23-4234-ad2e-fa5a3a426e9b})");
}
impl ::core::clone::Clone for NumeralSystemTranslator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for NumeralSystemTranslator {
    type Vtable = INumeralSystemTranslator_Vtbl;
}
unsafe impl ::windows_core::ComInterface for NumeralSystemTranslator {
    const IID: ::windows_core::GUID = <INumeralSystemTranslator as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for NumeralSystemTranslator {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.NumeralSystemTranslator";
}
::windows_core::imp::interface_hierarchy!(NumeralSystemTranslator, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for NumeralSystemTranslator {}
unsafe impl ::core::marker::Sync for NumeralSystemTranslator {}
#[doc = "*Required features: `\"Globalization_NumberFormatting\"`*"]
#[repr(transparent)]
pub struct PercentFormatter(::windows_core::IUnknown);
impl PercentFormatter {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PercentFormatter, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn FormatInt(&self, value: i64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FormatInt)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    pub fn FormatUInt(&self, value: u64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FormatUInt)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    pub fn FormatDouble(&self, value: f64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FormatDouble)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    pub fn FormatInt2(&self, value: i64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FormatInt)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    pub fn FormatUInt2(&self, value: u64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FormatUInt)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    pub fn FormatDouble2(&self, value: f64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FormatDouble)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Languages(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Languages)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GeographicRegion(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GeographicRegion)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IntegerDigits(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IntegerDigits)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIntegerDigits(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIntegerDigits)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FractionDigits(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FractionDigits)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetFractionDigits(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetFractionDigits)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsGrouped(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsGrouped)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsGrouped(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsGrouped)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDecimalPointAlwaysDisplayed(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsDecimalPointAlwaysDisplayed)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsDecimalPointAlwaysDisplayed(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsDecimalPointAlwaysDisplayed)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn NumeralSystem(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NumeralSystem)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNumeralSystem(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNumeralSystem)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ResolvedLanguage(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResolvedLanguage)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ResolvedGeographicRegion(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResolvedGeographicRegion)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ParseInt(&self, text: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IReference<i64>> {
        let this = &::windows_core::ComInterface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ParseInt)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(text), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ParseUInt(&self, text: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IReference<u64>> {
        let this = &::windows_core::ComInterface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ParseUInt)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(text), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ParseDouble(&self, text: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = &::windows_core::ComInterface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ParseDouble)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(text), &mut result__).from_abi(result__)
        }
    }
    pub fn NumberRounder(&self) -> ::windows_core::Result<INumberRounder> {
        let this = &::windows_core::ComInterface::cast::<INumberRounderOption>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NumberRounder)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNumberRounder<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<INumberRounder>,
    {
        let this = &::windows_core::ComInterface::cast::<INumberRounderOption>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNumberRounder)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreatePercentFormatter<P0>(languages: P0, geographicregion: &::windows_core::HSTRING) -> ::windows_core::Result<PercentFormatter>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        Self::IPercentFormatterFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreatePercentFormatter)(::windows_core::Interface::as_raw(this), languages.try_into_param()?.abi(), ::core::mem::transmute_copy(geographicregion), &mut result__).from_abi(result__)
        })
    }
    pub fn IsZeroSigned(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ISignedZeroOption>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsZeroSigned)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsZeroSigned(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISignedZeroOption>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsZeroSigned)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SignificantDigits(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<ISignificantDigitsOption>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SignificantDigits)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSignificantDigits(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISignificantDigitsOption>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSignificantDigits)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc(hidden)]
    pub fn IPercentFormatterFactory<R, F: FnOnce(&IPercentFormatterFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PercentFormatter, IPercentFormatterFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for PercentFormatter {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Globalization.NumberFormatting.PercentFormatter;{a5007c49-7676-4db7-8631-1b6ff265caa9})");
}
impl ::core::clone::Clone for PercentFormatter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PercentFormatter {
    type Vtable = INumberFormatter_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PercentFormatter {
    const IID: ::windows_core::GUID = <INumberFormatter as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PercentFormatter {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.PercentFormatter";
}
::windows_core::imp::interface_hierarchy!(PercentFormatter, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<INumberFormatter> for PercentFormatter {}
impl ::windows_core::CanTryInto<INumberFormatter2> for PercentFormatter {}
impl ::windows_core::CanTryInto<INumberFormatterOptions> for PercentFormatter {}
impl ::windows_core::CanTryInto<INumberParser> for PercentFormatter {}
impl ::windows_core::CanTryInto<INumberRounderOption> for PercentFormatter {}
impl ::windows_core::CanTryInto<ISignedZeroOption> for PercentFormatter {}
impl ::windows_core::CanTryInto<ISignificantDigitsOption> for PercentFormatter {}
unsafe impl ::core::marker::Send for PercentFormatter {}
unsafe impl ::core::marker::Sync for PercentFormatter {}
#[doc = "*Required features: `\"Globalization_NumberFormatting\"`*"]
#[repr(transparent)]
pub struct PermilleFormatter(::windows_core::IUnknown);
impl PermilleFormatter {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PermilleFormatter, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn FormatInt(&self, value: i64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FormatInt)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    pub fn FormatUInt(&self, value: u64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FormatUInt)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    pub fn FormatDouble(&self, value: f64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FormatDouble)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    pub fn FormatInt2(&self, value: i64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FormatInt)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    pub fn FormatUInt2(&self, value: u64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FormatUInt)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    pub fn FormatDouble2(&self, value: f64) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatter2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FormatDouble)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Languages(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Languages)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GeographicRegion(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GeographicRegion)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IntegerDigits(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IntegerDigits)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIntegerDigits(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIntegerDigits)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FractionDigits(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FractionDigits)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetFractionDigits(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetFractionDigits)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsGrouped(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsGrouped)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsGrouped(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsGrouped)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDecimalPointAlwaysDisplayed(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsDecimalPointAlwaysDisplayed)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsDecimalPointAlwaysDisplayed(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsDecimalPointAlwaysDisplayed)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn NumeralSystem(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NumeralSystem)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNumeralSystem(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatterOptions>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNumeralSystem)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ResolvedLanguage(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResolvedLanguage)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ResolvedGeographicRegion(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<INumberFormatterOptions>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResolvedGeographicRegion)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ParseInt(&self, text: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IReference<i64>> {
        let this = &::windows_core::ComInterface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ParseInt)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(text), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ParseUInt(&self, text: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IReference<u64>> {
        let this = &::windows_core::ComInterface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ParseUInt)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(text), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ParseDouble(&self, text: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = &::windows_core::ComInterface::cast::<INumberParser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ParseDouble)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(text), &mut result__).from_abi(result__)
        }
    }
    pub fn NumberRounder(&self) -> ::windows_core::Result<INumberRounder> {
        let this = &::windows_core::ComInterface::cast::<INumberRounderOption>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NumberRounder)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNumberRounder<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<INumberRounder>,
    {
        let this = &::windows_core::ComInterface::cast::<INumberRounderOption>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNumberRounder)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreatePermilleFormatter<P0>(languages: P0, geographicregion: &::windows_core::HSTRING) -> ::windows_core::Result<PermilleFormatter>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        Self::IPermilleFormatterFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreatePermilleFormatter)(::windows_core::Interface::as_raw(this), languages.try_into_param()?.abi(), ::core::mem::transmute_copy(geographicregion), &mut result__).from_abi(result__)
        })
    }
    pub fn IsZeroSigned(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ISignedZeroOption>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsZeroSigned)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsZeroSigned(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISignedZeroOption>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsZeroSigned)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SignificantDigits(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<ISignificantDigitsOption>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SignificantDigits)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSignificantDigits(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISignificantDigitsOption>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSignificantDigits)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc(hidden)]
    pub fn IPermilleFormatterFactory<R, F: FnOnce(&IPermilleFormatterFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PermilleFormatter, IPermilleFormatterFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for PermilleFormatter {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Globalization.NumberFormatting.PermilleFormatter;{a5007c49-7676-4db7-8631-1b6ff265caa9})");
}
impl ::core::clone::Clone for PermilleFormatter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PermilleFormatter {
    type Vtable = INumberFormatter_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PermilleFormatter {
    const IID: ::windows_core::GUID = <INumberFormatter as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PermilleFormatter {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.PermilleFormatter";
}
::windows_core::imp::interface_hierarchy!(PermilleFormatter, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<INumberFormatter> for PermilleFormatter {}
impl ::windows_core::CanTryInto<INumberFormatter2> for PermilleFormatter {}
impl ::windows_core::CanTryInto<INumberFormatterOptions> for PermilleFormatter {}
impl ::windows_core::CanTryInto<INumberParser> for PermilleFormatter {}
impl ::windows_core::CanTryInto<INumberRounderOption> for PermilleFormatter {}
impl ::windows_core::CanTryInto<ISignedZeroOption> for PermilleFormatter {}
impl ::windows_core::CanTryInto<ISignificantDigitsOption> for PermilleFormatter {}
unsafe impl ::core::marker::Send for PermilleFormatter {}
unsafe impl ::core::marker::Sync for PermilleFormatter {}
#[doc = "*Required features: `\"Globalization_NumberFormatting\"`*"]
#[repr(transparent)]
pub struct SignificantDigitsNumberRounder(::windows_core::IUnknown);
impl SignificantDigitsNumberRounder {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SignificantDigitsNumberRounder, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn RoundInt32(&self, value: i32) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RoundInt32)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    pub fn RoundUInt32(&self, value: u32) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RoundUInt32)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    pub fn RoundInt64(&self, value: i64) -> ::windows_core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RoundInt64)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    pub fn RoundUInt64(&self, value: u64) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RoundUInt64)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    pub fn RoundSingle(&self, value: f32) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RoundSingle)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    pub fn RoundDouble(&self, value: f64) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RoundDouble)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    pub fn RoundingAlgorithm(&self) -> ::windows_core::Result<RoundingAlgorithm> {
        let this = &::windows_core::ComInterface::cast::<ISignificantDigitsNumberRounder>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RoundingAlgorithm)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRoundingAlgorithm(&self, value: RoundingAlgorithm) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISignificantDigitsNumberRounder>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRoundingAlgorithm)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SignificantDigits(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<ISignificantDigitsNumberRounder>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SignificantDigits)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSignificantDigits(&self, value: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISignificantDigitsNumberRounder>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSignificantDigits)(::windows_core::Interface::as_raw(this), value).ok() }
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
impl ::windows_core::RuntimeType for SignificantDigitsNumberRounder {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Globalization.NumberFormatting.SignificantDigitsNumberRounder;{5473c375-38ed-4631-b80c-ef34fc48b7f5})");
}
impl ::core::clone::Clone for SignificantDigitsNumberRounder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SignificantDigitsNumberRounder {
    type Vtable = INumberRounder_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SignificantDigitsNumberRounder {
    const IID: ::windows_core::GUID = <INumberRounder as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SignificantDigitsNumberRounder {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.SignificantDigitsNumberRounder";
}
::windows_core::imp::interface_hierarchy!(SignificantDigitsNumberRounder, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<INumberRounder> for SignificantDigitsNumberRounder {}
unsafe impl ::core::marker::Send for SignificantDigitsNumberRounder {}
unsafe impl ::core::marker::Sync for SignificantDigitsNumberRounder {}
#[doc = "*Required features: `\"Globalization_NumberFormatting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for CurrencyFormatterMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CurrencyFormatterMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CurrencyFormatterMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CurrencyFormatterMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CurrencyFormatterMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Globalization.NumberFormatting.CurrencyFormatterMode;i4)");
}
#[doc = "*Required features: `\"Globalization_NumberFormatting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for RoundingAlgorithm {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for RoundingAlgorithm {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for RoundingAlgorithm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RoundingAlgorithm").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for RoundingAlgorithm {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Globalization.NumberFormatting.RoundingAlgorithm;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
