#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct EmailDataProviderConnection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EmailDataProviderConnection {}
impl ::core::clone::Clone for EmailDataProviderConnection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EmailDataProviderTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EmailDataProviderTriggerDetails {}
impl ::core::clone::Clone for EmailDataProviderTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EmailMailboxCreateFolderRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EmailMailboxCreateFolderRequest {}
impl ::core::clone::Clone for EmailMailboxCreateFolderRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EmailMailboxCreateFolderRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EmailMailboxCreateFolderRequestEventArgs {}
impl ::core::clone::Clone for EmailMailboxCreateFolderRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EmailMailboxDeleteFolderRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EmailMailboxDeleteFolderRequest {}
impl ::core::clone::Clone for EmailMailboxDeleteFolderRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EmailMailboxDeleteFolderRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EmailMailboxDeleteFolderRequestEventArgs {}
impl ::core::clone::Clone for EmailMailboxDeleteFolderRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EmailMailboxDownloadAttachmentRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EmailMailboxDownloadAttachmentRequest {}
impl ::core::clone::Clone for EmailMailboxDownloadAttachmentRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EmailMailboxDownloadAttachmentRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EmailMailboxDownloadAttachmentRequestEventArgs {}
impl ::core::clone::Clone for EmailMailboxDownloadAttachmentRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EmailMailboxDownloadMessageRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EmailMailboxDownloadMessageRequest {}
impl ::core::clone::Clone for EmailMailboxDownloadMessageRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EmailMailboxDownloadMessageRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EmailMailboxDownloadMessageRequestEventArgs {}
impl ::core::clone::Clone for EmailMailboxDownloadMessageRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EmailMailboxEmptyFolderRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EmailMailboxEmptyFolderRequest {}
impl ::core::clone::Clone for EmailMailboxEmptyFolderRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EmailMailboxEmptyFolderRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EmailMailboxEmptyFolderRequestEventArgs {}
impl ::core::clone::Clone for EmailMailboxEmptyFolderRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EmailMailboxForwardMeetingRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EmailMailboxForwardMeetingRequest {}
impl ::core::clone::Clone for EmailMailboxForwardMeetingRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EmailMailboxForwardMeetingRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EmailMailboxForwardMeetingRequestEventArgs {}
impl ::core::clone::Clone for EmailMailboxForwardMeetingRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EmailMailboxGetAutoReplySettingsRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EmailMailboxGetAutoReplySettingsRequest {}
impl ::core::clone::Clone for EmailMailboxGetAutoReplySettingsRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EmailMailboxGetAutoReplySettingsRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EmailMailboxGetAutoReplySettingsRequestEventArgs {}
impl ::core::clone::Clone for EmailMailboxGetAutoReplySettingsRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EmailMailboxMoveFolderRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EmailMailboxMoveFolderRequest {}
impl ::core::clone::Clone for EmailMailboxMoveFolderRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EmailMailboxMoveFolderRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EmailMailboxMoveFolderRequestEventArgs {}
impl ::core::clone::Clone for EmailMailboxMoveFolderRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EmailMailboxProposeNewTimeForMeetingRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EmailMailboxProposeNewTimeForMeetingRequest {}
impl ::core::clone::Clone for EmailMailboxProposeNewTimeForMeetingRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EmailMailboxProposeNewTimeForMeetingRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EmailMailboxProposeNewTimeForMeetingRequestEventArgs {}
impl ::core::clone::Clone for EmailMailboxProposeNewTimeForMeetingRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EmailMailboxResolveRecipientsRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EmailMailboxResolveRecipientsRequest {}
impl ::core::clone::Clone for EmailMailboxResolveRecipientsRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EmailMailboxResolveRecipientsRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EmailMailboxResolveRecipientsRequestEventArgs {}
impl ::core::clone::Clone for EmailMailboxResolveRecipientsRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EmailMailboxServerSearchReadBatchRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EmailMailboxServerSearchReadBatchRequest {}
impl ::core::clone::Clone for EmailMailboxServerSearchReadBatchRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EmailMailboxServerSearchReadBatchRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EmailMailboxServerSearchReadBatchRequestEventArgs {}
impl ::core::clone::Clone for EmailMailboxServerSearchReadBatchRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EmailMailboxSetAutoReplySettingsRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EmailMailboxSetAutoReplySettingsRequest {}
impl ::core::clone::Clone for EmailMailboxSetAutoReplySettingsRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EmailMailboxSetAutoReplySettingsRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EmailMailboxSetAutoReplySettingsRequestEventArgs {}
impl ::core::clone::Clone for EmailMailboxSetAutoReplySettingsRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EmailMailboxSyncManagerSyncRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EmailMailboxSyncManagerSyncRequest {}
impl ::core::clone::Clone for EmailMailboxSyncManagerSyncRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EmailMailboxSyncManagerSyncRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EmailMailboxSyncManagerSyncRequestEventArgs {}
impl ::core::clone::Clone for EmailMailboxSyncManagerSyncRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EmailMailboxUpdateMeetingResponseRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EmailMailboxUpdateMeetingResponseRequest {}
impl ::core::clone::Clone for EmailMailboxUpdateMeetingResponseRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EmailMailboxUpdateMeetingResponseRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EmailMailboxUpdateMeetingResponseRequestEventArgs {}
impl ::core::clone::Clone for EmailMailboxUpdateMeetingResponseRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EmailMailboxValidateCertificatesRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EmailMailboxValidateCertificatesRequest {}
impl ::core::clone::Clone for EmailMailboxValidateCertificatesRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EmailMailboxValidateCertificatesRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EmailMailboxValidateCertificatesRequestEventArgs {}
impl ::core::clone::Clone for EmailMailboxValidateCertificatesRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEmailDataProviderConnection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEmailDataProviderConnection {}
impl ::core::clone::Clone for IEmailDataProviderConnection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEmailDataProviderTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEmailDataProviderTriggerDetails {}
impl ::core::clone::Clone for IEmailDataProviderTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEmailMailboxCreateFolderRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEmailMailboxCreateFolderRequest {}
impl ::core::clone::Clone for IEmailMailboxCreateFolderRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEmailMailboxCreateFolderRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEmailMailboxCreateFolderRequestEventArgs {}
impl ::core::clone::Clone for IEmailMailboxCreateFolderRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEmailMailboxDeleteFolderRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEmailMailboxDeleteFolderRequest {}
impl ::core::clone::Clone for IEmailMailboxDeleteFolderRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEmailMailboxDeleteFolderRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEmailMailboxDeleteFolderRequestEventArgs {}
impl ::core::clone::Clone for IEmailMailboxDeleteFolderRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEmailMailboxDownloadAttachmentRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEmailMailboxDownloadAttachmentRequest {}
impl ::core::clone::Clone for IEmailMailboxDownloadAttachmentRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEmailMailboxDownloadAttachmentRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEmailMailboxDownloadAttachmentRequestEventArgs {}
impl ::core::clone::Clone for IEmailMailboxDownloadAttachmentRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEmailMailboxDownloadMessageRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEmailMailboxDownloadMessageRequest {}
impl ::core::clone::Clone for IEmailMailboxDownloadMessageRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEmailMailboxDownloadMessageRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEmailMailboxDownloadMessageRequestEventArgs {}
impl ::core::clone::Clone for IEmailMailboxDownloadMessageRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEmailMailboxEmptyFolderRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEmailMailboxEmptyFolderRequest {}
impl ::core::clone::Clone for IEmailMailboxEmptyFolderRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEmailMailboxEmptyFolderRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEmailMailboxEmptyFolderRequestEventArgs {}
impl ::core::clone::Clone for IEmailMailboxEmptyFolderRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEmailMailboxForwardMeetingRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEmailMailboxForwardMeetingRequest {}
impl ::core::clone::Clone for IEmailMailboxForwardMeetingRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEmailMailboxForwardMeetingRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEmailMailboxForwardMeetingRequestEventArgs {}
impl ::core::clone::Clone for IEmailMailboxForwardMeetingRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEmailMailboxGetAutoReplySettingsRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEmailMailboxGetAutoReplySettingsRequest {}
impl ::core::clone::Clone for IEmailMailboxGetAutoReplySettingsRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEmailMailboxGetAutoReplySettingsRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEmailMailboxGetAutoReplySettingsRequestEventArgs {}
impl ::core::clone::Clone for IEmailMailboxGetAutoReplySettingsRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEmailMailboxMoveFolderRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEmailMailboxMoveFolderRequest {}
impl ::core::clone::Clone for IEmailMailboxMoveFolderRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEmailMailboxMoveFolderRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEmailMailboxMoveFolderRequestEventArgs {}
impl ::core::clone::Clone for IEmailMailboxMoveFolderRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEmailMailboxProposeNewTimeForMeetingRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEmailMailboxProposeNewTimeForMeetingRequest {}
impl ::core::clone::Clone for IEmailMailboxProposeNewTimeForMeetingRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEmailMailboxProposeNewTimeForMeetingRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEmailMailboxProposeNewTimeForMeetingRequestEventArgs {}
impl ::core::clone::Clone for IEmailMailboxProposeNewTimeForMeetingRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEmailMailboxResolveRecipientsRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEmailMailboxResolveRecipientsRequest {}
impl ::core::clone::Clone for IEmailMailboxResolveRecipientsRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEmailMailboxResolveRecipientsRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEmailMailboxResolveRecipientsRequestEventArgs {}
impl ::core::clone::Clone for IEmailMailboxResolveRecipientsRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEmailMailboxServerSearchReadBatchRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEmailMailboxServerSearchReadBatchRequest {}
impl ::core::clone::Clone for IEmailMailboxServerSearchReadBatchRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEmailMailboxServerSearchReadBatchRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEmailMailboxServerSearchReadBatchRequestEventArgs {}
impl ::core::clone::Clone for IEmailMailboxServerSearchReadBatchRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEmailMailboxSetAutoReplySettingsRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEmailMailboxSetAutoReplySettingsRequest {}
impl ::core::clone::Clone for IEmailMailboxSetAutoReplySettingsRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEmailMailboxSetAutoReplySettingsRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEmailMailboxSetAutoReplySettingsRequestEventArgs {}
impl ::core::clone::Clone for IEmailMailboxSetAutoReplySettingsRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEmailMailboxSyncManagerSyncRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEmailMailboxSyncManagerSyncRequest {}
impl ::core::clone::Clone for IEmailMailboxSyncManagerSyncRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEmailMailboxSyncManagerSyncRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEmailMailboxSyncManagerSyncRequestEventArgs {}
impl ::core::clone::Clone for IEmailMailboxSyncManagerSyncRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEmailMailboxUpdateMeetingResponseRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEmailMailboxUpdateMeetingResponseRequest {}
impl ::core::clone::Clone for IEmailMailboxUpdateMeetingResponseRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEmailMailboxUpdateMeetingResponseRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEmailMailboxUpdateMeetingResponseRequestEventArgs {}
impl ::core::clone::Clone for IEmailMailboxUpdateMeetingResponseRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEmailMailboxValidateCertificatesRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEmailMailboxValidateCertificatesRequest {}
impl ::core::clone::Clone for IEmailMailboxValidateCertificatesRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEmailMailboxValidateCertificatesRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEmailMailboxValidateCertificatesRequestEventArgs {}
impl ::core::clone::Clone for IEmailMailboxValidateCertificatesRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
