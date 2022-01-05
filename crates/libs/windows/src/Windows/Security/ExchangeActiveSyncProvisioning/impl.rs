#[cfg(feature = "implement_exclusive")]
pub trait IEasClientDeviceInformationImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn OperatingSystem(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SystemManufacturer(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SystemProductName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SystemSku(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEasClientDeviceInformation2Impl: Sized + IEasClientDeviceInformationImpl {
    fn SystemHardwareVersion(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SystemFirmwareVersion(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEasClientSecurityPolicyImpl: Sized {
    fn RequireEncryption(&self) -> ::windows::core::Result<bool>;
    fn SetRequireEncryption(&self, value: bool) -> ::windows::core::Result<()>;
    fn MinPasswordLength(&self) -> ::windows::core::Result<u8>;
    fn SetMinPasswordLength(&self, value: u8) -> ::windows::core::Result<()>;
    fn DisallowConvenienceLogon(&self) -> ::windows::core::Result<bool>;
    fn SetDisallowConvenienceLogon(&self, value: bool) -> ::windows::core::Result<()>;
    fn MinPasswordComplexCharacters(&self) -> ::windows::core::Result<u8>;
    fn SetMinPasswordComplexCharacters(&self, value: u8) -> ::windows::core::Result<()>;
    fn PasswordExpiration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetPasswordExpiration(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn PasswordHistory(&self) -> ::windows::core::Result<u32>;
    fn SetPasswordHistory(&self, value: u32) -> ::windows::core::Result<()>;
    fn MaxPasswordFailedAttempts(&self) -> ::windows::core::Result<u8>;
    fn SetMaxPasswordFailedAttempts(&self, value: u8) -> ::windows::core::Result<()>;
    fn MaxInactivityTimeLock(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetMaxInactivityTimeLock(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn CheckCompliance(&self) -> ::windows::core::Result<EasComplianceResults>;
    fn ApplyAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EasComplianceResults>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEasComplianceResultsImpl: Sized {
    fn Compliant(&self) -> ::windows::core::Result<bool>;
    fn RequireEncryptionResult(&self) -> ::windows::core::Result<EasRequireEncryptionResult>;
    fn MinPasswordLengthResult(&self) -> ::windows::core::Result<EasMinPasswordLengthResult>;
    fn DisallowConvenienceLogonResult(&self) -> ::windows::core::Result<EasDisallowConvenienceLogonResult>;
    fn MinPasswordComplexCharactersResult(&self) -> ::windows::core::Result<EasMinPasswordComplexCharactersResult>;
    fn PasswordExpirationResult(&self) -> ::windows::core::Result<EasPasswordExpirationResult>;
    fn PasswordHistoryResult(&self) -> ::windows::core::Result<EasPasswordHistoryResult>;
    fn MaxPasswordFailedAttemptsResult(&self) -> ::windows::core::Result<EasMaxPasswordFailedAttemptsResult>;
    fn MaxInactivityTimeLockResult(&self) -> ::windows::core::Result<EasMaxInactivityTimeLockResult>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEasComplianceResults2Impl: Sized + IEasComplianceResultsImpl {
    fn EncryptionProviderType(&self) -> ::windows::core::Result<EasEncryptionProviderType>;
}
