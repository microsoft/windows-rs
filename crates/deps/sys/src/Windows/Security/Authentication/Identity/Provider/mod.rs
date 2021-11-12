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
    pub const Invalid: Self = Self(0i32);
    pub const SwipeUpWelcome: Self = Self(1i32);
    pub const TapWelcome: Self = Self(2i32);
    pub const DeviceNeedsAttention: Self = Self(3i32);
    pub const LookingForDevice: Self = Self(4i32);
    pub const LookingForDevicePluggedin: Self = Self(5i32);
    pub const BluetoothIsDisabled: Self = Self(6i32);
    pub const NfcIsDisabled: Self = Self(7i32);
    pub const WiFiIsDisabled: Self = Self(8i32);
    pub const ExtraTapIsRequired: Self = Self(9i32);
    pub const DisabledByPolicy: Self = Self(10i32);
    pub const TapOnDeviceRequired: Self = Self(11i32);
    pub const HoldFinger: Self = Self(12i32);
    pub const ScanFinger: Self = Self(13i32);
    pub const UnauthorizedUser: Self = Self(14i32);
    pub const ReregisterRequired: Self = Self(15i32);
    pub const TryAgain: Self = Self(16i32);
    pub const SayPassphrase: Self = Self(17i32);
    pub const ReadyToSignIn: Self = Self(18i32);
    pub const UseAnotherSignInOption: Self = Self(19i32);
    pub const ConnectionRequired: Self = Self(20i32);
    pub const TimeLimitExceeded: Self = Self(21i32);
    pub const CanceledByUser: Self = Self(22i32);
    pub const CenterHand: Self = Self(23i32);
    pub const MoveHandCloser: Self = Self(24i32);
    pub const MoveHandFarther: Self = Self(25i32);
    pub const PlaceHandAbove: Self = Self(26i32);
    pub const RecognitionFailed: Self = Self(27i32);
    pub const DeviceUnavailable: Self = Self(28i32);
}
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorAuthenticationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorAuthenticationScenario(pub i32);
impl SecondaryAuthenticationFactorAuthenticationScenario {
    pub const SignIn: Self = Self(0i32);
    pub const CredentialPrompt: Self = Self(1i32);
}
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorAuthenticationStage(pub i32);
impl SecondaryAuthenticationFactorAuthenticationStage {
    pub const NotStarted: Self = Self(0i32);
    pub const WaitingForUserConfirmation: Self = Self(1i32);
    pub const CollectingCredential: Self = Self(2i32);
    pub const SuspendingAuthentication: Self = Self(3i32);
    pub const CredentialCollected: Self = Self(4i32);
    pub const CredentialAuthenticated: Self = Self(5i32);
    pub const StoppingAuthentication: Self = Self(6i32);
    pub const ReadyForLock: Self = Self(7i32);
    pub const CheckingDevicePresence: Self = Self(8i32);
}
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorAuthenticationStageInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorAuthenticationStatus(pub i32);
impl SecondaryAuthenticationFactorAuthenticationStatus {
    pub const Failed: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const UnknownDevice: Self = Self(2i32);
    pub const DisabledByPolicy: Self = Self(3i32);
    pub const InvalidAuthenticationStage: Self = Self(4i32);
}
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorDeviceCapabilities(pub u32);
impl SecondaryAuthenticationFactorDeviceCapabilities {
    pub const None: Self = Self(0u32);
    pub const SecureStorage: Self = Self(1u32);
    pub const StoreKeys: Self = Self(2u32);
    pub const ConfirmUserIntentToAuthenticate: Self = Self(4u32);
    pub const SupportSecureUserPresenceCheck: Self = Self(8u32);
    pub const TransmittedDataIsEncrypted: Self = Self(16u32);
    pub const HMacSha256: Self = Self(32u32);
    pub const CloseRangeDataTransmission: Self = Self(64u32);
}
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorDeviceFindScope(pub i32);
impl SecondaryAuthenticationFactorDeviceFindScope {
    pub const User: Self = Self(0i32);
    pub const AllUsers: Self = Self(1i32);
}
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorDevicePresence(pub i32);
impl SecondaryAuthenticationFactorDevicePresence {
    pub const Absent: Self = Self(0i32);
    pub const Present: Self = Self(1i32);
}
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorDevicePresenceMonitoringMode(pub i32);
impl SecondaryAuthenticationFactorDevicePresenceMonitoringMode {
    pub const Unsupported: Self = Self(0i32);
    pub const AppManaged: Self = Self(1i32);
    pub const SystemManaged: Self = Self(2i32);
}
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus(pub i32);
impl SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus {
    pub const Unsupported: Self = Self(0i32);
    pub const Succeeded: Self = Self(1i32);
    pub const DisabledByPolicy: Self = Self(2i32);
}
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorFinishAuthenticationStatus(pub i32);
impl SecondaryAuthenticationFactorFinishAuthenticationStatus {
    pub const Failed: Self = Self(0i32);
    pub const Completed: Self = Self(1i32);
    pub const NonceExpired: Self = Self(2i32);
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
    pub const Failed: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const CanceledByUser: Self = Self(2i32);
    pub const PinSetupRequired: Self = Self(3i32);
    pub const DisabledByPolicy: Self = Self(4i32);
}
