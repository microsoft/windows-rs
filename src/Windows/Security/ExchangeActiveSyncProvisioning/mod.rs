#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EasClientDeviceInformation(pub ::windows::core::IInspectable);
impl EasClientDeviceInformation {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<EasClientDeviceInformation, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn OperatingSystem(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn FriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn SystemManufacturer(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn SystemProductName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn SystemSku(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn SystemHardwareVersion(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IEasClientDeviceInformation2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn SystemFirmwareVersion(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IEasClientDeviceInformation2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for EasClientDeviceInformation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.ExchangeActiveSyncProvisioning.EasClientDeviceInformation;{54dfd981-1968-4ca3-b958-e595d16505eb})");
}
unsafe impl ::windows::core::Interface for EasClientDeviceInformation {
    type Vtable = IEasClientDeviceInformation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x54dfd981_1968_4ca3_b958_e595d16505eb);
}
impl ::windows::core::RuntimeName for EasClientDeviceInformation {
    const NAME: &'static str = "Windows.Security.ExchangeActiveSyncProvisioning.EasClientDeviceInformation";
}
impl ::core::convert::From<EasClientDeviceInformation> for ::windows::core::IUnknown {
    fn from(value: EasClientDeviceInformation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EasClientDeviceInformation> for ::windows::core::IUnknown {
    fn from(value: &EasClientDeviceInformation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EasClientDeviceInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EasClientDeviceInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EasClientDeviceInformation> for ::windows::core::IInspectable {
    fn from(value: EasClientDeviceInformation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EasClientDeviceInformation> for ::windows::core::IInspectable {
    fn from(value: &EasClientDeviceInformation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EasClientDeviceInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EasClientDeviceInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EasClientSecurityPolicy(pub ::windows::core::IInspectable);
impl EasClientSecurityPolicy {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<EasClientSecurityPolicy, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn RequireEncryption(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn SetRequireEncryption(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn MinPasswordLength(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn SetMinPasswordLength(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn DisallowConvenienceLogon(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn SetDisallowConvenienceLogon(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn MinPasswordComplexCharacters(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn SetMinPasswordComplexCharacters(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`, `Foundation`*"]
    pub fn PasswordExpiration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`, `Foundation`*"]
    pub fn SetPasswordExpiration<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn PasswordHistory(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn SetPasswordHistory(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn MaxPasswordFailedAttempts(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn SetMaxPasswordFailedAttempts(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`, `Foundation`*"]
    pub fn MaxInactivityTimeLock(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`, `Foundation`*"]
    pub fn SetMaxInactivityTimeLock<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn CheckCompliance(&self) -> ::windows::core::Result<EasComplianceResults> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<EasComplianceResults>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`, `Foundation`*"]
    pub fn ApplyAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EasComplianceResults>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<EasComplianceResults>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for EasClientSecurityPolicy {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.ExchangeActiveSyncProvisioning.EasClientSecurityPolicy;{45b72362-dfba-4a9b-aced-6fe2adcb6420})");
}
unsafe impl ::windows::core::Interface for EasClientSecurityPolicy {
    type Vtable = IEasClientSecurityPolicy_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x45b72362_dfba_4a9b_aced_6fe2adcb6420);
}
impl ::windows::core::RuntimeName for EasClientSecurityPolicy {
    const NAME: &'static str = "Windows.Security.ExchangeActiveSyncProvisioning.EasClientSecurityPolicy";
}
impl ::core::convert::From<EasClientSecurityPolicy> for ::windows::core::IUnknown {
    fn from(value: EasClientSecurityPolicy) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EasClientSecurityPolicy> for ::windows::core::IUnknown {
    fn from(value: &EasClientSecurityPolicy) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EasClientSecurityPolicy {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EasClientSecurityPolicy {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EasClientSecurityPolicy> for ::windows::core::IInspectable {
    fn from(value: EasClientSecurityPolicy) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EasClientSecurityPolicy> for ::windows::core::IInspectable {
    fn from(value: &EasClientSecurityPolicy) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EasClientSecurityPolicy {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EasClientSecurityPolicy {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EasComplianceResults(pub ::windows::core::IInspectable);
impl EasComplianceResults {
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn Compliant(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn RequireEncryptionResult(&self) -> ::windows::core::Result<EasRequireEncryptionResult> {
        let this = self;
        unsafe {
            let mut result__: EasRequireEncryptionResult = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<EasRequireEncryptionResult>(result__)
        }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn MinPasswordLengthResult(&self) -> ::windows::core::Result<EasMinPasswordLengthResult> {
        let this = self;
        unsafe {
            let mut result__: EasMinPasswordLengthResult = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<EasMinPasswordLengthResult>(result__)
        }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn DisallowConvenienceLogonResult(&self) -> ::windows::core::Result<EasDisallowConvenienceLogonResult> {
        let this = self;
        unsafe {
            let mut result__: EasDisallowConvenienceLogonResult = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<EasDisallowConvenienceLogonResult>(result__)
        }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn MinPasswordComplexCharactersResult(&self) -> ::windows::core::Result<EasMinPasswordComplexCharactersResult> {
        let this = self;
        unsafe {
            let mut result__: EasMinPasswordComplexCharactersResult = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<EasMinPasswordComplexCharactersResult>(result__)
        }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn PasswordExpirationResult(&self) -> ::windows::core::Result<EasPasswordExpirationResult> {
        let this = self;
        unsafe {
            let mut result__: EasPasswordExpirationResult = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<EasPasswordExpirationResult>(result__)
        }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn PasswordHistoryResult(&self) -> ::windows::core::Result<EasPasswordHistoryResult> {
        let this = self;
        unsafe {
            let mut result__: EasPasswordHistoryResult = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<EasPasswordHistoryResult>(result__)
        }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn MaxPasswordFailedAttemptsResult(&self) -> ::windows::core::Result<EasMaxPasswordFailedAttemptsResult> {
        let this = self;
        unsafe {
            let mut result__: EasMaxPasswordFailedAttemptsResult = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<EasMaxPasswordFailedAttemptsResult>(result__)
        }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn MaxInactivityTimeLockResult(&self) -> ::windows::core::Result<EasMaxInactivityTimeLockResult> {
        let this = self;
        unsafe {
            let mut result__: EasMaxInactivityTimeLockResult = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<EasMaxInactivityTimeLockResult>(result__)
        }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn EncryptionProviderType(&self) -> ::windows::core::Result<EasEncryptionProviderType> {
        let this = &::windows::core::Interface::cast::<IEasComplianceResults2>(self)?;
        unsafe {
            let mut result__: EasEncryptionProviderType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<EasEncryptionProviderType>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for EasComplianceResults {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.ExchangeActiveSyncProvisioning.EasComplianceResults;{463c299c-7f19-4c66-b403-cb45dd57a2b3})");
}
unsafe impl ::windows::core::Interface for EasComplianceResults {
    type Vtable = IEasComplianceResults_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x463c299c_7f19_4c66_b403_cb45dd57a2b3);
}
impl ::windows::core::RuntimeName for EasComplianceResults {
    const NAME: &'static str = "Windows.Security.ExchangeActiveSyncProvisioning.EasComplianceResults";
}
impl ::core::convert::From<EasComplianceResults> for ::windows::core::IUnknown {
    fn from(value: EasComplianceResults) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EasComplianceResults> for ::windows::core::IUnknown {
    fn from(value: &EasComplianceResults) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EasComplianceResults {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EasComplianceResults {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EasComplianceResults> for ::windows::core::IInspectable {
    fn from(value: EasComplianceResults) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EasComplianceResults> for ::windows::core::IInspectable {
    fn from(value: &EasComplianceResults) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EasComplianceResults {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EasComplianceResults {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[derive(:: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy)]
pub struct EasContract(pub u8);
#[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct EasDisallowConvenienceLogonResult(pub i32);
impl EasDisallowConvenienceLogonResult {
    pub const NotEvaluated: EasDisallowConvenienceLogonResult = EasDisallowConvenienceLogonResult(0i32);
    pub const Compliant: EasDisallowConvenienceLogonResult = EasDisallowConvenienceLogonResult(1i32);
    pub const CanBeCompliant: EasDisallowConvenienceLogonResult = EasDisallowConvenienceLogonResult(2i32);
    pub const RequestedPolicyIsStricter: EasDisallowConvenienceLogonResult = EasDisallowConvenienceLogonResult(3i32);
}
impl ::core::convert::From<i32> for EasDisallowConvenienceLogonResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for EasDisallowConvenienceLogonResult {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for EasDisallowConvenienceLogonResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.ExchangeActiveSyncProvisioning.EasDisallowConvenienceLogonResult;i4)");
}
impl ::windows::core::DefaultType for EasDisallowConvenienceLogonResult {
    type DefaultType = Self;
}
#[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct EasEncryptionProviderType(pub i32);
impl EasEncryptionProviderType {
    pub const NotEvaluated: EasEncryptionProviderType = EasEncryptionProviderType(0i32);
    pub const WindowsEncryption: EasEncryptionProviderType = EasEncryptionProviderType(1i32);
    pub const OtherEncryption: EasEncryptionProviderType = EasEncryptionProviderType(2i32);
}
impl ::core::convert::From<i32> for EasEncryptionProviderType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for EasEncryptionProviderType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for EasEncryptionProviderType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.ExchangeActiveSyncProvisioning.EasEncryptionProviderType;i4)");
}
impl ::windows::core::DefaultType for EasEncryptionProviderType {
    type DefaultType = Self;
}
#[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct EasMaxInactivityTimeLockResult(pub i32);
impl EasMaxInactivityTimeLockResult {
    pub const NotEvaluated: EasMaxInactivityTimeLockResult = EasMaxInactivityTimeLockResult(0i32);
    pub const Compliant: EasMaxInactivityTimeLockResult = EasMaxInactivityTimeLockResult(1i32);
    pub const CanBeCompliant: EasMaxInactivityTimeLockResult = EasMaxInactivityTimeLockResult(2i32);
    pub const RequestedPolicyIsStricter: EasMaxInactivityTimeLockResult = EasMaxInactivityTimeLockResult(3i32);
    pub const InvalidParameter: EasMaxInactivityTimeLockResult = EasMaxInactivityTimeLockResult(4i32);
}
impl ::core::convert::From<i32> for EasMaxInactivityTimeLockResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for EasMaxInactivityTimeLockResult {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for EasMaxInactivityTimeLockResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.ExchangeActiveSyncProvisioning.EasMaxInactivityTimeLockResult;i4)");
}
impl ::windows::core::DefaultType for EasMaxInactivityTimeLockResult {
    type DefaultType = Self;
}
#[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct EasMaxPasswordFailedAttemptsResult(pub i32);
impl EasMaxPasswordFailedAttemptsResult {
    pub const NotEvaluated: EasMaxPasswordFailedAttemptsResult = EasMaxPasswordFailedAttemptsResult(0i32);
    pub const Compliant: EasMaxPasswordFailedAttemptsResult = EasMaxPasswordFailedAttemptsResult(1i32);
    pub const CanBeCompliant: EasMaxPasswordFailedAttemptsResult = EasMaxPasswordFailedAttemptsResult(2i32);
    pub const RequestedPolicyIsStricter: EasMaxPasswordFailedAttemptsResult = EasMaxPasswordFailedAttemptsResult(3i32);
    pub const InvalidParameter: EasMaxPasswordFailedAttemptsResult = EasMaxPasswordFailedAttemptsResult(4i32);
}
impl ::core::convert::From<i32> for EasMaxPasswordFailedAttemptsResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for EasMaxPasswordFailedAttemptsResult {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for EasMaxPasswordFailedAttemptsResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.ExchangeActiveSyncProvisioning.EasMaxPasswordFailedAttemptsResult;i4)");
}
impl ::windows::core::DefaultType for EasMaxPasswordFailedAttemptsResult {
    type DefaultType = Self;
}
#[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct EasMinPasswordComplexCharactersResult(pub i32);
impl EasMinPasswordComplexCharactersResult {
    pub const NotEvaluated: EasMinPasswordComplexCharactersResult = EasMinPasswordComplexCharactersResult(0i32);
    pub const Compliant: EasMinPasswordComplexCharactersResult = EasMinPasswordComplexCharactersResult(1i32);
    pub const CanBeCompliant: EasMinPasswordComplexCharactersResult = EasMinPasswordComplexCharactersResult(2i32);
    pub const RequestedPolicyIsStricter: EasMinPasswordComplexCharactersResult = EasMinPasswordComplexCharactersResult(3i32);
    pub const RequestedPolicyNotEnforceable: EasMinPasswordComplexCharactersResult = EasMinPasswordComplexCharactersResult(4i32);
    pub const InvalidParameter: EasMinPasswordComplexCharactersResult = EasMinPasswordComplexCharactersResult(5i32);
    pub const CurrentUserHasBlankPassword: EasMinPasswordComplexCharactersResult = EasMinPasswordComplexCharactersResult(6i32);
    pub const AdminsHaveBlankPassword: EasMinPasswordComplexCharactersResult = EasMinPasswordComplexCharactersResult(7i32);
    pub const UserCannotChangePassword: EasMinPasswordComplexCharactersResult = EasMinPasswordComplexCharactersResult(8i32);
    pub const AdminsCannotChangePassword: EasMinPasswordComplexCharactersResult = EasMinPasswordComplexCharactersResult(9i32);
    pub const LocalControlledUsersCannotChangePassword: EasMinPasswordComplexCharactersResult = EasMinPasswordComplexCharactersResult(10i32);
    pub const ConnectedAdminsProviderPolicyIsWeak: EasMinPasswordComplexCharactersResult = EasMinPasswordComplexCharactersResult(11i32);
    pub const ConnectedUserProviderPolicyIsWeak: EasMinPasswordComplexCharactersResult = EasMinPasswordComplexCharactersResult(12i32);
    pub const ChangeConnectedAdminsPassword: EasMinPasswordComplexCharactersResult = EasMinPasswordComplexCharactersResult(13i32);
    pub const ChangeConnectedUserPassword: EasMinPasswordComplexCharactersResult = EasMinPasswordComplexCharactersResult(14i32);
}
impl ::core::convert::From<i32> for EasMinPasswordComplexCharactersResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for EasMinPasswordComplexCharactersResult {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for EasMinPasswordComplexCharactersResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.ExchangeActiveSyncProvisioning.EasMinPasswordComplexCharactersResult;i4)");
}
impl ::windows::core::DefaultType for EasMinPasswordComplexCharactersResult {
    type DefaultType = Self;
}
#[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct EasMinPasswordLengthResult(pub i32);
impl EasMinPasswordLengthResult {
    pub const NotEvaluated: EasMinPasswordLengthResult = EasMinPasswordLengthResult(0i32);
    pub const Compliant: EasMinPasswordLengthResult = EasMinPasswordLengthResult(1i32);
    pub const CanBeCompliant: EasMinPasswordLengthResult = EasMinPasswordLengthResult(2i32);
    pub const RequestedPolicyIsStricter: EasMinPasswordLengthResult = EasMinPasswordLengthResult(3i32);
    pub const RequestedPolicyNotEnforceable: EasMinPasswordLengthResult = EasMinPasswordLengthResult(4i32);
    pub const InvalidParameter: EasMinPasswordLengthResult = EasMinPasswordLengthResult(5i32);
    pub const CurrentUserHasBlankPassword: EasMinPasswordLengthResult = EasMinPasswordLengthResult(6i32);
    pub const AdminsHaveBlankPassword: EasMinPasswordLengthResult = EasMinPasswordLengthResult(7i32);
    pub const UserCannotChangePassword: EasMinPasswordLengthResult = EasMinPasswordLengthResult(8i32);
    pub const AdminsCannotChangePassword: EasMinPasswordLengthResult = EasMinPasswordLengthResult(9i32);
    pub const LocalControlledUsersCannotChangePassword: EasMinPasswordLengthResult = EasMinPasswordLengthResult(10i32);
    pub const ConnectedAdminsProviderPolicyIsWeak: EasMinPasswordLengthResult = EasMinPasswordLengthResult(11i32);
    pub const ConnectedUserProviderPolicyIsWeak: EasMinPasswordLengthResult = EasMinPasswordLengthResult(12i32);
    pub const ChangeConnectedAdminsPassword: EasMinPasswordLengthResult = EasMinPasswordLengthResult(13i32);
    pub const ChangeConnectedUserPassword: EasMinPasswordLengthResult = EasMinPasswordLengthResult(14i32);
}
impl ::core::convert::From<i32> for EasMinPasswordLengthResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for EasMinPasswordLengthResult {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for EasMinPasswordLengthResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.ExchangeActiveSyncProvisioning.EasMinPasswordLengthResult;i4)");
}
impl ::windows::core::DefaultType for EasMinPasswordLengthResult {
    type DefaultType = Self;
}
#[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct EasPasswordExpirationResult(pub i32);
impl EasPasswordExpirationResult {
    pub const NotEvaluated: EasPasswordExpirationResult = EasPasswordExpirationResult(0i32);
    pub const Compliant: EasPasswordExpirationResult = EasPasswordExpirationResult(1i32);
    pub const CanBeCompliant: EasPasswordExpirationResult = EasPasswordExpirationResult(2i32);
    pub const RequestedPolicyIsStricter: EasPasswordExpirationResult = EasPasswordExpirationResult(3i32);
    pub const RequestedExpirationIncompatible: EasPasswordExpirationResult = EasPasswordExpirationResult(4i32);
    pub const InvalidParameter: EasPasswordExpirationResult = EasPasswordExpirationResult(5i32);
    pub const UserCannotChangePassword: EasPasswordExpirationResult = EasPasswordExpirationResult(6i32);
    pub const AdminsCannotChangePassword: EasPasswordExpirationResult = EasPasswordExpirationResult(7i32);
    pub const LocalControlledUsersCannotChangePassword: EasPasswordExpirationResult = EasPasswordExpirationResult(8i32);
}
impl ::core::convert::From<i32> for EasPasswordExpirationResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for EasPasswordExpirationResult {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for EasPasswordExpirationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.ExchangeActiveSyncProvisioning.EasPasswordExpirationResult;i4)");
}
impl ::windows::core::DefaultType for EasPasswordExpirationResult {
    type DefaultType = Self;
}
#[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct EasPasswordHistoryResult(pub i32);
impl EasPasswordHistoryResult {
    pub const NotEvaluated: EasPasswordHistoryResult = EasPasswordHistoryResult(0i32);
    pub const Compliant: EasPasswordHistoryResult = EasPasswordHistoryResult(1i32);
    pub const CanBeCompliant: EasPasswordHistoryResult = EasPasswordHistoryResult(2i32);
    pub const RequestedPolicyIsStricter: EasPasswordHistoryResult = EasPasswordHistoryResult(3i32);
    pub const InvalidParameter: EasPasswordHistoryResult = EasPasswordHistoryResult(4i32);
}
impl ::core::convert::From<i32> for EasPasswordHistoryResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for EasPasswordHistoryResult {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for EasPasswordHistoryResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.ExchangeActiveSyncProvisioning.EasPasswordHistoryResult;i4)");
}
impl ::windows::core::DefaultType for EasPasswordHistoryResult {
    type DefaultType = Self;
}
#[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct EasRequireEncryptionResult(pub i32);
impl EasRequireEncryptionResult {
    pub const NotEvaluated: EasRequireEncryptionResult = EasRequireEncryptionResult(0i32);
    pub const Compliant: EasRequireEncryptionResult = EasRequireEncryptionResult(1i32);
    pub const CanBeCompliant: EasRequireEncryptionResult = EasRequireEncryptionResult(2i32);
    pub const NotProvisionedOnAllVolumes: EasRequireEncryptionResult = EasRequireEncryptionResult(3i32);
    pub const DeFixedDataNotSupported: EasRequireEncryptionResult = EasRequireEncryptionResult(4i32);
    pub const FixedDataNotSupported: EasRequireEncryptionResult = EasRequireEncryptionResult(4i32);
    pub const DeHardwareNotCompliant: EasRequireEncryptionResult = EasRequireEncryptionResult(5i32);
    pub const HardwareNotCompliant: EasRequireEncryptionResult = EasRequireEncryptionResult(5i32);
    pub const DeWinReNotConfigured: EasRequireEncryptionResult = EasRequireEncryptionResult(6i32);
    pub const LockNotConfigured: EasRequireEncryptionResult = EasRequireEncryptionResult(6i32);
    pub const DeProtectionSuspended: EasRequireEncryptionResult = EasRequireEncryptionResult(7i32);
    pub const ProtectionSuspended: EasRequireEncryptionResult = EasRequireEncryptionResult(7i32);
    pub const DeOsVolumeNotProtected: EasRequireEncryptionResult = EasRequireEncryptionResult(8i32);
    pub const OsVolumeNotProtected: EasRequireEncryptionResult = EasRequireEncryptionResult(8i32);
    pub const DeProtectionNotYetEnabled: EasRequireEncryptionResult = EasRequireEncryptionResult(9i32);
    pub const ProtectionNotYetEnabled: EasRequireEncryptionResult = EasRequireEncryptionResult(9i32);
    pub const NoFeatureLicense: EasRequireEncryptionResult = EasRequireEncryptionResult(10i32);
    pub const OsNotProtected: EasRequireEncryptionResult = EasRequireEncryptionResult(11i32);
    pub const UnexpectedFailure: EasRequireEncryptionResult = EasRequireEncryptionResult(12i32);
}
impl ::core::convert::From<i32> for EasRequireEncryptionResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for EasRequireEncryptionResult {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for EasRequireEncryptionResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.ExchangeActiveSyncProvisioning.EasRequireEncryptionResult;i4)");
}
impl ::windows::core::DefaultType for EasRequireEncryptionResult {
    type DefaultType = Self;
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IEasClientDeviceInformation(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEasClientDeviceInformation {
    type Vtable = IEasClientDeviceInformation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x54dfd981_1968_4ca3_b958_e595d16505eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEasClientDeviceInformation_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEasClientDeviceInformation2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEasClientDeviceInformation2 {
    type Vtable = IEasClientDeviceInformation2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xffb35923_bb26_4d6a_81bc_165aee0ad754);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEasClientDeviceInformation2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEasClientSecurityPolicy(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEasClientSecurityPolicy {
    type Vtable = IEasClientSecurityPolicy_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x45b72362_dfba_4a9b_aced_6fe2adcb6420);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEasClientSecurityPolicy_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u8) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u8) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEasComplianceResults(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEasComplianceResults {
    type Vtable = IEasComplianceResults_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x463c299c_7f19_4c66_b403_cb45dd57a2b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEasComplianceResults_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut EasRequireEncryptionResult) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut EasMinPasswordLengthResult) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut EasDisallowConvenienceLogonResult) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut EasMinPasswordComplexCharactersResult) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut EasPasswordExpirationResult) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut EasPasswordHistoryResult) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut EasMaxPasswordFailedAttemptsResult) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut EasMaxInactivityTimeLockResult) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEasComplianceResults2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEasComplianceResults2 {
    type Vtable = IEasComplianceResults2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2fbe60c9_1aa8_47f5_88bb_cb3ef0bffb15);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEasComplianceResults2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut EasEncryptionProviderType) -> ::windows::core::HRESULT,
);
