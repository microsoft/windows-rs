#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn IPlatformTelemetryClientStatics();
    fn IPlatformTelemetryRegistrationResult();
    fn IPlatformTelemetryRegistrationSettings();
    fn PlatformTelemetryClient();
    fn PlatformTelemetryRegistrationResult();
    fn PlatformTelemetryRegistrationSettings();
    fn PlatformTelemetryRegistrationStatus();
}
