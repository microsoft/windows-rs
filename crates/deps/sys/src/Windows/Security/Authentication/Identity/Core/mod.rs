#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn IMicrosoftAccountMultiFactorAuthenticationManager();
    fn IMicrosoftAccountMultiFactorAuthenticatorStatics();
    fn IMicrosoftAccountMultiFactorGetSessionsResult();
    fn IMicrosoftAccountMultiFactorOneTimeCodedInfo();
    fn IMicrosoftAccountMultiFactorSessionInfo();
    fn IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo();
    fn MicrosoftAccountMultiFactorAuthenticationManager();
    fn MicrosoftAccountMultiFactorAuthenticationType();
    fn MicrosoftAccountMultiFactorGetSessionsResult();
    fn MicrosoftAccountMultiFactorOneTimeCodedInfo();
    fn MicrosoftAccountMultiFactorServiceResponse();
    fn MicrosoftAccountMultiFactorSessionApprovalStatus();
    fn MicrosoftAccountMultiFactorSessionAuthenticationStatus();
    fn MicrosoftAccountMultiFactorSessionInfo();
    fn MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo();
}
