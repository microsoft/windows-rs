#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CardAddedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CardAddedEventArgs {}
impl ::core::clone::Clone for CardAddedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CardRemovedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CardRemovedEventArgs {}
impl ::core::clone::Clone for CardRemovedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICardAddedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICardAddedEventArgs {}
impl ::core::clone::Clone for ICardAddedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICardRemovedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICardRemovedEventArgs {}
impl ::core::clone::Clone for ICardRemovedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKnownSmartCardAppletIds(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKnownSmartCardAppletIds {}
impl ::core::clone::Clone for IKnownSmartCardAppletIds {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmartCard(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmartCard {}
impl ::core::clone::Clone for ISmartCard {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmartCardAppletIdGroup(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmartCardAppletIdGroup {}
impl ::core::clone::Clone for ISmartCardAppletIdGroup {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmartCardAppletIdGroup2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmartCardAppletIdGroup2 {}
impl ::core::clone::Clone for ISmartCardAppletIdGroup2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmartCardAppletIdGroupFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmartCardAppletIdGroupFactory {}
impl ::core::clone::Clone for ISmartCardAppletIdGroupFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmartCardAppletIdGroupRegistration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmartCardAppletIdGroupRegistration {}
impl ::core::clone::Clone for ISmartCardAppletIdGroupRegistration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmartCardAppletIdGroupRegistration2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmartCardAppletIdGroupRegistration2 {}
impl ::core::clone::Clone for ISmartCardAppletIdGroupRegistration2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmartCardAppletIdGroupStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmartCardAppletIdGroupStatics {}
impl ::core::clone::Clone for ISmartCardAppletIdGroupStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmartCardAutomaticResponseApdu(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmartCardAutomaticResponseApdu {}
impl ::core::clone::Clone for ISmartCardAutomaticResponseApdu {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmartCardAutomaticResponseApdu2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmartCardAutomaticResponseApdu2 {}
impl ::core::clone::Clone for ISmartCardAutomaticResponseApdu2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmartCardAutomaticResponseApdu3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmartCardAutomaticResponseApdu3 {}
impl ::core::clone::Clone for ISmartCardAutomaticResponseApdu3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmartCardAutomaticResponseApduFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmartCardAutomaticResponseApduFactory {}
impl ::core::clone::Clone for ISmartCardAutomaticResponseApduFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmartCardChallengeContext(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmartCardChallengeContext {}
impl ::core::clone::Clone for ISmartCardChallengeContext {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmartCardConnect(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmartCardConnect {}
impl ::core::clone::Clone for ISmartCardConnect {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmartCardConnection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmartCardConnection {}
impl ::core::clone::Clone for ISmartCardConnection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmartCardCryptogramGenerator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmartCardCryptogramGenerator {}
impl ::core::clone::Clone for ISmartCardCryptogramGenerator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmartCardCryptogramGenerator2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmartCardCryptogramGenerator2 {}
impl ::core::clone::Clone for ISmartCardCryptogramGenerator2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmartCardCryptogramGeneratorStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmartCardCryptogramGeneratorStatics {}
impl ::core::clone::Clone for ISmartCardCryptogramGeneratorStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmartCardCryptogramGeneratorStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmartCardCryptogramGeneratorStatics2 {}
impl ::core::clone::Clone for ISmartCardCryptogramGeneratorStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult {}
impl ::core::clone::Clone for ISmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult {}
impl ::core::clone::Clone for ISmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult {}
impl ::core::clone::Clone for ISmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmartCardCryptogramMaterialCharacteristics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmartCardCryptogramMaterialCharacteristics {}
impl ::core::clone::Clone for ISmartCardCryptogramMaterialCharacteristics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmartCardCryptogramMaterialPackageCharacteristics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmartCardCryptogramMaterialPackageCharacteristics {}
impl ::core::clone::Clone for ISmartCardCryptogramMaterialPackageCharacteristics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmartCardCryptogramMaterialPossessionProof(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmartCardCryptogramMaterialPossessionProof {}
impl ::core::clone::Clone for ISmartCardCryptogramMaterialPossessionProof {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmartCardCryptogramPlacementStep(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmartCardCryptogramPlacementStep {}
impl ::core::clone::Clone for ISmartCardCryptogramPlacementStep {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmartCardCryptogramStorageKeyCharacteristics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmartCardCryptogramStorageKeyCharacteristics {}
impl ::core::clone::Clone for ISmartCardCryptogramStorageKeyCharacteristics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmartCardCryptogramStorageKeyInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmartCardCryptogramStorageKeyInfo {}
impl ::core::clone::Clone for ISmartCardCryptogramStorageKeyInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmartCardCryptogramStorageKeyInfo2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmartCardCryptogramStorageKeyInfo2 {}
impl ::core::clone::Clone for ISmartCardCryptogramStorageKeyInfo2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmartCardEmulator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmartCardEmulator {}
impl ::core::clone::Clone for ISmartCardEmulator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmartCardEmulator2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmartCardEmulator2 {}
impl ::core::clone::Clone for ISmartCardEmulator2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmartCardEmulatorApduReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmartCardEmulatorApduReceivedEventArgs {}
impl ::core::clone::Clone for ISmartCardEmulatorApduReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmartCardEmulatorApduReceivedEventArgs2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmartCardEmulatorApduReceivedEventArgs2 {}
impl ::core::clone::Clone for ISmartCardEmulatorApduReceivedEventArgs2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmartCardEmulatorApduReceivedEventArgsWithCryptograms(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmartCardEmulatorApduReceivedEventArgsWithCryptograms {}
impl ::core::clone::Clone for ISmartCardEmulatorApduReceivedEventArgsWithCryptograms {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmartCardEmulatorConnectionDeactivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmartCardEmulatorConnectionDeactivatedEventArgs {}
impl ::core::clone::Clone for ISmartCardEmulatorConnectionDeactivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmartCardEmulatorConnectionProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmartCardEmulatorConnectionProperties {}
impl ::core::clone::Clone for ISmartCardEmulatorConnectionProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmartCardEmulatorStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmartCardEmulatorStatics {}
impl ::core::clone::Clone for ISmartCardEmulatorStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmartCardEmulatorStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmartCardEmulatorStatics2 {}
impl ::core::clone::Clone for ISmartCardEmulatorStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmartCardEmulatorStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmartCardEmulatorStatics3 {}
impl ::core::clone::Clone for ISmartCardEmulatorStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmartCardPinPolicy(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmartCardPinPolicy {}
impl ::core::clone::Clone for ISmartCardPinPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmartCardPinResetDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmartCardPinResetDeferral {}
impl ::core::clone::Clone for ISmartCardPinResetDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmartCardPinResetRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmartCardPinResetRequest {}
impl ::core::clone::Clone for ISmartCardPinResetRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmartCardProvisioning(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmartCardProvisioning {}
impl ::core::clone::Clone for ISmartCardProvisioning {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmartCardProvisioning2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmartCardProvisioning2 {}
impl ::core::clone::Clone for ISmartCardProvisioning2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmartCardProvisioningStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmartCardProvisioningStatics {}
impl ::core::clone::Clone for ISmartCardProvisioningStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmartCardProvisioningStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmartCardProvisioningStatics2 {}
impl ::core::clone::Clone for ISmartCardProvisioningStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmartCardReader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmartCardReader {}
impl ::core::clone::Clone for ISmartCardReader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmartCardReaderStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmartCardReaderStatics {}
impl ::core::clone::Clone for ISmartCardReaderStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmartCardTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmartCardTriggerDetails {}
impl ::core::clone::Clone for ISmartCardTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmartCardTriggerDetails2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmartCardTriggerDetails2 {}
impl ::core::clone::Clone for ISmartCardTriggerDetails2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmartCardTriggerDetails3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmartCardTriggerDetails3 {}
impl ::core::clone::Clone for ISmartCardTriggerDetails3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmartCard(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SmartCard {}
impl ::core::clone::Clone for SmartCard {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for SmartCardAppletIdGroup {}
impl ::core::clone::Clone for SmartCardAppletIdGroup {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for SmartCardAppletIdGroupRegistration {}
impl ::core::clone::Clone for SmartCardAppletIdGroupRegistration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmartCardAutomaticResponseApdu(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SmartCardAutomaticResponseApdu {}
impl ::core::clone::Clone for SmartCardAutomaticResponseApdu {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for SmartCardChallengeContext {}
impl ::core::clone::Clone for SmartCardChallengeContext {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmartCardConnection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SmartCardConnection {}
impl ::core::clone::Clone for SmartCardConnection {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for SmartCardCryptogramGenerator {}
impl ::core::clone::Clone for SmartCardCryptogramGenerator {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult {}
impl ::core::clone::Clone for SmartCardCryptogramGetAllCryptogramMaterialCharacteristicsResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult {}
impl ::core::clone::Clone for SmartCardCryptogramGetAllCryptogramMaterialPackageCharacteristicsResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult {}
impl ::core::clone::Clone for SmartCardCryptogramGetAllCryptogramStorageKeyCharacteristicsResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmartCardCryptogramMaterialCharacteristics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SmartCardCryptogramMaterialCharacteristics {}
impl ::core::clone::Clone for SmartCardCryptogramMaterialCharacteristics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmartCardCryptogramMaterialPackageCharacteristics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SmartCardCryptogramMaterialPackageCharacteristics {}
impl ::core::clone::Clone for SmartCardCryptogramMaterialPackageCharacteristics {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for SmartCardCryptogramMaterialPossessionProof {}
impl ::core::clone::Clone for SmartCardCryptogramMaterialPossessionProof {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for SmartCardCryptogramPlacementStep {}
impl ::core::clone::Clone for SmartCardCryptogramPlacementStep {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for SmartCardCryptogramStorageKeyCharacteristics {}
impl ::core::clone::Clone for SmartCardCryptogramStorageKeyCharacteristics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmartCardCryptogramStorageKeyInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SmartCardCryptogramStorageKeyInfo {}
impl ::core::clone::Clone for SmartCardCryptogramStorageKeyInfo {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for SmartCardEmulator {}
impl ::core::clone::Clone for SmartCardEmulator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmartCardEmulatorApduReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SmartCardEmulatorApduReceivedEventArgs {}
impl ::core::clone::Clone for SmartCardEmulatorApduReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmartCardEmulatorConnectionDeactivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SmartCardEmulatorConnectionDeactivatedEventArgs {}
impl ::core::clone::Clone for SmartCardEmulatorConnectionDeactivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for SmartCardEmulatorConnectionProperties {}
impl ::core::clone::Clone for SmartCardEmulatorConnectionProperties {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for SmartCardPinPolicy {}
impl ::core::clone::Clone for SmartCardPinPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmartCardPinResetDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SmartCardPinResetDeferral {}
impl ::core::clone::Clone for SmartCardPinResetDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmartCardPinResetHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SmartCardPinResetHandler {}
impl ::core::clone::Clone for SmartCardPinResetHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmartCardPinResetRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SmartCardPinResetRequest {}
impl ::core::clone::Clone for SmartCardPinResetRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmartCardProvisioning(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SmartCardProvisioning {}
impl ::core::clone::Clone for SmartCardProvisioning {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmartCardReader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SmartCardReader {}
impl ::core::clone::Clone for SmartCardReader {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for SmartCardTriggerDetails {}
impl ::core::clone::Clone for SmartCardTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
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
