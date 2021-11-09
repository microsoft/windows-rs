#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneNumberFormatter(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPhoneNumberFormatter {
    type Vtable = IPhoneNumberFormatter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(358003870, 47828, 19274, [144, 13, 68, 7, 173, 183, 201, 129]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneNumberFormatter_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, number: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, number: ::windows::runtime::RawPtr, numberformat: PhoneNumberFormat, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, number: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, number: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, number: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneNumberFormatterStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPhoneNumberFormatterStatics {
    type Vtable = IPhoneNumberFormatterStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1554446641, 34009, 16715, [171, 78, 160, 85, 44, 135, 134, 2]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneNumberFormatterStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, regioncode: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, phonenumber: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, regioncode: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, regioncode: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, stripnondigit: bool, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, number: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneNumberInfo(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPhoneNumberInfo {
    type Vtable = IPhoneNumberInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(477947101, 51380, 20131, [154, 239, 179, 66, 226, 197, 180, 23]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneNumberInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PredictedPhoneNumberKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, othernumber: ::windows::runtime::RawPtr, result__: *mut PhoneNumberMatchResult) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneNumberInfoFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPhoneNumberInfoFactory {
    type Vtable = IPhoneNumberInfoFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2181216612, 44458, 19711, [143, 207, 23, 231, 81, 106, 40, 255]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneNumberInfoFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, number: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhoneNumberInfoStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPhoneNumberInfoStatics {
    type Vtable = IPhoneNumberInfoStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1530875754, 34473, 16617, [134, 73, 109, 97, 22, 25, 40, 212]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneNumberInfoStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, phonenumber: *mut ::windows::runtime::RawPtr, result__: *mut PhoneNumberParseResult) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, regioncode: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, phonenumber: *mut ::windows::runtime::RawPtr, result__: *mut PhoneNumberParseResult) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Globalization_PhoneNumberFormatting`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhoneNumberFormat(pub i32);
impl PhoneNumberFormat {
    pub const E164: PhoneNumberFormat = PhoneNumberFormat(0i32);
    pub const International: PhoneNumberFormat = PhoneNumberFormat(1i32);
    pub const National: PhoneNumberFormat = PhoneNumberFormat(2i32);
    pub const Rfc3966: PhoneNumberFormat = PhoneNumberFormat(3i32);
}
impl ::core::convert::From<i32> for PhoneNumberFormat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PhoneNumberFormat {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PhoneNumberFormat {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Globalization.PhoneNumberFormatting.PhoneNumberFormat;i4)");
}
impl ::windows::runtime::DefaultType for PhoneNumberFormat {
    type DefaultType = Self;
}
#[doc = "*Required features: `Globalization_PhoneNumberFormatting`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PhoneNumberFormatter(pub ::windows::runtime::IInspectable);
impl PhoneNumberFormatter {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PhoneNumberFormatter, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Globalization_PhoneNumberFormatting`*"]
    pub fn Format<'a, Param0: ::windows::runtime::IntoParam<'a, PhoneNumberInfo>>(&self, number: Param0) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), number.into_param().abi(), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_PhoneNumberFormatting`*"]
    pub fn FormatWithOutputFormat<'a, Param0: ::windows::runtime::IntoParam<'a, PhoneNumberInfo>>(&self, number: Param0, numberformat: PhoneNumberFormat) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), number.into_param().abi(), numberformat, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_PhoneNumberFormatting`*"]
    pub fn FormatPartialString<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, number: Param0) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), number.into_param().abi(), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_PhoneNumberFormatting`*"]
    pub fn FormatString<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, number: Param0) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), number.into_param().abi(), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_PhoneNumberFormatting`*"]
    pub fn FormatStringWithLeftToRightMarkers<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, number: Param0) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), number.into_param().abi(), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_PhoneNumberFormatting`*"]
    pub fn TryCreate<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(regioncode: Param0, phonenumber: &mut ::core::option::Option<PhoneNumberFormatter>) -> ::windows::runtime::Result<()> {
        Self::IPhoneNumberFormatterStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), regioncode.into_param().abi(), phonenumber as *mut _ as _).ok() })
    }
    #[doc = "*Required features: `Globalization_PhoneNumberFormatting`*"]
    pub fn GetCountryCodeForRegion<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(regioncode: Param0) -> ::windows::runtime::Result<i32> {
        Self::IPhoneNumberFormatterStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), regioncode.into_param().abi(), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `Globalization_PhoneNumberFormatting`*"]
    pub fn GetNationalDirectDialingPrefixForRegion<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(regioncode: Param0, stripnondigit: bool) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IPhoneNumberFormatterStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), regioncode.into_param().abi(), stripnondigit, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Globalization_PhoneNumberFormatting`*"]
    pub fn WrapWithLeftToRightMarkers<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(number: Param0) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IPhoneNumberFormatterStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), number.into_param().abi(), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn IPhoneNumberFormatterStatics<R, F: FnOnce(&IPhoneNumberFormatterStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PhoneNumberFormatter, IPhoneNumberFormatterStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PhoneNumberFormatter {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Globalization.PhoneNumberFormatting.PhoneNumberFormatter;{1556b49e-bad4-4b4a-900d-4407adb7c981})");
}
unsafe impl ::windows::runtime::Interface for PhoneNumberFormatter {
    type Vtable = IPhoneNumberFormatter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(358003870, 47828, 19274, [144, 13, 68, 7, 173, 183, 201, 129]);
}
impl ::windows::runtime::RuntimeName for PhoneNumberFormatter {
    const NAME: &'static str = "Windows.Globalization.PhoneNumberFormatting.PhoneNumberFormatter";
}
impl ::core::convert::From<PhoneNumberFormatter> for ::windows::runtime::IUnknown {
    fn from(value: PhoneNumberFormatter) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PhoneNumberFormatter> for ::windows::runtime::IUnknown {
    fn from(value: &PhoneNumberFormatter) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PhoneNumberFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PhoneNumberFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PhoneNumberFormatter> for ::windows::runtime::IInspectable {
    fn from(value: PhoneNumberFormatter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PhoneNumberFormatter> for ::windows::runtime::IInspectable {
    fn from(value: &PhoneNumberFormatter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PhoneNumberFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PhoneNumberFormatter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PhoneNumberFormatter {}
unsafe impl ::core::marker::Sync for PhoneNumberFormatter {}
#[doc = "*Required features: `Globalization_PhoneNumberFormatting`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PhoneNumberInfo(pub ::windows::runtime::IInspectable);
impl PhoneNumberInfo {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Globalization_PhoneNumberFormatting`, `Foundation`*"]
    pub fn ToString(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_PhoneNumberFormatting`*"]
    pub fn CountryCode(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_PhoneNumberFormatting`*"]
    pub fn PhoneNumber(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_PhoneNumberFormatting`*"]
    pub fn GetLengthOfGeographicalAreaCode(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_PhoneNumberFormatting`*"]
    pub fn GetNationalSignificantNumber(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_PhoneNumberFormatting`*"]
    pub fn GetLengthOfNationalDestinationCode(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_PhoneNumberFormatting`*"]
    pub fn PredictNumberKind(&self) -> ::windows::runtime::Result<PredictedPhoneNumberKind> {
        let this = self;
        unsafe {
            let mut result__: PredictedPhoneNumberKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PredictedPhoneNumberKind>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_PhoneNumberFormatting`*"]
    pub fn GetGeographicRegionCode(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_PhoneNumberFormatting`*"]
    pub fn CheckNumberMatch<'a, Param0: ::windows::runtime::IntoParam<'a, PhoneNumberInfo>>(&self, othernumber: Param0) -> ::windows::runtime::Result<PhoneNumberMatchResult> {
        let this = self;
        unsafe {
            let mut result__: PhoneNumberMatchResult = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), othernumber.into_param().abi(), &mut result__).from_abi::<PhoneNumberMatchResult>(result__)
        }
    }
    #[doc = "*Required features: `Globalization_PhoneNumberFormatting`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(number: Param0) -> ::windows::runtime::Result<PhoneNumberInfo> {
        Self::IPhoneNumberInfoFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), number.into_param().abi(), &mut result__).from_abi::<PhoneNumberInfo>(result__)
        })
    }
    #[doc = "*Required features: `Globalization_PhoneNumberFormatting`*"]
    pub fn TryParse<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(input: Param0, phonenumber: &mut ::core::option::Option<PhoneNumberInfo>) -> ::windows::runtime::Result<PhoneNumberParseResult> {
        Self::IPhoneNumberInfoStatics(|this| unsafe {
            let mut result__: PhoneNumberParseResult = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), input.into_param().abi(), phonenumber as *mut _ as _, &mut result__).from_abi::<PhoneNumberParseResult>(result__)
        })
    }
    #[doc = "*Required features: `Globalization_PhoneNumberFormatting`*"]
    pub fn TryParseWithRegion<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(input: Param0, regioncode: Param1, phonenumber: &mut ::core::option::Option<PhoneNumberInfo>) -> ::windows::runtime::Result<PhoneNumberParseResult> {
        Self::IPhoneNumberInfoStatics(|this| unsafe {
            let mut result__: PhoneNumberParseResult = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), input.into_param().abi(), regioncode.into_param().abi(), phonenumber as *mut _ as _, &mut result__).from_abi::<PhoneNumberParseResult>(result__)
        })
    }
    pub fn IPhoneNumberInfoFactory<R, F: FnOnce(&IPhoneNumberInfoFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PhoneNumberInfo, IPhoneNumberInfoFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IPhoneNumberInfoStatics<R, F: FnOnce(&IPhoneNumberInfoStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PhoneNumberInfo, IPhoneNumberInfoStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PhoneNumberInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Globalization.PhoneNumberFormatting.PhoneNumberInfo;{1c7ce4dd-c8b4-4ea3-9aef-b342e2c5b417})");
}
unsafe impl ::windows::runtime::Interface for PhoneNumberInfo {
    type Vtable = IPhoneNumberInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(477947101, 51380, 20131, [154, 239, 179, 66, 226, 197, 180, 23]);
}
impl ::windows::runtime::RuntimeName for PhoneNumberInfo {
    const NAME: &'static str = "Windows.Globalization.PhoneNumberFormatting.PhoneNumberInfo";
}
impl ::core::convert::From<PhoneNumberInfo> for ::windows::runtime::IUnknown {
    fn from(value: PhoneNumberInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PhoneNumberInfo> for ::windows::runtime::IUnknown {
    fn from(value: &PhoneNumberInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PhoneNumberInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PhoneNumberInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PhoneNumberInfo> for ::windows::runtime::IInspectable {
    fn from(value: PhoneNumberInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PhoneNumberInfo> for ::windows::runtime::IInspectable {
    fn from(value: &PhoneNumberInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PhoneNumberInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PhoneNumberInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<PhoneNumberInfo> for super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: PhoneNumberInfo) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&PhoneNumberInfo> for super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &PhoneNumberInfo) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IStringable> for PhoneNumberInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IStringable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IStringable> for &PhoneNumberInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IStringable> {
        ::core::convert::TryInto::<super::super::Foundation::IStringable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for PhoneNumberInfo {}
unsafe impl ::core::marker::Sync for PhoneNumberInfo {}
#[doc = "*Required features: `Globalization_PhoneNumberFormatting`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhoneNumberMatchResult(pub i32);
impl PhoneNumberMatchResult {
    pub const NoMatch: PhoneNumberMatchResult = PhoneNumberMatchResult(0i32);
    pub const ShortNationalSignificantNumberMatch: PhoneNumberMatchResult = PhoneNumberMatchResult(1i32);
    pub const NationalSignificantNumberMatch: PhoneNumberMatchResult = PhoneNumberMatchResult(2i32);
    pub const ExactMatch: PhoneNumberMatchResult = PhoneNumberMatchResult(3i32);
}
impl ::core::convert::From<i32> for PhoneNumberMatchResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PhoneNumberMatchResult {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PhoneNumberMatchResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Globalization.PhoneNumberFormatting.PhoneNumberMatchResult;i4)");
}
impl ::windows::runtime::DefaultType for PhoneNumberMatchResult {
    type DefaultType = Self;
}
#[doc = "*Required features: `Globalization_PhoneNumberFormatting`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhoneNumberParseResult(pub i32);
impl PhoneNumberParseResult {
    pub const Valid: PhoneNumberParseResult = PhoneNumberParseResult(0i32);
    pub const NotANumber: PhoneNumberParseResult = PhoneNumberParseResult(1i32);
    pub const InvalidCountryCode: PhoneNumberParseResult = PhoneNumberParseResult(2i32);
    pub const TooShort: PhoneNumberParseResult = PhoneNumberParseResult(3i32);
    pub const TooLong: PhoneNumberParseResult = PhoneNumberParseResult(4i32);
}
impl ::core::convert::From<i32> for PhoneNumberParseResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PhoneNumberParseResult {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PhoneNumberParseResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Globalization.PhoneNumberFormatting.PhoneNumberParseResult;i4)");
}
impl ::windows::runtime::DefaultType for PhoneNumberParseResult {
    type DefaultType = Self;
}
#[doc = "*Required features: `Globalization_PhoneNumberFormatting`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PredictedPhoneNumberKind(pub i32);
impl PredictedPhoneNumberKind {
    pub const FixedLine: PredictedPhoneNumberKind = PredictedPhoneNumberKind(0i32);
    pub const Mobile: PredictedPhoneNumberKind = PredictedPhoneNumberKind(1i32);
    pub const FixedLineOrMobile: PredictedPhoneNumberKind = PredictedPhoneNumberKind(2i32);
    pub const TollFree: PredictedPhoneNumberKind = PredictedPhoneNumberKind(3i32);
    pub const PremiumRate: PredictedPhoneNumberKind = PredictedPhoneNumberKind(4i32);
    pub const SharedCost: PredictedPhoneNumberKind = PredictedPhoneNumberKind(5i32);
    pub const Voip: PredictedPhoneNumberKind = PredictedPhoneNumberKind(6i32);
    pub const PersonalNumber: PredictedPhoneNumberKind = PredictedPhoneNumberKind(7i32);
    pub const Pager: PredictedPhoneNumberKind = PredictedPhoneNumberKind(8i32);
    pub const UniversalAccountNumber: PredictedPhoneNumberKind = PredictedPhoneNumberKind(9i32);
    pub const Voicemail: PredictedPhoneNumberKind = PredictedPhoneNumberKind(10i32);
    pub const Unknown: PredictedPhoneNumberKind = PredictedPhoneNumberKind(11i32);
}
impl ::core::convert::From<i32> for PredictedPhoneNumberKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PredictedPhoneNumberKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PredictedPhoneNumberKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Globalization.PhoneNumberFormatting.PredictedPhoneNumberKind;i4)");
}
impl ::windows::runtime::DefaultType for PredictedPhoneNumberKind {
    type DefaultType = Self;
}
