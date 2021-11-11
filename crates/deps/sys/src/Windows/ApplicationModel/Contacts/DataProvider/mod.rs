#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn ContactDataProviderConnection();
    fn ContactDataProviderTriggerDetails();
    fn ContactListCreateOrUpdateContactRequest();
    fn ContactListCreateOrUpdateContactRequestEventArgs();
    fn ContactListDeleteContactRequest();
    fn ContactListDeleteContactRequestEventArgs();
    fn ContactListServerSearchReadBatchRequest();
    fn ContactListServerSearchReadBatchRequestEventArgs();
    fn ContactListSyncManagerSyncRequest();
    fn ContactListSyncManagerSyncRequestEventArgs();
    fn IContactDataProviderConnection();
    fn IContactDataProviderConnection2();
    fn IContactDataProviderTriggerDetails();
    fn IContactListCreateOrUpdateContactRequest();
    fn IContactListCreateOrUpdateContactRequestEventArgs();
    fn IContactListDeleteContactRequest();
    fn IContactListDeleteContactRequestEventArgs();
    fn IContactListServerSearchReadBatchRequest();
    fn IContactListServerSearchReadBatchRequestEventArgs();
    fn IContactListSyncManagerSyncRequest();
    fn IContactListSyncManagerSyncRequestEventArgs();
}
