#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn ContactPartnerProvisioningManager();
    fn IContactPartnerProvisioningManagerStatics();
    fn IContactPartnerProvisioningManagerStatics2();
    fn IMessagePartnerProvisioningManagerStatics();
    fn MessagePartnerProvisioningManager();
}
