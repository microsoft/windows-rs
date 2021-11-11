#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn IMdmAllowPolicyStatics();
    fn IMdmPolicyStatics2();
    fn IWorkplaceSettingsStatics();
    fn MdmPolicy();
    fn MessagingSyncPolicy();
    fn WorkplaceSettings();
    fn WorkplaceSettingsContract();
}
