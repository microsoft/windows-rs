#[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
#[repr(transparent)]
pub struct EasClientDeviceInformation(::windows::core::IUnknown);
impl EasClientDeviceInformation {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<EasClientDeviceInformation, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::GUID>::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
    pub fn OperatingSystem(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).OperatingSystem)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
    pub fn FriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).FriendlyName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
    pub fn SystemManufacturer(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).SystemManufacturer)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
    pub fn SystemProductName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).SystemProductName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
    pub fn SystemSku(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).SystemSku)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
    pub fn SystemHardwareVersion(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IEasClientDeviceInformation2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).SystemHardwareVersion)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
    pub fn SystemFirmwareVersion(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IEasClientDeviceInformation2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).SystemFirmwareVersion)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for EasClientDeviceInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EasClientDeviceInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EasClientDeviceInformation {}
impl ::core::fmt::Debug for EasClientDeviceInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EasClientDeviceInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EasClientDeviceInformation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.ExchangeActiveSyncProvisioning.EasClientDeviceInformation;{54dfd981-1968-4ca3-b958-e595d16505eb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for EasClientDeviceInformation {
    type Vtable = IEasClientDeviceInformation_Vtbl;
    const IID: ::windows::core::GUID = <IEasClientDeviceInformation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EasClientDeviceInformation {
    const NAME: &'static str = "Windows.Security.ExchangeActiveSyncProvisioning.EasClientDeviceInformation";
}
impl ::core::convert::From<EasClientDeviceInformation> for ::windows::core::IUnknown {
    fn from(value: EasClientDeviceInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EasClientDeviceInformation> for ::windows::core::IUnknown {
    fn from(value: &EasClientDeviceInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EasClientDeviceInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EasClientDeviceInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EasClientDeviceInformation> for ::windows::core::IInspectable {
    fn from(value: EasClientDeviceInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EasClientDeviceInformation> for ::windows::core::IInspectable {
    fn from(value: &EasClientDeviceInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EasClientDeviceInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EasClientDeviceInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
#[repr(transparent)]
pub struct EasClientSecurityPolicy(::windows::core::IUnknown);
impl EasClientSecurityPolicy {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<EasClientSecurityPolicy, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
    pub fn RequireEncryption(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).RequireEncryption)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
    pub fn SetRequireEncryption(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRequireEncryption)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
    pub fn MinPasswordLength(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u8>::zeroed();
            (::windows::core::Interface::vtable(this).MinPasswordLength)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
    pub fn SetMinPasswordLength(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMinPasswordLength)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
    pub fn DisallowConvenienceLogon(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).DisallowConvenienceLogon)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
    pub fn SetDisallowConvenienceLogon(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDisallowConvenienceLogon)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
    pub fn MinPasswordComplexCharacters(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u8>::zeroed();
            (::windows::core::Interface::vtable(this).MinPasswordComplexCharacters)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
    pub fn SetMinPasswordComplexCharacters(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMinPasswordComplexCharacters)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PasswordExpiration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::TimeSpan>::zeroed();
            (::windows::core::Interface::vtable(this).PasswordExpiration)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPasswordExpiration<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPasswordExpiration)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
    pub fn PasswordHistory(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows::core::Interface::vtable(this).PasswordHistory)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
    pub fn SetPasswordHistory(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPasswordHistory)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
    pub fn MaxPasswordFailedAttempts(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u8>::zeroed();
            (::windows::core::Interface::vtable(this).MaxPasswordFailedAttempts)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
    pub fn SetMaxPasswordFailedAttempts(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMaxPasswordFailedAttempts)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxInactivityTimeLock(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::TimeSpan>::zeroed();
            (::windows::core::Interface::vtable(this).MaxInactivityTimeLock)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxInactivityTimeLock<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMaxInactivityTimeLock)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
    pub fn CheckCompliance(&self) -> ::windows::core::Result<EasComplianceResults> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).CheckCompliance)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EasComplianceResults>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ApplyAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EasComplianceResults>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).ApplyAsync)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<EasComplianceResults>>(result__)
        }
    }
}
impl ::core::clone::Clone for EasClientSecurityPolicy {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EasClientSecurityPolicy {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EasClientSecurityPolicy {}
impl ::core::fmt::Debug for EasClientSecurityPolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EasClientSecurityPolicy").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EasClientSecurityPolicy {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.ExchangeActiveSyncProvisioning.EasClientSecurityPolicy;{45b72362-dfba-4a9b-aced-6fe2adcb6420})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for EasClientSecurityPolicy {
    type Vtable = IEasClientSecurityPolicy_Vtbl;
    const IID: ::windows::core::GUID = <IEasClientSecurityPolicy as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EasClientSecurityPolicy {
    const NAME: &'static str = "Windows.Security.ExchangeActiveSyncProvisioning.EasClientSecurityPolicy";
}
impl ::core::convert::From<EasClientSecurityPolicy> for ::windows::core::IUnknown {
    fn from(value: EasClientSecurityPolicy) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EasClientSecurityPolicy> for ::windows::core::IUnknown {
    fn from(value: &EasClientSecurityPolicy) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EasClientSecurityPolicy {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EasClientSecurityPolicy {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EasClientSecurityPolicy> for ::windows::core::IInspectable {
    fn from(value: EasClientSecurityPolicy) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EasClientSecurityPolicy> for ::windows::core::IInspectable {
    fn from(value: &EasClientSecurityPolicy) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EasClientSecurityPolicy {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EasClientSecurityPolicy {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
#[repr(transparent)]
pub struct EasComplianceResults(::windows::core::IUnknown);
impl EasComplianceResults {
    #[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
    pub fn Compliant(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).Compliant)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
    pub fn RequireEncryptionResult(&self) -> ::windows::core::Result<EasRequireEncryptionResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<EasRequireEncryptionResult>::zeroed();
            (::windows::core::Interface::vtable(this).RequireEncryptionResult)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EasRequireEncryptionResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
    pub fn MinPasswordLengthResult(&self) -> ::windows::core::Result<EasMinPasswordLengthResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<EasMinPasswordLengthResult>::zeroed();
            (::windows::core::Interface::vtable(this).MinPasswordLengthResult)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EasMinPasswordLengthResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
    pub fn DisallowConvenienceLogonResult(&self) -> ::windows::core::Result<EasDisallowConvenienceLogonResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<EasDisallowConvenienceLogonResult>::zeroed();
            (::windows::core::Interface::vtable(this).DisallowConvenienceLogonResult)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EasDisallowConvenienceLogonResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
    pub fn MinPasswordComplexCharactersResult(&self) -> ::windows::core::Result<EasMinPasswordComplexCharactersResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<EasMinPasswordComplexCharactersResult>::zeroed();
            (::windows::core::Interface::vtable(this).MinPasswordComplexCharactersResult)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EasMinPasswordComplexCharactersResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
    pub fn PasswordExpirationResult(&self) -> ::windows::core::Result<EasPasswordExpirationResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<EasPasswordExpirationResult>::zeroed();
            (::windows::core::Interface::vtable(this).PasswordExpirationResult)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EasPasswordExpirationResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
    pub fn PasswordHistoryResult(&self) -> ::windows::core::Result<EasPasswordHistoryResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<EasPasswordHistoryResult>::zeroed();
            (::windows::core::Interface::vtable(this).PasswordHistoryResult)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EasPasswordHistoryResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
    pub fn MaxPasswordFailedAttemptsResult(&self) -> ::windows::core::Result<EasMaxPasswordFailedAttemptsResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<EasMaxPasswordFailedAttemptsResult>::zeroed();
            (::windows::core::Interface::vtable(this).MaxPasswordFailedAttemptsResult)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EasMaxPasswordFailedAttemptsResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
    pub fn MaxInactivityTimeLockResult(&self) -> ::windows::core::Result<EasMaxInactivityTimeLockResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<EasMaxInactivityTimeLockResult>::zeroed();
            (::windows::core::Interface::vtable(this).MaxInactivityTimeLockResult)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EasMaxInactivityTimeLockResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
    pub fn EncryptionProviderType(&self) -> ::windows::core::Result<EasEncryptionProviderType> {
        let this = &::windows::core::Interface::cast::<IEasComplianceResults2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<EasEncryptionProviderType>::zeroed();
            (::windows::core::Interface::vtable(this).EncryptionProviderType)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EasEncryptionProviderType>(result__)
        }
    }
}
impl ::core::clone::Clone for EasComplianceResults {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EasComplianceResults {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EasComplianceResults {}
impl ::core::fmt::Debug for EasComplianceResults {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EasComplianceResults").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EasComplianceResults {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.ExchangeActiveSyncProvisioning.EasComplianceResults;{463c299c-7f19-4c66-b403-cb45dd57a2b3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for EasComplianceResults {
    type Vtable = IEasComplianceResults_Vtbl;
    const IID: ::windows::core::GUID = <IEasComplianceResults as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EasComplianceResults {
    const NAME: &'static str = "Windows.Security.ExchangeActiveSyncProvisioning.EasComplianceResults";
}
impl ::core::convert::From<EasComplianceResults> for ::windows::core::IUnknown {
    fn from(value: EasComplianceResults) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EasComplianceResults> for ::windows::core::IUnknown {
    fn from(value: &EasComplianceResults) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EasComplianceResults {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EasComplianceResults {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EasComplianceResults> for ::windows::core::IInspectable {
    fn from(value: EasComplianceResults) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EasComplianceResults> for ::windows::core::IInspectable {
    fn from(value: &EasComplianceResults) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EasComplianceResults {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EasComplianceResults {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EasDisallowConvenienceLogonResult(pub i32);
impl EasDisallowConvenienceLogonResult {
    pub const NotEvaluated: Self = Self(0i32);
    pub const Compliant: Self = Self(1i32);
    pub const CanBeCompliant: Self = Self(2i32);
    pub const RequestedPolicyIsStricter: Self = Self(3i32);
}
impl ::core::marker::Copy for EasDisallowConvenienceLogonResult {}
impl ::core::clone::Clone for EasDisallowConvenienceLogonResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EasDisallowConvenienceLogonResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EasDisallowConvenienceLogonResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for EasDisallowConvenienceLogonResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EasDisallowConvenienceLogonResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EasDisallowConvenienceLogonResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.ExchangeActiveSyncProvisioning.EasDisallowConvenienceLogonResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EasEncryptionProviderType(pub i32);
impl EasEncryptionProviderType {
    pub const NotEvaluated: Self = Self(0i32);
    pub const WindowsEncryption: Self = Self(1i32);
    pub const OtherEncryption: Self = Self(2i32);
}
impl ::core::marker::Copy for EasEncryptionProviderType {}
impl ::core::clone::Clone for EasEncryptionProviderType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EasEncryptionProviderType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EasEncryptionProviderType {
    type Abi = Self;
}
impl ::core::fmt::Debug for EasEncryptionProviderType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EasEncryptionProviderType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EasEncryptionProviderType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.ExchangeActiveSyncProvisioning.EasEncryptionProviderType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EasMaxInactivityTimeLockResult(pub i32);
impl EasMaxInactivityTimeLockResult {
    pub const NotEvaluated: Self = Self(0i32);
    pub const Compliant: Self = Self(1i32);
    pub const CanBeCompliant: Self = Self(2i32);
    pub const RequestedPolicyIsStricter: Self = Self(3i32);
    pub const InvalidParameter: Self = Self(4i32);
}
impl ::core::marker::Copy for EasMaxInactivityTimeLockResult {}
impl ::core::clone::Clone for EasMaxInactivityTimeLockResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EasMaxInactivityTimeLockResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EasMaxInactivityTimeLockResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for EasMaxInactivityTimeLockResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EasMaxInactivityTimeLockResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EasMaxInactivityTimeLockResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.ExchangeActiveSyncProvisioning.EasMaxInactivityTimeLockResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EasMaxPasswordFailedAttemptsResult(pub i32);
impl EasMaxPasswordFailedAttemptsResult {
    pub const NotEvaluated: Self = Self(0i32);
    pub const Compliant: Self = Self(1i32);
    pub const CanBeCompliant: Self = Self(2i32);
    pub const RequestedPolicyIsStricter: Self = Self(3i32);
    pub const InvalidParameter: Self = Self(4i32);
}
impl ::core::marker::Copy for EasMaxPasswordFailedAttemptsResult {}
impl ::core::clone::Clone for EasMaxPasswordFailedAttemptsResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EasMaxPasswordFailedAttemptsResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EasMaxPasswordFailedAttemptsResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for EasMaxPasswordFailedAttemptsResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EasMaxPasswordFailedAttemptsResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EasMaxPasswordFailedAttemptsResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.ExchangeActiveSyncProvisioning.EasMaxPasswordFailedAttemptsResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EasMinPasswordComplexCharactersResult(pub i32);
impl EasMinPasswordComplexCharactersResult {
    pub const NotEvaluated: Self = Self(0i32);
    pub const Compliant: Self = Self(1i32);
    pub const CanBeCompliant: Self = Self(2i32);
    pub const RequestedPolicyIsStricter: Self = Self(3i32);
    pub const RequestedPolicyNotEnforceable: Self = Self(4i32);
    pub const InvalidParameter: Self = Self(5i32);
    pub const CurrentUserHasBlankPassword: Self = Self(6i32);
    pub const AdminsHaveBlankPassword: Self = Self(7i32);
    pub const UserCannotChangePassword: Self = Self(8i32);
    pub const AdminsCannotChangePassword: Self = Self(9i32);
    pub const LocalControlledUsersCannotChangePassword: Self = Self(10i32);
    pub const ConnectedAdminsProviderPolicyIsWeak: Self = Self(11i32);
    pub const ConnectedUserProviderPolicyIsWeak: Self = Self(12i32);
    pub const ChangeConnectedAdminsPassword: Self = Self(13i32);
    pub const ChangeConnectedUserPassword: Self = Self(14i32);
}
impl ::core::marker::Copy for EasMinPasswordComplexCharactersResult {}
impl ::core::clone::Clone for EasMinPasswordComplexCharactersResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EasMinPasswordComplexCharactersResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EasMinPasswordComplexCharactersResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for EasMinPasswordComplexCharactersResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EasMinPasswordComplexCharactersResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EasMinPasswordComplexCharactersResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.ExchangeActiveSyncProvisioning.EasMinPasswordComplexCharactersResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EasMinPasswordLengthResult(pub i32);
impl EasMinPasswordLengthResult {
    pub const NotEvaluated: Self = Self(0i32);
    pub const Compliant: Self = Self(1i32);
    pub const CanBeCompliant: Self = Self(2i32);
    pub const RequestedPolicyIsStricter: Self = Self(3i32);
    pub const RequestedPolicyNotEnforceable: Self = Self(4i32);
    pub const InvalidParameter: Self = Self(5i32);
    pub const CurrentUserHasBlankPassword: Self = Self(6i32);
    pub const AdminsHaveBlankPassword: Self = Self(7i32);
    pub const UserCannotChangePassword: Self = Self(8i32);
    pub const AdminsCannotChangePassword: Self = Self(9i32);
    pub const LocalControlledUsersCannotChangePassword: Self = Self(10i32);
    pub const ConnectedAdminsProviderPolicyIsWeak: Self = Self(11i32);
    pub const ConnectedUserProviderPolicyIsWeak: Self = Self(12i32);
    pub const ChangeConnectedAdminsPassword: Self = Self(13i32);
    pub const ChangeConnectedUserPassword: Self = Self(14i32);
}
impl ::core::marker::Copy for EasMinPasswordLengthResult {}
impl ::core::clone::Clone for EasMinPasswordLengthResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EasMinPasswordLengthResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EasMinPasswordLengthResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for EasMinPasswordLengthResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EasMinPasswordLengthResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EasMinPasswordLengthResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.ExchangeActiveSyncProvisioning.EasMinPasswordLengthResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EasPasswordExpirationResult(pub i32);
impl EasPasswordExpirationResult {
    pub const NotEvaluated: Self = Self(0i32);
    pub const Compliant: Self = Self(1i32);
    pub const CanBeCompliant: Self = Self(2i32);
    pub const RequestedPolicyIsStricter: Self = Self(3i32);
    pub const RequestedExpirationIncompatible: Self = Self(4i32);
    pub const InvalidParameter: Self = Self(5i32);
    pub const UserCannotChangePassword: Self = Self(6i32);
    pub const AdminsCannotChangePassword: Self = Self(7i32);
    pub const LocalControlledUsersCannotChangePassword: Self = Self(8i32);
}
impl ::core::marker::Copy for EasPasswordExpirationResult {}
impl ::core::clone::Clone for EasPasswordExpirationResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EasPasswordExpirationResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EasPasswordExpirationResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for EasPasswordExpirationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EasPasswordExpirationResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EasPasswordExpirationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.ExchangeActiveSyncProvisioning.EasPasswordExpirationResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EasPasswordHistoryResult(pub i32);
impl EasPasswordHistoryResult {
    pub const NotEvaluated: Self = Self(0i32);
    pub const Compliant: Self = Self(1i32);
    pub const CanBeCompliant: Self = Self(2i32);
    pub const RequestedPolicyIsStricter: Self = Self(3i32);
    pub const InvalidParameter: Self = Self(4i32);
}
impl ::core::marker::Copy for EasPasswordHistoryResult {}
impl ::core::clone::Clone for EasPasswordHistoryResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EasPasswordHistoryResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EasPasswordHistoryResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for EasPasswordHistoryResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EasPasswordHistoryResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EasPasswordHistoryResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.ExchangeActiveSyncProvisioning.EasPasswordHistoryResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EasRequireEncryptionResult(pub i32);
impl EasRequireEncryptionResult {
    pub const NotEvaluated: Self = Self(0i32);
    pub const Compliant: Self = Self(1i32);
    pub const CanBeCompliant: Self = Self(2i32);
    pub const NotProvisionedOnAllVolumes: Self = Self(3i32);
    pub const DeFixedDataNotSupported: Self = Self(4i32);
    pub const FixedDataNotSupported: Self = Self(4i32);
    pub const DeHardwareNotCompliant: Self = Self(5i32);
    pub const HardwareNotCompliant: Self = Self(5i32);
    pub const DeWinReNotConfigured: Self = Self(6i32);
    pub const LockNotConfigured: Self = Self(6i32);
    pub const DeProtectionSuspended: Self = Self(7i32);
    pub const ProtectionSuspended: Self = Self(7i32);
    pub const DeOsVolumeNotProtected: Self = Self(8i32);
    pub const OsVolumeNotProtected: Self = Self(8i32);
    pub const DeProtectionNotYetEnabled: Self = Self(9i32);
    pub const ProtectionNotYetEnabled: Self = Self(9i32);
    pub const NoFeatureLicense: Self = Self(10i32);
    pub const OsNotProtected: Self = Self(11i32);
    pub const UnexpectedFailure: Self = Self(12i32);
}
impl ::core::marker::Copy for EasRequireEncryptionResult {}
impl ::core::clone::Clone for EasRequireEncryptionResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EasRequireEncryptionResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EasRequireEncryptionResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for EasRequireEncryptionResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EasRequireEncryptionResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EasRequireEncryptionResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.ExchangeActiveSyncProvisioning.EasRequireEncryptionResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEasClientDeviceInformation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEasClientDeviceInformation {
    type Vtable = IEasClientDeviceInformation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x54dfd981_1968_4ca3_b958_e595d16505eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEasClientDeviceInformation_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub OperatingSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub FriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SystemManufacturer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SystemProductName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SystemSku: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEasClientDeviceInformation2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEasClientDeviceInformation2 {
    type Vtable = IEasClientDeviceInformation2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xffb35923_bb26_4d6a_81bc_165aee0ad754);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEasClientDeviceInformation2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub SystemHardwareVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SystemFirmwareVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEasClientSecurityPolicy(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEasClientSecurityPolicy {
    type Vtable = IEasClientSecurityPolicy_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x45b72362_dfba_4a9b_aced_6fe2adcb6420);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEasClientSecurityPolicy_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub RequireEncryption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetRequireEncryption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub MinPasswordLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub SetMinPasswordLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT,
    pub DisallowConvenienceLogon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetDisallowConvenienceLogon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub MinPasswordComplexCharacters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub SetMinPasswordComplexCharacters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PasswordExpiration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PasswordExpiration: usize,
    #[cfg(feature = "Foundation")]
    pub SetPasswordExpiration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPasswordExpiration: usize,
    pub PasswordHistory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetPasswordHistory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub MaxPasswordFailedAttempts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub SetMaxPasswordFailedAttempts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MaxInactivityTimeLock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxInactivityTimeLock: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxInactivityTimeLock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxInactivityTimeLock: usize,
    pub CheckCompliance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ApplyAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ApplyAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEasComplianceResults(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEasComplianceResults {
    type Vtable = IEasComplianceResults_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x463c299c_7f19_4c66_b403_cb45dd57a2b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEasComplianceResults_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Compliant: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub RequireEncryptionResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EasRequireEncryptionResult) -> ::windows::core::HRESULT,
    pub MinPasswordLengthResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EasMinPasswordLengthResult) -> ::windows::core::HRESULT,
    pub DisallowConvenienceLogonResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EasDisallowConvenienceLogonResult) -> ::windows::core::HRESULT,
    pub MinPasswordComplexCharactersResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EasMinPasswordComplexCharactersResult) -> ::windows::core::HRESULT,
    pub PasswordExpirationResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EasPasswordExpirationResult) -> ::windows::core::HRESULT,
    pub PasswordHistoryResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EasPasswordHistoryResult) -> ::windows::core::HRESULT,
    pub MaxPasswordFailedAttemptsResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EasMaxPasswordFailedAttemptsResult) -> ::windows::core::HRESULT,
    pub MaxInactivityTimeLockResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EasMaxInactivityTimeLockResult) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEasComplianceResults2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEasComplianceResults2 {
    type Vtable = IEasComplianceResults2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2fbe60c9_1aa8_47f5_88bb_cb3ef0bffb15);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEasComplianceResults2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub EncryptionProviderType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EasEncryptionProviderType) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
