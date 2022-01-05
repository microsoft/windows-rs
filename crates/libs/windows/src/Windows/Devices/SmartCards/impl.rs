#[cfg(feature = "implement_exclusive")]
pub trait ICardAddedEventArgsImpl: Sized {
    fn SmartCard(&self) -> ::windows::core::Result<SmartCard>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICardRemovedEventArgsImpl: Sized {
    fn SmartCard(&self) -> ::windows::core::Result<SmartCard>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IKnownSmartCardAppletIdsImpl: Sized {
    fn PaymentSystemEnvironment(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn ProximityPaymentSystemEnvironment(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardImpl: Sized {
    fn Reader(&self) -> ::windows::core::Result<SmartCardReader>;
    fn GetStatusAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmartCardStatus>>;
    fn GetAnswerToResetAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardAppletIdGroupImpl: Sized {
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AppletIds(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Storage::Streams::IBuffer>>;
    fn SmartCardEmulationCategory(&self) -> ::windows::core::Result<SmartCardEmulationCategory>;
    fn SetSmartCardEmulationCategory(&self, value: SmartCardEmulationCategory) -> ::windows::core::Result<()>;
    fn SmartCardEmulationType(&self) -> ::windows::core::Result<SmartCardEmulationType>;
    fn SetSmartCardEmulationType(&self, value: SmartCardEmulationType) -> ::windows::core::Result<()>;
    fn AutomaticEnablement(&self) -> ::windows::core::Result<bool>;
    fn SetAutomaticEnablement(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardAppletIdGroup2Impl: Sized {
    fn Logo(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn SetLogo(&self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
    fn SecureUserAuthenticationRequired(&self) -> ::windows::core::Result<bool>;
    fn SetSecureUserAuthenticationRequired(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardAppletIdGroupFactoryImpl: Sized {
    fn Create(&self, displayname: &::windows::core::HSTRING, appletids: &::core::option::Option<super::super::Foundation::Collections::IVector<super::super::Storage::Streams::IBuffer>>, emulationcategory: SmartCardEmulationCategory, emulationtype: SmartCardEmulationType) -> ::windows::core::Result<SmartCardAppletIdGroup>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardAppletIdGroupRegistrationImpl: Sized {
    fn ActivationPolicy(&self) -> ::windows::core::Result<SmartCardAppletIdGroupActivationPolicy>;
    fn AppletIdGroup(&self) -> ::windows::core::Result<SmartCardAppletIdGroup>;
    fn RequestActivationPolicyChangeAsync(&self, policy: SmartCardAppletIdGroupActivationPolicy) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmartCardActivationPolicyChangeResult>>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SetAutomaticResponseApdusAsync(&self, apdus: &::core::option::Option<super::super::Foundation::Collections::IIterable<SmartCardAutomaticResponseApdu>>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardAppletIdGroupRegistration2Impl: Sized {
    fn SmartCardReaderId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPropertiesAsync(&self, props: &::core::option::Option<super::super::Foundation::Collections::ValueSet>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardAppletIdGroupStaticsImpl: Sized {
    fn MaxAppletIds(&self) -> ::windows::core::Result<u16>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardAutomaticResponseApduImpl: Sized {
    fn CommandApdu(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn SetCommandApdu(&self, value: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn CommandApduBitMask(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn SetCommandApduBitMask(&self, value: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn ShouldMatchLength(&self) -> ::windows::core::Result<bool>;
    fn SetShouldMatchLength(&self, value: bool) -> ::windows::core::Result<()>;
    fn AppletId(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn SetAppletId(&self, value: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn ResponseApdu(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn SetResponseApdu(&self, value: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardAutomaticResponseApdu2Impl: Sized {
    fn InputState(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn SetInputState(&self, value: &::core::option::Option<super::super::Foundation::IReference<u32>>) -> ::windows::core::Result<()>;
    fn OutputState(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn SetOutputState(&self, value: &::core::option::Option<super::super::Foundation::IReference<u32>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardAutomaticResponseApdu3Impl: Sized {
    fn AllowWhenCryptogramGeneratorNotPrepared(&self) -> ::windows::core::Result<bool>;
    fn SetAllowWhenCryptogramGeneratorNotPrepared(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardAutomaticResponseApduFactoryImpl: Sized {
    fn Create(&self, commandapdu: &::core::option::Option<super::super::Storage::Streams::IBuffer>, responseapdu: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<SmartCardAutomaticResponseApdu>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISmartCardChallengeContextImpl: Sized + IClosableImpl {
    fn Challenge(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn VerifyResponseAsync(&self, response: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn ProvisionAsync(&self, response: &::core::option::Option<super::super::Storage::Streams::IBuffer>, formatcard: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ProvisionAsyncWithNewCardId(&self, response: &::core::option::Option<super::super::Storage::Streams::IBuffer>, formatcard: bool, newcardid: &::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ChangeAdministrativeKeyAsync(&self, response: &::core::option::Option<super::super::Storage::Streams::IBuffer>, newadministrativekey: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardConnectImpl: Sized {
    fn ConnectAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmartCardConnection>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISmartCardConnectionImpl: Sized + IClosableImpl {
    fn TransmitAsync(&self, command: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardCryptogramGeneratorImpl: Sized {
    fn SupportedCryptogramMaterialTypes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramMaterialType>>;
    fn SupportedCryptogramAlgorithms(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramAlgorithm>>;
    fn SupportedCryptogramMaterialPackageFormats(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramMaterialPackageFormat>>;
    fn SupportedCryptogramMaterialPackageConfirmationResponseFormats(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramMaterialPackageConfirmationResponseFormat>>;
    fn SupportedSmartCardCryptogramStorageKeyCapabilities(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramStorageKeyCapabilities>>;
    fn DeleteCryptogramMaterialStorageKeyAsync(&self, storagekeyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>>;
    fn CreateCryptogramMaterialStorageKeyAsync(&self, promptingbehavior: SmartCardUnlockPromptingBehavior, storagekeyname: &::windows::core::HSTRING, algorithm: SmartCardCryptogramStorageKeyAlgorithm, capabilities: SmartCardCryptogramStorageKeyCapabilities) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>>;
    fn RequestCryptogramMaterialStorageKeyInfoAsync(&self, promptingbehavior: SmartCardUnlockPromptingBehavior, storagekeyname: &::windows::core::HSTRING, format: super::super::Security::Cryptography::Core::CryptographicPublicKeyBlobType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramStorageKeyInfo>>;
    fn ImportCryptogramMaterialPackageAsync(&self, format: SmartCardCryptogramMaterialPackageFormat, storagekeyname: &::windows::core::HSTRING, materialpackagename: &::windows::core::HSTRING, cryptogrammaterialpackage: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>>;
    fn TryProvePossessionOfCryptogramMaterialPackageAsync(&self, promptingbehavior: SmartCardUnlockPromptingBehavior, responseformat: SmartCardCryptogramMaterialPackageConfirmationResponseFormat, materialpackagename: &::windows::core::HSTRING, materialname: &::windows::core::HSTRING, challenge: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramMaterialPossessionProof>>;
    fn RequestUnlockCryptogramMaterialForUseAsync(&self, promptingbehavior: SmartCardUnlockPromptingBehavior) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>>;
    fn DeleteCryptogramMaterialPackageAsync(&self, materialpackagename: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardCryptogramGenerator2Impl: Sized {
    fn ValidateRequestApduAsync(&self, promptingbehavior: SmartCardUnlockPromptingBehavior, apdutovalidate: &::core::option::Option<super::super::Storage::Streams::IBuffer>, cryptogramplacementsteps: &::core::option::Option<super::super::Foundation::Collections::IIterable<SmartCardCryptogramPlacementStep>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>>;
    fn GetAllCryptogramStorageKeyCharacteristicsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult>>;
    fn GetAllCryptogramMaterialPackageCharacteristicsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult>>;
    fn GetAllCryptogramMaterialPackageCharacteristicsWithStorageKeyAsync(&self, storagekeyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult>>;
    fn GetAllCryptogramMaterialCharacteristicsAsync(&self, promptingbehavior: SmartCardUnlockPromptingBehavior, materialpackagename: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardCryptogramGeneratorStaticsImpl: Sized {
    fn GetSmartCardCryptogramGeneratorAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGenerator>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardCryptogramGeneratorStatics2Impl: Sized {
    fn IsSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResultImpl: Sized {
    fn OperationStatus(&self) -> ::windows::core::Result<SmartCardCryptogramGeneratorOperationStatus>;
    fn Characteristics(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramMaterialCharacteristics>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResultImpl: Sized {
    fn OperationStatus(&self) -> ::windows::core::Result<SmartCardCryptogramGeneratorOperationStatus>;
    fn Characteristics(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramMaterialPackageCharacteristics>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResultImpl: Sized {
    fn OperationStatus(&self) -> ::windows::core::Result<SmartCardCryptogramGeneratorOperationStatus>;
    fn Characteristics(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramStorageKeyCharacteristics>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardCryptogramMaterialCharacteristicsImpl: Sized {
    fn MaterialName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AllowedAlgorithms(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramAlgorithm>>;
    fn AllowedProofOfPossessionAlgorithms(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramMaterialPackageConfirmationResponseFormat>>;
    fn AllowedValidations(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SmartCardCryptogramAlgorithm>>;
    fn MaterialType(&self) -> ::windows::core::Result<SmartCardCryptogramMaterialType>;
    fn ProtectionMethod(&self) -> ::windows::core::Result<SmartCardCryptogramMaterialProtectionMethod>;
    fn ProtectionVersion(&self) -> ::windows::core::Result<i32>;
    fn MaterialLength(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardCryptogramMaterialPackageCharacteristicsImpl: Sized {
    fn PackageName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn StorageKeyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DateImported(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn PackageFormat(&self) -> ::windows::core::Result<SmartCardCryptogramMaterialPackageFormat>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardCryptogramMaterialPossessionProofImpl: Sized {
    fn OperationStatus(&self) -> ::windows::core::Result<SmartCardCryptogramGeneratorOperationStatus>;
    fn Proof(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardCryptogramPlacementStepImpl: Sized {
    fn Algorithm(&self) -> ::windows::core::Result<SmartCardCryptogramAlgorithm>;
    fn SetAlgorithm(&self, value: SmartCardCryptogramAlgorithm) -> ::windows::core::Result<()>;
    fn SourceData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn SetSourceData(&self, value: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn CryptogramMaterialPackageName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCryptogramMaterialPackageName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn CryptogramMaterialName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCryptogramMaterialName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TemplateOffset(&self) -> ::windows::core::Result<i32>;
    fn SetTemplateOffset(&self, value: i32) -> ::windows::core::Result<()>;
    fn CryptogramOffset(&self) -> ::windows::core::Result<i32>;
    fn SetCryptogramOffset(&self, value: i32) -> ::windows::core::Result<()>;
    fn CryptogramLength(&self) -> ::windows::core::Result<i32>;
    fn SetCryptogramLength(&self, value: i32) -> ::windows::core::Result<()>;
    fn CryptogramPlacementOptions(&self) -> ::windows::core::Result<SmartCardCryptogramPlacementOptions>;
    fn SetCryptogramPlacementOptions(&self, value: SmartCardCryptogramPlacementOptions) -> ::windows::core::Result<()>;
    fn ChainedOutputStep(&self) -> ::windows::core::Result<SmartCardCryptogramPlacementStep>;
    fn SetChainedOutputStep(&self, value: &::core::option::Option<SmartCardCryptogramPlacementStep>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardCryptogramStorageKeyCharacteristicsImpl: Sized {
    fn StorageKeyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DateCreated(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn Algorithm(&self) -> ::windows::core::Result<SmartCardCryptogramStorageKeyAlgorithm>;
    fn Capabilities(&self) -> ::windows::core::Result<SmartCardCryptogramStorageKeyCapabilities>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardCryptogramStorageKeyInfoImpl: Sized {
    fn OperationStatus(&self) -> ::windows::core::Result<SmartCardCryptogramGeneratorOperationStatus>;
    fn PublicKeyBlobType(&self) -> ::windows::core::Result<super::super::Security::Cryptography::Core::CryptographicPublicKeyBlobType>;
    fn PublicKey(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn AttestationStatus(&self) -> ::windows::core::Result<SmartCardCryptographicKeyAttestationStatus>;
    fn Attestation(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn AttestationCertificateChain(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn Capabilities(&self) -> ::windows::core::Result<SmartCardCryptogramStorageKeyCapabilities>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardCryptogramStorageKeyInfo2Impl: Sized {
    fn OperationalRequirements(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardEmulatorImpl: Sized {
    fn EnablementPolicy(&self) -> ::windows::core::Result<SmartCardEmulatorEnablementPolicy>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardEmulator2Impl: Sized {
    fn ApduReceived(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<SmartCardEmulator, SmartCardEmulatorApduReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveApduReceived(&self, value: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ConnectionDeactivated(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<SmartCardEmulator, SmartCardEmulatorConnectionDeactivatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveConnectionDeactivated(&self, value: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn IsHostCardEmulationSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardEmulatorApduReceivedEventArgsImpl: Sized {
    fn CommandApdu(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn ConnectionProperties(&self) -> ::windows::core::Result<SmartCardEmulatorConnectionProperties>;
    fn TryRespondAsync(&self, responseapdu: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn AutomaticResponseStatus(&self) -> ::windows::core::Result<SmartCardAutomaticResponseStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardEmulatorApduReceivedEventArgs2Impl: Sized {
    fn State(&self) -> ::windows::core::Result<u32>;
    fn TryRespondWithStateAsync(&self, responseapdu: &::core::option::Option<super::super::Storage::Streams::IBuffer>, nextstate: &::core::option::Option<super::super::Foundation::IReference<u32>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardEmulatorApduReceivedEventArgsWithCryptogramsImpl: Sized {
    fn TryRespondWithCryptogramsAsync(&self, responsetemplate: &::core::option::Option<super::super::Storage::Streams::IBuffer>, cryptogramplacementsteps: &::core::option::Option<super::super::Foundation::Collections::IIterable<SmartCardCryptogramPlacementStep>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>>;
    fn TryRespondWithCryptogramsAndStateAsync(&self, responsetemplate: &::core::option::Option<super::super::Storage::Streams::IBuffer>, cryptogramplacementsteps: &::core::option::Option<super::super::Foundation::Collections::IIterable<SmartCardCryptogramPlacementStep>>, nextstate: &::core::option::Option<super::super::Foundation::IReference<u32>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmartCardCryptogramGeneratorOperationStatus>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardEmulatorConnectionDeactivatedEventArgsImpl: Sized {
    fn ConnectionProperties(&self) -> ::windows::core::Result<SmartCardEmulatorConnectionProperties>;
    fn Reason(&self) -> ::windows::core::Result<SmartCardEmulatorConnectionDeactivatedReason>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardEmulatorConnectionPropertiesImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Source(&self) -> ::windows::core::Result<SmartCardEmulatorConnectionSource>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardEmulatorStaticsImpl: Sized {
    fn GetDefaultAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmartCardEmulator>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardEmulatorStatics2Impl: Sized {
    fn GetAppletIdGroupRegistrationsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<SmartCardAppletIdGroupRegistration>>>;
    fn RegisterAppletIdGroupAsync(&self, appletidgroup: &::core::option::Option<SmartCardAppletIdGroup>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmartCardAppletIdGroupRegistration>>;
    fn UnregisterAppletIdGroupAsync(&self, registration: &::core::option::Option<SmartCardAppletIdGroupRegistration>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn MaxAppletIdGroupRegistrations(&self) -> ::windows::core::Result<u16>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardEmulatorStatics3Impl: Sized {
    fn IsSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardPinPolicyImpl: Sized {
    fn MinLength(&self) -> ::windows::core::Result<u32>;
    fn SetMinLength(&self, value: u32) -> ::windows::core::Result<()>;
    fn MaxLength(&self) -> ::windows::core::Result<u32>;
    fn SetMaxLength(&self, value: u32) -> ::windows::core::Result<()>;
    fn UppercaseLetters(&self) -> ::windows::core::Result<SmartCardPinCharacterPolicyOption>;
    fn SetUppercaseLetters(&self, value: SmartCardPinCharacterPolicyOption) -> ::windows::core::Result<()>;
    fn LowercaseLetters(&self) -> ::windows::core::Result<SmartCardPinCharacterPolicyOption>;
    fn SetLowercaseLetters(&self, value: SmartCardPinCharacterPolicyOption) -> ::windows::core::Result<()>;
    fn Digits(&self) -> ::windows::core::Result<SmartCardPinCharacterPolicyOption>;
    fn SetDigits(&self, value: SmartCardPinCharacterPolicyOption) -> ::windows::core::Result<()>;
    fn SpecialCharacters(&self) -> ::windows::core::Result<SmartCardPinCharacterPolicyOption>;
    fn SetSpecialCharacters(&self, value: SmartCardPinCharacterPolicyOption) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardPinResetDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardPinResetRequestImpl: Sized {
    fn Challenge(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn Deadline(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn GetDeferral(&self) -> ::windows::core::Result<SmartCardPinResetDeferral>;
    fn SetResponse(&self, response: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardProvisioningImpl: Sized {
    fn SmartCard(&self) -> ::windows::core::Result<SmartCard>;
    fn GetIdAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::GUID>>;
    fn GetNameAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetChallengeContextAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmartCardChallengeContext>>;
    fn RequestPinChangeAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RequestPinResetAsync(&self, handler: &::core::option::Option<SmartCardPinResetHandler>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardProvisioning2Impl: Sized {
    fn GetAuthorityKeyContainerNameAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardProvisioningStaticsImpl: Sized {
    fn FromSmartCardAsync(&self, card: &::core::option::Option<SmartCard>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmartCardProvisioning>>;
    fn RequestVirtualSmartCardCreationAsync(&self, friendlyname: &::windows::core::HSTRING, administrativekey: &::core::option::Option<super::super::Storage::Streams::IBuffer>, pinpolicy: &::core::option::Option<SmartCardPinPolicy>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmartCardProvisioning>>;
    fn RequestVirtualSmartCardCreationAsyncWithCardId(&self, friendlyname: &::windows::core::HSTRING, administrativekey: &::core::option::Option<super::super::Storage::Streams::IBuffer>, pinpolicy: &::core::option::Option<SmartCardPinPolicy>, cardid: &::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmartCardProvisioning>>;
    fn RequestVirtualSmartCardDeletionAsync(&self, card: &::core::option::Option<SmartCard>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardProvisioningStatics2Impl: Sized {
    fn RequestAttestedVirtualSmartCardCreationAsync(&self, friendlyname: &::windows::core::HSTRING, administrativekey: &::core::option::Option<super::super::Storage::Streams::IBuffer>, pinpolicy: &::core::option::Option<SmartCardPinPolicy>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmartCardProvisioning>>;
    fn RequestAttestedVirtualSmartCardCreationAsyncWithCardId(&self, friendlyname: &::windows::core::HSTRING, administrativekey: &::core::option::Option<super::super::Storage::Streams::IBuffer>, pinpolicy: &::core::option::Option<SmartCardPinPolicy>, cardid: &::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmartCardProvisioning>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardReaderImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Kind(&self) -> ::windows::core::Result<SmartCardReaderKind>;
    fn GetStatusAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmartCardReaderStatus>>;
    fn FindAllCardsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<SmartCard>>>;
    fn CardAdded(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SmartCardReader, CardAddedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCardAdded(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CardRemoved(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SmartCardReader, CardRemovedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCardRemoved(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardReaderStaticsImpl: Sized {
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorWithKind(&self, kind: SmartCardReaderKind) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmartCardReader>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardTriggerDetailsImpl: Sized {
    fn TriggerType(&self) -> ::windows::core::Result<SmartCardTriggerType>;
    fn SourceAppletId(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn TriggerData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardTriggerDetails2Impl: Sized {
    fn Emulator(&self) -> ::windows::core::Result<SmartCardEmulator>;
    fn TryLaunchCurrentAppAsync(&self, arguments: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryLaunchCurrentAppWithBehaviorAsync(&self, arguments: &::windows::core::HSTRING, behavior: SmartCardLaunchBehavior) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardTriggerDetails3Impl: Sized {
    fn SmartCard(&self) -> ::windows::core::Result<SmartCard>;
}
