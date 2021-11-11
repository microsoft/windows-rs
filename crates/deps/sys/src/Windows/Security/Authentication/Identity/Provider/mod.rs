#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn ISecondaryAuthenticationFactorAuthentication();
    fn ISecondaryAuthenticationFactorAuthenticationResult();
    fn ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs();
    fn ISecondaryAuthenticationFactorAuthenticationStageInfo();
    fn ISecondaryAuthenticationFactorAuthenticationStatics();
    fn ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics();
    fn ISecondaryAuthenticationFactorInfo();
    fn ISecondaryAuthenticationFactorInfo2();
    fn ISecondaryAuthenticationFactorRegistration();
    fn ISecondaryAuthenticationFactorRegistrationResult();
    fn ISecondaryAuthenticationFactorRegistrationStatics();
    fn SecondaryAuthenticationFactorAuthentication();
    fn SecondaryAuthenticationFactorAuthenticationMessage();
    fn SecondaryAuthenticationFactorAuthenticationResult();
    fn SecondaryAuthenticationFactorAuthenticationScenario();
    fn SecondaryAuthenticationFactorAuthenticationStage();
    fn SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs();
    fn SecondaryAuthenticationFactorAuthenticationStageInfo();
    fn SecondaryAuthenticationFactorAuthenticationStatus();
    fn SecondaryAuthenticationFactorDeviceCapabilities();
    fn SecondaryAuthenticationFactorDeviceFindScope();
    fn SecondaryAuthenticationFactorDevicePresence();
    fn SecondaryAuthenticationFactorDevicePresenceMonitoringMode();
    fn SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus();
    fn SecondaryAuthenticationFactorFinishAuthenticationStatus();
    fn SecondaryAuthenticationFactorInfo();
    fn SecondaryAuthenticationFactorRegistration();
    fn SecondaryAuthenticationFactorRegistrationResult();
    fn SecondaryAuthenticationFactorRegistrationStatus();
}
