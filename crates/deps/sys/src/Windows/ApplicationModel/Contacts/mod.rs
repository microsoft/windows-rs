#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "ApplicationModel_Contacts_DataProvider")]
pub mod DataProvider;
#[cfg(feature = "ApplicationModel_Contacts_Provider")]
pub mod Provider;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AggregateContactManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AggregateContactManager {}
impl ::core::clone::Clone for AggregateContactManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Contact(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Contact {}
impl ::core::clone::Clone for Contact {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactAddress(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactAddress {}
impl ::core::clone::Clone for ContactAddress {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactAddressKind(pub i32);
impl ContactAddressKind {
    pub const Home: Self = Self(0i32);
    pub const Work: Self = Self(1i32);
    pub const Other: Self = Self(2i32);
}
impl ::core::marker::Copy for ContactAddressKind {}
impl ::core::clone::Clone for ContactAddressKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactAnnotation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactAnnotation {}
impl ::core::clone::Clone for ContactAnnotation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactAnnotationList(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactAnnotationList {}
impl ::core::clone::Clone for ContactAnnotationList {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for ContactAnnotationOperations {}
impl ::core::clone::Clone for ContactAnnotationOperations {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactAnnotationStore(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactAnnotationStore {}
impl ::core::clone::Clone for ContactAnnotationStore {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactAnnotationStoreAccessType(pub i32);
impl ContactAnnotationStoreAccessType {
    pub const AppAnnotationsReadWrite: Self = Self(0i32);
    pub const AllAnnotationsReadWrite: Self = Self(1i32);
}
impl ::core::marker::Copy for ContactAnnotationStoreAccessType {}
impl ::core::clone::Clone for ContactAnnotationStoreAccessType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactBatch(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactBatch {}
impl ::core::clone::Clone for ContactBatch {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactBatchStatus(pub i32);
impl ContactBatchStatus {
    pub const Success: Self = Self(0i32);
    pub const ServerSearchSyncManagerError: Self = Self(1i32);
    pub const ServerSearchUnknownError: Self = Self(2i32);
}
impl ::core::marker::Copy for ContactBatchStatus {}
impl ::core::clone::Clone for ContactBatchStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactCardDelayedDataLoader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactCardDelayedDataLoader {}
impl ::core::clone::Clone for ContactCardDelayedDataLoader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactCardHeaderKind(pub i32);
impl ContactCardHeaderKind {
    pub const Default: Self = Self(0i32);
    pub const Basic: Self = Self(1i32);
    pub const Enterprise: Self = Self(2i32);
}
impl ::core::marker::Copy for ContactCardHeaderKind {}
impl ::core::clone::Clone for ContactCardHeaderKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactCardOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactCardOptions {}
impl ::core::clone::Clone for ContactCardOptions {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for ContactCardTabKind {}
impl ::core::clone::Clone for ContactCardTabKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactChange(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactChange {}
impl ::core::clone::Clone for ContactChange {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactChangeReader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactChangeReader {}
impl ::core::clone::Clone for ContactChangeReader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactChangeTracker(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactChangeTracker {}
impl ::core::clone::Clone for ContactChangeTracker {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactChangeType(pub i32);
impl ContactChangeType {
    pub const Created: Self = Self(0i32);
    pub const Modified: Self = Self(1i32);
    pub const Deleted: Self = Self(2i32);
    pub const ChangeTrackingLost: Self = Self(3i32);
}
impl ::core::marker::Copy for ContactChangeType {}
impl ::core::clone::Clone for ContactChangeType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactChangedDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactChangedDeferral {}
impl ::core::clone::Clone for ContactChangedDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactChangedEventArgs {}
impl ::core::clone::Clone for ContactChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactConnectedServiceAccount(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactConnectedServiceAccount {}
impl ::core::clone::Clone for ContactConnectedServiceAccount {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactDate(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactDate {}
impl ::core::clone::Clone for ContactDate {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactDateKind(pub i32);
impl ContactDateKind {
    pub const Birthday: Self = Self(0i32);
    pub const Anniversary: Self = Self(1i32);
    pub const Other: Self = Self(2i32);
}
impl ::core::marker::Copy for ContactDateKind {}
impl ::core::clone::Clone for ContactDateKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactEmail(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactEmail {}
impl ::core::clone::Clone for ContactEmail {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactEmailKind(pub i32);
impl ContactEmailKind {
    pub const Personal: Self = Self(0i32);
    pub const Work: Self = Self(1i32);
    pub const Other: Self = Self(2i32);
}
impl ::core::marker::Copy for ContactEmailKind {}
impl ::core::clone::Clone for ContactEmailKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactField(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactField {}
impl ::core::clone::Clone for ContactField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactFieldCategory(pub i32);
impl ContactFieldCategory {
    pub const None: Self = Self(0i32);
    pub const Home: Self = Self(1i32);
    pub const Work: Self = Self(2i32);
    pub const Mobile: Self = Self(3i32);
    pub const Other: Self = Self(4i32);
}
impl ::core::marker::Copy for ContactFieldCategory {}
impl ::core::clone::Clone for ContactFieldCategory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactFieldFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactFieldFactory {}
impl ::core::clone::Clone for ContactFieldFactory {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for ContactFieldType {}
impl ::core::clone::Clone for ContactFieldType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactGroup(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactGroup {}
impl ::core::clone::Clone for ContactGroup {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactInformation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactInformation {}
impl ::core::clone::Clone for ContactInformation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactInstantMessageField(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactInstantMessageField {}
impl ::core::clone::Clone for ContactInstantMessageField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactJobInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactJobInfo {}
impl ::core::clone::Clone for ContactJobInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactList(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactList {}
impl ::core::clone::Clone for ContactList {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactListLimitedWriteOperations(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactListLimitedWriteOperations {}
impl ::core::clone::Clone for ContactListLimitedWriteOperations {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactListOtherAppReadAccess(pub i32);
impl ContactListOtherAppReadAccess {
    pub const SystemOnly: Self = Self(0i32);
    pub const Limited: Self = Self(1i32);
    pub const Full: Self = Self(2i32);
    pub const None: Self = Self(3i32);
}
impl ::core::marker::Copy for ContactListOtherAppReadAccess {}
impl ::core::clone::Clone for ContactListOtherAppReadAccess {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactListOtherAppWriteAccess(pub i32);
impl ContactListOtherAppWriteAccess {
    pub const None: Self = Self(0i32);
    pub const SystemOnly: Self = Self(1i32);
    pub const Limited: Self = Self(2i32);
}
impl ::core::marker::Copy for ContactListOtherAppWriteAccess {}
impl ::core::clone::Clone for ContactListOtherAppWriteAccess {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactListSyncConstraints(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactListSyncConstraints {}
impl ::core::clone::Clone for ContactListSyncConstraints {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactListSyncManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactListSyncManager {}
impl ::core::clone::Clone for ContactListSyncManager {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for ContactListSyncStatus {}
impl ::core::clone::Clone for ContactListSyncStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactLocationField(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactLocationField {}
impl ::core::clone::Clone for ContactLocationField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactManagerForUser(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactManagerForUser {}
impl ::core::clone::Clone for ContactManagerForUser {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactMatchReason(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactMatchReason {}
impl ::core::clone::Clone for ContactMatchReason {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for ContactMatchReasonKind {}
impl ::core::clone::Clone for ContactMatchReasonKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactNameOrder(pub i32);
impl ContactNameOrder {
    pub const FirstNameLastName: Self = Self(0i32);
    pub const LastNameFirstName: Self = Self(1i32);
}
impl ::core::marker::Copy for ContactNameOrder {}
impl ::core::clone::Clone for ContactNameOrder {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactPanel(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactPanel {}
impl ::core::clone::Clone for ContactPanel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactPanelClosingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactPanelClosingEventArgs {}
impl ::core::clone::Clone for ContactPanelClosingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactPanelLaunchFullAppRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactPanelLaunchFullAppRequestedEventArgs {}
impl ::core::clone::Clone for ContactPanelLaunchFullAppRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactPhone(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactPhone {}
impl ::core::clone::Clone for ContactPhone {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for ContactPhoneKind {}
impl ::core::clone::Clone for ContactPhoneKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactPicker(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactPicker {}
impl ::core::clone::Clone for ContactPicker {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactQueryDesiredFields(pub u32);
impl ContactQueryDesiredFields {
    pub const None: Self = Self(0u32);
    pub const PhoneNumber: Self = Self(1u32);
    pub const EmailAddress: Self = Self(2u32);
    pub const PostalAddress: Self = Self(4u32);
}
impl ::core::marker::Copy for ContactQueryDesiredFields {}
impl ::core::clone::Clone for ContactQueryDesiredFields {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactQueryOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactQueryOptions {}
impl ::core::clone::Clone for ContactQueryOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactQuerySearchFields(pub u32);
impl ContactQuerySearchFields {
    pub const None: Self = Self(0u32);
    pub const Name: Self = Self(1u32);
    pub const Email: Self = Self(2u32);
    pub const Phone: Self = Self(4u32);
    pub const All: Self = Self(4294967295u32);
}
impl ::core::marker::Copy for ContactQuerySearchFields {}
impl ::core::clone::Clone for ContactQuerySearchFields {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactQuerySearchScope(pub i32);
impl ContactQuerySearchScope {
    pub const Local: Self = Self(0i32);
    pub const Server: Self = Self(1i32);
}
impl ::core::marker::Copy for ContactQuerySearchScope {}
impl ::core::clone::Clone for ContactQuerySearchScope {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactQueryTextSearch(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactQueryTextSearch {}
impl ::core::clone::Clone for ContactQueryTextSearch {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactReader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactReader {}
impl ::core::clone::Clone for ContactReader {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for ContactRelationship {}
impl ::core::clone::Clone for ContactRelationship {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactSelectionMode(pub i32);
impl ContactSelectionMode {
    pub const Contacts: Self = Self(0i32);
    pub const Fields: Self = Self(1i32);
}
impl ::core::marker::Copy for ContactSelectionMode {}
impl ::core::clone::Clone for ContactSelectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactSignificantOther(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactSignificantOther {}
impl ::core::clone::Clone for ContactSignificantOther {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactStore(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactStore {}
impl ::core::clone::Clone for ContactStore {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactStoreAccessType(pub i32);
impl ContactStoreAccessType {
    pub const AppContactsReadWrite: Self = Self(0i32);
    pub const AllContactsReadOnly: Self = Self(1i32);
    pub const AllContactsReadWrite: Self = Self(2i32);
}
impl ::core::marker::Copy for ContactStoreAccessType {}
impl ::core::clone::Clone for ContactStoreAccessType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactStoreNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactStoreNotificationTriggerDetails {}
impl ::core::clone::Clone for ContactStoreNotificationTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactWebsite(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactWebsite {}
impl ::core::clone::Clone for ContactWebsite {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FullContactCardOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FullContactCardOptions {}
impl ::core::clone::Clone for FullContactCardOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAggregateContactManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAggregateContactManager {}
impl ::core::clone::Clone for IAggregateContactManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAggregateContactManager2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAggregateContactManager2 {}
impl ::core::clone::Clone for IAggregateContactManager2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContact(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContact {}
impl ::core::clone::Clone for IContact {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContact2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContact2 {}
impl ::core::clone::Clone for IContact2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContact3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContact3 {}
impl ::core::clone::Clone for IContact3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactAddress(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactAddress {}
impl ::core::clone::Clone for IContactAddress {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactAnnotation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactAnnotation {}
impl ::core::clone::Clone for IContactAnnotation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactAnnotation2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactAnnotation2 {}
impl ::core::clone::Clone for IContactAnnotation2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactAnnotationList(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactAnnotationList {}
impl ::core::clone::Clone for IContactAnnotationList {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactAnnotationStore(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactAnnotationStore {}
impl ::core::clone::Clone for IContactAnnotationStore {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactAnnotationStore2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactAnnotationStore2 {}
impl ::core::clone::Clone for IContactAnnotationStore2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactBatch(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactBatch {}
impl ::core::clone::Clone for IContactBatch {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactCardDelayedDataLoader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactCardDelayedDataLoader {}
impl ::core::clone::Clone for IContactCardDelayedDataLoader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactCardOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactCardOptions {}
impl ::core::clone::Clone for IContactCardOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactCardOptions2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactCardOptions2 {}
impl ::core::clone::Clone for IContactCardOptions2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactChange(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactChange {}
impl ::core::clone::Clone for IContactChange {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactChangeReader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactChangeReader {}
impl ::core::clone::Clone for IContactChangeReader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactChangeTracker(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactChangeTracker {}
impl ::core::clone::Clone for IContactChangeTracker {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactChangeTracker2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactChangeTracker2 {}
impl ::core::clone::Clone for IContactChangeTracker2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactChangedDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactChangedDeferral {}
impl ::core::clone::Clone for IContactChangedDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactChangedEventArgs {}
impl ::core::clone::Clone for IContactChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactConnectedServiceAccount(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactConnectedServiceAccount {}
impl ::core::clone::Clone for IContactConnectedServiceAccount {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactDate(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactDate {}
impl ::core::clone::Clone for IContactDate {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactEmail(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactEmail {}
impl ::core::clone::Clone for IContactEmail {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactField(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactField {}
impl ::core::clone::Clone for IContactField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactFieldFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactFieldFactory {}
impl ::core::clone::Clone for IContactFieldFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactGroup(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactGroup {}
impl ::core::clone::Clone for IContactGroup {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactInformation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactInformation {}
impl ::core::clone::Clone for IContactInformation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactInstantMessageField(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactInstantMessageField {}
impl ::core::clone::Clone for IContactInstantMessageField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactInstantMessageFieldFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactInstantMessageFieldFactory {}
impl ::core::clone::Clone for IContactInstantMessageFieldFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactJobInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactJobInfo {}
impl ::core::clone::Clone for IContactJobInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactLaunchActionVerbsStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactLaunchActionVerbsStatics {}
impl ::core::clone::Clone for IContactLaunchActionVerbsStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactList(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactList {}
impl ::core::clone::Clone for IContactList {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactList2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactList2 {}
impl ::core::clone::Clone for IContactList2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactList3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactList3 {}
impl ::core::clone::Clone for IContactList3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactListLimitedWriteOperations(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactListLimitedWriteOperations {}
impl ::core::clone::Clone for IContactListLimitedWriteOperations {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactListSyncConstraints(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactListSyncConstraints {}
impl ::core::clone::Clone for IContactListSyncConstraints {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactListSyncManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactListSyncManager {}
impl ::core::clone::Clone for IContactListSyncManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactListSyncManager2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactListSyncManager2 {}
impl ::core::clone::Clone for IContactListSyncManager2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactLocationField(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactLocationField {}
impl ::core::clone::Clone for IContactLocationField {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactLocationFieldFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactLocationFieldFactory {}
impl ::core::clone::Clone for IContactLocationFieldFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactManagerForUser(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactManagerForUser {}
impl ::core::clone::Clone for IContactManagerForUser {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactManagerForUser2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactManagerForUser2 {}
impl ::core::clone::Clone for IContactManagerForUser2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactManagerStatics {}
impl ::core::clone::Clone for IContactManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactManagerStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactManagerStatics2 {}
impl ::core::clone::Clone for IContactManagerStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactManagerStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactManagerStatics3 {}
impl ::core::clone::Clone for IContactManagerStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactManagerStatics4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactManagerStatics4 {}
impl ::core::clone::Clone for IContactManagerStatics4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactManagerStatics5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactManagerStatics5 {}
impl ::core::clone::Clone for IContactManagerStatics5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactMatchReason(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactMatchReason {}
impl ::core::clone::Clone for IContactMatchReason {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactName(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactName {}
impl ::core::clone::Clone for IContactName {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactPanel(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactPanel {}
impl ::core::clone::Clone for IContactPanel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactPanelClosingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactPanelClosingEventArgs {}
impl ::core::clone::Clone for IContactPanelClosingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactPanelLaunchFullAppRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactPanelLaunchFullAppRequestedEventArgs {}
impl ::core::clone::Clone for IContactPanelLaunchFullAppRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactPhone(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactPhone {}
impl ::core::clone::Clone for IContactPhone {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactPicker(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactPicker {}
impl ::core::clone::Clone for IContactPicker {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactPicker2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactPicker2 {}
impl ::core::clone::Clone for IContactPicker2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactPicker3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactPicker3 {}
impl ::core::clone::Clone for IContactPicker3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactPickerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactPickerStatics {}
impl ::core::clone::Clone for IContactPickerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactQueryOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactQueryOptions {}
impl ::core::clone::Clone for IContactQueryOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactQueryOptionsFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactQueryOptionsFactory {}
impl ::core::clone::Clone for IContactQueryOptionsFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactQueryTextSearch(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactQueryTextSearch {}
impl ::core::clone::Clone for IContactQueryTextSearch {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactReader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactReader {}
impl ::core::clone::Clone for IContactReader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactSignificantOther(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactSignificantOther {}
impl ::core::clone::Clone for IContactSignificantOther {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactSignificantOther2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactSignificantOther2 {}
impl ::core::clone::Clone for IContactSignificantOther2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactStore(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactStore {}
impl ::core::clone::Clone for IContactStore {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactStore2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactStore2 {}
impl ::core::clone::Clone for IContactStore2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactStore3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactStore3 {}
impl ::core::clone::Clone for IContactStore3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactStoreNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactStoreNotificationTriggerDetails {}
impl ::core::clone::Clone for IContactStoreNotificationTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactWebsite(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactWebsite {}
impl ::core::clone::Clone for IContactWebsite {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactWebsite2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactWebsite2 {}
impl ::core::clone::Clone for IContactWebsite2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFullContactCardOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFullContactCardOptions {}
impl ::core::clone::Clone for IFullContactCardOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKnownContactFieldStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKnownContactFieldStatics {}
impl ::core::clone::Clone for IKnownContactFieldStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPinnedContactIdsQueryResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPinnedContactIdsQueryResult {}
impl ::core::clone::Clone for IPinnedContactIdsQueryResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPinnedContactManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPinnedContactManager {}
impl ::core::clone::Clone for IPinnedContactManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPinnedContactManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPinnedContactManagerStatics {}
impl ::core::clone::Clone for IPinnedContactManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PinnedContactIdsQueryResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PinnedContactIdsQueryResult {}
impl ::core::clone::Clone for PinnedContactIdsQueryResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PinnedContactManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PinnedContactManager {}
impl ::core::clone::Clone for PinnedContactManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PinnedContactSurface(pub i32);
impl PinnedContactSurface {
    pub const StartMenu: Self = Self(0i32);
    pub const Taskbar: Self = Self(1i32);
}
impl ::core::marker::Copy for PinnedContactSurface {}
impl ::core::clone::Clone for PinnedContactSurface {
    fn clone(&self) -> Self {
        *self
    }
}
