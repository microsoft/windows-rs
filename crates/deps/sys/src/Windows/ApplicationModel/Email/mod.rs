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
    pub const NotDownloaded: EmailAttachmentDownloadState = EmailAttachmentDownloadState(0i32);
    pub const Downloading: EmailAttachmentDownloadState = EmailAttachmentDownloadState(1i32);
    pub const Downloaded: EmailAttachmentDownloadState = EmailAttachmentDownloadState(2i32);
    pub const Failed: EmailAttachmentDownloadState = EmailAttachmentDownloadState(3i32);
}
#[repr(transparent)]
pub struct EmailBatchStatus(pub i32);
impl EmailBatchStatus {
    pub const Success: EmailBatchStatus = EmailBatchStatus(0i32);
    pub const ServerSearchSyncManagerError: EmailBatchStatus = EmailBatchStatus(1i32);
    pub const ServerSearchUnknownError: EmailBatchStatus = EmailBatchStatus(2i32);
}
#[repr(transparent)]
pub struct EmailCertificateValidationStatus(pub i32);
impl EmailCertificateValidationStatus {
    pub const Success: EmailCertificateValidationStatus = EmailCertificateValidationStatus(0i32);
    pub const NoMatch: EmailCertificateValidationStatus = EmailCertificateValidationStatus(1i32);
    pub const InvalidUsage: EmailCertificateValidationStatus = EmailCertificateValidationStatus(2i32);
    pub const InvalidCertificate: EmailCertificateValidationStatus = EmailCertificateValidationStatus(3i32);
    pub const Revoked: EmailCertificateValidationStatus = EmailCertificateValidationStatus(4i32);
    pub const ChainRevoked: EmailCertificateValidationStatus = EmailCertificateValidationStatus(5i32);
    pub const RevocationServerFailure: EmailCertificateValidationStatus = EmailCertificateValidationStatus(6i32);
    pub const Expired: EmailCertificateValidationStatus = EmailCertificateValidationStatus(7i32);
    pub const Untrusted: EmailCertificateValidationStatus = EmailCertificateValidationStatus(8i32);
    pub const ServerError: EmailCertificateValidationStatus = EmailCertificateValidationStatus(9i32);
    pub const UnknownFailure: EmailCertificateValidationStatus = EmailCertificateValidationStatus(10i32);
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
    pub const Unflagged: EmailFlagState = EmailFlagState(0i32);
    pub const Flagged: EmailFlagState = EmailFlagState(1i32);
    pub const Completed: EmailFlagState = EmailFlagState(2i32);
    pub const Cleared: EmailFlagState = EmailFlagState(3i32);
}
#[repr(transparent)]
pub struct EmailFolder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailImportance(pub i32);
impl EmailImportance {
    pub const Normal: EmailImportance = EmailImportance(0i32);
    pub const High: EmailImportance = EmailImportance(1i32);
    pub const Low: EmailImportance = EmailImportance(2i32);
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
    pub const MarkMessageAsSeen: EmailMailboxActionKind = EmailMailboxActionKind(0i32);
    pub const MarkMessageRead: EmailMailboxActionKind = EmailMailboxActionKind(1i32);
    pub const ChangeMessageFlagState: EmailMailboxActionKind = EmailMailboxActionKind(2i32);
    pub const MoveMessage: EmailMailboxActionKind = EmailMailboxActionKind(3i32);
    pub const SaveDraft: EmailMailboxActionKind = EmailMailboxActionKind(4i32);
    pub const SendMessage: EmailMailboxActionKind = EmailMailboxActionKind(5i32);
    pub const CreateResponseReplyMessage: EmailMailboxActionKind = EmailMailboxActionKind(6i32);
    pub const CreateResponseReplyAllMessage: EmailMailboxActionKind = EmailMailboxActionKind(7i32);
    pub const CreateResponseForwardMessage: EmailMailboxActionKind = EmailMailboxActionKind(8i32);
    pub const MoveFolder: EmailMailboxActionKind = EmailMailboxActionKind(9i32);
    pub const MarkFolderForSyncEnabled: EmailMailboxActionKind = EmailMailboxActionKind(10i32);
}
#[repr(transparent)]
pub struct EmailMailboxAllowedSmimeEncryptionAlgorithmNegotiation(pub i32);
impl EmailMailboxAllowedSmimeEncryptionAlgorithmNegotiation {
    pub const None: EmailMailboxAllowedSmimeEncryptionAlgorithmNegotiation = EmailMailboxAllowedSmimeEncryptionAlgorithmNegotiation(0i32);
    pub const StrongAlgorithm: EmailMailboxAllowedSmimeEncryptionAlgorithmNegotiation = EmailMailboxAllowedSmimeEncryptionAlgorithmNegotiation(1i32);
    pub const AnyAlgorithm: EmailMailboxAllowedSmimeEncryptionAlgorithmNegotiation = EmailMailboxAllowedSmimeEncryptionAlgorithmNegotiation(2i32);
}
#[repr(transparent)]
pub struct EmailMailboxAutoReply(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMailboxAutoReplyMessageResponseKind(pub i32);
impl EmailMailboxAutoReplyMessageResponseKind {
    pub const Html: EmailMailboxAutoReplyMessageResponseKind = EmailMailboxAutoReplyMessageResponseKind(0i32);
    pub const PlainText: EmailMailboxAutoReplyMessageResponseKind = EmailMailboxAutoReplyMessageResponseKind(1i32);
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
    pub const MessageCreated: EmailMailboxChangeType = EmailMailboxChangeType(0i32);
    pub const MessageModified: EmailMailboxChangeType = EmailMailboxChangeType(1i32);
    pub const MessageDeleted: EmailMailboxChangeType = EmailMailboxChangeType(2i32);
    pub const FolderCreated: EmailMailboxChangeType = EmailMailboxChangeType(3i32);
    pub const FolderModified: EmailMailboxChangeType = EmailMailboxChangeType(4i32);
    pub const FolderDeleted: EmailMailboxChangeType = EmailMailboxChangeType(5i32);
    pub const ChangeTrackingLost: EmailMailboxChangeType = EmailMailboxChangeType(6i32);
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
    pub const Success: EmailMailboxCreateFolderStatus = EmailMailboxCreateFolderStatus(0i32);
    pub const NetworkError: EmailMailboxCreateFolderStatus = EmailMailboxCreateFolderStatus(1i32);
    pub const PermissionsError: EmailMailboxCreateFolderStatus = EmailMailboxCreateFolderStatus(2i32);
    pub const ServerError: EmailMailboxCreateFolderStatus = EmailMailboxCreateFolderStatus(3i32);
    pub const UnknownFailure: EmailMailboxCreateFolderStatus = EmailMailboxCreateFolderStatus(4i32);
    pub const NameCollision: EmailMailboxCreateFolderStatus = EmailMailboxCreateFolderStatus(5i32);
    pub const ServerRejected: EmailMailboxCreateFolderStatus = EmailMailboxCreateFolderStatus(6i32);
}
#[repr(transparent)]
pub struct EmailMailboxDeleteFolderStatus(pub i32);
impl EmailMailboxDeleteFolderStatus {
    pub const Success: EmailMailboxDeleteFolderStatus = EmailMailboxDeleteFolderStatus(0i32);
    pub const NetworkError: EmailMailboxDeleteFolderStatus = EmailMailboxDeleteFolderStatus(1i32);
    pub const PermissionsError: EmailMailboxDeleteFolderStatus = EmailMailboxDeleteFolderStatus(2i32);
    pub const ServerError: EmailMailboxDeleteFolderStatus = EmailMailboxDeleteFolderStatus(3i32);
    pub const UnknownFailure: EmailMailboxDeleteFolderStatus = EmailMailboxDeleteFolderStatus(4i32);
    pub const CouldNotDeleteEverything: EmailMailboxDeleteFolderStatus = EmailMailboxDeleteFolderStatus(5i32);
}
#[repr(transparent)]
pub struct EmailMailboxEmptyFolderStatus(pub i32);
impl EmailMailboxEmptyFolderStatus {
    pub const Success: EmailMailboxEmptyFolderStatus = EmailMailboxEmptyFolderStatus(0i32);
    pub const NetworkError: EmailMailboxEmptyFolderStatus = EmailMailboxEmptyFolderStatus(1i32);
    pub const PermissionsError: EmailMailboxEmptyFolderStatus = EmailMailboxEmptyFolderStatus(2i32);
    pub const ServerError: EmailMailboxEmptyFolderStatus = EmailMailboxEmptyFolderStatus(3i32);
    pub const UnknownFailure: EmailMailboxEmptyFolderStatus = EmailMailboxEmptyFolderStatus(4i32);
    pub const CouldNotDeleteEverything: EmailMailboxEmptyFolderStatus = EmailMailboxEmptyFolderStatus(5i32);
}
#[repr(transparent)]
pub struct EmailMailboxOtherAppReadAccess(pub i32);
impl EmailMailboxOtherAppReadAccess {
    pub const SystemOnly: EmailMailboxOtherAppReadAccess = EmailMailboxOtherAppReadAccess(0i32);
    pub const Full: EmailMailboxOtherAppReadAccess = EmailMailboxOtherAppReadAccess(1i32);
    pub const None: EmailMailboxOtherAppReadAccess = EmailMailboxOtherAppReadAccess(2i32);
}
#[repr(transparent)]
pub struct EmailMailboxOtherAppWriteAccess(pub i32);
impl EmailMailboxOtherAppWriteAccess {
    pub const None: EmailMailboxOtherAppWriteAccess = EmailMailboxOtherAppWriteAccess(0i32);
    pub const Limited: EmailMailboxOtherAppWriteAccess = EmailMailboxOtherAppWriteAccess(1i32);
}
#[repr(transparent)]
pub struct EmailMailboxPolicies(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMailboxSmimeEncryptionAlgorithm(pub i32);
impl EmailMailboxSmimeEncryptionAlgorithm {
    pub const Any: EmailMailboxSmimeEncryptionAlgorithm = EmailMailboxSmimeEncryptionAlgorithm(0i32);
    pub const TripleDes: EmailMailboxSmimeEncryptionAlgorithm = EmailMailboxSmimeEncryptionAlgorithm(1i32);
    pub const Des: EmailMailboxSmimeEncryptionAlgorithm = EmailMailboxSmimeEncryptionAlgorithm(2i32);
    pub const RC2128Bit: EmailMailboxSmimeEncryptionAlgorithm = EmailMailboxSmimeEncryptionAlgorithm(3i32);
    pub const RC264Bit: EmailMailboxSmimeEncryptionAlgorithm = EmailMailboxSmimeEncryptionAlgorithm(4i32);
    pub const RC240Bit: EmailMailboxSmimeEncryptionAlgorithm = EmailMailboxSmimeEncryptionAlgorithm(5i32);
}
#[repr(transparent)]
pub struct EmailMailboxSmimeSigningAlgorithm(pub i32);
impl EmailMailboxSmimeSigningAlgorithm {
    pub const Any: EmailMailboxSmimeSigningAlgorithm = EmailMailboxSmimeSigningAlgorithm(0i32);
    pub const Sha1: EmailMailboxSmimeSigningAlgorithm = EmailMailboxSmimeSigningAlgorithm(1i32);
    pub const MD5: EmailMailboxSmimeSigningAlgorithm = EmailMailboxSmimeSigningAlgorithm(2i32);
}
#[repr(transparent)]
pub struct EmailMailboxSyncManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMailboxSyncStatus(pub i32);
impl EmailMailboxSyncStatus {
    pub const Idle: EmailMailboxSyncStatus = EmailMailboxSyncStatus(0i32);
    pub const Syncing: EmailMailboxSyncStatus = EmailMailboxSyncStatus(1i32);
    pub const UpToDate: EmailMailboxSyncStatus = EmailMailboxSyncStatus(2i32);
    pub const AuthenticationError: EmailMailboxSyncStatus = EmailMailboxSyncStatus(3i32);
    pub const PolicyError: EmailMailboxSyncStatus = EmailMailboxSyncStatus(4i32);
    pub const UnknownError: EmailMailboxSyncStatus = EmailMailboxSyncStatus(5i32);
    pub const ManualAccountRemovalRequired: EmailMailboxSyncStatus = EmailMailboxSyncStatus(6i32);
}
#[repr(transparent)]
pub struct EmailManagerForUser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMeetingInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMeetingResponseType(pub i32);
impl EmailMeetingResponseType {
    pub const Accept: EmailMeetingResponseType = EmailMeetingResponseType(0i32);
    pub const Decline: EmailMeetingResponseType = EmailMeetingResponseType(1i32);
    pub const Tentative: EmailMeetingResponseType = EmailMeetingResponseType(2i32);
}
#[repr(transparent)]
pub struct EmailMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMessageBatch(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMessageBodyKind(pub i32);
impl EmailMessageBodyKind {
    pub const Html: EmailMessageBodyKind = EmailMessageBodyKind(0i32);
    pub const PlainText: EmailMessageBodyKind = EmailMessageBodyKind(1i32);
}
#[repr(transparent)]
pub struct EmailMessageDownloadState(pub i32);
impl EmailMessageDownloadState {
    pub const PartiallyDownloaded: EmailMessageDownloadState = EmailMessageDownloadState(0i32);
    pub const Downloading: EmailMessageDownloadState = EmailMessageDownloadState(1i32);
    pub const Downloaded: EmailMessageDownloadState = EmailMessageDownloadState(2i32);
    pub const Failed: EmailMessageDownloadState = EmailMessageDownloadState(3i32);
}
#[repr(transparent)]
pub struct EmailMessageReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMessageResponseKind(pub i32);
impl EmailMessageResponseKind {
    pub const None: EmailMessageResponseKind = EmailMessageResponseKind(0i32);
    pub const Reply: EmailMessageResponseKind = EmailMessageResponseKind(1i32);
    pub const ReplyAll: EmailMessageResponseKind = EmailMessageResponseKind(2i32);
    pub const Forward: EmailMessageResponseKind = EmailMessageResponseKind(3i32);
}
#[repr(transparent)]
pub struct EmailMessageSmimeKind(pub i32);
impl EmailMessageSmimeKind {
    pub const None: EmailMessageSmimeKind = EmailMessageSmimeKind(0i32);
    pub const ClearSigned: EmailMessageSmimeKind = EmailMessageSmimeKind(1i32);
    pub const OpaqueSigned: EmailMessageSmimeKind = EmailMessageSmimeKind(2i32);
    pub const Encrypted: EmailMessageSmimeKind = EmailMessageSmimeKind(3i32);
}
#[repr(transparent)]
pub struct EmailQueryKind(pub i32);
impl EmailQueryKind {
    pub const All: EmailQueryKind = EmailQueryKind(0i32);
    pub const Important: EmailQueryKind = EmailQueryKind(1i32);
    pub const Flagged: EmailQueryKind = EmailQueryKind(2i32);
    pub const Unread: EmailQueryKind = EmailQueryKind(3i32);
    pub const Read: EmailQueryKind = EmailQueryKind(4i32);
    pub const Unseen: EmailQueryKind = EmailQueryKind(5i32);
}
#[repr(transparent)]
pub struct EmailQueryOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailQuerySearchFields(pub u32);
impl EmailQuerySearchFields {
    pub const None: EmailQuerySearchFields = EmailQuerySearchFields(0u32);
    pub const Subject: EmailQuerySearchFields = EmailQuerySearchFields(1u32);
    pub const Sender: EmailQuerySearchFields = EmailQuerySearchFields(2u32);
    pub const Preview: EmailQuerySearchFields = EmailQuerySearchFields(4u32);
    pub const Recipients: EmailQuerySearchFields = EmailQuerySearchFields(8u32);
    pub const All: EmailQuerySearchFields = EmailQuerySearchFields(4294967295u32);
}
#[repr(transparent)]
pub struct EmailQuerySearchScope(pub i32);
impl EmailQuerySearchScope {
    pub const Local: EmailQuerySearchScope = EmailQuerySearchScope(0i32);
    pub const Server: EmailQuerySearchScope = EmailQuerySearchScope(1i32);
}
#[repr(transparent)]
pub struct EmailQuerySortDirection(pub i32);
impl EmailQuerySortDirection {
    pub const Descending: EmailQuerySortDirection = EmailQuerySortDirection(0i32);
    pub const Ascending: EmailQuerySortDirection = EmailQuerySortDirection(1i32);
}
#[repr(transparent)]
pub struct EmailQuerySortProperty(pub i32);
impl EmailQuerySortProperty {
    pub const Date: EmailQuerySortProperty = EmailQuerySortProperty(0i32);
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
    pub const Success: EmailRecipientResolutionStatus = EmailRecipientResolutionStatus(0i32);
    pub const RecipientNotFound: EmailRecipientResolutionStatus = EmailRecipientResolutionStatus(1i32);
    pub const AmbiguousRecipient: EmailRecipientResolutionStatus = EmailRecipientResolutionStatus(2i32);
    pub const NoCertificate: EmailRecipientResolutionStatus = EmailRecipientResolutionStatus(3i32);
    pub const CertificateRequestLimitReached: EmailRecipientResolutionStatus = EmailRecipientResolutionStatus(4i32);
    pub const CannotResolveDistributionList: EmailRecipientResolutionStatus = EmailRecipientResolutionStatus(5i32);
    pub const ServerError: EmailRecipientResolutionStatus = EmailRecipientResolutionStatus(6i32);
    pub const UnknownFailure: EmailRecipientResolutionStatus = EmailRecipientResolutionStatus(7i32);
}
#[repr(transparent)]
pub struct EmailSpecialFolderKind(pub i32);
impl EmailSpecialFolderKind {
    pub const None: EmailSpecialFolderKind = EmailSpecialFolderKind(0i32);
    pub const Root: EmailSpecialFolderKind = EmailSpecialFolderKind(1i32);
    pub const Inbox: EmailSpecialFolderKind = EmailSpecialFolderKind(2i32);
    pub const Outbox: EmailSpecialFolderKind = EmailSpecialFolderKind(3i32);
    pub const Drafts: EmailSpecialFolderKind = EmailSpecialFolderKind(4i32);
    pub const DeletedItems: EmailSpecialFolderKind = EmailSpecialFolderKind(5i32);
    pub const Sent: EmailSpecialFolderKind = EmailSpecialFolderKind(6i32);
}
#[repr(transparent)]
pub struct EmailStore(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailStoreAccessType(pub i32);
impl EmailStoreAccessType {
    pub const AppMailboxesReadWrite: EmailStoreAccessType = EmailStoreAccessType(0i32);
    pub const AllMailboxesLimitedReadWrite: EmailStoreAccessType = EmailStoreAccessType(1i32);
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
