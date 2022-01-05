#[cfg(feature = "implement_exclusive")]
pub trait IEmailDataProviderConnectionImpl: Sized {
    fn MailboxSyncRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxSyncManagerSyncRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMailboxSyncRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DownloadMessageRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxDownloadMessageRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDownloadMessageRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DownloadAttachmentRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxDownloadAttachmentRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDownloadAttachmentRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CreateFolderRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxCreateFolderRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCreateFolderRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DeleteFolderRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxDeleteFolderRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDeleteFolderRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EmptyFolderRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxEmptyFolderRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveEmptyFolderRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MoveFolderRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxMoveFolderRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMoveFolderRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn UpdateMeetingResponseRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxUpdateMeetingResponseRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveUpdateMeetingResponseRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ForwardMeetingRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxForwardMeetingRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveForwardMeetingRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ProposeNewTimeForMeetingRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxProposeNewTimeForMeetingRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveProposeNewTimeForMeetingRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SetAutoReplySettingsRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxSetAutoReplySettingsRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSetAutoReplySettingsRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetAutoReplySettingsRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxGetAutoReplySettingsRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveGetAutoReplySettingsRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ResolveRecipientsRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxResolveRecipientsRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveResolveRecipientsRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ValidateCertificatesRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxValidateCertificatesRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveValidateCertificatesRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ServerSearchReadBatchRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxServerSearchReadBatchRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveServerSearchReadBatchRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Start(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailDataProviderTriggerDetailsImpl: Sized {
    fn Connection(&self) -> ::windows::core::Result<EmailDataProviderConnection>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxCreateFolderRequestImpl: Sized {
    fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ParentFolderId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReportCompletedAsync(&self, folder: &::core::option::Option<super::EmailFolder>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self, status: super::EmailMailboxCreateFolderStatus) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxCreateFolderRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<EmailMailboxCreateFolderRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxDeleteFolderRequestImpl: Sized {
    fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EmailFolderId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self, status: super::EmailMailboxDeleteFolderStatus) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxDeleteFolderRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<EmailMailboxDeleteFolderRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxDownloadAttachmentRequestImpl: Sized {
    fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EmailMessageId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EmailAttachmentId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxDownloadAttachmentRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<EmailMailboxDownloadAttachmentRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxDownloadMessageRequestImpl: Sized {
    fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EmailMessageId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxDownloadMessageRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<EmailMailboxDownloadMessageRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxEmptyFolderRequestImpl: Sized {
    fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EmailFolderId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self, status: super::EmailMailboxEmptyFolderStatus) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxEmptyFolderRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<EmailMailboxEmptyFolderRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxForwardMeetingRequestImpl: Sized {
    fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EmailMessageId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Recipients(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::EmailRecipient>>;
    fn Subject(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ForwardHeaderType(&self) -> ::windows::core::Result<super::EmailMessageBodyKind>;
    fn ForwardHeader(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxForwardMeetingRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<EmailMailboxForwardMeetingRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxGetAutoReplySettingsRequestImpl: Sized {
    fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RequestedFormat(&self) -> ::windows::core::Result<super::EmailMailboxAutoReplyMessageResponseKind>;
    fn ReportCompletedAsync(&self, autoreplysettings: &::core::option::Option<super::EmailMailboxAutoReplySettings>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxGetAutoReplySettingsRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<EmailMailboxGetAutoReplySettingsRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxMoveFolderRequestImpl: Sized {
    fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EmailFolderId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NewParentFolderId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NewFolderName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxMoveFolderRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<EmailMailboxMoveFolderRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxProposeNewTimeForMeetingRequestImpl: Sized {
    fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EmailMessageId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NewStartTime(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn NewDuration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn Subject(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxProposeNewTimeForMeetingRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<EmailMailboxProposeNewTimeForMeetingRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxResolveRecipientsRequestImpl: Sized {
    fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Recipients(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn ReportCompletedAsync(&self, resolutionresults: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<super::EmailRecipientResolutionResult>>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxResolveRecipientsRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<EmailMailboxResolveRecipientsRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxServerSearchReadBatchRequestImpl: Sized {
    fn SessionId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EmailFolderId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Options(&self) -> ::windows::core::Result<super::EmailQueryOptions>;
    fn SuggestedBatchSize(&self) -> ::windows::core::Result<u32>;
    fn SaveMessageAsync(&self, message: &::core::option::Option<super::EmailMessage>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self, batchstatus: super::EmailBatchStatus) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxServerSearchReadBatchRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<EmailMailboxServerSearchReadBatchRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxSetAutoReplySettingsRequestImpl: Sized {
    fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AutoReplySettings(&self) -> ::windows::core::Result<super::EmailMailboxAutoReplySettings>;
    fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxSetAutoReplySettingsRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<EmailMailboxSetAutoReplySettingsRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxSyncManagerSyncRequestImpl: Sized {
    fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxSyncManagerSyncRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<EmailMailboxSyncManagerSyncRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxUpdateMeetingResponseRequestImpl: Sized {
    fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EmailMessageId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Response(&self) -> ::windows::core::Result<super::EmailMeetingResponseType>;
    fn Subject(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SendUpdate(&self) -> ::windows::core::Result<bool>;
    fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxUpdateMeetingResponseRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<EmailMailboxUpdateMeetingResponseRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxValidateCertificatesRequestImpl: Sized {
    fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Certificates(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Security::Cryptography::Certificates::Certificate>>;
    fn ReportCompletedAsync(&self, validationstatuses: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<super::EmailCertificateValidationStatus>>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxValidateCertificatesRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<EmailMailboxValidateCertificatesRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
