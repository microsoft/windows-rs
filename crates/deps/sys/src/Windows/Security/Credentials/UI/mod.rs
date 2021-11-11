#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn AuthenticationProtocol();
    fn CredentialPicker();
    fn CredentialPickerOptions();
    fn CredentialPickerResults();
    fn CredentialSaveOption();
    fn ICredentialPickerOptions();
    fn ICredentialPickerResults();
    fn ICredentialPickerStatics();
    fn IUserConsentVerifierStatics();
    fn UserConsentVerificationResult();
    fn UserConsentVerifier();
    fn UserConsentVerifierAvailability();
}
