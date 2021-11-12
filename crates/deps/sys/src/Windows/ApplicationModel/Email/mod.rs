#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "ApplicationModel_Email_DataProvider")]
pub mod DataProvider;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct EmailAttachment(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct EmailAttachmentDownloadState(i32);
#[repr(C)]
pub struct EmailBatchStatus(i32);
#[repr(C)]
pub struct EmailCertificateValidationStatus(i32);
#[repr(transparent)]
pub struct EmailConversation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailConversationBatch(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailConversationReader(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct EmailFlagState(i32);
#[repr(transparent)]
pub struct EmailFolder(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct EmailImportance(i32);
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
#[repr(C)]
pub struct EmailMailboxActionKind(i32);
#[repr(C)]
pub struct EmailMailboxAllowedSmimeEncryptionAlgorithmNegotiation(i32);
#[repr(transparent)]
pub struct EmailMailboxAutoReply(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct EmailMailboxAutoReplyMessageResponseKind(i32);
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
#[repr(C)]
pub struct EmailMailboxChangeType(i32);
#[repr(transparent)]
pub struct EmailMailboxChangedDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMailboxChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMailboxCreateFolderResult(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct EmailMailboxCreateFolderStatus(i32);
#[repr(C)]
pub struct EmailMailboxDeleteFolderStatus(i32);
#[repr(C)]
pub struct EmailMailboxEmptyFolderStatus(i32);
#[repr(C)]
pub struct EmailMailboxOtherAppReadAccess(i32);
#[repr(C)]
pub struct EmailMailboxOtherAppWriteAccess(i32);
#[repr(transparent)]
pub struct EmailMailboxPolicies(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct EmailMailboxSmimeEncryptionAlgorithm(i32);
#[repr(C)]
pub struct EmailMailboxSmimeSigningAlgorithm(i32);
#[repr(transparent)]
pub struct EmailMailboxSyncManager(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct EmailMailboxSyncStatus(i32);
#[repr(transparent)]
pub struct EmailManagerForUser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMeetingInfo(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct EmailMeetingResponseType(i32);
#[repr(transparent)]
pub struct EmailMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMessageBatch(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct EmailMessageBodyKind(i32);
#[repr(C)]
pub struct EmailMessageDownloadState(i32);
#[repr(transparent)]
pub struct EmailMessageReader(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct EmailMessageResponseKind(i32);
#[repr(C)]
pub struct EmailMessageSmimeKind(i32);
#[repr(C)]
pub struct EmailQueryKind(i32);
#[repr(transparent)]
pub struct EmailQueryOptions(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct EmailQuerySearchFields(i32);
#[repr(C)]
pub struct EmailQuerySearchScope(i32);
#[repr(C)]
pub struct EmailQuerySortDirection(i32);
#[repr(C)]
pub struct EmailQuerySortProperty(i32);
#[repr(transparent)]
pub struct EmailQueryTextSearch(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailRecipient(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailRecipientResolutionResult(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct EmailRecipientResolutionStatus(i32);
#[repr(C)]
pub struct EmailSpecialFolderKind(i32);
#[repr(transparent)]
pub struct EmailStore(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct EmailStoreAccessType(i32);
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
