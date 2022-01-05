#[cfg(feature = "implement_exclusive")]
pub trait ICredentialFactoryImpl: Sized {
    fn CreatePasswordCredential(&self, resource: &::windows::core::HSTRING, username: &::windows::core::HSTRING, password: &::windows::core::HSTRING) -> ::windows::core::Result<PasswordCredential>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyCredentialImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RetrievePublicKeyWithDefaultBlobType(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn RetrievePublicKeyWithBlobType(&self, blobtype: super::Cryptography::Core::CryptographicPublicKeyBlobType) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn RequestSignAsync(&self, data: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<KeyCredentialOperationResult>>;
    fn GetAttestationAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<KeyCredentialAttestationResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyCredentialAttestationResultImpl: Sized {
    fn CertificateChainBuffer(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn AttestationBuffer(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn Status(&self) -> ::windows::core::Result<KeyCredentialAttestationStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyCredentialManagerStaticsImpl: Sized {
    fn IsSupportedAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RenewAttestationAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn RequestCreateAsync(&self, name: &::windows::core::HSTRING, option: KeyCredentialCreationOption) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<KeyCredentialRetrievalResult>>;
    fn OpenAsync(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<KeyCredentialRetrievalResult>>;
    fn DeleteAsync(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyCredentialOperationResultImpl: Sized {
    fn Result(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn Status(&self) -> ::windows::core::Result<KeyCredentialStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyCredentialRetrievalResultImpl: Sized {
    fn Credential(&self) -> ::windows::core::Result<KeyCredential>;
    fn Status(&self) -> ::windows::core::Result<KeyCredentialStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPasswordCredentialImpl: Sized {
    fn Resource(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetResource(&self, resource: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn UserName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetUserName(&self, username: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Password(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPassword(&self, password: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn RetrievePassword(&self) -> ::windows::core::Result<()>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPasswordVaultImpl: Sized {
    fn Add(&self, credential: &::core::option::Option<PasswordCredential>) -> ::windows::core::Result<()>;
    fn Remove(&self, credential: &::core::option::Option<PasswordCredential>) -> ::windows::core::Result<()>;
    fn Retrieve(&self, resource: &::windows::core::HSTRING, username: &::windows::core::HSTRING) -> ::windows::core::Result<PasswordCredential>;
    fn FindAllByResource(&self, resource: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PasswordCredential>>;
    fn FindAllByUserName(&self, username: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PasswordCredential>>;
    fn RetrieveAll(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PasswordCredential>>;
}
pub trait IWebAccountImpl: Sized {
    fn WebAccountProvider(&self) -> ::windows::core::Result<WebAccountProvider>;
    fn UserName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn State(&self) -> ::windows::core::Result<WebAccountState>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAccount2Impl: Sized + IWebAccountImpl {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>;
    fn GetPictureAsync(&self, desizedsize: WebAccountPictureSize) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>>;
    fn SignOutAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SignOutWithClientIdAsync(&self, clientid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAccountFactoryImpl: Sized {
    fn CreateWebAccount(&self, webaccountprovider: &::core::option::Option<WebAccountProvider>, username: &::windows::core::HSTRING, state: WebAccountState) -> ::windows::core::Result<WebAccount>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAccountProviderImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IconUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAccountProvider2Impl: Sized + IWebAccountProviderImpl {
    fn DisplayPurpose(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Authority(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAccountProvider3Impl: Sized + IWebAccountProviderImpl + IWebAccountProvider2Impl {
    fn User(&self) -> ::windows::core::Result<super::super::System::User>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAccountProvider4Impl: Sized {
    fn IsSystemProvider(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAccountProviderFactoryImpl: Sized {
    fn CreateWebAccountProvider(&self, id: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING, iconuri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<WebAccountProvider>;
}
