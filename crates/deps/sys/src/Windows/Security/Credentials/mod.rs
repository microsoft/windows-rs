#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Security_Credentials_UI")]
pub mod UI;
#[link(name = "windows")]
extern "system" {
    fn ICredentialFactory();
    fn IKeyCredential();
    fn IKeyCredentialAttestationResult();
    fn IKeyCredentialManagerStatics();
    fn IKeyCredentialOperationResult();
    fn IKeyCredentialRetrievalResult();
    fn IPasswordCredential();
    fn IPasswordVault();
    fn IWebAccount();
    fn IWebAccount2();
    fn IWebAccountFactory();
    fn IWebAccountProvider();
    fn IWebAccountProvider2();
    fn IWebAccountProvider3();
    fn IWebAccountProvider4();
    fn IWebAccountProviderFactory();
    fn KeyCredential();
    fn KeyCredentialAttestationResult();
    fn KeyCredentialAttestationStatus();
    fn KeyCredentialCreationOption();
    fn KeyCredentialManager();
    fn KeyCredentialOperationResult();
    fn KeyCredentialRetrievalResult();
    fn KeyCredentialStatus();
    fn PasswordCredential();
    fn PasswordCredentialPropertyStore();
    fn PasswordVault();
    fn WebAccount();
    fn WebAccountPictureSize();
    fn WebAccountProvider();
    fn WebAccountState();
}
