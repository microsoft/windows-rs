#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn FindAllAccountsResult();
    fn FindAllWebAccountsStatus();
    fn IFindAllAccountsResult();
    fn IWebAccountEventArgs();
    fn IWebAccountMonitor();
    fn IWebAccountMonitor2();
    fn IWebAuthenticationCoreManagerStatics();
    fn IWebAuthenticationCoreManagerStatics2();
    fn IWebAuthenticationCoreManagerStatics3();
    fn IWebAuthenticationCoreManagerStatics4();
    fn IWebProviderError();
    fn IWebProviderErrorFactory();
    fn IWebTokenRequest();
    fn IWebTokenRequest2();
    fn IWebTokenRequest3();
    fn IWebTokenRequestFactory();
    fn IWebTokenRequestResult();
    fn IWebTokenResponse();
    fn IWebTokenResponseFactory();
    fn WebAccountEventArgs();
    fn WebAccountMonitor();
    fn WebAuthenticationCoreManager();
    fn WebProviderError();
    fn WebTokenRequest();
    fn WebTokenRequestPromptType();
    fn WebTokenRequestResult();
    fn WebTokenRequestStatus();
    fn WebTokenResponse();
}
