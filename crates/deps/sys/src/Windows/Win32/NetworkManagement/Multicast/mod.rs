#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn IPNG_ADDRESS();
    fn MCAST_API_CURRENT_VERSION();
    fn MCAST_API_VERSION_0();
    fn MCAST_API_VERSION_1();
    fn MCAST_CLIENT_ID_LEN();
    fn MCAST_CLIENT_UID();
    fn MCAST_LEASE_REQUEST();
    fn MCAST_LEASE_RESPONSE();
    fn MCAST_SCOPE_CTX();
    fn MCAST_SCOPE_ENTRY();
    fn McastApiCleanup();
    fn McastApiStartup();
    fn McastEnumerateScopes();
    fn McastGenUID();
    fn McastReleaseAddress();
    fn McastRenewAddress();
    fn McastRequestAddress();
}
