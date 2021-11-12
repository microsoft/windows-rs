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
    pub const Home: Self = Self(0i32);
    pub const Work: Self = Self(1i32);
    pub const Other: Self = Self(2i32);
}
#[repr(transparent)]
pub struct ContactAnnotation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactAnnotationList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactAnnotationOperations(pub u32);
impl ContactAnnotationOperations {
    pub const None: Self = Self(0u32);
    pub const ContactProfile: Self = Self(1u32);
    pub const Message: Self = Self(2u32);
    pub const AudioCall: Self = Self(4u32);
    pub const VideoCall: Self = Self(8u32);
    pub const SocialFeeds: Self = Self(16u32);
    pub const Share: Self = Self(32u32);
}
#[repr(transparent)]
pub struct ContactAnnotationStore(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactAnnotationStoreAccessType(pub i32);
impl ContactAnnotationStoreAccessType {
    pub const AppAnnotationsReadWrite: Self = Self(0i32);
    pub const AllAnnotationsReadWrite: Self = Self(1i32);
}
#[repr(transparent)]
pub struct ContactBatch(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactBatchStatus(pub i32);
impl ContactBatchStatus {
    pub const Success: Self = Self(0i32);
    pub const ServerSearchSyncManagerError: Self = Self(1i32);
    pub const ServerSearchUnknownError: Self = Self(2i32);
}
#[repr(transparent)]
pub struct ContactCardDelayedDataLoader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactCardHeaderKind(pub i32);
impl ContactCardHeaderKind {
    pub const Default: Self = Self(0i32);
    pub const Basic: Self = Self(1i32);
    pub const Enterprise: Self = Self(2i32);
}
#[repr(transparent)]
pub struct ContactCardOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactCardTabKind(pub i32);
impl ContactCardTabKind {
    pub const Default: Self = Self(0i32);
    pub const Email: Self = Self(1i32);
    pub const Messaging: Self = Self(2i32);
    pub const Phone: Self = Self(3i32);
    pub const Video: Self = Self(4i32);
    pub const OrganizationalHierarchy: Self = Self(5i32);
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
    pub const Created: Self = Self(0i32);
    pub const Modified: Self = Self(1i32);
    pub const Deleted: Self = Self(2i32);
    pub const ChangeTrackingLost: Self = Self(3i32);
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
    pub const Birthday: Self = Self(0i32);
    pub const Anniversary: Self = Self(1i32);
    pub const Other: Self = Self(2i32);
}
#[repr(transparent)]
pub struct ContactEmail(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactEmailKind(pub i32);
impl ContactEmailKind {
    pub const Personal: Self = Self(0i32);
    pub const Work: Self = Self(1i32);
    pub const Other: Self = Self(2i32);
}
#[repr(transparent)]
pub struct ContactField(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactFieldCategory(pub i32);
impl ContactFieldCategory {
    pub const None: Self = Self(0i32);
    pub const Home: Self = Self(1i32);
    pub const Work: Self = Self(2i32);
    pub const Mobile: Self = Self(3i32);
    pub const Other: Self = Self(4i32);
}
#[repr(transparent)]
pub struct ContactFieldFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactFieldType(pub i32);
impl ContactFieldType {
    pub const Email: Self = Self(0i32);
    pub const PhoneNumber: Self = Self(1i32);
    pub const Location: Self = Self(2i32);
    pub const InstantMessage: Self = Self(3i32);
    pub const Custom: Self = Self(4i32);
    pub const ConnectedServiceAccount: Self = Self(5i32);
    pub const ImportantDate: Self = Self(6i32);
    pub const Address: Self = Self(7i32);
    pub const SignificantOther: Self = Self(8i32);
    pub const Notes: Self = Self(9i32);
    pub const Website: Self = Self(10i32);
    pub const JobInfo: Self = Self(11i32);
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
    pub const SystemOnly: Self = Self(0i32);
    pub const Limited: Self = Self(1i32);
    pub const Full: Self = Self(2i32);
    pub const None: Self = Self(3i32);
}
#[repr(transparent)]
pub struct ContactListOtherAppWriteAccess(pub i32);
impl ContactListOtherAppWriteAccess {
    pub const None: Self = Self(0i32);
    pub const SystemOnly: Self = Self(1i32);
    pub const Limited: Self = Self(2i32);
}
#[repr(transparent)]
pub struct ContactListSyncConstraints(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactListSyncManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactListSyncStatus(pub i32);
impl ContactListSyncStatus {
    pub const Idle: Self = Self(0i32);
    pub const Syncing: Self = Self(1i32);
    pub const UpToDate: Self = Self(2i32);
    pub const AuthenticationError: Self = Self(3i32);
    pub const PolicyError: Self = Self(4i32);
    pub const UnknownError: Self = Self(5i32);
    pub const ManualAccountRemovalRequired: Self = Self(6i32);
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
    pub const Name: Self = Self(0i32);
    pub const EmailAddress: Self = Self(1i32);
    pub const PhoneNumber: Self = Self(2i32);
    pub const JobInfo: Self = Self(3i32);
    pub const YomiName: Self = Self(4i32);
    pub const Other: Self = Self(5i32);
}
#[repr(transparent)]
pub struct ContactNameOrder(pub i32);
impl ContactNameOrder {
    pub const FirstNameLastName: Self = Self(0i32);
    pub const LastNameFirstName: Self = Self(1i32);
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
    pub const Home: Self = Self(0i32);
    pub const Mobile: Self = Self(1i32);
    pub const Work: Self = Self(2i32);
    pub const Other: Self = Self(3i32);
    pub const Pager: Self = Self(4i32);
    pub const BusinessFax: Self = Self(5i32);
    pub const HomeFax: Self = Self(6i32);
    pub const Company: Self = Self(7i32);
    pub const Assistant: Self = Self(8i32);
    pub const Radio: Self = Self(9i32);
}
#[repr(transparent)]
pub struct ContactPicker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactQueryDesiredFields(pub u32);
impl ContactQueryDesiredFields {
    pub const None: Self = Self(0u32);
    pub const PhoneNumber: Self = Self(1u32);
    pub const EmailAddress: Self = Self(2u32);
    pub const PostalAddress: Self = Self(4u32);
}
#[repr(transparent)]
pub struct ContactQueryOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactQuerySearchFields(pub u32);
impl ContactQuerySearchFields {
    pub const None: Self = Self(0u32);
    pub const Name: Self = Self(1u32);
    pub const Email: Self = Self(2u32);
    pub const Phone: Self = Self(4u32);
    pub const All: Self = Self(4294967295u32);
}
#[repr(transparent)]
pub struct ContactQuerySearchScope(pub i32);
impl ContactQuerySearchScope {
    pub const Local: Self = Self(0i32);
    pub const Server: Self = Self(1i32);
}
#[repr(transparent)]
pub struct ContactQueryTextSearch(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactRelationship(pub i32);
impl ContactRelationship {
    pub const Other: Self = Self(0i32);
    pub const Spouse: Self = Self(1i32);
    pub const Partner: Self = Self(2i32);
    pub const Sibling: Self = Self(3i32);
    pub const Parent: Self = Self(4i32);
    pub const Child: Self = Self(5i32);
}
#[repr(transparent)]
pub struct ContactSelectionMode(pub i32);
impl ContactSelectionMode {
    pub const Contacts: Self = Self(0i32);
    pub const Fields: Self = Self(1i32);
}
#[repr(transparent)]
pub struct ContactSignificantOther(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactStore(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactStoreAccessType(pub i32);
impl ContactStoreAccessType {
    pub const AppContactsReadWrite: Self = Self(0i32);
    pub const AllContactsReadOnly: Self = Self(1i32);
    pub const AllContactsReadWrite: Self = Self(2i32);
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
    pub const StartMenu: Self = Self(0i32);
    pub const Taskbar: Self = Self(1i32);
}
