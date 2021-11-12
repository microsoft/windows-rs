#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ChatCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ChatConversation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ChatConversationReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ChatConversationThreadingInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ChatConversationThreadingKind(pub i32);
impl ChatConversationThreadingKind {
    pub const Participants: ChatConversationThreadingKind = ChatConversationThreadingKind(0i32);
    pub const ContactId: ChatConversationThreadingKind = ChatConversationThreadingKind(1i32);
    pub const ConversationId: ChatConversationThreadingKind = ChatConversationThreadingKind(2i32);
    pub const Custom: ChatConversationThreadingKind = ChatConversationThreadingKind(3i32);
}
#[repr(transparent)]
pub struct ChatItemKind(pub i32);
impl ChatItemKind {
    pub const Message: ChatItemKind = ChatItemKind(0i32);
    pub const Conversation: ChatItemKind = ChatItemKind(1i32);
}
#[repr(transparent)]
pub struct ChatMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ChatMessageAttachment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ChatMessageChange(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ChatMessageChangeReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ChatMessageChangeTracker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ChatMessageChangeType(pub i32);
impl ChatMessageChangeType {
    pub const MessageCreated: ChatMessageChangeType = ChatMessageChangeType(0i32);
    pub const MessageModified: ChatMessageChangeType = ChatMessageChangeType(1i32);
    pub const MessageDeleted: ChatMessageChangeType = ChatMessageChangeType(2i32);
    pub const ChangeTrackingLost: ChatMessageChangeType = ChatMessageChangeType(3i32);
}
#[repr(transparent)]
pub struct ChatMessageChangedDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ChatMessageChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ChatMessageKind(pub i32);
impl ChatMessageKind {
    pub const Standard: ChatMessageKind = ChatMessageKind(0i32);
    pub const FileTransferRequest: ChatMessageKind = ChatMessageKind(1i32);
    pub const TransportCustom: ChatMessageKind = ChatMessageKind(2i32);
    pub const JoinedConversation: ChatMessageKind = ChatMessageKind(3i32);
    pub const LeftConversation: ChatMessageKind = ChatMessageKind(4i32);
    pub const OtherParticipantJoinedConversation: ChatMessageKind = ChatMessageKind(5i32);
    pub const OtherParticipantLeftConversation: ChatMessageKind = ChatMessageKind(6i32);
}
#[repr(transparent)]
pub struct ChatMessageNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ChatMessageOperatorKind(pub i32);
impl ChatMessageOperatorKind {
    pub const Unspecified: ChatMessageOperatorKind = ChatMessageOperatorKind(0i32);
    pub const Sms: ChatMessageOperatorKind = ChatMessageOperatorKind(1i32);
    pub const Mms: ChatMessageOperatorKind = ChatMessageOperatorKind(2i32);
    pub const Rcs: ChatMessageOperatorKind = ChatMessageOperatorKind(3i32);
}
#[repr(transparent)]
pub struct ChatMessageReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ChatMessageStatus(pub i32);
impl ChatMessageStatus {
    pub const Draft: ChatMessageStatus = ChatMessageStatus(0i32);
    pub const Sending: ChatMessageStatus = ChatMessageStatus(1i32);
    pub const Sent: ChatMessageStatus = ChatMessageStatus(2i32);
    pub const SendRetryNeeded: ChatMessageStatus = ChatMessageStatus(3i32);
    pub const SendFailed: ChatMessageStatus = ChatMessageStatus(4i32);
    pub const Received: ChatMessageStatus = ChatMessageStatus(5i32);
    pub const ReceiveDownloadNeeded: ChatMessageStatus = ChatMessageStatus(6i32);
    pub const ReceiveDownloadFailed: ChatMessageStatus = ChatMessageStatus(7i32);
    pub const ReceiveDownloading: ChatMessageStatus = ChatMessageStatus(8i32);
    pub const Deleted: ChatMessageStatus = ChatMessageStatus(9i32);
    pub const Declined: ChatMessageStatus = ChatMessageStatus(10i32);
    pub const Cancelled: ChatMessageStatus = ChatMessageStatus(11i32);
    pub const Recalled: ChatMessageStatus = ChatMessageStatus(12i32);
    pub const ReceiveRetryNeeded: ChatMessageStatus = ChatMessageStatus(13i32);
}
#[repr(transparent)]
pub struct ChatMessageStore(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ChatMessageStoreChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ChatMessageTransport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ChatMessageTransportConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ChatMessageTransportKind(pub i32);
impl ChatMessageTransportKind {
    pub const Text: ChatMessageTransportKind = ChatMessageTransportKind(0i32);
    pub const Untriaged: ChatMessageTransportKind = ChatMessageTransportKind(1i32);
    pub const Blocked: ChatMessageTransportKind = ChatMessageTransportKind(2i32);
    pub const Custom: ChatMessageTransportKind = ChatMessageTransportKind(3i32);
}
#[repr(transparent)]
pub struct ChatMessageValidationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ChatMessageValidationStatus(pub i32);
impl ChatMessageValidationStatus {
    pub const Valid: ChatMessageValidationStatus = ChatMessageValidationStatus(0i32);
    pub const NoRecipients: ChatMessageValidationStatus = ChatMessageValidationStatus(1i32);
    pub const InvalidData: ChatMessageValidationStatus = ChatMessageValidationStatus(2i32);
    pub const MessageTooLarge: ChatMessageValidationStatus = ChatMessageValidationStatus(3i32);
    pub const TooManyRecipients: ChatMessageValidationStatus = ChatMessageValidationStatus(4i32);
    pub const TransportInactive: ChatMessageValidationStatus = ChatMessageValidationStatus(5i32);
    pub const TransportNotFound: ChatMessageValidationStatus = ChatMessageValidationStatus(6i32);
    pub const TooManyAttachments: ChatMessageValidationStatus = ChatMessageValidationStatus(7i32);
    pub const InvalidRecipients: ChatMessageValidationStatus = ChatMessageValidationStatus(8i32);
    pub const InvalidBody: ChatMessageValidationStatus = ChatMessageValidationStatus(9i32);
    pub const InvalidOther: ChatMessageValidationStatus = ChatMessageValidationStatus(10i32);
    pub const ValidWithLargeMessage: ChatMessageValidationStatus = ChatMessageValidationStatus(11i32);
    pub const VoiceRoamingRestriction: ChatMessageValidationStatus = ChatMessageValidationStatus(12i32);
    pub const DataRoamingRestriction: ChatMessageValidationStatus = ChatMessageValidationStatus(13i32);
}
#[repr(transparent)]
pub struct ChatQueryOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ChatRecipientDeliveryInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ChatRestoreHistorySpan(pub i32);
impl ChatRestoreHistorySpan {
    pub const LastMonth: ChatRestoreHistorySpan = ChatRestoreHistorySpan(0i32);
    pub const LastYear: ChatRestoreHistorySpan = ChatRestoreHistorySpan(1i32);
    pub const AnyTime: ChatRestoreHistorySpan = ChatRestoreHistorySpan(2i32);
}
#[repr(transparent)]
pub struct ChatSearchReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ChatStoreChangedEventKind(pub i32);
impl ChatStoreChangedEventKind {
    pub const NotificationsMissed: ChatStoreChangedEventKind = ChatStoreChangedEventKind(0i32);
    pub const StoreModified: ChatStoreChangedEventKind = ChatStoreChangedEventKind(1i32);
    pub const MessageCreated: ChatStoreChangedEventKind = ChatStoreChangedEventKind(2i32);
    pub const MessageModified: ChatStoreChangedEventKind = ChatStoreChangedEventKind(3i32);
    pub const MessageDeleted: ChatStoreChangedEventKind = ChatStoreChangedEventKind(4i32);
    pub const ConversationModified: ChatStoreChangedEventKind = ChatStoreChangedEventKind(5i32);
    pub const ConversationDeleted: ChatStoreChangedEventKind = ChatStoreChangedEventKind(6i32);
    pub const ConversationTransportDeleted: ChatStoreChangedEventKind = ChatStoreChangedEventKind(7i32);
}
#[repr(transparent)]
pub struct ChatSyncConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ChatSyncManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ChatTransportErrorCodeCategory(pub i32);
impl ChatTransportErrorCodeCategory {
    pub const None: ChatTransportErrorCodeCategory = ChatTransportErrorCodeCategory(0i32);
    pub const Http: ChatTransportErrorCodeCategory = ChatTransportErrorCodeCategory(1i32);
    pub const Network: ChatTransportErrorCodeCategory = ChatTransportErrorCodeCategory(2i32);
    pub const MmsServer: ChatTransportErrorCodeCategory = ChatTransportErrorCodeCategory(3i32);
}
#[repr(transparent)]
pub struct ChatTransportInterpretedErrorCode(pub i32);
impl ChatTransportInterpretedErrorCode {
    pub const None: ChatTransportInterpretedErrorCode = ChatTransportInterpretedErrorCode(0i32);
    pub const Unknown: ChatTransportInterpretedErrorCode = ChatTransportInterpretedErrorCode(1i32);
    pub const InvalidRecipientAddress: ChatTransportInterpretedErrorCode = ChatTransportInterpretedErrorCode(2i32);
    pub const NetworkConnectivity: ChatTransportInterpretedErrorCode = ChatTransportInterpretedErrorCode(3i32);
    pub const ServiceDenied: ChatTransportInterpretedErrorCode = ChatTransportInterpretedErrorCode(4i32);
    pub const Timeout: ChatTransportInterpretedErrorCode = ChatTransportInterpretedErrorCode(5i32);
}
#[repr(transparent)]
pub struct IChatCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IChatCapabilitiesManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IChatCapabilitiesManagerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IChatConversation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IChatConversation2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IChatConversationReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IChatConversationThreadingInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IChatItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IChatMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IChatMessage2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IChatMessage3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IChatMessage4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IChatMessageAttachment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IChatMessageAttachment2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IChatMessageAttachmentFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IChatMessageBlockingStatic(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IChatMessageChange(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IChatMessageChangeReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IChatMessageChangeTracker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IChatMessageChangedDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IChatMessageChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IChatMessageManager2Statics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IChatMessageManagerStatic(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IChatMessageManagerStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IChatMessageNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IChatMessageNotificationTriggerDetails2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IChatMessageReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IChatMessageReader2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IChatMessageStore(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IChatMessageStore2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IChatMessageStore3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IChatMessageStoreChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IChatMessageTransport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IChatMessageTransport2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IChatMessageTransportConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IChatMessageValidationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IChatQueryOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IChatRecipientDeliveryInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IChatSearchReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IChatSyncConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IChatSyncManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRcsEndUserMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRcsEndUserMessageAction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRcsEndUserMessageAvailableEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRcsEndUserMessageAvailableTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRcsEndUserMessageManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRcsManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRcsManagerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRcsServiceKindSupportedChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRcsTransport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRcsTransportConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRemoteParticipantComposingChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RcsEndUserMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RcsEndUserMessageAction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RcsEndUserMessageAvailableEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RcsEndUserMessageAvailableTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RcsEndUserMessageManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RcsServiceKind(pub i32);
impl RcsServiceKind {
    pub const Chat: RcsServiceKind = RcsServiceKind(0i32);
    pub const GroupChat: RcsServiceKind = RcsServiceKind(1i32);
    pub const FileTransfer: RcsServiceKind = RcsServiceKind(2i32);
    pub const Capability: RcsServiceKind = RcsServiceKind(3i32);
}
#[repr(transparent)]
pub struct RcsServiceKindSupportedChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RcsTransport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RcsTransportConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RemoteParticipantComposingChangedEventArgs(pub *mut ::core::ffi::c_void);
