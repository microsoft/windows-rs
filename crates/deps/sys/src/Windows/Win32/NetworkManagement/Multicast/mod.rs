#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn McastApiCleanup();
    fn McastApiStartup();
    fn McastEnumerateScopes();
    fn McastGenUID();
    fn McastReleaseAddress();
    fn McastRenewAddress();
    fn McastRequestAddress();
}
