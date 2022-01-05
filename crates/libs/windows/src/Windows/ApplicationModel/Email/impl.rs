#[cfg(feature = "implement_exclusive")]
pub trait IEmailAttachmentImpl: Sized {
    fn FileName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetFileName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Data(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn SetData(&self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailAttachment2Impl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ContentId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetContentId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ContentLocation(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetContentLocation(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DownloadState(&self) -> ::windows::core::Result<EmailAttachmentDownloadState>;
    fn SetDownloadState(&self, value: EmailAttachmentDownloadState) -> ::windows::core::Result<()>;
    fn EstimatedDownloadSizeInBytes(&self) -> ::windows::core::Result<u64>;
    fn SetEstimatedDownloadSizeInBytes(&self, value: u64) -> ::windows::core::Result<()>;
    fn IsFromBaseMessage(&self) -> ::windows::core::Result<bool>;
    fn IsInline(&self) -> ::windows::core::Result<bool>;
    fn SetIsInline(&self, value: bool) -> ::windows::core::Result<()>;
    fn MimeType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetMimeType(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailAttachmentFactoryImpl: Sized {
    fn Create(&self, filename: &::windows::core::HSTRING, data: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<EmailAttachment>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailAttachmentFactory2Impl: Sized {
    fn Create(&self, filename: &::windows::core::HSTRING, data: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>, mimetype: &::windows::core::HSTRING) -> ::windows::core::Result<EmailAttachment>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailConversationImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FlagState(&self) -> ::windows::core::Result<EmailFlagState>;
    fn HasAttachment(&self) -> ::windows::core::Result<bool>;
    fn Importance(&self) -> ::windows::core::Result<EmailImportance>;
    fn LastEmailResponseKind(&self) -> ::windows::core::Result<EmailMessageResponseKind>;
    fn MessageCount(&self) -> ::windows::core::Result<u32>;
    fn MostRecentMessageId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MostRecentMessageTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn Preview(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LatestSender(&self) -> ::windows::core::Result<EmailRecipient>;
    fn Subject(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn UnreadMessageCount(&self) -> ::windows::core::Result<u32>;
    fn FindMessagesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<EmailMessage>>>;
    fn FindMessagesWithCountAsync(&self, count: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<EmailMessage>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailConversationBatchImpl: Sized {
    fn Conversations(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<EmailConversation>>;
    fn Status(&self) -> ::windows::core::Result<EmailBatchStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailConversationReaderImpl: Sized {
    fn ReadBatchAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailConversationBatch>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailFolderImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RemoteId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetRemoteId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn MailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ParentFolderId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsSyncEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsSyncEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn LastSuccessfulSyncTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn SetLastSuccessfulSyncTime(&self, value: &super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn Kind(&self) -> ::windows::core::Result<EmailSpecialFolderKind>;
    fn CreateFolderAsync(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailFolder>>;
    fn DeleteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn FindChildFoldersAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<EmailFolder>>>;
    fn GetConversationReader(&self) -> ::windows::core::Result<EmailConversationReader>;
    fn GetConversationReaderWithOptions(&self, options: &::core::option::Option<EmailQueryOptions>) -> ::windows::core::Result<EmailConversationReader>;
    fn GetMessageAsync(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailMessage>>;
    fn GetMessageReader(&self) -> ::windows::core::Result<EmailMessageReader>;
    fn GetMessageReaderWithOptions(&self, options: &::core::option::Option<EmailQueryOptions>) -> ::windows::core::Result<EmailMessageReader>;
    fn GetMessageCountsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailItemCounts>>;
    fn TryMoveAsync(&self, newparentfolder: &::core::option::Option<EmailFolder>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryMoveWithNewNameAsync(&self, newparentfolder: &::core::option::Option<EmailFolder>, newfoldername: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TrySaveAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn SaveMessageAsync(&self, message: &::core::option::Option<EmailMessage>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailIrmInfoImpl: Sized {
    fn CanEdit(&self) -> ::windows::core::Result<bool>;
    fn SetCanEdit(&self, value: bool) -> ::windows::core::Result<()>;
    fn CanExtractData(&self) -> ::windows::core::Result<bool>;
    fn SetCanExtractData(&self, value: bool) -> ::windows::core::Result<()>;
    fn CanForward(&self) -> ::windows::core::Result<bool>;
    fn SetCanForward(&self, value: bool) -> ::windows::core::Result<()>;
    fn CanModifyRecipientsOnResponse(&self) -> ::windows::core::Result<bool>;
    fn SetCanModifyRecipientsOnResponse(&self, value: bool) -> ::windows::core::Result<()>;
    fn CanPrintData(&self) -> ::windows::core::Result<bool>;
    fn SetCanPrintData(&self, value: bool) -> ::windows::core::Result<()>;
    fn CanRemoveIrmOnResponse(&self) -> ::windows::core::Result<bool>;
    fn SetCanRemoveIrmOnResponse(&self, value: bool) -> ::windows::core::Result<()>;
    fn CanReply(&self) -> ::windows::core::Result<bool>;
    fn SetCanReply(&self, value: bool) -> ::windows::core::Result<()>;
    fn CanReplyAll(&self) -> ::windows::core::Result<bool>;
    fn SetCanReplyAll(&self, value: bool) -> ::windows::core::Result<()>;
    fn ExpirationDate(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn SetExpirationDate(&self, value: &super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn IsIrmOriginator(&self) -> ::windows::core::Result<bool>;
    fn SetIsIrmOriginator(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsProgramaticAccessAllowed(&self) -> ::windows::core::Result<bool>;
    fn SetIsProgramaticAccessAllowed(&self, value: bool) -> ::windows::core::Result<()>;
    fn Template(&self) -> ::windows::core::Result<EmailIrmTemplate>;
    fn SetTemplate(&self, value: &::core::option::Option<EmailIrmTemplate>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailIrmInfoFactoryImpl: Sized {
    fn Create(&self, expiration: &super::super::Foundation::DateTime, irmtemplate: &::core::option::Option<EmailIrmTemplate>) -> ::windows::core::Result<EmailIrmInfo>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailIrmTemplateImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailIrmTemplateFactoryImpl: Sized {
    fn Create(&self, id: &::windows::core::HSTRING, name: &::windows::core::HSTRING, description: &::windows::core::HSTRING) -> ::windows::core::Result<EmailIrmTemplate>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailItemCountsImpl: Sized {
    fn Flagged(&self) -> ::windows::core::Result<u32>;
    fn Important(&self) -> ::windows::core::Result<u32>;
    fn Total(&self) -> ::windows::core::Result<u32>;
    fn Unread(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxImpl: Sized {
    fn Capabilities(&self) -> ::windows::core::Result<EmailMailboxCapabilities>;
    fn ChangeTracker(&self) -> ::windows::core::Result<EmailMailboxChangeTracker>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsOwnedByCurrentApp(&self) -> ::windows::core::Result<bool>;
    fn IsDataEncryptedUnderLock(&self) -> ::windows::core::Result<bool>;
    fn MailAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetMailAddress(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn MailAddressAliases(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn OtherAppReadAccess(&self) -> ::windows::core::Result<EmailMailboxOtherAppReadAccess>;
    fn SetOtherAppReadAccess(&self, value: EmailMailboxOtherAppReadAccess) -> ::windows::core::Result<()>;
    fn OtherAppWriteAccess(&self) -> ::windows::core::Result<EmailMailboxOtherAppWriteAccess>;
    fn SetOtherAppWriteAccess(&self, value: EmailMailboxOtherAppWriteAccess) -> ::windows::core::Result<()>;
    fn Policies(&self) -> ::windows::core::Result<EmailMailboxPolicies>;
    fn SourceDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SyncManager(&self) -> ::windows::core::Result<EmailMailboxSyncManager>;
    fn UserDataAccountId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetConversationReader(&self) -> ::windows::core::Result<EmailConversationReader>;
    fn GetConversationReaderWithOptions(&self, options: &::core::option::Option<EmailQueryOptions>) -> ::windows::core::Result<EmailConversationReader>;
    fn GetMessageReader(&self) -> ::windows::core::Result<EmailMessageReader>;
    fn GetMessageReaderWithOptions(&self, options: &::core::option::Option<EmailQueryOptions>) -> ::windows::core::Result<EmailMessageReader>;
    fn DeleteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn GetConversationAsync(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailConversation>>;
    fn GetFolderAsync(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailFolder>>;
    fn GetMessageAsync(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailMessage>>;
    fn GetSpecialFolderAsync(&self, foldertype: EmailSpecialFolderKind) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailFolder>>;
    fn SaveAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn MarkMessageAsSeenAsync(&self, messageid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn MarkFolderAsSeenAsync(&self, folderid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn MarkMessageReadAsync(&self, messageid: &::windows::core::HSTRING, isread: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ChangeMessageFlagStateAsync(&self, messageid: &::windows::core::HSTRING, flagstate: EmailFlagState) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn TryMoveMessageAsync(&self, messageid: &::windows::core::HSTRING, newparentfolderid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryMoveFolderAsync(&self, folderid: &::windows::core::HSTRING, newparentfolderid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryMoveFolderWithNewNameAsync(&self, folderid: &::windows::core::HSTRING, newparentfolderid: &::windows::core::HSTRING, newfoldername: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn DeleteMessageAsync(&self, messageid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn MarkFolderSyncEnabledAsync(&self, folderid: &::windows::core::HSTRING, issyncenabled: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SendMessageAsync(&self, message: &::core::option::Option<EmailMessage>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SaveDraftAsync(&self, message: &::core::option::Option<EmailMessage>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DownloadMessageAsync(&self, messageid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DownloadAttachmentAsync(&self, attachmentid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn CreateResponseMessageAsync(&self, messageid: &::windows::core::HSTRING, responsetype: EmailMessageResponseKind, subject: &::windows::core::HSTRING, responseheadertype: EmailMessageBodyKind, responseheader: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailMessage>>;
    fn TryUpdateMeetingResponseAsync(&self, meeting: &::core::option::Option<EmailMessage>, response: EmailMeetingResponseType, subject: &::windows::core::HSTRING, comment: &::windows::core::HSTRING, sendupdate: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryForwardMeetingAsync(&self, meeting: &::core::option::Option<EmailMessage>, recipients: &::core::option::Option<super::super::Foundation::Collections::IIterable<EmailRecipient>>, subject: &::windows::core::HSTRING, forwardheadertype: EmailMessageBodyKind, forwardheader: &::windows::core::HSTRING, comment: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryProposeNewTimeForMeetingAsync(&self, meeting: &::core::option::Option<EmailMessage>, newstarttime: &super::super::Foundation::DateTime, newduration: &super::super::Foundation::TimeSpan, subject: &::windows::core::HSTRING, comment: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn MailboxChanged(&self, phandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<EmailMailbox, EmailMailboxChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMailboxChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SmartSendMessageAsync(&self, message: &::core::option::Option<EmailMessage>, smartsend: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn TrySetAutoReplySettingsAsync(&self, autoreplysettings: &::core::option::Option<EmailMailboxAutoReplySettings>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryGetAutoReplySettingsAsync(&self, requestedformat: EmailMailboxAutoReplyMessageResponseKind) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailMailboxAutoReplySettings>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailbox2Impl: Sized + IEmailMailboxImpl {
    fn LinkedMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NetworkAccountId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NetworkId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailbox3Impl: Sized + IEmailMailboxImpl + IEmailMailbox2Impl {
    fn ResolveRecipientsAsync(&self, recipients: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<EmailRecipientResolutionResult>>>;
    fn ValidateCertificatesAsync(&self, certificates: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Security::Cryptography::Certificates::Certificate>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<EmailCertificateValidationStatus>>>;
    fn TryEmptyFolderAsync(&self, folderid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailMailboxEmptyFolderStatus>>;
    fn TryCreateFolderAsync(&self, parentfolderid: &::windows::core::HSTRING, name: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailMailboxCreateFolderResult>>;
    fn TryDeleteFolderAsync(&self, folderid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailMailboxDeleteFolderStatus>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailbox4Impl: Sized {
    fn RegisterSyncManagerAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailbox5Impl: Sized {
    fn GetChangeTracker(&self, identity: &::windows::core::HSTRING) -> ::windows::core::Result<EmailMailboxChangeTracker>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxActionImpl: Sized {
    fn Kind(&self) -> ::windows::core::Result<EmailMailboxActionKind>;
    fn ChangeNumber(&self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxAutoReplyImpl: Sized {
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn Response(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetResponse(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxAutoReplySettingsImpl: Sized {
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn ResponseKind(&self) -> ::windows::core::Result<EmailMailboxAutoReplyMessageResponseKind>;
    fn SetResponseKind(&self, value: EmailMailboxAutoReplyMessageResponseKind) -> ::windows::core::Result<()>;
    fn StartTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn SetStartTime(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn EndTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn SetEndTime(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn InternalReply(&self) -> ::windows::core::Result<EmailMailboxAutoReply>;
    fn KnownExternalReply(&self) -> ::windows::core::Result<EmailMailboxAutoReply>;
    fn UnknownExternalReply(&self) -> ::windows::core::Result<EmailMailboxAutoReply>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxCapabilitiesImpl: Sized {
    fn CanForwardMeetings(&self) -> ::windows::core::Result<bool>;
    fn CanGetAndSetExternalAutoReplies(&self) -> ::windows::core::Result<bool>;
    fn CanGetAndSetInternalAutoReplies(&self) -> ::windows::core::Result<bool>;
    fn CanUpdateMeetingResponses(&self) -> ::windows::core::Result<bool>;
    fn CanServerSearchFolders(&self) -> ::windows::core::Result<bool>;
    fn CanServerSearchMailbox(&self) -> ::windows::core::Result<bool>;
    fn CanProposeNewTimeForMeetings(&self) -> ::windows::core::Result<bool>;
    fn CanSmartSend(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxCapabilities2Impl: Sized {
    fn CanResolveRecipients(&self) -> ::windows::core::Result<bool>;
    fn CanValidateCertificates(&self) -> ::windows::core::Result<bool>;
    fn CanEmptyFolder(&self) -> ::windows::core::Result<bool>;
    fn CanCreateFolder(&self) -> ::windows::core::Result<bool>;
    fn CanDeleteFolder(&self) -> ::windows::core::Result<bool>;
    fn CanMoveFolder(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxCapabilities3Impl: Sized {
    fn SetCanForwardMeetings(&self, value: bool) -> ::windows::core::Result<()>;
    fn SetCanGetAndSetExternalAutoReplies(&self, value: bool) -> ::windows::core::Result<()>;
    fn SetCanGetAndSetInternalAutoReplies(&self, value: bool) -> ::windows::core::Result<()>;
    fn SetCanUpdateMeetingResponses(&self, value: bool) -> ::windows::core::Result<()>;
    fn SetCanServerSearchFolders(&self, value: bool) -> ::windows::core::Result<()>;
    fn SetCanServerSearchMailbox(&self, value: bool) -> ::windows::core::Result<()>;
    fn SetCanProposeNewTimeForMeetings(&self, value: bool) -> ::windows::core::Result<()>;
    fn SetCanSmartSend(&self, value: bool) -> ::windows::core::Result<()>;
    fn SetCanResolveRecipients(&self, value: bool) -> ::windows::core::Result<()>;
    fn SetCanValidateCertificates(&self, value: bool) -> ::windows::core::Result<()>;
    fn SetCanEmptyFolder(&self, value: bool) -> ::windows::core::Result<()>;
    fn SetCanCreateFolder(&self, value: bool) -> ::windows::core::Result<()>;
    fn SetCanDeleteFolder(&self, value: bool) -> ::windows::core::Result<()>;
    fn SetCanMoveFolder(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxChangeImpl: Sized {
    fn ChangeType(&self) -> ::windows::core::Result<EmailMailboxChangeType>;
    fn MailboxActions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<EmailMailboxAction>>;
    fn Message(&self) -> ::windows::core::Result<EmailMessage>;
    fn Folder(&self) -> ::windows::core::Result<EmailFolder>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxChangeReaderImpl: Sized {
    fn AcceptChanges(&self) -> ::windows::core::Result<()>;
    fn AcceptChangesThrough(&self, lastchangetoacknowledge: &::core::option::Option<EmailMailboxChange>) -> ::windows::core::Result<()>;
    fn ReadBatchAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<EmailMailboxChange>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxChangeTrackerImpl: Sized {
    fn IsTracking(&self) -> ::windows::core::Result<bool>;
    fn Enable(&self) -> ::windows::core::Result<()>;
    fn GetChangeReader(&self) -> ::windows::core::Result<EmailMailboxChangeReader>;
    fn Reset(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxChangedDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxChangedEventArgsImpl: Sized {
    fn GetDeferral(&self) -> ::windows::core::Result<EmailMailboxChangedDeferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxCreateFolderResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<EmailMailboxCreateFolderStatus>;
    fn Folder(&self) -> ::windows::core::Result<EmailFolder>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxPoliciesImpl: Sized {
    fn AllowedSmimeEncryptionAlgorithmNegotiation(&self) -> ::windows::core::Result<EmailMailboxAllowedSmimeEncryptionAlgorithmNegotiation>;
    fn AllowSmimeSoftCertificates(&self) -> ::windows::core::Result<bool>;
    fn RequiredSmimeEncryptionAlgorithm(&self) -> ::windows::core::Result<super::super::Foundation::IReference<EmailMailboxSmimeEncryptionAlgorithm>>;
    fn RequiredSmimeSigningAlgorithm(&self) -> ::windows::core::Result<super::super::Foundation::IReference<EmailMailboxSmimeSigningAlgorithm>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxPolicies2Impl: Sized {
    fn MustEncryptSmimeMessages(&self) -> ::windows::core::Result<bool>;
    fn MustSignSmimeMessages(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxPolicies3Impl: Sized {
    fn SetAllowedSmimeEncryptionAlgorithmNegotiation(&self, value: EmailMailboxAllowedSmimeEncryptionAlgorithmNegotiation) -> ::windows::core::Result<()>;
    fn SetAllowSmimeSoftCertificates(&self, value: bool) -> ::windows::core::Result<()>;
    fn SetRequiredSmimeEncryptionAlgorithm(&self, value: &::core::option::Option<super::super::Foundation::IReference<EmailMailboxSmimeEncryptionAlgorithm>>) -> ::windows::core::Result<()>;
    fn SetRequiredSmimeSigningAlgorithm(&self, value: &::core::option::Option<super::super::Foundation::IReference<EmailMailboxSmimeSigningAlgorithm>>) -> ::windows::core::Result<()>;
    fn SetMustEncryptSmimeMessages(&self, value: bool) -> ::windows::core::Result<()>;
    fn SetMustSignSmimeMessages(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxSyncManagerImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<EmailMailboxSyncStatus>;
    fn LastSuccessfulSyncTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn LastAttemptedSyncTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn SyncAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn SyncStatusChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<EmailMailboxSyncManager, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSyncStatusChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxSyncManager2Impl: Sized {
    fn SetStatus(&self, value: EmailMailboxSyncStatus) -> ::windows::core::Result<()>;
    fn SetLastSuccessfulSyncTime(&self, value: &super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn SetLastAttemptedSyncTime(&self, value: &super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailManagerForUserImpl: Sized {
    fn ShowComposeNewEmailAsync(&self, message: &::core::option::Option<EmailMessage>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn RequestStoreAsync(&self, accesstype: EmailStoreAccessType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailStore>>;
    fn User(&self) -> ::windows::core::Result<super::super::System::User>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailManagerStaticsImpl: Sized {
    fn ShowComposeNewEmailAsync(&self, message: &::core::option::Option<EmailMessage>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailManagerStatics2Impl: Sized {
    fn RequestStoreAsync(&self, accesstype: EmailStoreAccessType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailStore>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailManagerStatics3Impl: Sized {
    fn GetForUser(&self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<EmailManagerForUser>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMeetingInfoImpl: Sized {
    fn AllowNewTimeProposal(&self) -> ::windows::core::Result<bool>;
    fn SetAllowNewTimeProposal(&self, value: bool) -> ::windows::core::Result<()>;
    fn AppointmentRoamingId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAppointmentRoamingId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AppointmentOriginalStartTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn SetAppointmentOriginalStartTime(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetDuration(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn IsAllDay(&self) -> ::windows::core::Result<bool>;
    fn SetIsAllDay(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsResponseRequested(&self) -> ::windows::core::Result<bool>;
    fn SetIsResponseRequested(&self, value: bool) -> ::windows::core::Result<()>;
    fn Location(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLocation(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ProposedStartTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn SetProposedStartTime(&self, proposedstarttime: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn ProposedDuration(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn SetProposedDuration(&self, duration: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn RecurrenceStartTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn SetRecurrenceStartTime(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn Recurrence(&self) -> ::windows::core::Result<super::Appointments::AppointmentRecurrence>;
    fn SetRecurrence(&self, value: &::core::option::Option<super::Appointments::AppointmentRecurrence>) -> ::windows::core::Result<()>;
    fn RemoteChangeNumber(&self) -> ::windows::core::Result<u64>;
    fn SetRemoteChangeNumber(&self, value: u64) -> ::windows::core::Result<()>;
    fn StartTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn SetStartTime(&self, value: &super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMeetingInfo2Impl: Sized {
    fn IsReportedOutOfDateByServer(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMessageImpl: Sized {
    fn Subject(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSubject(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Body(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetBody(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn To(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<EmailRecipient>>;
    fn CC(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<EmailRecipient>>;
    fn Bcc(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<EmailRecipient>>;
    fn Attachments(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<EmailAttachment>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMessage2Impl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RemoteId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetRemoteId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn MailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ConversationId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FolderId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AllowInternetImages(&self) -> ::windows::core::Result<bool>;
    fn SetAllowInternetImages(&self, value: bool) -> ::windows::core::Result<()>;
    fn ChangeNumber(&self) -> ::windows::core::Result<u64>;
    fn DownloadState(&self) -> ::windows::core::Result<EmailMessageDownloadState>;
    fn SetDownloadState(&self, value: EmailMessageDownloadState) -> ::windows::core::Result<()>;
    fn EstimatedDownloadSizeInBytes(&self) -> ::windows::core::Result<u32>;
    fn SetEstimatedDownloadSizeInBytes(&self, value: u32) -> ::windows::core::Result<()>;
    fn FlagState(&self) -> ::windows::core::Result<EmailFlagState>;
    fn SetFlagState(&self, value: EmailFlagState) -> ::windows::core::Result<()>;
    fn HasPartialBodies(&self) -> ::windows::core::Result<bool>;
    fn Importance(&self) -> ::windows::core::Result<EmailImportance>;
    fn SetImportance(&self, value: EmailImportance) -> ::windows::core::Result<()>;
    fn InResponseToMessageId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IrmInfo(&self) -> ::windows::core::Result<EmailIrmInfo>;
    fn SetIrmInfo(&self, value: &::core::option::Option<EmailIrmInfo>) -> ::windows::core::Result<()>;
    fn IsDraftMessage(&self) -> ::windows::core::Result<bool>;
    fn IsRead(&self) -> ::windows::core::Result<bool>;
    fn SetIsRead(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsSeen(&self) -> ::windows::core::Result<bool>;
    fn SetIsSeen(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsServerSearchMessage(&self) -> ::windows::core::Result<bool>;
    fn IsSmartSendable(&self) -> ::windows::core::Result<bool>;
    fn MessageClass(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetMessageClass(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn NormalizedSubject(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn OriginalCodePage(&self) -> ::windows::core::Result<i32>;
    fn SetOriginalCodePage(&self, value: i32) -> ::windows::core::Result<()>;
    fn Preview(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPreview(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn LastResponseKind(&self) -> ::windows::core::Result<EmailMessageResponseKind>;
    fn SetLastResponseKind(&self, value: EmailMessageResponseKind) -> ::windows::core::Result<()>;
    fn Sender(&self) -> ::windows::core::Result<EmailRecipient>;
    fn SetSender(&self, value: &::core::option::Option<EmailRecipient>) -> ::windows::core::Result<()>;
    fn SentTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn SetSentTime(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn MeetingInfo(&self) -> ::windows::core::Result<EmailMeetingInfo>;
    fn SetMeetingInfo(&self, value: &::core::option::Option<EmailMeetingInfo>) -> ::windows::core::Result<()>;
    fn GetBodyStream(&self, r#type: EmailMessageBodyKind) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn SetBodyStream(&self, r#type: EmailMessageBodyKind, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMessage3Impl: Sized {
    fn SmimeData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn SetSmimeData(&self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<()>;
    fn SmimeKind(&self) -> ::windows::core::Result<EmailMessageSmimeKind>;
    fn SetSmimeKind(&self, value: EmailMessageSmimeKind) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMessage4Impl: Sized {
    fn ReplyTo(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<EmailRecipient>>;
    fn SentRepresenting(&self) -> ::windows::core::Result<EmailRecipient>;
    fn SetSentRepresenting(&self, value: &::core::option::Option<EmailRecipient>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMessageBatchImpl: Sized {
    fn Messages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<EmailMessage>>;
    fn Status(&self) -> ::windows::core::Result<EmailBatchStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMessageReaderImpl: Sized {
    fn ReadBatchAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailMessageBatch>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailQueryOptionsImpl: Sized {
    fn TextSearch(&self) -> ::windows::core::Result<EmailQueryTextSearch>;
    fn SortDirection(&self) -> ::windows::core::Result<EmailQuerySortDirection>;
    fn SetSortDirection(&self, value: EmailQuerySortDirection) -> ::windows::core::Result<()>;
    fn SortProperty(&self) -> ::windows::core::Result<EmailQuerySortProperty>;
    fn SetSortProperty(&self, value: EmailQuerySortProperty) -> ::windows::core::Result<()>;
    fn Kind(&self) -> ::windows::core::Result<EmailQueryKind>;
    fn SetKind(&self, value: EmailQueryKind) -> ::windows::core::Result<()>;
    fn FolderIds(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailQueryOptionsFactoryImpl: Sized {
    fn CreateWithText(&self, text: &::windows::core::HSTRING) -> ::windows::core::Result<EmailQueryOptions>;
    fn CreateWithTextAndFields(&self, text: &::windows::core::HSTRING, fields: EmailQuerySearchFields) -> ::windows::core::Result<EmailQueryOptions>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailQueryTextSearchImpl: Sized {
    fn Fields(&self) -> ::windows::core::Result<EmailQuerySearchFields>;
    fn SetFields(&self, value: EmailQuerySearchFields) -> ::windows::core::Result<()>;
    fn SearchScope(&self) -> ::windows::core::Result<EmailQuerySearchScope>;
    fn SetSearchScope(&self, value: EmailQuerySearchScope) -> ::windows::core::Result<()>;
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailRecipientImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Address(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAddress(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailRecipientFactoryImpl: Sized {
    fn Create(&self, address: &::windows::core::HSTRING) -> ::windows::core::Result<EmailRecipient>;
    fn CreateWithName(&self, address: &::windows::core::HSTRING, name: &::windows::core::HSTRING) -> ::windows::core::Result<EmailRecipient>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailRecipientResolutionResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<EmailRecipientResolutionStatus>;
    fn PublicKeys(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::Certificate>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailRecipientResolutionResult2Impl: Sized {
    fn SetStatus(&self, value: EmailRecipientResolutionStatus) -> ::windows::core::Result<()>;
    fn SetPublicKeys(&self, value: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Security::Cryptography::Certificates::Certificate>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailStoreImpl: Sized {
    fn FindMailboxesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<EmailMailbox>>>;
    fn GetConversationReader(&self) -> ::windows::core::Result<EmailConversationReader>;
    fn GetConversationReaderWithOptions(&self, options: &::core::option::Option<EmailQueryOptions>) -> ::windows::core::Result<EmailConversationReader>;
    fn GetMessageReader(&self) -> ::windows::core::Result<EmailMessageReader>;
    fn GetMessageReaderWithOptions(&self, options: &::core::option::Option<EmailQueryOptions>) -> ::windows::core::Result<EmailMessageReader>;
    fn GetMailboxAsync(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailMailbox>>;
    fn GetConversationAsync(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailConversation>>;
    fn GetFolderAsync(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailFolder>>;
    fn GetMessageAsync(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailMessage>>;
    fn CreateMailboxAsync(&self, accountname: &::windows::core::HSTRING, accountaddress: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailMailbox>>;
    fn CreateMailboxInAccountAsync(&self, accountname: &::windows::core::HSTRING, accountaddress: &::windows::core::HSTRING, userdataaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailMailbox>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailStoreNotificationTriggerDetailsImpl: Sized {}
