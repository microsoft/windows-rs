#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "ApplicationModel_Contacts_DataProvider")]
pub mod DataProvider;
#[cfg(feature = "ApplicationModel_Contacts_Provider")]
pub mod Provider;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AggregateContactManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Contact(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactAddress(pub *mut ::core::ffi::c_void);
pub struct ContactAddressKind(i32);
#[repr(transparent)]
pub struct ContactAnnotation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactAnnotationList(pub *mut ::core::ffi::c_void);
pub struct ContactAnnotationOperations(i32);
#[repr(transparent)]
pub struct ContactAnnotationStore(pub *mut ::core::ffi::c_void);
pub struct ContactAnnotationStoreAccessType(i32);
#[repr(transparent)]
pub struct ContactBatch(pub *mut ::core::ffi::c_void);
pub struct ContactBatchStatus(i32);
#[repr(transparent)]
pub struct ContactCardDelayedDataLoader(pub *mut ::core::ffi::c_void);
pub struct ContactCardHeaderKind(i32);
#[repr(transparent)]
pub struct ContactCardOptions(pub *mut ::core::ffi::c_void);
pub struct ContactCardTabKind(i32);
#[repr(transparent)]
pub struct ContactChange(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactChangeReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactChangeTracker(pub *mut ::core::ffi::c_void);
pub struct ContactChangeType(i32);
#[repr(transparent)]
pub struct ContactChangedDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactConnectedServiceAccount(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactDate(pub *mut ::core::ffi::c_void);
pub struct ContactDateKind(i32);
#[repr(transparent)]
pub struct ContactEmail(pub *mut ::core::ffi::c_void);
pub struct ContactEmailKind(i32);
#[repr(transparent)]
pub struct ContactField(pub *mut ::core::ffi::c_void);
pub struct ContactFieldCategory(i32);
#[repr(transparent)]
pub struct ContactFieldFactory(pub *mut ::core::ffi::c_void);
pub struct ContactFieldType(i32);
#[repr(transparent)]
pub struct ContactGroup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactInstantMessageField(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactJobInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactLaunchActionVerbs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactListLimitedWriteOperations(pub *mut ::core::ffi::c_void);
pub struct ContactListOtherAppReadAccess(i32);
pub struct ContactListOtherAppWriteAccess(i32);
#[repr(transparent)]
pub struct ContactListSyncConstraints(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactListSyncManager(pub *mut ::core::ffi::c_void);
pub struct ContactListSyncStatus(i32);
#[repr(transparent)]
pub struct ContactLocationField(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactManagerForUser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactMatchReason(pub *mut ::core::ffi::c_void);
pub struct ContactMatchReasonKind(i32);
pub struct ContactNameOrder(i32);
#[repr(transparent)]
pub struct ContactPanel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactPanelClosingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactPanelLaunchFullAppRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactPhone(pub *mut ::core::ffi::c_void);
pub struct ContactPhoneKind(i32);
#[repr(transparent)]
pub struct ContactPicker(pub *mut ::core::ffi::c_void);
pub struct ContactQueryDesiredFields(i32);
#[repr(transparent)]
pub struct ContactQueryOptions(pub *mut ::core::ffi::c_void);
pub struct ContactQuerySearchFields(i32);
pub struct ContactQuerySearchScope(i32);
#[repr(transparent)]
pub struct ContactQueryTextSearch(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactReader(pub *mut ::core::ffi::c_void);
pub struct ContactRelationship(i32);
pub struct ContactSelectionMode(i32);
#[repr(transparent)]
pub struct ContactSignificantOther(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactStore(pub *mut ::core::ffi::c_void);
pub struct ContactStoreAccessType(i32);
#[repr(transparent)]
pub struct ContactStoreNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactWebsite(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FullContactCardOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAggregateContactManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAggregateContactManager2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContact(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContact2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContact3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactAddress(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactAnnotation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactAnnotation2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactAnnotationList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactAnnotationStore(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactAnnotationStore2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactBatch(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactCardDelayedDataLoader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactCardOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactCardOptions2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactChange(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactChangeReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactChangeTracker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactChangeTracker2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactChangedDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactConnectedServiceAccount(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactDate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactEmail(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactField(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactFieldFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactGroup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactInstantMessageField(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactInstantMessageFieldFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactJobInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactLaunchActionVerbsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactList2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactList3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactListLimitedWriteOperations(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactListSyncConstraints(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactListSyncManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactListSyncManager2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactLocationField(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactLocationFieldFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactManagerForUser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactManagerForUser2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactManagerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactManagerStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactManagerStatics4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactManagerStatics5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactMatchReason(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactName(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactPanel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactPanelClosingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactPanelLaunchFullAppRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactPhone(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactPicker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactPicker2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactPicker3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactPickerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactQueryOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactQueryOptionsFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactQueryTextSearch(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactSignificantOther(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactSignificantOther2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactStore(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactStore2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactStore3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactStoreNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactWebsite(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactWebsite2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFullContactCardOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKnownContactFieldStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPinnedContactIdsQueryResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPinnedContactManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPinnedContactManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct KnownContactField(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PinnedContactIdsQueryResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PinnedContactManager(pub *mut ::core::ffi::c_void);
pub struct PinnedContactSurface(i32);
