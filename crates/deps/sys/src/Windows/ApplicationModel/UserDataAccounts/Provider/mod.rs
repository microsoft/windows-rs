#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn IUserDataAccountPartnerAccountInfo();
    fn IUserDataAccountProviderAddAccountOperation();
    fn IUserDataAccountProviderOperation();
    fn IUserDataAccountProviderResolveErrorsOperation();
    fn IUserDataAccountProviderSettingsOperation();
    fn UserDataAccountPartnerAccountInfo();
    fn UserDataAccountProviderAddAccountOperation();
    fn UserDataAccountProviderOperationKind();
    fn UserDataAccountProviderPartnerAccountKind();
    fn UserDataAccountProviderResolveErrorsOperation();
    fn UserDataAccountProviderSettingsOperation();
}
