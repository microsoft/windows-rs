#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct EmailDataProviderConnection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailDataProviderTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMailboxCreateFolderRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMailboxCreateFolderRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMailboxDeleteFolderRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMailboxDeleteFolderRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMailboxDownloadAttachmentRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMailboxDownloadAttachmentRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMailboxDownloadMessageRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMailboxDownloadMessageRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMailboxEmptyFolderRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMailboxEmptyFolderRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMailboxForwardMeetingRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMailboxForwardMeetingRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMailboxGetAutoReplySettingsRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMailboxGetAutoReplySettingsRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMailboxMoveFolderRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMailboxMoveFolderRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMailboxProposeNewTimeForMeetingRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMailboxProposeNewTimeForMeetingRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMailboxResolveRecipientsRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMailboxResolveRecipientsRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMailboxServerSearchReadBatchRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMailboxServerSearchReadBatchRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMailboxSetAutoReplySettingsRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMailboxSetAutoReplySettingsRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMailboxSyncManagerSyncRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMailboxSyncManagerSyncRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMailboxUpdateMeetingResponseRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMailboxUpdateMeetingResponseRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMailboxValidateCertificatesRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailMailboxValidateCertificatesRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailDataProviderConnection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailDataProviderTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMailboxCreateFolderRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMailboxCreateFolderRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMailboxDeleteFolderRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMailboxDeleteFolderRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMailboxDownloadAttachmentRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMailboxDownloadAttachmentRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMailboxDownloadMessageRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMailboxDownloadMessageRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMailboxEmptyFolderRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMailboxEmptyFolderRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMailboxForwardMeetingRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMailboxForwardMeetingRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMailboxGetAutoReplySettingsRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMailboxGetAutoReplySettingsRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMailboxMoveFolderRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMailboxMoveFolderRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMailboxProposeNewTimeForMeetingRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMailboxProposeNewTimeForMeetingRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMailboxResolveRecipientsRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMailboxResolveRecipientsRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMailboxServerSearchReadBatchRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMailboxServerSearchReadBatchRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMailboxSetAutoReplySettingsRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMailboxSetAutoReplySettingsRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMailboxSyncManagerSyncRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMailboxSyncManagerSyncRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMailboxUpdateMeetingResponseRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMailboxUpdateMeetingResponseRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMailboxValidateCertificatesRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailMailboxValidateCertificatesRequestEventArgs(pub *mut ::core::ffi::c_void);
