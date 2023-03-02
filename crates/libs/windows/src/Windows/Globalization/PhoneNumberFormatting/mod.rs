#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneNumberFormatter(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneNumberFormatter {
    type Vtable = IPhoneNumberFormatter_Vtbl;
}
impl ::core::clone::Clone for IPhoneNumberFormatter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPhoneNumberFormatter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1556b49e_bad4_4b4a_900d_4407adb7c981);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneNumberFormatter_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Format: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, number: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub FormatWithOutputFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, number: *mut ::core::ffi::c_void, numberformat: PhoneNumberFormat, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub FormatPartialString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, number: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub FormatString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, number: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub FormatStringWithLeftToRightMarkers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, number: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneNumberFormatterStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneNumberFormatterStatics {
    type Vtable = IPhoneNumberFormatterStatics_Vtbl;
}
impl ::core::clone::Clone for IPhoneNumberFormatterStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPhoneNumberFormatterStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5ca6f931_84d9_414b_ab4e_a0552c878602);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneNumberFormatterStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TryCreate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, regioncode: ::std::mem::MaybeUninit<::windows::core::HSTRING>, phonenumber: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCountryCodeForRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, regioncode: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut i32) -> ::windows::core::HRESULT,
    pub GetNationalDirectDialingPrefixForRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, regioncode: ::std::mem::MaybeUninit<::windows::core::HSTRING>, stripnondigit: bool, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub WrapWithLeftToRightMarkers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, number: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneNumberInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneNumberInfo {
    type Vtable = IPhoneNumberInfo_Vtbl;
}
impl ::core::clone::Clone for IPhoneNumberInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPhoneNumberInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1c7ce4dd_c8b4_4ea3_9aef_b342e2c5b417);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneNumberInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CountryCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub PhoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetLengthOfGeographicalAreaCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub GetNationalSignificantNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetLengthOfNationalDestinationCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub PredictNumberKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PredictedPhoneNumberKind) -> ::windows::core::HRESULT,
    pub GetGeographicRegionCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub CheckNumberMatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, othernumber: *mut ::core::ffi::c_void, result__: *mut PhoneNumberMatchResult) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneNumberInfoFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneNumberInfoFactory {
    type Vtable = IPhoneNumberInfoFactory_Vtbl;
}
impl ::core::clone::Clone for IPhoneNumberInfoFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPhoneNumberInfoFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8202b964_adaa_4cff_8fcf_17e7516a28ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneNumberInfoFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, number: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneNumberInfoStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneNumberInfoStatics {
    type Vtable = IPhoneNumberInfoStatics_Vtbl;
}
impl ::core::clone::Clone for IPhoneNumberInfoStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPhoneNumberInfoStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b3f4f6a_86a9_40e9_8649_6d61161928d4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneNumberInfoStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TryParse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::std::mem::MaybeUninit<::windows::core::HSTRING>, phonenumber: *mut *mut ::core::ffi::c_void, result__: *mut PhoneNumberParseResult) -> ::windows::core::HRESULT,
    pub TryParseWithRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::std::mem::MaybeUninit<::windows::core::HSTRING>, regioncode: ::std::mem::MaybeUninit<::windows::core::HSTRING>, phonenumber: *mut *mut ::core::ffi::c_void, result__: *mut PhoneNumberParseResult) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Globalization_PhoneNumberFormatting\"`*"]
