#![allow(non_snake_case, non_camel_case_types)]
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
#[repr(C)]
pub struct SmartCardActivationPolicyChangeResult(i32);
#[repr(transparent)]
pub struct SmartCardAppletIdGroup(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct SmartCardAppletIdGroupActivationPolicy(i32);
#[repr(transparent)]
pub struct SmartCardAppletIdGroupRegistration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmartCardAutomaticResponseApdu(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct SmartCardAutomaticResponseStatus(i32);
#[repr(C)]
pub struct SmartCardBackgroundTriggerContract(i32);
#[repr(transparent)]
pub struct SmartCardChallengeContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmartCardConnection(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct SmartCardCryptogramAlgorithm(i32);
#[repr(transparent)]
pub struct SmartCardCryptogramGenerator(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct SmartCardCryptogramGeneratorOperationStatus(i32);
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
#[repr(C)]
pub struct SmartCardCryptogramMaterialPackageConfirmationResponseFormat(i32);
#[repr(C)]
pub struct SmartCardCryptogramMaterialPackageFormat(i32);
#[repr(transparent)]
pub struct SmartCardCryptogramMaterialPossessionProof(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct SmartCardCryptogramMaterialProtectionMethod(i32);
#[repr(C)]
pub struct SmartCardCryptogramMaterialType(i32);
#[repr(C)]
pub struct SmartCardCryptogramPlacementOptions(i32);
#[repr(transparent)]
pub struct SmartCardCryptogramPlacementStep(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct SmartCardCryptogramStorageKeyAlgorithm(i32);
#[repr(C)]
pub struct SmartCardCryptogramStorageKeyCapabilities(i32);
#[repr(transparent)]
pub struct SmartCardCryptogramStorageKeyCharacteristics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmartCardCryptogramStorageKeyInfo(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct SmartCardCryptographicKeyAttestationStatus(i32);
#[repr(C)]
pub struct SmartCardEmulationCategory(i32);
#[repr(C)]
pub struct SmartCardEmulationType(i32);
#[repr(transparent)]
pub struct SmartCardEmulator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmartCardEmulatorApduReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmartCardEmulatorConnectionDeactivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct SmartCardEmulatorConnectionDeactivatedReason(i32);
#[repr(transparent)]
pub struct SmartCardEmulatorConnectionProperties(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct SmartCardEmulatorConnectionSource(i32);
#[repr(C)]
pub struct SmartCardEmulatorContract(i32);
#[repr(C)]
pub struct SmartCardEmulatorEnablementPolicy(i32);
#[repr(C)]
pub struct SmartCardLaunchBehavior(i32);
#[repr(C)]
pub struct SmartCardPinCharacterPolicyOption(i32);
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
#[repr(C)]
pub struct SmartCardReaderKind(i32);
#[repr(C)]
pub struct SmartCardReaderStatus(i32);
#[repr(C)]
pub struct SmartCardStatus(i32);
#[repr(transparent)]
pub struct SmartCardTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct SmartCardTriggerType(i32);
#[repr(C)]
pub struct SmartCardUnlockPromptingBehavior(i32);
