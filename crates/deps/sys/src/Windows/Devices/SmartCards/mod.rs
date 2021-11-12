#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CardAddedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CardRemovedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICardAddedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICardRemovedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKnownSmartCardAppletIds(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmartCard(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmartCardAppletIdGroup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmartCardAppletIdGroup2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmartCardAppletIdGroupFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmartCardAppletIdGroupRegistration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmartCardAppletIdGroupRegistration2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmartCardAppletIdGroupStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmartCardAutomaticResponseApdu(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmartCardAutomaticResponseApdu2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmartCardAutomaticResponseApdu3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmartCardAutomaticResponseApduFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmartCardChallengeContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmartCardConnect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmartCardConnection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmartCardCryptogramGenerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmartCardCryptogramGenerator2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmartCardCryptogramGeneratorStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmartCardCryptogramGeneratorStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmartCardCryptogramMaterialCharacteristics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmartCardCryptogramMaterialPackageCharacteristics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmartCardCryptogramMaterialPossessionProof(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmartCardCryptogramPlacementStep(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmartCardCryptogramStorageKeyCharacteristics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmartCardCryptogramStorageKeyInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmartCardCryptogramStorageKeyInfo2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmartCardEmulator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmartCardEmulator2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmartCardEmulatorApduReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmartCardEmulatorApduReceivedEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmartCardEmulatorApduReceivedEventArgsWithCryptograms(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmartCardEmulatorConnectionDeactivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmartCardEmulatorConnectionProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmartCardEmulatorStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmartCardEmulatorStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmartCardEmulatorStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmartCardPinPolicy(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmartCardPinResetDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmartCardPinResetRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmartCardProvisioning(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmartCardProvisioning2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmartCardProvisioningStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmartCardProvisioningStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmartCardReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmartCardReaderStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmartCardTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmartCardTriggerDetails2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmartCardTriggerDetails3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmartCard(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmartCardActivationPolicyChangeResult(pub i32);
impl SmartCardActivationPolicyChangeResult {
    pub const Denied: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
}
impl ::core::marker::Copy for SmartCardActivationPolicyChangeResult {}
impl ::core::clone::Clone for SmartCardActivationPolicyChangeResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmartCardAppletIdGroup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmartCardAppletIdGroupActivationPolicy(pub i32);
impl SmartCardAppletIdGroupActivationPolicy {
    pub const Disabled: Self = Self(0i32);
    pub const ForegroundOverride: Self = Self(1i32);
    pub const Enabled: Self = Self(2i32);
}
impl ::core::marker::Copy for SmartCardAppletIdGroupActivationPolicy {}
impl ::core::clone::Clone for SmartCardAppletIdGroupActivationPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmartCardAppletIdGroupRegistration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmartCardAutomaticResponseApdu(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmartCardAutomaticResponseStatus(pub i32);
impl SmartCardAutomaticResponseStatus {
    pub const None: Self = Self(0i32);
    pub const Success: Self = Self(1i32);
    pub const UnknownError: Self = Self(2i32);
}
impl ::core::marker::Copy for SmartCardAutomaticResponseStatus {}
impl ::core::clone::Clone for SmartCardAutomaticResponseStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmartCardChallengeContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmartCardConnection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmartCardCryptogramAlgorithm(pub i32);
impl SmartCardCryptogramAlgorithm {
    pub const None: Self = Self(0i32);
    pub const CbcMac: Self = Self(1i32);
    pub const Cvc3Umd: Self = Self(2i32);
    pub const DecimalizedMsd: Self = Self(3i32);
    pub const Cvc3MD: Self = Self(4i32);
    pub const Sha1: Self = Self(5i32);
    pub const SignedDynamicApplicationData: Self = Self(6i32);
    pub const RsaPkcs1: Self = Self(7i32);
    pub const Sha256Hmac: Self = Self(8i32);
}
impl ::core::marker::Copy for SmartCardCryptogramAlgorithm {}
impl ::core::clone::Clone for SmartCardCryptogramAlgorithm {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmartCardCryptogramGenerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmartCardCryptogramGeneratorOperationStatus(pub i32);
impl SmartCardCryptogramGeneratorOperationStatus {
    pub const Success: Self = Self(0i32);
    pub const AuthorizationFailed: Self = Self(1i32);
    pub const AuthorizationCanceled: Self = Self(2i32);
    pub const AuthorizationRequired: Self = Self(3i32);
    pub const CryptogramMaterialPackageStorageKeyExists: Self = Self(4i32);
    pub const NoCryptogramMaterialPackageStorageKey: Self = Self(5i32);
    pub const NoCryptogramMaterialPackage: Self = Self(6i32);
    pub const UnsupportedCryptogramMaterialPackage: Self = Self(7i32);
    pub const UnknownCryptogramMaterialName: Self = Self(8i32);
    pub const InvalidCryptogramMaterialUsage: Self = Self(9i32);
    pub const ApduResponseNotSent: Self = Self(10i32);
    pub const OtherError: Self = Self(11i32);
    pub const ValidationFailed: Self = Self(12i32);
    pub const NotSupported: Self = Self(13i32);
}
impl ::core::marker::Copy for SmartCardCryptogramGeneratorOperationStatus {}
impl ::core::clone::Clone for SmartCardCryptogramGeneratorOperationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmartCardCryptogramMaterialCharacteristics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmartCardCryptogramMaterialPackageCharacteristics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmartCardCryptogramMaterialPackageConfirmationResponseFormat(pub i32);
impl SmartCardCryptogramMaterialPackageConfirmationResponseFormat {
    pub const None: Self = Self(0i32);
    pub const VisaHmac: Self = Self(1i32);
}
impl ::core::marker::Copy for SmartCardCryptogramMaterialPackageConfirmationResponseFormat {}
impl ::core::clone::Clone for SmartCardCryptogramMaterialPackageConfirmationResponseFormat {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmartCardCryptogramMaterialPackageFormat(pub i32);
impl SmartCardCryptogramMaterialPackageFormat {
    pub const None: Self = Self(0i32);
    pub const JweRsaPki: Self = Self(1i32);
}
impl ::core::marker::Copy for SmartCardCryptogramMaterialPackageFormat {}
impl ::core::clone::Clone for SmartCardCryptogramMaterialPackageFormat {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmartCardCryptogramMaterialPossessionProof(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmartCardCryptogramMaterialProtectionMethod(pub i32);
impl SmartCardCryptogramMaterialProtectionMethod {
    pub const None: Self = Self(0i32);
    pub const WhiteBoxing: Self = Self(1i32);
}
impl ::core::marker::Copy for SmartCardCryptogramMaterialProtectionMethod {}
impl ::core::clone::Clone for SmartCardCryptogramMaterialProtectionMethod {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmartCardCryptogramMaterialType(pub i32);
impl SmartCardCryptogramMaterialType {
    pub const None: Self = Self(0i32);
    pub const StaticDataAuthentication: Self = Self(1i32);
    pub const TripleDes112: Self = Self(2i32);
    pub const Aes: Self = Self(3i32);
    pub const RsaPkcs1: Self = Self(4i32);
}
impl ::core::marker::Copy for SmartCardCryptogramMaterialType {}
impl ::core::clone::Clone for SmartCardCryptogramMaterialType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmartCardCryptogramPlacementOptions(pub u32);
impl SmartCardCryptogramPlacementOptions {
    pub const None: Self = Self(0u32);
    pub const UnitsAreInNibbles: Self = Self(1u32);
    pub const ChainOutput: Self = Self(2u32);
}
impl ::core::marker::Copy for SmartCardCryptogramPlacementOptions {}
impl ::core::clone::Clone for SmartCardCryptogramPlacementOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmartCardCryptogramPlacementStep(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmartCardCryptogramStorageKeyAlgorithm(pub i32);
impl SmartCardCryptogramStorageKeyAlgorithm {
    pub const None: Self = Self(0i32);
    pub const Rsa2048: Self = Self(1i32);
}
impl ::core::marker::Copy for SmartCardCryptogramStorageKeyAlgorithm {}
impl ::core::clone::Clone for SmartCardCryptogramStorageKeyAlgorithm {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmartCardCryptogramStorageKeyCapabilities(pub u32);
impl SmartCardCryptogramStorageKeyCapabilities {
    pub const None: Self = Self(0u32);
    pub const HardwareProtection: Self = Self(1u32);
    pub const UnlockPrompt: Self = Self(2u32);
}
impl ::core::marker::Copy for SmartCardCryptogramStorageKeyCapabilities {}
impl ::core::clone::Clone for SmartCardCryptogramStorageKeyCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmartCardCryptogramStorageKeyCharacteristics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmartCardCryptogramStorageKeyInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmartCardCryptographicKeyAttestationStatus(pub i32);
impl SmartCardCryptographicKeyAttestationStatus {
    pub const NoAttestation: Self = Self(0i32);
    pub const SoftwareKeyWithoutTpm: Self = Self(1i32);
    pub const SoftwareKeyWithTpm: Self = Self(2i32);
    pub const TpmKeyUnknownAttestationStatus: Self = Self(3i32);
    pub const TpmKeyWithoutAttestationCapability: Self = Self(4i32);
    pub const TpmKeyWithTemporaryAttestationFailure: Self = Self(5i32);
    pub const TpmKeyWithLongTermAttestationFailure: Self = Self(6i32);
    pub const TpmKeyWithAttestation: Self = Self(7i32);
}
impl ::core::marker::Copy for SmartCardCryptographicKeyAttestationStatus {}
impl ::core::clone::Clone for SmartCardCryptographicKeyAttestationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmartCardEmulationCategory(pub i32);
impl SmartCardEmulationCategory {
    pub const Other: Self = Self(0i32);
    pub const Payment: Self = Self(1i32);
}
impl ::core::marker::Copy for SmartCardEmulationCategory {}
impl ::core::clone::Clone for SmartCardEmulationCategory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmartCardEmulationType(pub i32);
impl SmartCardEmulationType {
    pub const Host: Self = Self(0i32);
    pub const Uicc: Self = Self(1i32);
    pub const EmbeddedSE: Self = Self(2i32);
}
impl ::core::marker::Copy for SmartCardEmulationType {}
impl ::core::clone::Clone for SmartCardEmulationType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmartCardEmulator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmartCardEmulatorApduReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmartCardEmulatorConnectionDeactivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmartCardEmulatorConnectionDeactivatedReason(pub i32);
impl SmartCardEmulatorConnectionDeactivatedReason {
    pub const ConnectionLost: Self = Self(0i32);
    pub const ConnectionRedirected: Self = Self(1i32);
}
impl ::core::marker::Copy for SmartCardEmulatorConnectionDeactivatedReason {}
impl ::core::clone::Clone for SmartCardEmulatorConnectionDeactivatedReason {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmartCardEmulatorConnectionProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmartCardEmulatorConnectionSource(pub i32);
impl SmartCardEmulatorConnectionSource {
    pub const Unknown: Self = Self(0i32);
    pub const NfcReader: Self = Self(1i32);
}
impl ::core::marker::Copy for SmartCardEmulatorConnectionSource {}
impl ::core::clone::Clone for SmartCardEmulatorConnectionSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmartCardEmulatorEnablementPolicy(pub i32);
impl SmartCardEmulatorEnablementPolicy {
    pub const Never: Self = Self(0i32);
    pub const Always: Self = Self(1i32);
    pub const ScreenOn: Self = Self(2i32);
    pub const ScreenUnlocked: Self = Self(3i32);
}
impl ::core::marker::Copy for SmartCardEmulatorEnablementPolicy {}
impl ::core::clone::Clone for SmartCardEmulatorEnablementPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmartCardLaunchBehavior(pub i32);
impl SmartCardLaunchBehavior {
    pub const Default: Self = Self(0i32);
    pub const AboveLock: Self = Self(1i32);
}
impl ::core::marker::Copy for SmartCardLaunchBehavior {}
impl ::core::clone::Clone for SmartCardLaunchBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmartCardPinCharacterPolicyOption(pub i32);
impl SmartCardPinCharacterPolicyOption {
    pub const Allow: Self = Self(0i32);
    pub const RequireAtLeastOne: Self = Self(1i32);
    pub const Disallow: Self = Self(2i32);
}
impl ::core::marker::Copy for SmartCardPinCharacterPolicyOption {}
impl ::core::clone::Clone for SmartCardPinCharacterPolicyOption {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmartCardPinPolicy(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmartCardPinResetDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmartCardPinResetHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmartCardPinResetRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmartCardProvisioning(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmartCardReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmartCardReaderKind(pub i32);
impl SmartCardReaderKind {
    pub const Any: Self = Self(0i32);
    pub const Generic: Self = Self(1i32);
    pub const Tpm: Self = Self(2i32);
    pub const Nfc: Self = Self(3i32);
    pub const Uicc: Self = Self(4i32);
    pub const EmbeddedSE: Self = Self(5i32);
}
impl ::core::marker::Copy for SmartCardReaderKind {}
impl ::core::clone::Clone for SmartCardReaderKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmartCardReaderStatus(pub i32);
impl SmartCardReaderStatus {
    pub const Disconnected: Self = Self(0i32);
    pub const Ready: Self = Self(1i32);
    pub const Exclusive: Self = Self(2i32);
}
impl ::core::marker::Copy for SmartCardReaderStatus {}
impl ::core::clone::Clone for SmartCardReaderStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmartCardStatus(pub i32);
impl SmartCardStatus {
    pub const Disconnected: Self = Self(0i32);
    pub const Ready: Self = Self(1i32);
    pub const Shared: Self = Self(2i32);
    pub const Exclusive: Self = Self(3i32);
    pub const Unresponsive: Self = Self(4i32);
}
impl ::core::marker::Copy for SmartCardStatus {}
impl ::core::clone::Clone for SmartCardStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmartCardTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmartCardTriggerType(pub i32);
impl SmartCardTriggerType {
    pub const EmulatorTransaction: Self = Self(0i32);
    pub const EmulatorNearFieldEntry: Self = Self(1i32);
    pub const EmulatorNearFieldExit: Self = Self(2i32);
    pub const EmulatorHostApplicationActivated: Self = Self(3i32);
    pub const EmulatorAppletIdGroupRegistrationChanged: Self = Self(4i32);
    pub const ReaderCardAdded: Self = Self(5i32);
}
impl ::core::marker::Copy for SmartCardTriggerType {}
impl ::core::clone::Clone for SmartCardTriggerType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmartCardUnlockPromptingBehavior(pub i32);
impl SmartCardUnlockPromptingBehavior {
    pub const AllowUnlockPrompt: Self = Self(0i32);
    pub const RequireUnlockPrompt: Self = Self(1i32);
    pub const PreventUnlockPrompt: Self = Self(2i32);
}
impl ::core::marker::Copy for SmartCardUnlockPromptingBehavior {}
impl ::core::clone::Clone for SmartCardUnlockPromptingBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
