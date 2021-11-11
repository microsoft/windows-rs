#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn ACCOUNT_STATE();
    fn AsyncIAssociatedIdentityProvider();
    fn AsyncIConnectedIdentityProvider();
    fn AsyncIIdentityAdvise();
    fn AsyncIIdentityAuthentication();
    fn AsyncIIdentityProvider();
    fn AsyncIIdentityStore();
    fn AsyncIIdentityStoreEx();
    fn CIdentityProfileHandler();
    fn CoClassIdentityStore();
    fn IAssociatedIdentityProvider();
    fn IConnectedIdentityProvider();
    fn IDENTITY_TYPE();
    fn IDENTITY_URL();
    fn IIdentityAdvise();
    fn IIdentityAuthentication();
    fn IIdentityProvider();
    fn IIdentityStore();
    fn IIdentityStoreEx();
    fn IdentityUpdateEvent();
    fn OID_OAssociatedIdentityProviderObject();
}
