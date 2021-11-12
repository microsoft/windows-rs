#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
#[repr(transparent)]
pub struct ContactAddressKind(pub i32);
impl ContactAddressKind {
    pub const Home: ContactAddressKind = ContactAddressKind(0i32);
    pub const Work: ContactAddressKind = ContactAddressKind(1i32);
    pub const Other: ContactAddressKind = ContactAddressKind(2i32);
}
#[repr(transparent)]
pub struct ContactAnnotation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactAnnotationList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactAnnotationOperations(pub u32);
impl ContactAnnotationOperations {
    pub const None: ContactAnnotationOperations = ContactAnnotationOperations(0u32);
    pub const ContactProfile: ContactAnnotationOperations = ContactAnnotationOperations(1u32);
    pub const Message: ContactAnnotationOperations = ContactAnnotationOperations(2u32);
    pub const AudioCall: ContactAnnotationOperations = ContactAnnotationOperations(4u32);
    pub const VideoCall: ContactAnnotationOperations = ContactAnnotationOperations(8u32);
    pub const SocialFeeds: ContactAnnotationOperations = ContactAnnotationOperations(16u32);
    pub const Share: ContactAnnotationOperations = ContactAnnotationOperations(32u32);
}
#[repr(transparent)]
pub struct ContactAnnotationStore(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactAnnotationStoreAccessType(pub i32);
impl ContactAnnotationStoreAccessType {
    pub const AppAnnotationsReadWrite: ContactAnnotationStoreAccessType = ContactAnnotationStoreAccessType(0i32);
    pub const AllAnnotationsReadWrite: ContactAnnotationStoreAccessType = ContactAnnotationStoreAccessType(1i32);
}
#[repr(transparent)]
pub struct ContactBatch(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactBatchStatus(pub i32);
impl ContactBatchStatus {
    pub const Success: ContactBatchStatus = ContactBatchStatus(0i32);
    pub const ServerSearchSyncManagerError: ContactBatchStatus = ContactBatchStatus(1i32);
    pub const ServerSearchUnknownError: ContactBatchStatus = ContactBatchStatus(2i32);
}
#[repr(transparent)]
pub struct ContactCardDelayedDataLoader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactCardHeaderKind(pub i32);
impl ContactCardHeaderKind {
    pub const Default: ContactCardHeaderKind = ContactCardHeaderKind(0i32);
    pub const Basic: ContactCardHeaderKind = ContactCardHeaderKind(1i32);
    pub const Enterprise: ContactCardHeaderKind = ContactCardHeaderKind(2i32);
}
#[repr(transparent)]
pub struct ContactCardOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactCardTabKind(pub i32);
impl ContactCardTabKind {
    pub const Default: ContactCardTabKind = ContactCardTabKind(0i32);
    pub const Email: ContactCardTabKind = ContactCardTabKind(1i32);
    pub const Messaging: ContactCardTabKind = ContactCardTabKind(2i32);
    pub const Phone: ContactCardTabKind = ContactCardTabKind(3i32);
    pub const Video: ContactCardTabKind = ContactCardTabKind(4i32);
    pub const OrganizationalHierarchy: ContactCardTabKind = ContactCardTabKind(5i32);
}
#[repr(transparent)]
pub struct ContactChange(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactChangeReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactChangeTracker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactChangeType(pub i32);
impl ContactChangeType {
    pub const Created: ContactChangeType = ContactChangeType(0i32);
    pub const Modified: ContactChangeType = ContactChangeType(1i32);
    pub const Deleted: ContactChangeType = ContactChangeType(2i32);
    pub const ChangeTrackingLost: ContactChangeType = ContactChangeType(3i32);
}
#[repr(transparent)]
pub struct ContactChangedDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactConnectedServiceAccount(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactDate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactDateKind(pub i32);
impl ContactDateKind {
    pub const Birthday: ContactDateKind = ContactDateKind(0i32);
    pub const Anniversary: ContactDateKind = ContactDateKind(1i32);
    pub const Other: ContactDateKind = ContactDateKind(2i32);
}
#[repr(transparent)]
pub struct ContactEmail(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactEmailKind(pub i32);
impl ContactEmailKind {
    pub const Personal: ContactEmailKind = ContactEmailKind(0i32);
    pub const Work: ContactEmailKind = ContactEmailKind(1i32);
    pub const Other: ContactEmailKind = ContactEmailKind(2i32);
}
#[repr(transparent)]
pub struct ContactField(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactFieldCategory(pub i32);
impl ContactFieldCategory {
    pub const None: ContactFieldCategory = ContactFieldCategory(0i32);
    pub const Home: ContactFieldCategory = ContactFieldCategory(1i32);
    pub const Work: ContactFieldCategory = ContactFieldCategory(2i32);
    pub const Mobile: ContactFieldCategory = ContactFieldCategory(3i32);
    pub const Other: ContactFieldCategory = ContactFieldCategory(4i32);
}
#[repr(transparent)]
pub struct ContactFieldFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactFieldType(pub i32);
impl ContactFieldType {
    pub const Email: ContactFieldType = ContactFieldType(0i32);
    pub const PhoneNumber: ContactFieldType = ContactFieldType(1i32);
    pub const Location: ContactFieldType = ContactFieldType(2i32);
    pub const InstantMessage: ContactFieldType = ContactFieldType(3i32);
    pub const Custom: ContactFieldType = ContactFieldType(4i32);
    pub const ConnectedServiceAccount: ContactFieldType = ContactFieldType(5i32);
    pub const ImportantDate: ContactFieldType = ContactFieldType(6i32);
    pub const Address: ContactFieldType = ContactFieldType(7i32);
    pub const SignificantOther: ContactFieldType = ContactFieldType(8i32);
    pub const Notes: ContactFieldType = ContactFieldType(9i32);
    pub const Website: ContactFieldType = ContactFieldType(10i32);
    pub const JobInfo: ContactFieldType = ContactFieldType(11i32);
}
#[repr(transparent)]
pub struct ContactGroup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactInstantMessageField(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactJobInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactListLimitedWriteOperations(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactListOtherAppReadAccess(pub i32);
impl ContactListOtherAppReadAccess {
    pub const SystemOnly: ContactListOtherAppReadAccess = ContactListOtherAppReadAccess(0i32);
    pub const Limited: ContactListOtherAppReadAccess = ContactListOtherAppReadAccess(1i32);
    pub const Full: ContactListOtherAppReadAccess = ContactListOtherAppReadAccess(2i32);
    pub const None: ContactListOtherAppReadAccess = ContactListOtherAppReadAccess(3i32);
}
#[repr(transparent)]
pub struct ContactListOtherAppWriteAccess(pub i32);
impl ContactListOtherAppWriteAccess {
    pub const None: ContactListOtherAppWriteAccess = ContactListOtherAppWriteAccess(0i32);
    pub const SystemOnly: ContactListOtherAppWriteAccess = ContactListOtherAppWriteAccess(1i32);
    pub const Limited: ContactListOtherAppWriteAccess = ContactListOtherAppWriteAccess(2i32);
}
#[repr(transparent)]
pub struct ContactListSyncConstraints(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactListSyncManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactListSyncStatus(pub i32);
impl ContactListSyncStatus {
    pub const Idle: ContactListSyncStatus = ContactListSyncStatus(0i32);
    pub const Syncing: ContactListSyncStatus = ContactListSyncStatus(1i32);
    pub const UpToDate: ContactListSyncStatus = ContactListSyncStatus(2i32);
    pub const AuthenticationError: ContactListSyncStatus = ContactListSyncStatus(3i32);
    pub const PolicyError: ContactListSyncStatus = ContactListSyncStatus(4i32);
    pub const UnknownError: ContactListSyncStatus = ContactListSyncStatus(5i32);
    pub const ManualAccountRemovalRequired: ContactListSyncStatus = ContactListSyncStatus(6i32);
}
#[repr(transparent)]
pub struct ContactLocationField(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactManagerForUser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactMatchReason(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactMatchReasonKind(pub i32);
impl ContactMatchReasonKind {
    pub const Name: ContactMatchReasonKind = ContactMatchReasonKind(0i32);
    pub const EmailAddress: ContactMatchReasonKind = ContactMatchReasonKind(1i32);
    pub const PhoneNumber: ContactMatchReasonKind = ContactMatchReasonKind(2i32);
    pub const JobInfo: ContactMatchReasonKind = ContactMatchReasonKind(3i32);
    pub const YomiName: ContactMatchReasonKind = ContactMatchReasonKind(4i32);
    pub const Other: ContactMatchReasonKind = ContactMatchReasonKind(5i32);
}
#[repr(transparent)]
pub struct ContactNameOrder(pub i32);
impl ContactNameOrder {
    pub const FirstNameLastName: ContactNameOrder = ContactNameOrder(0i32);
    pub const LastNameFirstName: ContactNameOrder = ContactNameOrder(1i32);
}
#[repr(transparent)]
pub struct ContactPanel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactPanelClosingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactPanelLaunchFullAppRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactPhone(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactPhoneKind(pub i32);
impl ContactPhoneKind {
    pub const Home: ContactPhoneKind = ContactPhoneKind(0i32);
    pub const Mobile: ContactPhoneKind = ContactPhoneKind(1i32);
    pub const Work: ContactPhoneKind = ContactPhoneKind(2i32);
    pub const Other: ContactPhoneKind = ContactPhoneKind(3i32);
    pub const Pager: ContactPhoneKind = ContactPhoneKind(4i32);
    pub const BusinessFax: ContactPhoneKind = ContactPhoneKind(5i32);
    pub const HomeFax: ContactPhoneKind = ContactPhoneKind(6i32);
    pub const Company: ContactPhoneKind = ContactPhoneKind(7i32);
    pub const Assistant: ContactPhoneKind = ContactPhoneKind(8i32);
    pub const Radio: ContactPhoneKind = ContactPhoneKind(9i32);
}
#[repr(transparent)]
pub struct ContactPicker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactQueryDesiredFields(pub u32);
impl ContactQueryDesiredFields {
    pub const None: ContactQueryDesiredFields = ContactQueryDesiredFields(0u32);
    pub const PhoneNumber: ContactQueryDesiredFields = ContactQueryDesiredFields(1u32);
    pub const EmailAddress: ContactQueryDesiredFields = ContactQueryDesiredFields(2u32);
    pub const PostalAddress: ContactQueryDesiredFields = ContactQueryDesiredFields(4u32);
}
#[repr(transparent)]
pub struct ContactQueryOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactQuerySearchFields(pub u32);
impl ContactQuerySearchFields {
    pub const None: ContactQuerySearchFields = ContactQuerySearchFields(0u32);
    pub const Name: ContactQuerySearchFields = ContactQuerySearchFields(1u32);
    pub const Email: ContactQuerySearchFields = ContactQuerySearchFields(2u32);
    pub const Phone: ContactQuerySearchFields = ContactQuerySearchFields(4u32);
    pub const All: ContactQuerySearchFields = ContactQuerySearchFields(4294967295u32);
}
#[repr(transparent)]
pub struct ContactQuerySearchScope(pub i32);
impl ContactQuerySearchScope {
    pub const Local: ContactQuerySearchScope = ContactQuerySearchScope(0i32);
    pub const Server: ContactQuerySearchScope = ContactQuerySearchScope(1i32);
}
#[repr(transparent)]
pub struct ContactQueryTextSearch(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactRelationship(pub i32);
impl ContactRelationship {
    pub const Other: ContactRelationship = ContactRelationship(0i32);
    pub const Spouse: ContactRelationship = ContactRelationship(1i32);
    pub const Partner: ContactRelationship = ContactRelationship(2i32);
    pub const Sibling: ContactRelationship = ContactRelationship(3i32);
    pub const Parent: ContactRelationship = ContactRelationship(4i32);
    pub const Child: ContactRelationship = ContactRelationship(5i32);
}
#[repr(transparent)]
pub struct ContactSelectionMode(pub i32);
impl ContactSelectionMode {
    pub const Contacts: ContactSelectionMode = ContactSelectionMode(0i32);
    pub const Fields: ContactSelectionMode = ContactSelectionMode(1i32);
}
#[repr(transparent)]
pub struct ContactSignificantOther(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactStore(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactStoreAccessType(pub i32);
impl ContactStoreAccessType {
    pub const AppContactsReadWrite: ContactStoreAccessType = ContactStoreAccessType(0i32);
    pub const AllContactsReadOnly: ContactStoreAccessType = ContactStoreAccessType(1i32);
    pub const AllContactsReadWrite: ContactStoreAccessType = ContactStoreAccessType(2i32);
}
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
pub struct PinnedContactIdsQueryResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PinnedContactManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PinnedContactSurface(pub i32);
impl PinnedContactSurface {
    pub const StartMenu: PinnedContactSurface = PinnedContactSurface(0i32);
    pub const Taskbar: PinnedContactSurface = PinnedContactSurface(1i32);
}
