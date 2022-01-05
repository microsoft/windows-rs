#[cfg(feature = "implement_exclusive")]
pub trait ICredentialPickerOptionsImpl: Sized {
    fn SetCaption(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Caption(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetMessage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Message(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetErrorCode(&self, value: u32) -> ::windows::core::Result<()>;
    fn ErrorCode(&self) -> ::windows::core::Result<u32>;
    fn SetTargetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAuthenticationProtocol(&self, value: AuthenticationProtocol) -> ::windows::core::Result<()>;
    fn AuthenticationProtocol(&self) -> ::windows::core::Result<AuthenticationProtocol>;
    fn SetCustomAuthenticationProtocol(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn CustomAuthenticationProtocol(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPreviousCredential(&self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn PreviousCredential(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn SetAlwaysDisplayDialog(&self, value: bool) -> ::windows::core::Result<()>;
    fn AlwaysDisplayDialog(&self) -> ::windows::core::Result<bool>;
    fn SetCallerSavesCredential(&self, value: bool) -> ::windows::core::Result<()>;
    fn CallerSavesCredential(&self) -> ::windows::core::Result<bool>;
    fn SetCredentialSaveOption(&self, value: CredentialSaveOption) -> ::windows::core::Result<()>;
    fn CredentialSaveOption(&self) -> ::windows::core::Result<CredentialSaveOption>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICredentialPickerResultsImpl: Sized {
    fn ErrorCode(&self) -> ::windows::core::Result<u32>;
    fn CredentialSaveOption(&self) -> ::windows::core::Result<CredentialSaveOption>;
    fn CredentialSaved(&self) -> ::windows::core::Result<bool>;
    fn Credential(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn CredentialDomainName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CredentialUserName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CredentialPassword(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICredentialPickerStaticsImpl: Sized {
    fn PickWithOptionsAsync(&self, options: &::core::option::Option<CredentialPickerOptions>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<CredentialPickerResults>>;
    fn PickWithMessageAsync(&self, targetname: &::windows::core::HSTRING, message: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<CredentialPickerResults>>;
    fn PickWithCaptionAsync(&self, targetname: &::windows::core::HSTRING, message: &::windows::core::HSTRING, caption: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<CredentialPickerResults>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserConsentVerifierStaticsImpl: Sized {
    fn CheckAvailabilityAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<UserConsentVerifierAvailability>>;
    fn RequestVerificationAsync(&self, message: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<UserConsentVerificationResult>>;
}
