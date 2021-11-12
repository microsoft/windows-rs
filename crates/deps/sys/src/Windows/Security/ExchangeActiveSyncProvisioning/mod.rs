#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct EasClientDeviceInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EasClientSecurityPolicy(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EasComplianceResults(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct EasContract(i32);
#[repr(transparent)]
pub struct EasDisallowConvenienceLogonResult(pub i32);
impl EasDisallowConvenienceLogonResult {
    pub const NotEvaluated: EasDisallowConvenienceLogonResult = EasDisallowConvenienceLogonResult(0i32);
    pub const Compliant: EasDisallowConvenienceLogonResult = EasDisallowConvenienceLogonResult(1i32);
    pub const CanBeCompliant: EasDisallowConvenienceLogonResult = EasDisallowConvenienceLogonResult(2i32);
    pub const RequestedPolicyIsStricter: EasDisallowConvenienceLogonResult = EasDisallowConvenienceLogonResult(3i32);
}
#[repr(transparent)]
pub struct EasEncryptionProviderType(pub i32);
impl EasEncryptionProviderType {
    pub const NotEvaluated: EasEncryptionProviderType = EasEncryptionProviderType(0i32);
    pub const WindowsEncryption: EasEncryptionProviderType = EasEncryptionProviderType(1i32);
    pub const OtherEncryption: EasEncryptionProviderType = EasEncryptionProviderType(2i32);
}
#[repr(transparent)]
pub struct EasMaxInactivityTimeLockResult(pub i32);
impl EasMaxInactivityTimeLockResult {
    pub const NotEvaluated: EasMaxInactivityTimeLockResult = EasMaxInactivityTimeLockResult(0i32);
    pub const Compliant: EasMaxInactivityTimeLockResult = EasMaxInactivityTimeLockResult(1i32);
    pub const CanBeCompliant: EasMaxInactivityTimeLockResult = EasMaxInactivityTimeLockResult(2i32);
    pub const RequestedPolicyIsStricter: EasMaxInactivityTimeLockResult = EasMaxInactivityTimeLockResult(3i32);
    pub const InvalidParameter: EasMaxInactivityTimeLockResult = EasMaxInactivityTimeLockResult(4i32);
}
#[repr(transparent)]
pub struct EasMaxPasswordFailedAttemptsResult(pub i32);
impl EasMaxPasswordFailedAttemptsResult {
    pub const NotEvaluated: EasMaxPasswordFailedAttemptsResult = EasMaxPasswordFailedAttemptsResult(0i32);
    pub const Compliant: EasMaxPasswordFailedAttemptsResult = EasMaxPasswordFailedAttemptsResult(1i32);
    pub const CanBeCompliant: EasMaxPasswordFailedAttemptsResult = EasMaxPasswordFailedAttemptsResult(2i32);
    pub const RequestedPolicyIsStricter: EasMaxPasswordFailedAttemptsResult = EasMaxPasswordFailedAttemptsResult(3i32);
    pub const InvalidParameter: EasMaxPasswordFailedAttemptsResult = EasMaxPasswordFailedAttemptsResult(4i32);
}
#[repr(transparent)]
pub struct EasMinPasswordComplexCharactersResult(pub i32);
impl EasMinPasswordComplexCharactersResult {
    pub const NotEvaluated: EasMinPasswordComplexCharactersResult = EasMinPasswordComplexCharactersResult(0i32);
    pub const Compliant: EasMinPasswordComplexCharactersResult = EasMinPasswordComplexCharactersResult(1i32);
    pub const CanBeCompliant: EasMinPasswordComplexCharactersResult = EasMinPasswordComplexCharactersResult(2i32);
    pub const RequestedPolicyIsStricter: EasMinPasswordComplexCharactersResult = EasMinPasswordComplexCharactersResult(3i32);
    pub const RequestedPolicyNotEnforceable: EasMinPasswordComplexCharactersResult = EasMinPasswordComplexCharactersResult(4i32);
    pub const InvalidParameter: EasMinPasswordComplexCharactersResult = EasMinPasswordComplexCharactersResult(5i32);
    pub const CurrentUserHasBlankPassword: EasMinPasswordComplexCharactersResult = EasMinPasswordComplexCharactersResult(6i32);
    pub const AdminsHaveBlankPassword: EasMinPasswordComplexCharactersResult = EasMinPasswordComplexCharactersResult(7i32);
    pub const UserCannotChangePassword: EasMinPasswordComplexCharactersResult = EasMinPasswordComplexCharactersResult(8i32);
    pub const AdminsCannotChangePassword: EasMinPasswordComplexCharactersResult = EasMinPasswordComplexCharactersResult(9i32);
    pub const LocalControlledUsersCannotChangePassword: EasMinPasswordComplexCharactersResult = EasMinPasswordComplexCharactersResult(10i32);
    pub const ConnectedAdminsProviderPolicyIsWeak: EasMinPasswordComplexCharactersResult = EasMinPasswordComplexCharactersResult(11i32);
    pub const ConnectedUserProviderPolicyIsWeak: EasMinPasswordComplexCharactersResult = EasMinPasswordComplexCharactersResult(12i32);
    pub const ChangeConnectedAdminsPassword: EasMinPasswordComplexCharactersResult = EasMinPasswordComplexCharactersResult(13i32);
    pub const ChangeConnectedUserPassword: EasMinPasswordComplexCharactersResult = EasMinPasswordComplexCharactersResult(14i32);
}
#[repr(transparent)]
pub struct EasMinPasswordLengthResult(pub i32);
impl EasMinPasswordLengthResult {
    pub const NotEvaluated: EasMinPasswordLengthResult = EasMinPasswordLengthResult(0i32);
    pub const Compliant: EasMinPasswordLengthResult = EasMinPasswordLengthResult(1i32);
    pub const CanBeCompliant: EasMinPasswordLengthResult = EasMinPasswordLengthResult(2i32);
    pub const RequestedPolicyIsStricter: EasMinPasswordLengthResult = EasMinPasswordLengthResult(3i32);
    pub const RequestedPolicyNotEnforceable: EasMinPasswordLengthResult = EasMinPasswordLengthResult(4i32);
    pub const InvalidParameter: EasMinPasswordLengthResult = EasMinPasswordLengthResult(5i32);
    pub const CurrentUserHasBlankPassword: EasMinPasswordLengthResult = EasMinPasswordLengthResult(6i32);
    pub const AdminsHaveBlankPassword: EasMinPasswordLengthResult = EasMinPasswordLengthResult(7i32);
    pub const UserCannotChangePassword: EasMinPasswordLengthResult = EasMinPasswordLengthResult(8i32);
    pub const AdminsCannotChangePassword: EasMinPasswordLengthResult = EasMinPasswordLengthResult(9i32);
    pub const LocalControlledUsersCannotChangePassword: EasMinPasswordLengthResult = EasMinPasswordLengthResult(10i32);
    pub const ConnectedAdminsProviderPolicyIsWeak: EasMinPasswordLengthResult = EasMinPasswordLengthResult(11i32);
    pub const ConnectedUserProviderPolicyIsWeak: EasMinPasswordLengthResult = EasMinPasswordLengthResult(12i32);
    pub const ChangeConnectedAdminsPassword: EasMinPasswordLengthResult = EasMinPasswordLengthResult(13i32);
    pub const ChangeConnectedUserPassword: EasMinPasswordLengthResult = EasMinPasswordLengthResult(14i32);
}
#[repr(transparent)]
pub struct EasPasswordExpirationResult(pub i32);
impl EasPasswordExpirationResult {
    pub const NotEvaluated: EasPasswordExpirationResult = EasPasswordExpirationResult(0i32);
    pub const Compliant: EasPasswordExpirationResult = EasPasswordExpirationResult(1i32);
    pub const CanBeCompliant: EasPasswordExpirationResult = EasPasswordExpirationResult(2i32);
    pub const RequestedPolicyIsStricter: EasPasswordExpirationResult = EasPasswordExpirationResult(3i32);
    pub const RequestedExpirationIncompatible: EasPasswordExpirationResult = EasPasswordExpirationResult(4i32);
    pub const InvalidParameter: EasPasswordExpirationResult = EasPasswordExpirationResult(5i32);
    pub const UserCannotChangePassword: EasPasswordExpirationResult = EasPasswordExpirationResult(6i32);
    pub const AdminsCannotChangePassword: EasPasswordExpirationResult = EasPasswordExpirationResult(7i32);
    pub const LocalControlledUsersCannotChangePassword: EasPasswordExpirationResult = EasPasswordExpirationResult(8i32);
}
#[repr(transparent)]
pub struct EasPasswordHistoryResult(pub i32);
impl EasPasswordHistoryResult {
    pub const NotEvaluated: EasPasswordHistoryResult = EasPasswordHistoryResult(0i32);
    pub const Compliant: EasPasswordHistoryResult = EasPasswordHistoryResult(1i32);
    pub const CanBeCompliant: EasPasswordHistoryResult = EasPasswordHistoryResult(2i32);
    pub const RequestedPolicyIsStricter: EasPasswordHistoryResult = EasPasswordHistoryResult(3i32);
    pub const InvalidParameter: EasPasswordHistoryResult = EasPasswordHistoryResult(4i32);
}
#[repr(transparent)]
pub struct EasRequireEncryptionResult(pub i32);
impl EasRequireEncryptionResult {
    pub const NotEvaluated: EasRequireEncryptionResult = EasRequireEncryptionResult(0i32);
    pub const Compliant: EasRequireEncryptionResult = EasRequireEncryptionResult(1i32);
    pub const CanBeCompliant: EasRequireEncryptionResult = EasRequireEncryptionResult(2i32);
    pub const NotProvisionedOnAllVolumes: EasRequireEncryptionResult = EasRequireEncryptionResult(3i32);
    pub const DeFixedDataNotSupported: EasRequireEncryptionResult = EasRequireEncryptionResult(4i32);
    pub const FixedDataNotSupported: EasRequireEncryptionResult = EasRequireEncryptionResult(4i32);
    pub const DeHardwareNotCompliant: EasRequireEncryptionResult = EasRequireEncryptionResult(5i32);
    pub const HardwareNotCompliant: EasRequireEncryptionResult = EasRequireEncryptionResult(5i32);
    pub const DeWinReNotConfigured: EasRequireEncryptionResult = EasRequireEncryptionResult(6i32);
    pub const LockNotConfigured: EasRequireEncryptionResult = EasRequireEncryptionResult(6i32);
    pub const DeProtectionSuspended: EasRequireEncryptionResult = EasRequireEncryptionResult(7i32);
    pub const ProtectionSuspended: EasRequireEncryptionResult = EasRequireEncryptionResult(7i32);
    pub const DeOsVolumeNotProtected: EasRequireEncryptionResult = EasRequireEncryptionResult(8i32);
    pub const OsVolumeNotProtected: EasRequireEncryptionResult = EasRequireEncryptionResult(8i32);
    pub const DeProtectionNotYetEnabled: EasRequireEncryptionResult = EasRequireEncryptionResult(9i32);
    pub const ProtectionNotYetEnabled: EasRequireEncryptionResult = EasRequireEncryptionResult(9i32);
    pub const NoFeatureLicense: EasRequireEncryptionResult = EasRequireEncryptionResult(10i32);
    pub const OsNotProtected: EasRequireEncryptionResult = EasRequireEncryptionResult(11i32);
    pub const UnexpectedFailure: EasRequireEncryptionResult = EasRequireEncryptionResult(12i32);
}
#[repr(transparent)]
pub struct IEasClientDeviceInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEasClientDeviceInformation2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEasClientSecurityPolicy(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEasComplianceResults(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEasComplianceResults2(pub *mut ::core::ffi::c_void);
