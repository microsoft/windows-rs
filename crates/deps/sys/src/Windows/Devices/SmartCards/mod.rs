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
    pub const Denied: SmartCardActivationPolicyChangeResult = SmartCardActivationPolicyChangeResult(0i32);
    pub const Allowed: SmartCardActivationPolicyChangeResult = SmartCardActivationPolicyChangeResult(1i32);
}
#[repr(transparent)]
pub struct SmartCardAppletIdGroup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmartCardAppletIdGroupActivationPolicy(pub i32);
impl SmartCardAppletIdGroupActivationPolicy {
    pub const Disabled: SmartCardAppletIdGroupActivationPolicy = SmartCardAppletIdGroupActivationPolicy(0i32);
    pub const ForegroundOverride: SmartCardAppletIdGroupActivationPolicy = SmartCardAppletIdGroupActivationPolicy(1i32);
    pub const Enabled: SmartCardAppletIdGroupActivationPolicy = SmartCardAppletIdGroupActivationPolicy(2i32);
}
#[repr(transparent)]
pub struct SmartCardAppletIdGroupRegistration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmartCardAutomaticResponseApdu(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmartCardAutomaticResponseStatus(pub i32);
impl SmartCardAutomaticResponseStatus {
    pub const None: SmartCardAutomaticResponseStatus = SmartCardAutomaticResponseStatus(0i32);
    pub const Success: SmartCardAutomaticResponseStatus = SmartCardAutomaticResponseStatus(1i32);
    pub const UnknownError: SmartCardAutomaticResponseStatus = SmartCardAutomaticResponseStatus(2i32);
}
#[repr(C)]
pub struct SmartCardBackgroundTriggerContract(i32);
#[repr(transparent)]
pub struct SmartCardChallengeContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmartCardConnection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmartCardCryptogramAlgorithm(pub i32);
impl SmartCardCryptogramAlgorithm {
    pub const None: SmartCardCryptogramAlgorithm = SmartCardCryptogramAlgorithm(0i32);
    pub const CbcMac: SmartCardCryptogramAlgorithm = SmartCardCryptogramAlgorithm(1i32);
    pub const Cvc3Umd: SmartCardCryptogramAlgorithm = SmartCardCryptogramAlgorithm(2i32);
    pub const DecimalizedMsd: SmartCardCryptogramAlgorithm = SmartCardCryptogramAlgorithm(3i32);
    pub const Cvc3MD: SmartCardCryptogramAlgorithm = SmartCardCryptogramAlgorithm(4i32);
    pub const Sha1: SmartCardCryptogramAlgorithm = SmartCardCryptogramAlgorithm(5i32);
    pub const SignedDynamicApplicationData: SmartCardCryptogramAlgorithm = SmartCardCryptogramAlgorithm(6i32);
    pub const RsaPkcs1: SmartCardCryptogramAlgorithm = SmartCardCryptogramAlgorithm(7i32);
    pub const Sha256Hmac: SmartCardCryptogramAlgorithm = SmartCardCryptogramAlgorithm(8i32);
}
#[repr(transparent)]
pub struct SmartCardCryptogramGenerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmartCardCryptogramGeneratorOperationStatus(pub i32);
impl SmartCardCryptogramGeneratorOperationStatus {
    pub const Success: SmartCardCryptogramGeneratorOperationStatus = SmartCardCryptogramGeneratorOperationStatus(0i32);
    pub const AuthorizationFailed: SmartCardCryptogramGeneratorOperationStatus = SmartCardCryptogramGeneratorOperationStatus(1i32);
    pub const AuthorizationCanceled: SmartCardCryptogramGeneratorOperationStatus = SmartCardCryptogramGeneratorOperationStatus(2i32);
    pub const AuthorizationRequired: SmartCardCryptogramGeneratorOperationStatus = SmartCardCryptogramGeneratorOperationStatus(3i32);
    pub const CryptogramMaterialPackageStorageKeyExists: SmartCardCryptogramGeneratorOperationStatus = SmartCardCryptogramGeneratorOperationStatus(4i32);
    pub const NoCryptogramMaterialPackageStorageKey: SmartCardCryptogramGeneratorOperationStatus = SmartCardCryptogramGeneratorOperationStatus(5i32);
    pub const NoCryptogramMaterialPackage: SmartCardCryptogramGeneratorOperationStatus = SmartCardCryptogramGeneratorOperationStatus(6i32);
    pub const UnsupportedCryptogramMaterialPackage: SmartCardCryptogramGeneratorOperationStatus = SmartCardCryptogramGeneratorOperationStatus(7i32);
    pub const UnknownCryptogramMaterialName: SmartCardCryptogramGeneratorOperationStatus = SmartCardCryptogramGeneratorOperationStatus(8i32);
    pub const InvalidCryptogramMaterialUsage: SmartCardCryptogramGeneratorOperationStatus = SmartCardCryptogramGeneratorOperationStatus(9i32);
    pub const ApduResponseNotSent: SmartCardCryptogramGeneratorOperationStatus = SmartCardCryptogramGeneratorOperationStatus(10i32);
    pub const OtherError: SmartCardCryptogramGeneratorOperationStatus = SmartCardCryptogramGeneratorOperationStatus(11i32);
    pub const ValidationFailed: SmartCardCryptogramGeneratorOperationStatus = SmartCardCryptogramGeneratorOperationStatus(12i32);
    pub const NotSupported: SmartCardCryptogramGeneratorOperationStatus = SmartCardCryptogramGeneratorOperationStatus(13i32);
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
    pub const None: SmartCardCryptogramMaterialPackageConfirmationResponseFormat = SmartCardCryptogramMaterialPackageConfirmationResponseFormat(0i32);
    pub const VisaHmac: SmartCardCryptogramMaterialPackageConfirmationResponseFormat = SmartCardCryptogramMaterialPackageConfirmationResponseFormat(1i32);
}
#[repr(transparent)]
pub struct SmartCardCryptogramMaterialPackageFormat(pub i32);
impl SmartCardCryptogramMaterialPackageFormat {
    pub const None: SmartCardCryptogramMaterialPackageFormat = SmartCardCryptogramMaterialPackageFormat(0i32);
    pub const JweRsaPki: SmartCardCryptogramMaterialPackageFormat = SmartCardCryptogramMaterialPackageFormat(1i32);
}
#[repr(transparent)]
pub struct SmartCardCryptogramMaterialPossessionProof(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmartCardCryptogramMaterialProtectionMethod(pub i32);
impl SmartCardCryptogramMaterialProtectionMethod {
    pub const None: SmartCardCryptogramMaterialProtectionMethod = SmartCardCryptogramMaterialProtectionMethod(0i32);
    pub const WhiteBoxing: SmartCardCryptogramMaterialProtectionMethod = SmartCardCryptogramMaterialProtectionMethod(1i32);
}
#[repr(transparent)]
pub struct SmartCardCryptogramMaterialType(pub i32);
impl SmartCardCryptogramMaterialType {
    pub const None: SmartCardCryptogramMaterialType = SmartCardCryptogramMaterialType(0i32);
    pub const StaticDataAuthentication: SmartCardCryptogramMaterialType = SmartCardCryptogramMaterialType(1i32);
    pub const TripleDes112: SmartCardCryptogramMaterialType = SmartCardCryptogramMaterialType(2i32);
    pub const Aes: SmartCardCryptogramMaterialType = SmartCardCryptogramMaterialType(3i32);
    pub const RsaPkcs1: SmartCardCryptogramMaterialType = SmartCardCryptogramMaterialType(4i32);
}
#[repr(transparent)]
pub struct SmartCardCryptogramPlacementOptions(pub u32);
impl SmartCardCryptogramPlacementOptions {
    pub const None: SmartCardCryptogramPlacementOptions = SmartCardCryptogramPlacementOptions(0u32);
    pub const UnitsAreInNibbles: SmartCardCryptogramPlacementOptions = SmartCardCryptogramPlacementOptions(1u32);
    pub const ChainOutput: SmartCardCryptogramPlacementOptions = SmartCardCryptogramPlacementOptions(2u32);
}
#[repr(transparent)]
pub struct SmartCardCryptogramPlacementStep(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmartCardCryptogramStorageKeyAlgorithm(pub i32);
impl SmartCardCryptogramStorageKeyAlgorithm {
    pub const None: SmartCardCryptogramStorageKeyAlgorithm = SmartCardCryptogramStorageKeyAlgorithm(0i32);
    pub const Rsa2048: SmartCardCryptogramStorageKeyAlgorithm = SmartCardCryptogramStorageKeyAlgorithm(1i32);
}
#[repr(transparent)]
pub struct SmartCardCryptogramStorageKeyCapabilities(pub u32);
impl SmartCardCryptogramStorageKeyCapabilities {
    pub const None: SmartCardCryptogramStorageKeyCapabilities = SmartCardCryptogramStorageKeyCapabilities(0u32);
    pub const HardwareProtection: SmartCardCryptogramStorageKeyCapabilities = SmartCardCryptogramStorageKeyCapabilities(1u32);
    pub const UnlockPrompt: SmartCardCryptogramStorageKeyCapabilities = SmartCardCryptogramStorageKeyCapabilities(2u32);
}
#[repr(transparent)]
pub struct SmartCardCryptogramStorageKeyCharacteristics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmartCardCryptogramStorageKeyInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmartCardCryptographicKeyAttestationStatus(pub i32);
impl SmartCardCryptographicKeyAttestationStatus {
    pub const NoAttestation: SmartCardCryptographicKeyAttestationStatus = SmartCardCryptographicKeyAttestationStatus(0i32);
    pub const SoftwareKeyWithoutTpm: SmartCardCryptographicKeyAttestationStatus = SmartCardCryptographicKeyAttestationStatus(1i32);
    pub const SoftwareKeyWithTpm: SmartCardCryptographicKeyAttestationStatus = SmartCardCryptographicKeyAttestationStatus(2i32);
    pub const TpmKeyUnknownAttestationStatus: SmartCardCryptographicKeyAttestationStatus = SmartCardCryptographicKeyAttestationStatus(3i32);
    pub const TpmKeyWithoutAttestationCapability: SmartCardCryptographicKeyAttestationStatus = SmartCardCryptographicKeyAttestationStatus(4i32);
    pub const TpmKeyWithTemporaryAttestationFailure: SmartCardCryptographicKeyAttestationStatus = SmartCardCryptographicKeyAttestationStatus(5i32);
    pub const TpmKeyWithLongTermAttestationFailure: SmartCardCryptographicKeyAttestationStatus = SmartCardCryptographicKeyAttestationStatus(6i32);
    pub const TpmKeyWithAttestation: SmartCardCryptographicKeyAttestationStatus = SmartCardCryptographicKeyAttestationStatus(7i32);
}
#[repr(transparent)]
pub struct SmartCardEmulationCategory(pub i32);
impl SmartCardEmulationCategory {
    pub const Other: SmartCardEmulationCategory = SmartCardEmulationCategory(0i32);
    pub const Payment: SmartCardEmulationCategory = SmartCardEmulationCategory(1i32);
}
#[repr(transparent)]
pub struct SmartCardEmulationType(pub i32);
impl SmartCardEmulationType {
    pub const Host: SmartCardEmulationType = SmartCardEmulationType(0i32);
    pub const Uicc: SmartCardEmulationType = SmartCardEmulationType(1i32);
    pub const EmbeddedSE: SmartCardEmulationType = SmartCardEmulationType(2i32);
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
    pub const ConnectionLost: SmartCardEmulatorConnectionDeactivatedReason = SmartCardEmulatorConnectionDeactivatedReason(0i32);
    pub const ConnectionRedirected: SmartCardEmulatorConnectionDeactivatedReason = SmartCardEmulatorConnectionDeactivatedReason(1i32);
}
#[repr(transparent)]
pub struct SmartCardEmulatorConnectionProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmartCardEmulatorConnectionSource(pub i32);
impl SmartCardEmulatorConnectionSource {
    pub const Unknown: SmartCardEmulatorConnectionSource = SmartCardEmulatorConnectionSource(0i32);
    pub const NfcReader: SmartCardEmulatorConnectionSource = SmartCardEmulatorConnectionSource(1i32);
}
#[repr(C)]
pub struct SmartCardEmulatorContract(i32);
#[repr(transparent)]
pub struct SmartCardEmulatorEnablementPolicy(pub i32);
impl SmartCardEmulatorEnablementPolicy {
    pub const Never: SmartCardEmulatorEnablementPolicy = SmartCardEmulatorEnablementPolicy(0i32);
    pub const Always: SmartCardEmulatorEnablementPolicy = SmartCardEmulatorEnablementPolicy(1i32);
    pub const ScreenOn: SmartCardEmulatorEnablementPolicy = SmartCardEmulatorEnablementPolicy(2i32);
    pub const ScreenUnlocked: SmartCardEmulatorEnablementPolicy = SmartCardEmulatorEnablementPolicy(3i32);
}
#[repr(transparent)]
pub struct SmartCardLaunchBehavior(pub i32);
impl SmartCardLaunchBehavior {
    pub const Default: SmartCardLaunchBehavior = SmartCardLaunchBehavior(0i32);
    pub const AboveLock: SmartCardLaunchBehavior = SmartCardLaunchBehavior(1i32);
}
#[repr(transparent)]
pub struct SmartCardPinCharacterPolicyOption(pub i32);
impl SmartCardPinCharacterPolicyOption {
    pub const Allow: SmartCardPinCharacterPolicyOption = SmartCardPinCharacterPolicyOption(0i32);
    pub const RequireAtLeastOne: SmartCardPinCharacterPolicyOption = SmartCardPinCharacterPolicyOption(1i32);
    pub const Disallow: SmartCardPinCharacterPolicyOption = SmartCardPinCharacterPolicyOption(2i32);
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
    pub const Any: SmartCardReaderKind = SmartCardReaderKind(0i32);
    pub const Generic: SmartCardReaderKind = SmartCardReaderKind(1i32);
    pub const Tpm: SmartCardReaderKind = SmartCardReaderKind(2i32);
    pub const Nfc: SmartCardReaderKind = SmartCardReaderKind(3i32);
    pub const Uicc: SmartCardReaderKind = SmartCardReaderKind(4i32);
    pub const EmbeddedSE: SmartCardReaderKind = SmartCardReaderKind(5i32);
}
#[repr(transparent)]
pub struct SmartCardReaderStatus(pub i32);
impl SmartCardReaderStatus {
    pub const Disconnected: SmartCardReaderStatus = SmartCardReaderStatus(0i32);
    pub const Ready: SmartCardReaderStatus = SmartCardReaderStatus(1i32);
    pub const Exclusive: SmartCardReaderStatus = SmartCardReaderStatus(2i32);
}
#[repr(transparent)]
pub struct SmartCardStatus(pub i32);
impl SmartCardStatus {
    pub const Disconnected: SmartCardStatus = SmartCardStatus(0i32);
    pub const Ready: SmartCardStatus = SmartCardStatus(1i32);
    pub const Shared: SmartCardStatus = SmartCardStatus(2i32);
    pub const Exclusive: SmartCardStatus = SmartCardStatus(3i32);
    pub const Unresponsive: SmartCardStatus = SmartCardStatus(4i32);
}
#[repr(transparent)]
pub struct SmartCardTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmartCardTriggerType(pub i32);
impl SmartCardTriggerType {
    pub const EmulatorTransaction: SmartCardTriggerType = SmartCardTriggerType(0i32);
    pub const EmulatorNearFieldEntry: SmartCardTriggerType = SmartCardTriggerType(1i32);
    pub const EmulatorNearFieldExit: SmartCardTriggerType = SmartCardTriggerType(2i32);
    pub const EmulatorHostApplicationActivated: SmartCardTriggerType = SmartCardTriggerType(3i32);
    pub const EmulatorAppletIdGroupRegistrationChanged: SmartCardTriggerType = SmartCardTriggerType(4i32);
    pub const ReaderCardAdded: SmartCardTriggerType = SmartCardTriggerType(5i32);
}
#[repr(transparent)]
pub struct SmartCardUnlockPromptingBehavior(pub i32);
impl SmartCardUnlockPromptingBehavior {
    pub const AllowUnlockPrompt: SmartCardUnlockPromptingBehavior = SmartCardUnlockPromptingBehavior(0i32);
    pub const RequireUnlockPrompt: SmartCardUnlockPromptingBehavior = SmartCardUnlockPromptingBehavior(1i32);
    pub const PreventUnlockPrompt: SmartCardUnlockPromptingBehavior = SmartCardUnlockPromptingBehavior(2i32);
}
