#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "ApplicationModel_UserDataAccounts_Provider")]
pub mod Provider;
#[cfg(feature = "ApplicationModel_UserDataAccounts_SystemAccess")]
pub mod SystemAccess;
#[link(name = "windows")]
extern "system" {
    fn IUserDataAccount();
    fn IUserDataAccount2();
    fn IUserDataAccount3();
    fn IUserDataAccount4();
    fn IUserDataAccountManagerForUser();
    fn IUserDataAccountManagerStatics();
    fn IUserDataAccountManagerStatics2();
    fn IUserDataAccountStore();
    fn IUserDataAccountStore2();
    fn IUserDataAccountStore3();
    fn IUserDataAccountStoreChangedEventArgs();
    fn UserDataAccount();
    fn UserDataAccountContentKinds();
    fn UserDataAccountManager();
    fn UserDataAccountManagerForUser();
    fn UserDataAccountOtherAppReadAccess();
    fn UserDataAccountStore();
    fn UserDataAccountStoreAccessType();
    fn UserDataAccountStoreChangedEventArgs();
}