#[repr(transparent)]
pub struct PhoneNumberFormatter(::windows::core::IUnknown);
impl PhoneNumberFormatter {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<PhoneNumberFormatter, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Format(&self, number: &PhoneNumberInfo) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Format)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(number), &mut result__).from_abi(result__)
        }
    }
    pub fn FormatWithOutputFormat(&self, number: &PhoneNumberInfo, numberformat: PhoneNumberFormat) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).FormatWithOutputFormat)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(number), numberformat, &mut result__).from_abi(result__)
        }
    }
    pub fn FormatPartialString(&self, number: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).FormatPartialString)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(number), &mut result__).from_abi(result__)
        }
    }
    pub fn FormatString(&self, number: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).FormatString)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(number), &mut result__).from_abi(result__)
        }
    }
    pub fn FormatStringWithLeftToRightMarkers(&self, number: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).FormatStringWithLeftToRightMarkers)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(number), &mut result__).from_abi(result__)
        }
    }
    pub fn TryCreate(regioncode: &::windows::core::HSTRING, phonenumber: &mut ::core::option::Option<PhoneNumberFormatter>) -> ::windows::core::Result<()> {
        Self::IPhoneNumberFormatterStatics(|this| unsafe { (::windows::core::Interface::vtable(this).TryCreate)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(regioncode), phonenumber as *mut _ as _).ok() })
    }
    pub fn GetCountryCodeForRegion(regioncode: &::windows::core::HSTRING) -> ::windows::core::Result<i32> {
        Self::IPhoneNumberFormatterStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).GetCountryCodeForRegion)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(regioncode), &mut result__).from_abi(result__)
        })
    }
    pub fn GetNationalDirectDialingPrefixForRegion(regioncode: &::windows::core::HSTRING, stripnondigit: bool) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IPhoneNumberFormatterStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetNationalDirectDialingPrefixForRegion)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(regioncode), stripnondigit, &mut result__).from_abi(result__)
        })
    }
    pub fn WrapWithLeftToRightMarkers(number: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IPhoneNumberFormatterStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).WrapWithLeftToRightMarkers)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(number), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPhoneNumberFormatterStatics<R, F: FnOnce(&IPhoneNumberFormatterStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<PhoneNumberFormatter, IPhoneNumberFormatterStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for PhoneNumberFormatter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneNumberFormatter {}
impl ::core::fmt::Debug for PhoneNumberFormatter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneNumberFormatter").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PhoneNumberFormatter {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Globalization.PhoneNumberFormatting.PhoneNumberFormatter;{1556b49e-bad4-4b4a-900d-4407adb7c981})");
}
impl ::core::clone::Clone for PhoneNumberFormatter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PhoneNumberFormatter {
    type Vtable = IPhoneNumberFormatter_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PhoneNumberFormatter {
    const IID: ::windows::core::GUID = <IPhoneNumberFormatter as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PhoneNumberFormatter {
    const NAME: &'static str = "Windows.Globalization.PhoneNumberFormatting.PhoneNumberFormatter";
}
::windows::imp::interface_hierarchy!(PhoneNumberFormatter, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PhoneNumberFormatter {}
unsafe impl ::core::marker::Sync for PhoneNumberFormatter {}
#[doc = "*Required features: `\"Globalization_PhoneNumberFormatting\"`*"]
#[repr(transparent)]
pub struct PhoneNumberInfo(::windows::core::IUnknown);
impl PhoneNumberInfo {
    pub fn CountryCode(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).CountryCode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PhoneNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).PhoneNumber)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetLengthOfGeographicalAreaCode(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).GetLengthOfGeographicalAreaCode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetNationalSignificantNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetNationalSignificantNumber)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetLengthOfNationalDestinationCode(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).GetLengthOfNationalDestinationCode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PredictNumberKind(&self) -> ::windows::core::Result<PredictedPhoneNumberKind> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PredictedPhoneNumberKind>();
            (::windows::core::Interface::vtable(this).PredictNumberKind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetGeographicRegionCode(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetGeographicRegionCode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CheckNumberMatch(&self, othernumber: &PhoneNumberInfo) -> ::windows::core::Result<PhoneNumberMatchResult> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PhoneNumberMatchResult>();
            (::windows::core::Interface::vtable(this).CheckNumberMatch)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(othernumber), &mut result__).from_abi(result__)
        }
    }
    pub fn Create(number: &::windows::core::HSTRING) -> ::windows::core::Result<PhoneNumberInfo> {
        Self::IPhoneNumberInfoFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<PhoneNumberInfo>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(number), &mut result__).from_abi(result__)
        })
    }
    pub fn TryParse(input: &::windows::core::HSTRING, phonenumber: &mut ::core::option::Option<PhoneNumberInfo>) -> ::windows::core::Result<PhoneNumberParseResult> {
        Self::IPhoneNumberInfoStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<PhoneNumberParseResult>();
            (::windows::core::Interface::vtable(this).TryParse)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(input), phonenumber as *mut _ as _, &mut result__).from_abi(result__)
        })
    }
    pub fn TryParseWithRegion(input: &::windows::core::HSTRING, regioncode: &::windows::core::HSTRING, phonenumber: &mut ::core::option::Option<PhoneNumberInfo>) -> ::windows::core::Result<PhoneNumberParseResult> {
        Self::IPhoneNumberInfoStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<PhoneNumberParseResult>();
            (::windows::core::Interface::vtable(this).TryParseWithRegion)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(input), ::core::mem::transmute_copy(regioncode), phonenumber as *mut _ as _, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ToString)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IPhoneNumberInfoFactory<R, F: FnOnce(&IPhoneNumberInfoFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<PhoneNumberInfo, IPhoneNumberInfoFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IPhoneNumberInfoStatics<R, F: FnOnce(&IPhoneNumberInfoStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<PhoneNumberInfo, IPhoneNumberInfoStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for PhoneNumberInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneNumberInfo {}
impl ::core::fmt::Debug for PhoneNumberInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneNumberInfo").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PhoneNumberInfo {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Globalization.PhoneNumberFormatting.PhoneNumberInfo;{1c7ce4dd-c8b4-4ea3-9aef-b342e2c5b417})");
}
impl ::core::clone::Clone for PhoneNumberInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PhoneNumberInfo {
    type Vtable = IPhoneNumberInfo_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PhoneNumberInfo {
    const IID: ::windows::core::GUID = <IPhoneNumberInfo as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PhoneNumberInfo {
    const NAME: &'static str = "Windows.Globalization.PhoneNumberFormatting.PhoneNumberInfo";
}
::windows::imp::interface_hierarchy!(PhoneNumberInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IStringable> for PhoneNumberInfo {}
unsafe impl ::core::marker::Send for PhoneNumberInfo {}
unsafe impl ::core::marker::Sync for PhoneNumberInfo {}
#[doc = "*Required features: `\"Globalization_PhoneNumberFormatting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PhoneNumberFormat(pub i32);
impl PhoneNumberFormat {
    pub const E164: Self = Self(0i32);
    pub const International: Self = Self(1i32);
    pub const National: Self = Self(2i32);
    pub const Rfc3966: Self = Self(3i32);
}
impl ::core::marker::Copy for PhoneNumberFormat {}
impl ::core::clone::Clone for PhoneNumberFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneNumberFormat {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PhoneNumberFormat {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PhoneNumberFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneNumberFormat").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PhoneNumberFormat {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Globalization.PhoneNumberFormatting.PhoneNumberFormat;i4)");
}
#[doc = "*Required features: `\"Globalization_PhoneNumberFormatting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PhoneNumberMatchResult(pub i32);
impl PhoneNumberMatchResult {
    pub const NoMatch: Self = Self(0i32);
    pub const ShortNationalSignificantNumberMatch: Self = Self(1i32);
    pub const NationalSignificantNumberMatch: Self = Self(2i32);
    pub const ExactMatch: Self = Self(3i32);
}
impl ::core::marker::Copy for PhoneNumberMatchResult {}
impl ::core::clone::Clone for PhoneNumberMatchResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneNumberMatchResult {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PhoneNumberMatchResult {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PhoneNumberMatchResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneNumberMatchResult").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PhoneNumberMatchResult {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Globalization.PhoneNumberFormatting.PhoneNumberMatchResult;i4)");
}
#[doc = "*Required features: `\"Globalization_PhoneNumberFormatting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PhoneNumberParseResult(pub i32);
impl PhoneNumberParseResult {
    pub const Valid: Self = Self(0i32);
    pub const NotANumber: Self = Self(1i32);
    pub const InvalidCountryCode: Self = Self(2i32);
    pub const TooShort: Self = Self(3i32);
    pub const TooLong: Self = Self(4i32);
}
impl ::core::marker::Copy for PhoneNumberParseResult {}
impl ::core::clone::Clone for PhoneNumberParseResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneNumberParseResult {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PhoneNumberParseResult {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PhoneNumberParseResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneNumberParseResult").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PhoneNumberParseResult {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Globalization.PhoneNumberFormatting.PhoneNumberParseResult;i4)");
}
#[doc = "*Required features: `\"Globalization_PhoneNumberFormatting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PredictedPhoneNumberKind(pub i32);
impl PredictedPhoneNumberKind {
    pub const FixedLine: Self = Self(0i32);
    pub const Mobile: Self = Self(1i32);
    pub const FixedLineOrMobile: Self = Self(2i32);
    pub const TollFree: Self = Self(3i32);
    pub const PremiumRate: Self = Self(4i32);
    pub const SharedCost: Self = Self(5i32);
    pub const Voip: Self = Self(6i32);
    pub const PersonalNumber: Self = Self(7i32);
    pub const Pager: Self = Self(8i32);
    pub const UniversalAccountNumber: Self = Self(9i32);
    pub const Voicemail: Self = Self(10i32);
    pub const Unknown: Self = Self(11i32);
}
impl ::core::marker::Copy for PredictedPhoneNumberKind {}
impl ::core::clone::Clone for PredictedPhoneNumberKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PredictedPhoneNumberKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PredictedPhoneNumberKind {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PredictedPhoneNumberKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PredictedPhoneNumberKind").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PredictedPhoneNumberKind {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Globalization.PhoneNumberFormatting.PredictedPhoneNumberKind;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
