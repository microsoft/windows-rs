#[cfg(feature = "implement_exclusive")]
pub trait IChatCapabilitiesImpl: Sized {
    fn IsOnline(&self) -> ::windows::core::Result<bool>;
    fn IsChatCapable(&self) -> ::windows::core::Result<bool>;
    fn IsFileTransferCapable(&self) -> ::windows::core::Result<bool>;
    fn IsGeoLocationPushCapable(&self) -> ::windows::core::Result<bool>;
    fn IsIntegratedMessagingCapable(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IChatCapabilitiesManagerStaticsImpl: Sized {
    fn GetCachedCapabilitiesAsync(&self, address: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ChatCapabilities>>;
    fn GetCapabilitiesFromNetworkAsync(&self, address: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ChatCapabilities>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IChatCapabilitiesManagerStatics2Impl: Sized {
    fn GetCachedCapabilitiesForTransportAsync(&self, address: &::windows::core::HSTRING, transportid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ChatCapabilities>>;
    fn GetCapabilitiesFromNetworkForTransportAsync(&self, address: &::windows::core::HSTRING, transportid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ChatCapabilities>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IChatConversationImpl: Sized {
    fn HasUnreadMessages(&self) -> ::windows::core::Result<bool>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Subject(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSubject(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsConversationMuted(&self) -> ::windows::core::Result<bool>;
    fn SetIsConversationMuted(&self, value: bool) -> ::windows::core::Result<()>;
    fn MostRecentMessageId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Participants(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn ThreadingInfo(&self) -> ::windows::core::Result<ChatConversationThreadingInfo>;
    fn DeleteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn GetMessageReader(&self) -> ::windows::core::Result<ChatMessageReader>;
    fn MarkAllMessagesAsReadAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn MarkMessagesAsReadAsync(&self, value: &super::super::Foundation::DateTime) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SaveAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn NotifyLocalParticipantComposing(&self, transportid: &::windows::core::HSTRING, participantaddress: &::windows::core::HSTRING, iscomposing: bool) -> ::windows::core::Result<()>;
    fn NotifyRemoteParticipantComposing(&self, transportid: &::windows::core::HSTRING, participantaddress: &::windows::core::HSTRING, iscomposing: bool) -> ::windows::core::Result<()>;
    fn RemoteParticipantComposingChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ChatConversation, RemoteParticipantComposingChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRemoteParticipantComposingChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IChatConversation2Impl: Sized {
    fn CanModifyParticipants(&self) -> ::windows::core::Result<bool>;
    fn SetCanModifyParticipants(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IChatConversationReaderImpl: Sized {
    fn ReadBatchAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatConversation>>>;
    fn ReadBatchWithCountAsync(&self, count: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatConversation>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IChatConversationThreadingInfoImpl: Sized {
    fn ContactId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetContactId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Custom(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCustom(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ConversationId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetConversationId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Participants(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn Kind(&self) -> ::windows::core::Result<ChatConversationThreadingKind>;
    fn SetKind(&self, value: ChatConversationThreadingKind) -> ::windows::core::Result<()>;
}
pub trait IChatItemImpl: Sized {
    fn ItemKind(&self) -> ::windows::core::Result<ChatItemKind>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IChatMessageImpl: Sized {
    fn Attachments(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ChatMessageAttachment>>;
    fn Body(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetBody(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn From(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsForwardingDisabled(&self) -> ::windows::core::Result<bool>;
    fn IsIncoming(&self) -> ::windows::core::Result<bool>;
    fn IsRead(&self) -> ::windows::core::Result<bool>;
    fn LocalTimestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn NetworkTimestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn Recipients(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn RecipientSendStatuses(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ChatMessageStatus>>;
    fn Status(&self) -> ::windows::core::Result<ChatMessageStatus>;
    fn Subject(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TransportFriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TransportId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTransportId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IChatMessage2Impl: Sized + IChatMessageImpl + IChatMessage3Impl {
    fn EstimatedDownloadSize(&self) -> ::windows::core::Result<u64>;
    fn SetEstimatedDownloadSize(&self, value: u64) -> ::windows::core::Result<()>;
    fn SetFrom(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsAutoReply(&self) -> ::windows::core::Result<bool>;
    fn SetIsAutoReply(&self, value: bool) -> ::windows::core::Result<()>;
    fn SetIsForwardingDisabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsReplyDisabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsIncoming(&self, value: bool) -> ::windows::core::Result<()>;
    fn SetIsRead(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsSeen(&self) -> ::windows::core::Result<bool>;
    fn SetIsSeen(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsSimMessage(&self) -> ::windows::core::Result<bool>;
    fn SetLocalTimestamp(&self, value: &super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn MessageKind(&self) -> ::windows::core::Result<ChatMessageKind>;
    fn SetMessageKind(&self, value: ChatMessageKind) -> ::windows::core::Result<()>;
    fn MessageOperatorKind(&self) -> ::windows::core::Result<ChatMessageOperatorKind>;
    fn SetMessageOperatorKind(&self, value: ChatMessageOperatorKind) -> ::windows::core::Result<()>;
    fn SetNetworkTimestamp(&self, value: &super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn IsReceivedDuringQuietHours(&self) -> ::windows::core::Result<bool>;
    fn SetIsReceivedDuringQuietHours(&self, value: bool) -> ::windows::core::Result<()>;
    fn SetRemoteId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetStatus(&self, value: ChatMessageStatus) -> ::windows::core::Result<()>;
    fn SetSubject(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ShouldSuppressNotification(&self) -> ::windows::core::Result<bool>;
    fn SetShouldSuppressNotification(&self, value: bool) -> ::windows::core::Result<()>;
    fn ThreadingInfo(&self) -> ::windows::core::Result<ChatConversationThreadingInfo>;
    fn SetThreadingInfo(&self, value: &::core::option::Option<ChatConversationThreadingInfo>) -> ::windows::core::Result<()>;
    fn RecipientsDeliveryInfos(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ChatRecipientDeliveryInfo>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IChatMessage3Impl: Sized + IChatMessageImpl {
    fn RemoteId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IChatMessage4Impl: Sized + IChatMessageImpl {
    fn SyncId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSyncId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IChatMessageAttachmentImpl: Sized {
    fn DataStreamReference(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn SetDataStreamReference(&self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<()>;
    fn GroupId(&self) -> ::windows::core::Result<u32>;
    fn SetGroupId(&self, value: u32) -> ::windows::core::Result<()>;
    fn MimeType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetMimeType(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IChatMessageAttachment2Impl: Sized + IChatMessageAttachmentImpl {
    fn Thumbnail(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn SetThumbnail(&self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<()>;
    fn TransferProgress(&self) -> ::windows::core::Result<f64>;
    fn SetTransferProgress(&self, value: f64) -> ::windows::core::Result<()>;
    fn OriginalFileName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetOriginalFileName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IChatMessageAttachmentFactoryImpl: Sized {
    fn CreateChatMessageAttachment(&self, mimetype: &::windows::core::HSTRING, datastreamreference: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<ChatMessageAttachment>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IChatMessageBlockingStaticImpl: Sized {
    fn MarkMessageAsBlockedAsync(&self, localchatmessageid: &::windows::core::HSTRING, blocked: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IChatMessageChangeImpl: Sized {
    fn ChangeType(&self) -> ::windows::core::Result<ChatMessageChangeType>;
    fn Message(&self) -> ::windows::core::Result<ChatMessage>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IChatMessageChangeReaderImpl: Sized {
    fn AcceptChanges(&self) -> ::windows::core::Result<()>;
    fn AcceptChangesThrough(&self, lastchangetoacknowledge: &::core::option::Option<ChatMessageChange>) -> ::windows::core::Result<()>;
    fn ReadBatchAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatMessageChange>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IChatMessageChangeTrackerImpl: Sized {
    fn Enable(&self) -> ::windows::core::Result<()>;
    fn GetChangeReader(&self) -> ::windows::core::Result<ChatMessageChangeReader>;
    fn Reset(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IChatMessageChangedDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IChatMessageChangedEventArgsImpl: Sized {
    fn GetDeferral(&self) -> ::windows::core::Result<ChatMessageChangedDeferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IChatMessageManager2StaticsImpl: Sized + IChatMessageManagerStaticImpl {
    fn RegisterTransportAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetTransportAsync(&self, transportid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ChatMessageTransport>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IChatMessageManagerStaticImpl: Sized {
    fn GetTransportsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatMessageTransport>>>;
    fn RequestStoreAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ChatMessageStore>>;
    fn ShowComposeSmsMessageAsync(&self, message: &::core::option::Option<ChatMessage>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ShowSmsSettings(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IChatMessageManagerStatics3Impl: Sized + IChatMessageManagerStaticImpl {
    fn RequestSyncManagerAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ChatSyncManager>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IChatMessageNotificationTriggerDetailsImpl: Sized {
    fn ChatMessage(&self) -> ::windows::core::Result<ChatMessage>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IChatMessageNotificationTriggerDetails2Impl: Sized + IChatMessageNotificationTriggerDetailsImpl {
    fn ShouldDisplayToast(&self) -> ::windows::core::Result<bool>;
    fn ShouldUpdateDetailText(&self) -> ::windows::core::Result<bool>;
    fn ShouldUpdateBadge(&self) -> ::windows::core::Result<bool>;
    fn ShouldUpdateActionCenter(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IChatMessageReaderImpl: Sized {
    fn ReadBatchAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatMessage>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IChatMessageReader2Impl: Sized {
    fn ReadBatchWithCountAsync(&self, count: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatMessage>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IChatMessageStoreImpl: Sized {
    fn ChangeTracker(&self) -> ::windows::core::Result<ChatMessageChangeTracker>;
    fn DeleteMessageAsync(&self, localmessageid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DownloadMessageAsync(&self, localchatmessageid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn GetMessageAsync(&self, localchatmessageid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ChatMessage>>;
    fn GetMessageReader1(&self) -> ::windows::core::Result<ChatMessageReader>;
    fn GetMessageReader2(&self, recenttimelimit: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<ChatMessageReader>;
    fn MarkMessageReadAsync(&self, localchatmessageid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn RetrySendMessageAsync(&self, localchatmessageid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SendMessageAsync(&self, chatmessage: &::core::option::Option<ChatMessage>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ValidateMessage(&self, chatmessage: &::core::option::Option<ChatMessage>) -> ::windows::core::Result<ChatMessageValidationResult>;
    fn MessageChanged(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<ChatMessageStore, ChatMessageChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMessageChanged(&self, value: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IChatMessageStore2Impl: Sized + IChatMessageStoreImpl {
    fn ForwardMessageAsync(&self, localchatmessageid: &::windows::core::HSTRING, addresses: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ChatMessage>>;
    fn GetConversationAsync(&self, conversationid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ChatConversation>>;
    fn GetConversationForTransportsAsync(&self, conversationid: &::windows::core::HSTRING, transportids: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ChatConversation>>;
    fn GetConversationFromThreadingInfoAsync(&self, threadinginfo: &::core::option::Option<ChatConversationThreadingInfo>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ChatConversation>>;
    fn GetConversationReader(&self) -> ::windows::core::Result<ChatConversationReader>;
    fn GetConversationForTransportsReader(&self, transportids: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<ChatConversationReader>;
    fn GetMessageByRemoteIdAsync(&self, transportid: &::windows::core::HSTRING, remoteid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ChatMessage>>;
    fn GetUnseenCountAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<i32>>;
    fn GetUnseenCountForTransportsReaderAsync(&self, transportids: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<i32>>;
    fn MarkAsSeenAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn MarkAsSeenForTransportsAsync(&self, transportids: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn GetSearchReader(&self, value: &::core::option::Option<ChatQueryOptions>) -> ::windows::core::Result<ChatSearchReader>;
    fn SaveMessageAsync(&self, chatmessage: &::core::option::Option<ChatMessage>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn TryCancelDownloadMessageAsync(&self, localchatmessageid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryCancelSendMessageAsync(&self, localchatmessageid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn StoreChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ChatMessageStore, ChatMessageStoreChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStoreChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IChatMessageStore3Impl: Sized + IChatMessageStoreImpl {
    fn GetMessageBySyncIdAsync(&self, syncid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ChatMessage>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IChatMessageStoreChangedEventArgsImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Kind(&self) -> ::windows::core::Result<ChatStoreChangedEventKind>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IChatMessageTransportImpl: Sized {
    fn IsAppSetAsNotificationProvider(&self) -> ::windows::core::Result<bool>;
    fn IsActive(&self) -> ::windows::core::Result<bool>;
    fn TransportFriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TransportId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RequestSetAsNotificationProviderAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IChatMessageTransport2Impl: Sized + IChatMessageTransportImpl {
    fn Configuration(&self) -> ::windows::core::Result<ChatMessageTransportConfiguration>;
    fn TransportKind(&self) -> ::windows::core::Result<ChatMessageTransportKind>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IChatMessageTransportConfigurationImpl: Sized {
    fn MaxAttachmentCount(&self) -> ::windows::core::Result<i32>;
    fn MaxMessageSizeInKilobytes(&self) -> ::windows::core::Result<i32>;
    fn MaxRecipientCount(&self) -> ::windows::core::Result<i32>;
    fn SupportedVideoFormat(&self) -> ::windows::core::Result<super::super::Media::MediaProperties::MediaEncodingProfile>;
    fn ExtendedProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IChatMessageValidationResultImpl: Sized {
    fn MaxPartCount(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn PartCount(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn RemainingCharacterCountInPart(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn Status(&self) -> ::windows::core::Result<ChatMessageValidationStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IChatQueryOptionsImpl: Sized {
    fn SearchString(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSearchString(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IChatRecipientDeliveryInfoImpl: Sized {
    fn TransportAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTransportAddress(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DeliveryTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn SetDeliveryTime(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn ReadTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn SetReadTime(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn TransportErrorCodeCategory(&self) -> ::windows::core::Result<ChatTransportErrorCodeCategory>;
    fn TransportInterpretedErrorCode(&self) -> ::windows::core::Result<ChatTransportInterpretedErrorCode>;
    fn TransportErrorCode(&self) -> ::windows::core::Result<i32>;
    fn IsErrorPermanent(&self) -> ::windows::core::Result<bool>;
    fn Status(&self) -> ::windows::core::Result<ChatMessageStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IChatSearchReaderImpl: Sized {
    fn ReadBatchAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IChatItem>>>;
    fn ReadBatchWithCountAsync(&self, count: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IChatItem>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IChatSyncConfigurationImpl: Sized {
    fn IsSyncEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsSyncEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn RestoreHistorySpan(&self) -> ::windows::core::Result<ChatRestoreHistorySpan>;
    fn SetRestoreHistorySpan(&self, value: ChatRestoreHistorySpan) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IChatSyncManagerImpl: Sized {
    fn Configuration(&self) -> ::windows::core::Result<ChatSyncConfiguration>;
    fn AssociateAccountAsync(&self, webaccount: &::core::option::Option<super::super::Security::Credentials::WebAccount>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn UnassociateAccountAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn IsAccountAssociated(&self, webaccount: &::core::option::Option<super::super::Security::Credentials::WebAccount>) -> ::windows::core::Result<bool>;
    fn StartSync(&self) -> ::windows::core::Result<()>;
    fn SetConfigurationAsync(&self, configuration: &::core::option::Option<ChatSyncConfiguration>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRcsEndUserMessageImpl: Sized {
    fn TransportId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsPinRequired(&self) -> ::windows::core::Result<bool>;
    fn Actions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<RcsEndUserMessageAction>>;
    fn SendResponseAsync(&self, action: &::core::option::Option<RcsEndUserMessageAction>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SendResponseWithPinAsync(&self, action: &::core::option::Option<RcsEndUserMessageAction>, pin: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRcsEndUserMessageActionImpl: Sized {
    fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRcsEndUserMessageAvailableEventArgsImpl: Sized {
    fn IsMessageAvailable(&self) -> ::windows::core::Result<bool>;
    fn Message(&self) -> ::windows::core::Result<RcsEndUserMessage>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRcsEndUserMessageAvailableTriggerDetailsImpl: Sized {
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRcsEndUserMessageManagerImpl: Sized {
    fn MessageAvailableChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RcsEndUserMessageManager, RcsEndUserMessageAvailableEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMessageAvailableChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRcsManagerStaticsImpl: Sized {
    fn GetEndUserMessageManager(&self) -> ::windows::core::Result<RcsEndUserMessageManager>;
    fn GetTransportsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<RcsTransport>>>;
    fn GetTransportAsync(&self, transportid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<RcsTransport>>;
    fn LeaveConversationAsync(&self, conversation: &::core::option::Option<ChatConversation>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRcsManagerStatics2Impl: Sized {
    fn TransportListChanged(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveTransportListChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRcsServiceKindSupportedChangedEventArgsImpl: Sized {
    fn ServiceKind(&self) -> ::windows::core::Result<RcsServiceKind>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRcsTransportImpl: Sized {
    fn ExtendedProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
    fn IsActive(&self) -> ::windows::core::Result<bool>;
    fn TransportFriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TransportId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Configuration(&self) -> ::windows::core::Result<RcsTransportConfiguration>;
    fn IsStoreAndForwardEnabled(&self, servicekind: RcsServiceKind) -> ::windows::core::Result<bool>;
    fn IsServiceKindSupported(&self, servicekind: RcsServiceKind) -> ::windows::core::Result<bool>;
    fn ServiceKindSupportedChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RcsTransport, RcsServiceKindSupportedChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveServiceKindSupportedChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRcsTransportConfigurationImpl: Sized {
    fn MaxAttachmentCount(&self) -> ::windows::core::Result<i32>;
    fn MaxMessageSizeInKilobytes(&self) -> ::windows::core::Result<i32>;
    fn MaxGroupMessageSizeInKilobytes(&self) -> ::windows::core::Result<i32>;
    fn MaxRecipientCount(&self) -> ::windows::core::Result<i32>;
    fn MaxFileSizeInKilobytes(&self) -> ::windows::core::Result<i32>;
    fn WarningFileSizeInKilobytes(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteParticipantComposingChangedEventArgsImpl: Sized {
    fn TransportId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ParticipantAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsComposing(&self) -> ::windows::core::Result<bool>;
}
