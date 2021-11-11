#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn EasClientDeviceInformation();
    fn EasClientSecurityPolicy();
    fn EasComplianceResults();
    fn EasContract();
    fn EasDisallowConvenienceLogonResult();
    fn EasEncryptionProviderType();
    fn EasMaxInactivityTimeLockResult();
    fn EasMaxPasswordFailedAttemptsResult();
    fn EasMinPasswordComplexCharactersResult();
    fn EasMinPasswordLengthResult();
    fn EasPasswordExpirationResult();
    fn EasPasswordHistoryResult();
    fn EasRequireEncryptionResult();
    fn IEasClientDeviceInformation();
    fn IEasClientDeviceInformation2();
    fn IEasClientSecurityPolicy();
    fn IEasComplianceResults();
    fn IEasComplianceResults2();
}
