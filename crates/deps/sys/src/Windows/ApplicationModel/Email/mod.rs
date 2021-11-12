#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "ApplicationModel_Email_DataProvider")]
pub mod DataProvider;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct EmailAttachment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailAttachmentDownloadState(pub i32);
impl EmailAttachmentDownloadState {
    pub const NotDownloaded: Self = Self(0i32);
    pub const Downloading: Self = Self(1i32);
    pub const Downloaded: Self = Self(2i32);
    pub const Failed: Self = Self(3i32);
}
#[repr(transparent)]
pub struct EmailBatchStatus(pub i32);
impl EmailBatchStatus {
    pub const Success: Self = Self(0i32);
    pub const ServerSearchSyncManagerError: Self = Self(1i32);
    pub const ServerSearchUnknownError: Self = Self(2i32);
}
#[repr(transparent)]
pub struct EmailCertificateValidationStatus(pub i32);
impl EmailCertificateValidationStatus {
    pub const Success: Self = Self(0i32);
    pub const NoMatch: Self = Self(1i32);
    pub const InvalidUsage: Self = Self(2i32);
    pub const InvalidCertificate: Self = Self(3i32);
    pub const Revoked: Self = Self(4i32);
    pub const ChainRevoked: Self = Self(5i32);
    pub const RevocationServerFailure: Self = Self(6i32);
    pub const Expired: Self = Self(7i32);
    pub const Untrusted: Self = Self(8i32);
    pub const ServerError: Self = Self(9i32);
    pub const UnknownFailure: Self = Self(10i32);
}
#[repr(transparent)]
pub struct EmailConversation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailConversationBatch(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailConversationReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailFlagState(pub i32);
impl EmailFlagState {
    pub const Unflagged: Self = Self(0i32);
    pub const Flagged: Self = Self(1i32);
    pub const Completed: Self = Self(2i32);
    pub const Cleared: Self = Self(3i32);
}
#[repr(transparent)]
pub struct EmailFolder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailImportance(pub i32);
impl EmailImportance {
    pub const Normal: Self = Self(0i32);
    pub const High: Self = Self(1i32);
    pub const Low: Self = Self(2i32);
}
#[repr(transparent)]
pub struct EmailIrmInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailIrmTemplate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailItemCounts(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMailbox(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMailboxAction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMailboxActionKind(pub i32);
impl EmailMailboxActionKind {
    pub const MarkMessageAsSeen: Self = Self(0i32);
    pub const MarkMessageRead: Self = Self(1i32);
    pub const ChangeMessageFlagState: Self = Self(2i32);
    pub const MoveMessage: Self = Self(3i32);
    pub const SaveDraft: Self = Self(4i32);
    pub const SendMessage: Self = Self(5i32);
    pub const CreateResponseReplyMessage: Self = Self(6i32);
    pub const CreateResponseReplyAllMessage: Self = Self(7i32);
    pub const CreateResponseForwardMessage: Self = Self(8i32);
    pub const MoveFolder: Self = Self(9i32);
    pub const MarkFolderForSyncEnabled: Self = Self(10i32);
}
#[repr(transparent)]
pub struct EmailMailboxAllowedSmimeEncryptionAlgorithmNegotiation(pub i32);
impl EmailMailboxAllowedSmimeEncryptionAlgorithmNegotiation {
    pub const None: Self = Self(0i32);
    pub const StrongAlgorithm: Self = Self(1i32);
    pub const AnyAlgorithm: Self = Self(2i32);
}
#[repr(transparent)]
pub struct EmailMailboxAutoReply(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMailboxAutoReplyMessageResponseKind(pub i32);
impl EmailMailboxAutoReplyMessageResponseKind {
    pub const Html: Self = Self(0i32);
    pub const PlainText: Self = Self(1i32);
}
#[repr(transparent)]
pub struct EmailMailboxAutoReplySettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMailboxCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMailboxChange(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMailboxChangeReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMailboxChangeTracker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMailboxChangeType(pub i32);
impl EmailMailboxChangeType {
    pub const MessageCreated: Self = Self(0i32);
    pub const MessageModified: Self = Self(1i32);
    pub const MessageDeleted: Self = Self(2i32);
    pub const FolderCreated: Self = Self(3i32);
    pub const FolderModified: Self = Self(4i32);
    pub const FolderDeleted: Self = Self(5i32);
    pub const ChangeTrackingLost: Self = Self(6i32);
}
#[repr(transparent)]
pub struct EmailMailboxChangedDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMailboxChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMailboxCreateFolderResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMailboxCreateFolderStatus(pub i32);
impl EmailMailboxCreateFolderStatus {
    pub const Success: Self = Self(0i32);
    pub const NetworkError: Self = Self(1i32);
    pub const PermissionsError: Self = Self(2i32);
    pub const ServerError: Self = Self(3i32);
    pub const UnknownFailure: Self = Self(4i32);
    pub const NameCollision: Self = Self(5i32);
    pub const ServerRejected: Self = Self(6i32);
}
#[repr(transparent)]
pub struct EmailMailboxDeleteFolderStatus(pub i32);
impl EmailMailboxDeleteFolderStatus {
    pub const Success: Self = Self(0i32);
    pub const NetworkError: Self = Self(1i32);
    pub const PermissionsError: Self = Self(2i32);
    pub const ServerError: Self = Self(3i32);
    pub const UnknownFailure: Self = Self(4i32);
    pub const CouldNotDeleteEverything: Self = Self(5i32);
}
#[repr(transparent)]
pub struct EmailMailboxEmptyFolderStatus(pub i32);
impl EmailMailboxEmptyFolderStatus {
    pub const Success: Self = Self(0i32);
    pub const NetworkError: Self = Self(1i32);
    pub const PermissionsError: Self = Self(2i32);
    pub const ServerError: Self = Self(3i32);
    pub const UnknownFailure: Self = Self(4i32);
    pub const CouldNotDeleteEverything: Self = Self(5i32);
}
#[repr(transparent)]
pub struct EmailMailboxOtherAppReadAccess(pub i32);
impl EmailMailboxOtherAppReadAccess {
    pub const SystemOnly: Self = Self(0i32);
    pub const Full: Self = Self(1i32);
    pub const None: Self = Self(2i32);
}
#[repr(transparent)]
pub struct EmailMailboxOtherAppWriteAccess(pub i32);
impl EmailMailboxOtherAppWriteAccess {
    pub const None: Self = Self(0i32);
    pub const Limited: Self = Self(1i32);
}
#[repr(transparent)]
pub struct EmailMailboxPolicies(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMailboxSmimeEncryptionAlgorithm(pub i32);
impl EmailMailboxSmimeEncryptionAlgorithm {
    pub const Any: Self = Self(0i32);
    pub const TripleDes: Self = Self(1i32);
    pub const Des: Self = Self(2i32);
    pub const RC2128Bit: Self = Self(3i32);
    pub const RC264Bit: Self = Self(4i32);
    pub const RC240Bit: Self = Self(5i32);
}
#[repr(transparent)]
pub struct EmailMailboxSmimeSigningAlgorithm(pub i32);
impl EmailMailboxSmimeSigningAlgorithm {
    pub const Any: Self = Self(0i32);
    pub const Sha1: Self = Self(1i32);
    pub const MD5: Self = Self(2i32);
}
#[repr(transparent)]
pub struct EmailMailboxSyncManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMailboxSyncStatus(pub i32);
impl EmailMailboxSyncStatus {
    pub const Idle: Self = Self(0i32);
    pub const Syncing: Self = Self(1i32);
    pub const UpToDate: Self = Self(2i32);
    pub const AuthenticationError: Self = Self(3i32);
    pub const PolicyError: Self = Self(4i32);
    pub const UnknownError: Self = Self(5i32);
    pub const ManualAccountRemovalRequired: Self = Self(6i32);
}
#[repr(transparent)]
pub struct EmailManagerForUser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMeetingInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMeetingResponseType(pub i32);
impl EmailMeetingResponseType {
    pub const Accept: Self = Self(0i32);
    pub const Decline: Self = Self(1i32);
    pub const Tentative: Self = Self(2i32);
}
#[repr(transparent)]
pub struct EmailMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMessageBatch(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMessageBodyKind(pub i32);
impl EmailMessageBodyKind {
    pub const Html: Self = Self(0i32);
    pub const PlainText: Self = Self(1i32);
}
#[repr(transparent)]
pub struct EmailMessageDownloadState(pub i32);
impl EmailMessageDownloadState {
    pub const PartiallyDownloaded: Self = Self(0i32);
    pub const Downloading: Self = Self(1i32);
    pub const Downloaded: Self = Self(2i32);
    pub const Failed: Self = Self(3i32);
}
#[repr(transparent)]
pub struct EmailMessageReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMessageResponseKind(pub i32);
impl EmailMessageResponseKind {
    pub const None: Self = Self(0i32);
    pub const Reply: Self = Self(1i32);
    pub const ReplyAll: Self = Self(2i32);
    pub const Forward: Self = Self(3i32);
}
#[repr(transparent)]
pub struct EmailMessageSmimeKind(pub i32);
impl EmailMessageSmimeKind {
    pub const None: Self = Self(0i32);
    pub const ClearSigned: Self = Self(1i32);
    pub const OpaqueSigned: Self = Self(2i32);
    pub const Encrypted: Self = Self(3i32);
}
#[repr(transparent)]
pub struct EmailQueryKind(pub i32);
impl EmailQueryKind {
    pub const All: Self = Self(0i32);
    pub const Important: Self = Self(1i32);
    pub const Flagged: Self = Self(2i32);
    pub const Unread: Self = Self(3i32);
    pub const Read: Self = Self(4i32);
    pub const Unseen: Self = Self(5i32);
}
#[repr(transparent)]
pub struct EmailQueryOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailQuerySearchFields(pub u32);
impl EmailQuerySearchFields {
    pub const None: Self = Self(0u32);
    pub const Subject: Self = Self(1u32);
    pub const Sender: Self = Self(2u32);
    pub const Preview: Self = Self(4u32);
    pub const Recipients: Self = Self(8u32);
    pub const All: Self = Self(4294967295u32);
}
#[repr(transparent)]
pub struct EmailQuerySearchScope(pub i32);
impl EmailQuerySearchScope {
    pub const Local: Self = Self(0i32);
    pub const Server: Self = Self(1i32);
}
#[repr(transparent)]
pub struct EmailQuerySortDirection(pub i32);
impl EmailQuerySortDirection {
    pub const Descending: Self = Self(0i32);
    pub const Ascending: Self = Self(1i32);
}
#[repr(transparent)]
pub struct EmailQuerySortProperty(pub i32);
impl EmailQuerySortProperty {
    pub const Date: Self = Self(0i32);
}
#[repr(transparent)]
pub struct EmailQueryTextSearch(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailRecipient(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailRecipientResolutionResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailRecipientResolutionStatus(pub i32);
impl EmailRecipientResolutionStatus {
    pub const Success: Self = Self(0i32);
    pub const RecipientNotFound: Self = Self(1i32);
    pub const AmbiguousRecipient: Self = Self(2i32);
    pub const NoCertificate: Self = Self(3i32);
    pub const CertificateRequestLimitReached: Self = Self(4i32);
    pub const CannotResolveDistributionList: Self = Self(5i32);
    pub const ServerError: Self = Self(6i32);
    pub const UnknownFailure: Self = Self(7i32);
}
#[repr(transparent)]
pub struct EmailSpecialFolderKind(pub i32);
impl EmailSpecialFolderKind {
    pub const None: Self = Self(0i32);
    pub const Root: Self = Self(1i32);
    pub const Inbox: Self = Self(2i32);
    pub const Outbox: Self = Self(3i32);
    pub const Drafts: Self = Self(4i32);
    pub const DeletedItems: Self = Self(5i32);
    pub const Sent: Self = Self(6i32);
}
#[repr(transparent)]
pub struct EmailStore(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailStoreAccessType(pub i32);
impl EmailStoreAccessType {
    pub const AppMailboxesReadWrite: Self = Self(0i32);
    pub const AllMailboxesLimitedReadWrite: Self = Self(1i32);
}
#[repr(transparent)]
pub struct EmailStoreNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailAttachment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailAttachment2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailAttachmentFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailAttachmentFactory2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailConversation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailConversationBatch(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailConversationReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailFolder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailIrmInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailIrmInfoFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailIrmTemplate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailIrmTemplateFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailItemCounts(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMailbox(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMailbox2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMailbox3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMailbox4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMailbox5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMailboxAction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMailboxAutoReply(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMailboxAutoReplySettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMailboxCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMailboxCapabilities2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMailboxCapabilities3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMailboxChange(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMailboxChangeReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMailboxChangeTracker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMailboxChangedDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMailboxChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMailboxCreateFolderResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMailboxPolicies(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMailboxPolicies2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMailboxPolicies3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMailboxSyncManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMailboxSyncManager2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailManagerForUser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailManagerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailManagerStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMeetingInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMeetingInfo2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMessage2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMessage3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMessage4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMessageBatch(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMessageReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailQueryOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailQueryOptionsFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailQueryTextSearch(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailRecipient(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailRecipientFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailRecipientResolutionResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailRecipientResolutionResult2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailStore(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailStoreNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
