#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct ACCOUNT_STATE(i32);
pub struct AsyncIAssociatedIdentityProvider(i32);
pub struct AsyncIConnectedIdentityProvider(i32);
pub struct AsyncIIdentityAdvise(i32);
pub struct AsyncIIdentityAuthentication(i32);
pub struct AsyncIIdentityProvider(i32);
pub struct AsyncIIdentityStore(i32);
pub struct AsyncIIdentityStoreEx(i32);
pub struct CIdentityProfileHandler(i32);
pub struct CoClassIdentityStore(i32);
pub struct IAssociatedIdentityProvider(i32);
pub struct IConnectedIdentityProvider(i32);
pub struct IDENTITY_TYPE(i32);
pub struct IDENTITY_URL(i32);
pub struct IIdentityAdvise(i32);
pub struct IIdentityAuthentication(i32);
pub struct IIdentityProvider(i32);
pub struct IIdentityStore(i32);
pub struct IIdentityStoreEx(i32);
pub struct IdentityUpdateEvent(i32);
pub const OID_OAssociatedIdentityProviderObject: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2563089373,
    data2: 56168,
    data3: 20250,
    data4: [141, 43, 144, 121, 205, 254, 175, 97],
};
