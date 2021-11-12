#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ISecondaryAuthenticationFactorAuthentication(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISecondaryAuthenticationFactorAuthenticationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISecondaryAuthenticationFactorAuthenticationStageInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISecondaryAuthenticationFactorAuthenticationStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISecondaryAuthenticationFactorInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISecondaryAuthenticationFactorInfo2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISecondaryAuthenticationFactorRegistration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISecondaryAuthenticationFactorRegistrationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISecondaryAuthenticationFactorRegistrationStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorAuthentication(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorAuthenticationMessage(pub i32);
impl SecondaryAuthenticationFactorAuthenticationMessage {
    pub const Invalid: SecondaryAuthenticationFactorAuthenticationMessage = SecondaryAuthenticationFactorAuthenticationMessage(0i32);
    pub const SwipeUpWelcome: SecondaryAuthenticationFactorAuthenticationMessage = SecondaryAuthenticationFactorAuthenticationMessage(1i32);
    pub const TapWelcome: SecondaryAuthenticationFactorAuthenticationMessage = SecondaryAuthenticationFactorAuthenticationMessage(2i32);
    pub const DeviceNeedsAttention: SecondaryAuthenticationFactorAuthenticationMessage = SecondaryAuthenticationFactorAuthenticationMessage(3i32);
    pub const LookingForDevice: SecondaryAuthenticationFactorAuthenticationMessage = SecondaryAuthenticationFactorAuthenticationMessage(4i32);
    pub const LookingForDevicePluggedin: SecondaryAuthenticationFactorAuthenticationMessage = SecondaryAuthenticationFactorAuthenticationMessage(5i32);
    pub const BluetoothIsDisabled: SecondaryAuthenticationFactorAuthenticationMessage = SecondaryAuthenticationFactorAuthenticationMessage(6i32);
    pub const NfcIsDisabled: SecondaryAuthenticationFactorAuthenticationMessage = SecondaryAuthenticationFactorAuthenticationMessage(7i32);
    pub const WiFiIsDisabled: SecondaryAuthenticationFactorAuthenticationMessage = SecondaryAuthenticationFactorAuthenticationMessage(8i32);
    pub const ExtraTapIsRequired: SecondaryAuthenticationFactorAuthenticationMessage = SecondaryAuthenticationFactorAuthenticationMessage(9i32);
    pub const DisabledByPolicy: SecondaryAuthenticationFactorAuthenticationMessage = SecondaryAuthenticationFactorAuthenticationMessage(10i32);
    pub const TapOnDeviceRequired: SecondaryAuthenticationFactorAuthenticationMessage = SecondaryAuthenticationFactorAuthenticationMessage(11i32);
    pub const HoldFinger: SecondaryAuthenticationFactorAuthenticationMessage = SecondaryAuthenticationFactorAuthenticationMessage(12i32);
    pub const ScanFinger: SecondaryAuthenticationFactorAuthenticationMessage = SecondaryAuthenticationFactorAuthenticationMessage(13i32);
    pub const UnauthorizedUser: SecondaryAuthenticationFactorAuthenticationMessage = SecondaryAuthenticationFactorAuthenticationMessage(14i32);
    pub const ReregisterRequired: SecondaryAuthenticationFactorAuthenticationMessage = SecondaryAuthenticationFactorAuthenticationMessage(15i32);
    pub const TryAgain: SecondaryAuthenticationFactorAuthenticationMessage = SecondaryAuthenticationFactorAuthenticationMessage(16i32);
    pub const SayPassphrase: SecondaryAuthenticationFactorAuthenticationMessage = SecondaryAuthenticationFactorAuthenticationMessage(17i32);
    pub const ReadyToSignIn: SecondaryAuthenticationFactorAuthenticationMessage = SecondaryAuthenticationFactorAuthenticationMessage(18i32);
    pub const UseAnotherSignInOption: SecondaryAuthenticationFactorAuthenticationMessage = SecondaryAuthenticationFactorAuthenticationMessage(19i32);
    pub const ConnectionRequired: SecondaryAuthenticationFactorAuthenticationMessage = SecondaryAuthenticationFactorAuthenticationMessage(20i32);
    pub const TimeLimitExceeded: SecondaryAuthenticationFactorAuthenticationMessage = SecondaryAuthenticationFactorAuthenticationMessage(21i32);
    pub const CanceledByUser: SecondaryAuthenticationFactorAuthenticationMessage = SecondaryAuthenticationFactorAuthenticationMessage(22i32);
    pub const CenterHand: SecondaryAuthenticationFactorAuthenticationMessage = SecondaryAuthenticationFactorAuthenticationMessage(23i32);
    pub const MoveHandCloser: SecondaryAuthenticationFactorAuthenticationMessage = SecondaryAuthenticationFactorAuthenticationMessage(24i32);
    pub const MoveHandFarther: SecondaryAuthenticationFactorAuthenticationMessage = SecondaryAuthenticationFactorAuthenticationMessage(25i32);
    pub const PlaceHandAbove: SecondaryAuthenticationFactorAuthenticationMessage = SecondaryAuthenticationFactorAuthenticationMessage(26i32);
    pub const RecognitionFailed: SecondaryAuthenticationFactorAuthenticationMessage = SecondaryAuthenticationFactorAuthenticationMessage(27i32);
    pub const DeviceUnavailable: SecondaryAuthenticationFactorAuthenticationMessage = SecondaryAuthenticationFactorAuthenticationMessage(28i32);
}
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorAuthenticationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorAuthenticationScenario(pub i32);
impl SecondaryAuthenticationFactorAuthenticationScenario {
    pub const SignIn: SecondaryAuthenticationFactorAuthenticationScenario = SecondaryAuthenticationFactorAuthenticationScenario(0i32);
    pub const CredentialPrompt: SecondaryAuthenticationFactorAuthenticationScenario = SecondaryAuthenticationFactorAuthenticationScenario(1i32);
}
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorAuthenticationStage(pub i32);
impl SecondaryAuthenticationFactorAuthenticationStage {
    pub const NotStarted: SecondaryAuthenticationFactorAuthenticationStage = SecondaryAuthenticationFactorAuthenticationStage(0i32);
    pub const WaitingForUserConfirmation: SecondaryAuthenticationFactorAuthenticationStage = SecondaryAuthenticationFactorAuthenticationStage(1i32);
    pub const CollectingCredential: SecondaryAuthenticationFactorAuthenticationStage = SecondaryAuthenticationFactorAuthenticationStage(2i32);
    pub const SuspendingAuthentication: SecondaryAuthenticationFactorAuthenticationStage = SecondaryAuthenticationFactorAuthenticationStage(3i32);
    pub const CredentialCollected: SecondaryAuthenticationFactorAuthenticationStage = SecondaryAuthenticationFactorAuthenticationStage(4i32);
    pub const CredentialAuthenticated: SecondaryAuthenticationFactorAuthenticationStage = SecondaryAuthenticationFactorAuthenticationStage(5i32);
    pub const StoppingAuthentication: SecondaryAuthenticationFactorAuthenticationStage = SecondaryAuthenticationFactorAuthenticationStage(6i32);
    pub const ReadyForLock: SecondaryAuthenticationFactorAuthenticationStage = SecondaryAuthenticationFactorAuthenticationStage(7i32);
    pub const CheckingDevicePresence: SecondaryAuthenticationFactorAuthenticationStage = SecondaryAuthenticationFactorAuthenticationStage(8i32);
}
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorAuthenticationStageInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorAuthenticationStatus(pub i32);
impl SecondaryAuthenticationFactorAuthenticationStatus {
    pub const Failed: SecondaryAuthenticationFactorAuthenticationStatus = SecondaryAuthenticationFactorAuthenticationStatus(0i32);
    pub const Started: SecondaryAuthenticationFactorAuthenticationStatus = SecondaryAuthenticationFactorAuthenticationStatus(1i32);
    pub const UnknownDevice: SecondaryAuthenticationFactorAuthenticationStatus = SecondaryAuthenticationFactorAuthenticationStatus(2i32);
    pub const DisabledByPolicy: SecondaryAuthenticationFactorAuthenticationStatus = SecondaryAuthenticationFactorAuthenticationStatus(3i32);
    pub const InvalidAuthenticationStage: SecondaryAuthenticationFactorAuthenticationStatus = SecondaryAuthenticationFactorAuthenticationStatus(4i32);
}
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorDeviceCapabilities(pub u32);
impl SecondaryAuthenticationFactorDeviceCapabilities {
    pub const None: SecondaryAuthenticationFactorDeviceCapabilities = SecondaryAuthenticationFactorDeviceCapabilities(0u32);
    pub const SecureStorage: SecondaryAuthenticationFactorDeviceCapabilities = SecondaryAuthenticationFactorDeviceCapabilities(1u32);
    pub const StoreKeys: SecondaryAuthenticationFactorDeviceCapabilities = SecondaryAuthenticationFactorDeviceCapabilities(2u32);
    pub const ConfirmUserIntentToAuthenticate: SecondaryAuthenticationFactorDeviceCapabilities = SecondaryAuthenticationFactorDeviceCapabilities(4u32);
    pub const SupportSecureUserPresenceCheck: SecondaryAuthenticationFactorDeviceCapabilities = SecondaryAuthenticationFactorDeviceCapabilities(8u32);
    pub const TransmittedDataIsEncrypted: SecondaryAuthenticationFactorDeviceCapabilities = SecondaryAuthenticationFactorDeviceCapabilities(16u32);
    pub const HMacSha256: SecondaryAuthenticationFactorDeviceCapabilities = SecondaryAuthenticationFactorDeviceCapabilities(32u32);
    pub const CloseRangeDataTransmission: SecondaryAuthenticationFactorDeviceCapabilities = SecondaryAuthenticationFactorDeviceCapabilities(64u32);
}
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorDeviceFindScope(pub i32);
impl SecondaryAuthenticationFactorDeviceFindScope {
    pub const User: SecondaryAuthenticationFactorDeviceFindScope = SecondaryAuthenticationFactorDeviceFindScope(0i32);
    pub const AllUsers: SecondaryAuthenticationFactorDeviceFindScope = SecondaryAuthenticationFactorDeviceFindScope(1i32);
}
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorDevicePresence(pub i32);
impl SecondaryAuthenticationFactorDevicePresence {
    pub const Absent: SecondaryAuthenticationFactorDevicePresence = SecondaryAuthenticationFactorDevicePresence(0i32);
    pub const Present: SecondaryAuthenticationFactorDevicePresence = SecondaryAuthenticationFactorDevicePresence(1i32);
}
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorDevicePresenceMonitoringMode(pub i32);
impl SecondaryAuthenticationFactorDevicePresenceMonitoringMode {
    pub const Unsupported: SecondaryAuthenticationFactorDevicePresenceMonitoringMode = SecondaryAuthenticationFactorDevicePresenceMonitoringMode(0i32);
    pub const AppManaged: SecondaryAuthenticationFactorDevicePresenceMonitoringMode = SecondaryAuthenticationFactorDevicePresenceMonitoringMode(1i32);
    pub const SystemManaged: SecondaryAuthenticationFactorDevicePresenceMonitoringMode = SecondaryAuthenticationFactorDevicePresenceMonitoringMode(2i32);
}
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus(pub i32);
impl SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus {
    pub const Unsupported: SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus = SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus(0i32);
    pub const Succeeded: SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus = SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus(1i32);
    pub const DisabledByPolicy: SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus = SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus(2i32);
}
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorFinishAuthenticationStatus(pub i32);
impl SecondaryAuthenticationFactorFinishAuthenticationStatus {
    pub const Failed: SecondaryAuthenticationFactorFinishAuthenticationStatus = SecondaryAuthenticationFactorFinishAuthenticationStatus(0i32);
    pub const Completed: SecondaryAuthenticationFactorFinishAuthenticationStatus = SecondaryAuthenticationFactorFinishAuthenticationStatus(1i32);
    pub const NonceExpired: SecondaryAuthenticationFactorFinishAuthenticationStatus = SecondaryAuthenticationFactorFinishAuthenticationStatus(2i32);
}
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorRegistration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorRegistrationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorRegistrationStatus(pub i32);
impl SecondaryAuthenticationFactorRegistrationStatus {
    pub const Failed: SecondaryAuthenticationFactorRegistrationStatus = SecondaryAuthenticationFactorRegistrationStatus(0i32);
    pub const Started: SecondaryAuthenticationFactorRegistrationStatus = SecondaryAuthenticationFactorRegistrationStatus(1i32);
    pub const CanceledByUser: SecondaryAuthenticationFactorRegistrationStatus = SecondaryAuthenticationFactorRegistrationStatus(2i32);
    pub const PinSetupRequired: SecondaryAuthenticationFactorRegistrationStatus = SecondaryAuthenticationFactorRegistrationStatus(3i32);
    pub const DisabledByPolicy: SecondaryAuthenticationFactorRegistrationStatus = SecondaryAuthenticationFactorRegistrationStatus(4i32);
}
