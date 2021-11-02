#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct EasClientDeviceInformation(::windows::runtime::IInspectable);
impl EasClientDeviceInformation {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<EasClientDeviceInformation, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn OperatingSystem(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn FriendlyName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn SystemManufacturer(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn SystemProductName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn SystemSku(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn SystemHardwareVersion(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IEasClientDeviceInformation2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn SystemFirmwareVersion(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IEasClientDeviceInformation2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for EasClientDeviceInformation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.ExchangeActiveSyncProvisioning.EasClientDeviceInformation;{54dfd981-1968-4ca3-b958-e595d16505eb})");
}
unsafe impl ::windows::runtime::Interface for EasClientDeviceInformation {
    type Vtable = IEasClientDeviceInformation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1423956353, 6504, 19619, [185, 88, 229, 149, 209, 101, 5, 235]);
}
impl ::windows::runtime::RuntimeName for EasClientDeviceInformation {
    const NAME: &'static str = "Windows.Security.ExchangeActiveSyncProvisioning.EasClientDeviceInformation";
}
#[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct EasClientSecurityPolicy(::windows::runtime::IInspectable);
impl EasClientSecurityPolicy {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<EasClientSecurityPolicy, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn RequireEncryption(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn SetRequireEncryption(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn MinPasswordLength(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn SetMinPasswordLength(&self, value: u8) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn DisallowConvenienceLogon(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn SetDisallowConvenienceLogon(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn MinPasswordComplexCharacters(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn SetMinPasswordComplexCharacters(&self, value: u8) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`, `Foundation`*"]
    pub fn PasswordExpiration(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`, `Foundation`*"]
    pub fn SetPasswordExpiration<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn PasswordHistory(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn SetPasswordHistory(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn MaxPasswordFailedAttempts(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn SetMaxPasswordFailedAttempts(&self, value: u8) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`, `Foundation`*"]
    pub fn MaxInactivityTimeLock(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`, `Foundation`*"]
    pub fn SetMaxInactivityTimeLock<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn CheckCompliance(&self) -> ::windows::runtime::Result<EasComplianceResults> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), &mut result__).from_abi::<EasComplianceResults>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`, `Foundation`*"]
    pub fn ApplyAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<EasComplianceResults>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<EasComplianceResults>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for EasClientSecurityPolicy {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.ExchangeActiveSyncProvisioning.EasClientSecurityPolicy;{45b72362-dfba-4a9b-aced-6fe2adcb6420})");
}
unsafe impl ::windows::runtime::Interface for EasClientSecurityPolicy {
    type Vtable = IEasClientSecurityPolicy_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1169630050, 57274, 19099, [172, 237, 111, 226, 173, 203, 100, 32]);
}
impl ::windows::runtime::RuntimeName for EasClientSecurityPolicy {
    const NAME: &'static str = "Windows.Security.ExchangeActiveSyncProvisioning.EasClientSecurityPolicy";
}
#[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct EasComplianceResults(::windows::runtime::IInspectable);
impl EasComplianceResults {
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn Compliant(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn RequireEncryptionResult(&self) -> ::windows::runtime::Result<EasRequireEncryptionResult> {
        let this = self;
        unsafe {
            let mut result__: EasRequireEncryptionResult = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<EasRequireEncryptionResult>(result__)
        }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn MinPasswordLengthResult(&self) -> ::windows::runtime::Result<EasMinPasswordLengthResult> {
        let this = self;
        unsafe {
            let mut result__: EasMinPasswordLengthResult = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<EasMinPasswordLengthResult>(result__)
        }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn DisallowConvenienceLogonResult(&self) -> ::windows::runtime::Result<EasDisallowConvenienceLogonResult> {
        let this = self;
        unsafe {
            let mut result__: EasDisallowConvenienceLogonResult = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<EasDisallowConvenienceLogonResult>(result__)
        }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn MinPasswordComplexCharactersResult(&self) -> ::windows::runtime::Result<EasMinPasswordComplexCharactersResult> {
        let this = self;
        unsafe {
            let mut result__: EasMinPasswordComplexCharactersResult = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<EasMinPasswordComplexCharactersResult>(result__)
        }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn PasswordExpirationResult(&self) -> ::windows::runtime::Result<EasPasswordExpirationResult> {
        let this = self;
        unsafe {
            let mut result__: EasPasswordExpirationResult = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<EasPasswordExpirationResult>(result__)
        }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn PasswordHistoryResult(&self) -> ::windows::runtime::Result<EasPasswordHistoryResult> {
        let this = self;
        unsafe {
            let mut result__: EasPasswordHistoryResult = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<EasPasswordHistoryResult>(result__)
        }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn MaxPasswordFailedAttemptsResult(&self) -> ::windows::runtime::Result<EasMaxPasswordFailedAttemptsResult> {
        let this = self;
        unsafe {
            let mut result__: EasMaxPasswordFailedAttemptsResult = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<EasMaxPasswordFailedAttemptsResult>(result__)
        }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn MaxInactivityTimeLockResult(&self) -> ::windows::runtime::Result<EasMaxInactivityTimeLockResult> {
        let this = self;
        unsafe {
            let mut result__: EasMaxInactivityTimeLockResult = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<EasMaxInactivityTimeLockResult>(result__)
        }
    }
    #[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
    pub fn EncryptionProviderType(&self) -> ::windows::runtime::Result<EasEncryptionProviderType> {
        let this = &::windows::runtime::Interface::cast::<IEasComplianceResults2>(self)?;
        unsafe {
            let mut result__: EasEncryptionProviderType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<EasEncryptionProviderType>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for EasComplianceResults {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.ExchangeActiveSyncProvisioning.EasComplianceResults;{463c299c-7f19-4c66-b403-cb45dd57a2b3})");
}
unsafe impl ::windows::runtime::Interface for EasComplianceResults {
    type Vtable = IEasComplianceResults_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1178347932, 32537, 19558, [180, 3, 203, 69, 221, 87, 162, 179]);
}
impl ::windows::runtime::RuntimeName for EasComplianceResults {
    const NAME: &'static str = "Windows.Security.ExchangeActiveSyncProvisioning.EasComplianceResults";
}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct EasContract(pub u8);
#[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct EasDisallowConvenienceLogonResult(pub i32);
impl EasDisallowConvenienceLogonResult {
    pub const NotEvaluated: EasDisallowConvenienceLogonResult = EasDisallowConvenienceLogonResult(0i32);
    pub const Compliant: EasDisallowConvenienceLogonResult = EasDisallowConvenienceLogonResult(1i32);
    pub const CanBeCompliant: EasDisallowConvenienceLogonResult = EasDisallowConvenienceLogonResult(2i32);
    pub const RequestedPolicyIsStricter: EasDisallowConvenienceLogonResult = EasDisallowConvenienceLogonResult(3i32);
}
impl ::std::convert::From<i32> for EasDisallowConvenienceLogonResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EasDisallowConvenienceLogonResult {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for EasDisallowConvenienceLogonResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.ExchangeActiveSyncProvisioning.EasDisallowConvenienceLogonResult;i4)");
}
#[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct EasEncryptionProviderType(pub i32);
impl EasEncryptionProviderType {
    pub const NotEvaluated: EasEncryptionProviderType = EasEncryptionProviderType(0i32);
    pub const WindowsEncryption: EasEncryptionProviderType = EasEncryptionProviderType(1i32);
    pub const OtherEncryption: EasEncryptionProviderType = EasEncryptionProviderType(2i32);
}
impl ::std::convert::From<i32> for EasEncryptionProviderType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EasEncryptionProviderType {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for EasEncryptionProviderType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.ExchangeActiveSyncProvisioning.EasEncryptionProviderType;i4)");
}
#[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct EasMaxInactivityTimeLockResult(pub i32);
impl EasMaxInactivityTimeLockResult {
    pub const NotEvaluated: EasMaxInactivityTimeLockResult = EasMaxInactivityTimeLockResult(0i32);
    pub const Compliant: EasMaxInactivityTimeLockResult = EasMaxInactivityTimeLockResult(1i32);
    pub const CanBeCompliant: EasMaxInactivityTimeLockResult = EasMaxInactivityTimeLockResult(2i32);
    pub const RequestedPolicyIsStricter: EasMaxInactivityTimeLockResult = EasMaxInactivityTimeLockResult(3i32);
    pub const InvalidParameter: EasMaxInactivityTimeLockResult = EasMaxInactivityTimeLockResult(4i32);
}
impl ::std::convert::From<i32> for EasMaxInactivityTimeLockResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EasMaxInactivityTimeLockResult {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for EasMaxInactivityTimeLockResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.ExchangeActiveSyncProvisioning.EasMaxInactivityTimeLockResult;i4)");
}
#[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct EasMaxPasswordFailedAttemptsResult(pub i32);
impl EasMaxPasswordFailedAttemptsResult {
    pub const NotEvaluated: EasMaxPasswordFailedAttemptsResult = EasMaxPasswordFailedAttemptsResult(0i32);
    pub const Compliant: EasMaxPasswordFailedAttemptsResult = EasMaxPasswordFailedAttemptsResult(1i32);
    pub const CanBeCompliant: EasMaxPasswordFailedAttemptsResult = EasMaxPasswordFailedAttemptsResult(2i32);
    pub const RequestedPolicyIsStricter: EasMaxPasswordFailedAttemptsResult = EasMaxPasswordFailedAttemptsResult(3i32);
    pub const InvalidParameter: EasMaxPasswordFailedAttemptsResult = EasMaxPasswordFailedAttemptsResult(4i32);
}
impl ::std::convert::From<i32> for EasMaxPasswordFailedAttemptsResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EasMaxPasswordFailedAttemptsResult {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for EasMaxPasswordFailedAttemptsResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.ExchangeActiveSyncProvisioning.EasMaxPasswordFailedAttemptsResult;i4)");
}
#[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for EasMinPasswordComplexCharactersResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EasMinPasswordComplexCharactersResult {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for EasMinPasswordComplexCharactersResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.ExchangeActiveSyncProvisioning.EasMinPasswordComplexCharactersResult;i4)");
}
#[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for EasMinPasswordLengthResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EasMinPasswordLengthResult {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for EasMinPasswordLengthResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.ExchangeActiveSyncProvisioning.EasMinPasswordLengthResult;i4)");
}
#[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for EasPasswordExpirationResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EasPasswordExpirationResult {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for EasPasswordExpirationResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.ExchangeActiveSyncProvisioning.EasPasswordExpirationResult;i4)");
}
#[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct EasPasswordHistoryResult(pub i32);
impl EasPasswordHistoryResult {
    pub const NotEvaluated: EasPasswordHistoryResult = EasPasswordHistoryResult(0i32);
    pub const Compliant: EasPasswordHistoryResult = EasPasswordHistoryResult(1i32);
    pub const CanBeCompliant: EasPasswordHistoryResult = EasPasswordHistoryResult(2i32);
    pub const RequestedPolicyIsStricter: EasPasswordHistoryResult = EasPasswordHistoryResult(3i32);
    pub const InvalidParameter: EasPasswordHistoryResult = EasPasswordHistoryResult(4i32);
}
impl ::std::convert::From<i32> for EasPasswordHistoryResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EasPasswordHistoryResult {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for EasPasswordHistoryResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.ExchangeActiveSyncProvisioning.EasPasswordHistoryResult;i4)");
}
#[doc = "*Required features: `Security_ExchangeActiveSyncProvisioning`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for EasRequireEncryptionResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EasRequireEncryptionResult {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for EasRequireEncryptionResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.ExchangeActiveSyncProvisioning.EasRequireEncryptionResult;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IEasClientDeviceInformation(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEasClientDeviceInformation {
    type Vtable = IEasClientDeviceInformation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1423956353, 6504, 19619, [185, 88, 229, 149, 209, 101, 5, 235]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEasClientDeviceInformation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IEasClientDeviceInformation2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEasClientDeviceInformation2 {
    type Vtable = IEasClientDeviceInformation2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4289943843, 47910, 19818, [129, 188, 22, 90, 238, 10, 215, 84]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEasClientDeviceInformation2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IEasClientSecurityPolicy(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEasClientSecurityPolicy {
    type Vtable = IEasClientSecurityPolicy_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1169630050, 57274, 19099, [172, 237, 111, 226, 173, 203, 100, 32]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEasClientSecurityPolicy_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u8) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u8) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IEasComplianceResults(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEasComplianceResults {
    type Vtable = IEasComplianceResults_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1178347932, 32537, 19558, [180, 3, 203, 69, 221, 87, 162, 179]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEasComplianceResults_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut EasRequireEncryptionResult) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut EasMinPasswordLengthResult) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut EasDisallowConvenienceLogonResult) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut EasMinPasswordComplexCharactersResult) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut EasPasswordExpirationResult) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut EasPasswordHistoryResult) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut EasMaxPasswordFailedAttemptsResult) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut EasMaxInactivityTimeLockResult) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IEasComplianceResults2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEasComplianceResults2 {
    type Vtable = IEasComplianceResults2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(801005769, 6824, 18421, [136, 187, 203, 62, 240, 191, 251, 21]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEasComplianceResults2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut EasEncryptionProviderType) -> ::windows::runtime::HRESULT,
);
