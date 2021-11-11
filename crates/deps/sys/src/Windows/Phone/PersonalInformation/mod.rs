#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Phone_PersonalInformation_Provisioning")]
pub mod Provisioning;
#[link(name = "windows")]
extern "system" {
    fn ContactAddress();
    fn ContactChangeRecord();
    fn ContactChangeType();
    fn ContactInformation();
    fn ContactQueryOptions();
    fn ContactQueryResult();
    fn ContactQueryResultOrdering();
    fn ContactStore();
    fn ContactStoreApplicationAccessMode();
    fn ContactStoreSystemAccessMode();
    fn IContactAddress();
    fn IContactChangeRecord();
    fn IContactInformation();
    fn IContactInformation2();
    fn IContactInformationStatics();
    fn IContactQueryOptions();
    fn IContactQueryResult();
    fn IContactStore();
    fn IContactStore2();
    fn IContactStoreStatics();
    fn IKnownContactPropertiesStatics();
    fn IStoredContact();
    fn IStoredContactFactory();
    fn KnownContactProperties();
    fn StoredContact();
    fn VCardFormat();
}
