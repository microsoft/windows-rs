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
    pub const Participants: Self = Self(0i32);
    pub const ContactId: Self = Self(1i32);
    pub const ConversationId: Self = Self(2i32);
    pub const Custom: Self = Self(3i32);
}
#[repr(transparent)]
pub struct ChatItemKind(pub i32);
impl ChatItemKind {
    pub const Message: Self = Self(0i32);
    pub const Conversation: Self = Self(1i32);
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
    pub const MessageCreated: Self = Self(0i32);
    pub const MessageModified: Self = Self(1i32);
    pub const MessageDeleted: Self = Self(2i32);
    pub const ChangeTrackingLost: Self = Self(3i32);
}
#[repr(transparent)]
pub struct ChatMessageChangedDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ChatMessageChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ChatMessageKind(pub i32);
impl ChatMessageKind {
    pub const Standard: Self = Self(0i32);
    pub const FileTransferRequest: Self = Self(1i32);
    pub const TransportCustom: Self = Self(2i32);
    pub const JoinedConversation: Self = Self(3i32);
    pub const LeftConversation: Self = Self(4i32);
    pub const OtherParticipantJoinedConversation: Self = Self(5i32);
    pub const OtherParticipantLeftConversation: Self = Self(6i32);
}
#[repr(transparent)]
pub struct ChatMessageNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ChatMessageOperatorKind(pub i32);
impl ChatMessageOperatorKind {
    pub const Unspecified: Self = Self(0i32);
    pub const Sms: Self = Self(1i32);
    pub const Mms: Self = Self(2i32);
    pub const Rcs: Self = Self(3i32);
}
#[repr(transparent)]
pub struct ChatMessageReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ChatMessageStatus(pub i32);
impl ChatMessageStatus {
    pub const Draft: Self = Self(0i32);
    pub const Sending: Self = Self(1i32);
    pub const Sent: Self = Self(2i32);
    pub const SendRetryNeeded: Self = Self(3i32);
    pub const SendFailed: Self = Self(4i32);
    pub const Received: Self = Self(5i32);
    pub const ReceiveDownloadNeeded: Self = Self(6i32);
    pub const ReceiveDownloadFailed: Self = Self(7i32);
    pub const ReceiveDownloading: Self = Self(8i32);
    pub const Deleted: Self = Self(9i32);
    pub const Declined: Self = Self(10i32);
    pub const Cancelled: Self = Self(11i32);
    pub const Recalled: Self = Self(12i32);
    pub const ReceiveRetryNeeded: Self = Self(13i32);
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
    pub const Text: Self = Self(0i32);
    pub const Untriaged: Self = Self(1i32);
    pub const Blocked: Self = Self(2i32);
    pub const Custom: Self = Self(3i32);
}
#[repr(transparent)]
pub struct ChatMessageValidationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ChatMessageValidationStatus(pub i32);
impl ChatMessageValidationStatus {
    pub const Valid: Self = Self(0i32);
    pub const NoRecipients: Self = Self(1i32);
    pub const InvalidData: Self = Self(2i32);
    pub const MessageTooLarge: Self = Self(3i32);
    pub const TooManyRecipients: Self = Self(4i32);
    pub const TransportInactive: Self = Self(5i32);
    pub const TransportNotFound: Self = Self(6i32);
    pub const TooManyAttachments: Self = Self(7i32);
    pub const InvalidRecipients: Self = Self(8i32);
    pub const InvalidBody: Self = Self(9i32);
    pub const InvalidOther: Self = Self(10i32);
    pub const ValidWithLargeMessage: Self = Self(11i32);
    pub const VoiceRoamingRestriction: Self = Self(12i32);
    pub const DataRoamingRestriction: Self = Self(13i32);
}
#[repr(transparent)]
pub struct ChatQueryOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ChatRecipientDeliveryInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ChatRestoreHistorySpan(pub i32);
impl ChatRestoreHistorySpan {
    pub const LastMonth: Self = Self(0i32);
    pub const LastYear: Self = Self(1i32);
    pub const AnyTime: Self = Self(2i32);
}
#[repr(transparent)]
pub struct ChatSearchReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ChatStoreChangedEventKind(pub i32);
impl ChatStoreChangedEventKind {
    pub const NotificationsMissed: Self = Self(0i32);
    pub const StoreModified: Self = Self(1i32);
    pub const MessageCreated: Self = Self(2i32);
    pub const MessageModified: Self = Self(3i32);
    pub const MessageDeleted: Self = Self(4i32);
    pub const ConversationModified: Self = Self(5i32);
    pub const ConversationDeleted: Self = Self(6i32);
    pub const ConversationTransportDeleted: Self = Self(7i32);
}
#[repr(transparent)]
pub struct ChatSyncConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ChatSyncManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ChatTransportErrorCodeCategory(pub i32);
impl ChatTransportErrorCodeCategory {
    pub const None: Self = Self(0i32);
    pub const Http: Self = Self(1i32);
    pub const Network: Self = Self(2i32);
    pub const MmsServer: Self = Self(3i32);
}
#[repr(transparent)]
pub struct ChatTransportInterpretedErrorCode(pub i32);
impl ChatTransportInterpretedErrorCode {
    pub const None: Self = Self(0i32);
    pub const Unknown: Self = Self(1i32);
    pub const InvalidRecipientAddress: Self = Self(2i32);
    pub const NetworkConnectivity: Self = Self(3i32);
    pub const ServiceDenied: Self = Self(4i32);
    pub const Timeout: Self = Self(5i32);
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
    pub const Chat: Self = Self(0i32);
    pub const GroupChat: Self = Self(1i32);
    pub const FileTransfer: Self = Self(2i32);
    pub const Capability: Self = Self(3i32);
}
#[repr(transparent)]
pub struct RcsServiceKindSupportedChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RcsTransport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RcsTransportConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RemoteParticipantComposingChangedEventArgs(pub *mut ::core::ffi::c_void);
