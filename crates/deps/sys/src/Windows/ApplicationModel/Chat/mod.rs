#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ChatCapabilities(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ChatCapabilities {}
impl ::core::clone::Clone for ChatCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ChatConversation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ChatConversation {}
impl ::core::clone::Clone for ChatConversation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ChatConversationReader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ChatConversationReader {}
impl ::core::clone::Clone for ChatConversationReader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ChatConversationThreadingInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ChatConversationThreadingInfo {}
impl ::core::clone::Clone for ChatConversationThreadingInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ChatConversationThreadingKind(pub i32);
impl ChatConversationThreadingKind {
    pub const Participants: Self = Self(0i32);
    pub const ContactId: Self = Self(1i32);
    pub const ConversationId: Self = Self(2i32);
    pub const Custom: Self = Self(3i32);
}
impl ::core::marker::Copy for ChatConversationThreadingKind {}
impl ::core::clone::Clone for ChatConversationThreadingKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ChatItemKind(pub i32);
impl ChatItemKind {
    pub const Message: Self = Self(0i32);
    pub const Conversation: Self = Self(1i32);
}
impl ::core::marker::Copy for ChatItemKind {}
impl ::core::clone::Clone for ChatItemKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ChatMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ChatMessage {}
impl ::core::clone::Clone for ChatMessage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ChatMessageAttachment(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ChatMessageAttachment {}
impl ::core::clone::Clone for ChatMessageAttachment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ChatMessageChange(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ChatMessageChange {}
impl ::core::clone::Clone for ChatMessageChange {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ChatMessageChangeReader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ChatMessageChangeReader {}
impl ::core::clone::Clone for ChatMessageChangeReader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ChatMessageChangeTracker(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ChatMessageChangeTracker {}
impl ::core::clone::Clone for ChatMessageChangeTracker {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ChatMessageChangeType(pub i32);
impl ChatMessageChangeType {
    pub const MessageCreated: Self = Self(0i32);
    pub const MessageModified: Self = Self(1i32);
    pub const MessageDeleted: Self = Self(2i32);
    pub const ChangeTrackingLost: Self = Self(3i32);
}
impl ::core::marker::Copy for ChatMessageChangeType {}
impl ::core::clone::Clone for ChatMessageChangeType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ChatMessageChangedDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ChatMessageChangedDeferral {}
impl ::core::clone::Clone for ChatMessageChangedDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ChatMessageChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ChatMessageChangedEventArgs {}
impl ::core::clone::Clone for ChatMessageChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for ChatMessageKind {}
impl ::core::clone::Clone for ChatMessageKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ChatMessageNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ChatMessageNotificationTriggerDetails {}
impl ::core::clone::Clone for ChatMessageNotificationTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ChatMessageOperatorKind(pub i32);
impl ChatMessageOperatorKind {
    pub const Unspecified: Self = Self(0i32);
    pub const Sms: Self = Self(1i32);
    pub const Mms: Self = Self(2i32);
    pub const Rcs: Self = Self(3i32);
}
impl ::core::marker::Copy for ChatMessageOperatorKind {}
impl ::core::clone::Clone for ChatMessageOperatorKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ChatMessageReader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ChatMessageReader {}
impl ::core::clone::Clone for ChatMessageReader {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for ChatMessageStatus {}
impl ::core::clone::Clone for ChatMessageStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ChatMessageStore(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ChatMessageStore {}
impl ::core::clone::Clone for ChatMessageStore {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ChatMessageStoreChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ChatMessageStoreChangedEventArgs {}
impl ::core::clone::Clone for ChatMessageStoreChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ChatMessageTransport(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ChatMessageTransport {}
impl ::core::clone::Clone for ChatMessageTransport {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ChatMessageTransportConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ChatMessageTransportConfiguration {}
impl ::core::clone::Clone for ChatMessageTransportConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ChatMessageTransportKind(pub i32);
impl ChatMessageTransportKind {
    pub const Text: Self = Self(0i32);
    pub const Untriaged: Self = Self(1i32);
    pub const Blocked: Self = Self(2i32);
    pub const Custom: Self = Self(3i32);
}
impl ::core::marker::Copy for ChatMessageTransportKind {}
impl ::core::clone::Clone for ChatMessageTransportKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ChatMessageValidationResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ChatMessageValidationResult {}
impl ::core::clone::Clone for ChatMessageValidationResult {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for ChatMessageValidationStatus {}
impl ::core::clone::Clone for ChatMessageValidationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ChatQueryOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ChatQueryOptions {}
impl ::core::clone::Clone for ChatQueryOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ChatRecipientDeliveryInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ChatRecipientDeliveryInfo {}
impl ::core::clone::Clone for ChatRecipientDeliveryInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ChatRestoreHistorySpan(pub i32);
impl ChatRestoreHistorySpan {
    pub const LastMonth: Self = Self(0i32);
    pub const LastYear: Self = Self(1i32);
    pub const AnyTime: Self = Self(2i32);
}
impl ::core::marker::Copy for ChatRestoreHistorySpan {}
impl ::core::clone::Clone for ChatRestoreHistorySpan {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ChatSearchReader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ChatSearchReader {}
impl ::core::clone::Clone for ChatSearchReader {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for ChatStoreChangedEventKind {}
impl ::core::clone::Clone for ChatStoreChangedEventKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ChatSyncConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ChatSyncConfiguration {}
impl ::core::clone::Clone for ChatSyncConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ChatSyncManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ChatSyncManager {}
impl ::core::clone::Clone for ChatSyncManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ChatTransportErrorCodeCategory(pub i32);
impl ChatTransportErrorCodeCategory {
    pub const None: Self = Self(0i32);
    pub const Http: Self = Self(1i32);
    pub const Network: Self = Self(2i32);
    pub const MmsServer: Self = Self(3i32);
}
impl ::core::marker::Copy for ChatTransportErrorCodeCategory {}
impl ::core::clone::Clone for ChatTransportErrorCodeCategory {
    fn clone(&self) -> Self {
        *self
    }
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
impl ::core::marker::Copy for ChatTransportInterpretedErrorCode {}
impl ::core::clone::Clone for ChatTransportInterpretedErrorCode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IChatCapabilities(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IChatCapabilities {}
impl ::core::clone::Clone for IChatCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IChatCapabilitiesManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IChatCapabilitiesManagerStatics {}
impl ::core::clone::Clone for IChatCapabilitiesManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IChatCapabilitiesManagerStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IChatCapabilitiesManagerStatics2 {}
impl ::core::clone::Clone for IChatCapabilitiesManagerStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IChatConversation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IChatConversation {}
impl ::core::clone::Clone for IChatConversation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IChatConversation2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IChatConversation2 {}
impl ::core::clone::Clone for IChatConversation2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IChatConversationReader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IChatConversationReader {}
impl ::core::clone::Clone for IChatConversationReader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IChatConversationThreadingInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IChatConversationThreadingInfo {}
impl ::core::clone::Clone for IChatConversationThreadingInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IChatItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IChatItem {}
impl ::core::clone::Clone for IChatItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IChatMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IChatMessage {}
impl ::core::clone::Clone for IChatMessage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IChatMessage2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IChatMessage2 {}
impl ::core::clone::Clone for IChatMessage2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IChatMessage3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IChatMessage3 {}
impl ::core::clone::Clone for IChatMessage3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IChatMessage4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IChatMessage4 {}
impl ::core::clone::Clone for IChatMessage4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IChatMessageAttachment(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IChatMessageAttachment {}
impl ::core::clone::Clone for IChatMessageAttachment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IChatMessageAttachment2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IChatMessageAttachment2 {}
impl ::core::clone::Clone for IChatMessageAttachment2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IChatMessageAttachmentFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IChatMessageAttachmentFactory {}
impl ::core::clone::Clone for IChatMessageAttachmentFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IChatMessageBlockingStatic(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IChatMessageBlockingStatic {}
impl ::core::clone::Clone for IChatMessageBlockingStatic {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IChatMessageChange(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IChatMessageChange {}
impl ::core::clone::Clone for IChatMessageChange {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IChatMessageChangeReader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IChatMessageChangeReader {}
impl ::core::clone::Clone for IChatMessageChangeReader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IChatMessageChangeTracker(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IChatMessageChangeTracker {}
impl ::core::clone::Clone for IChatMessageChangeTracker {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IChatMessageChangedDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IChatMessageChangedDeferral {}
impl ::core::clone::Clone for IChatMessageChangedDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IChatMessageChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IChatMessageChangedEventArgs {}
impl ::core::clone::Clone for IChatMessageChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IChatMessageManager2Statics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IChatMessageManager2Statics {}
impl ::core::clone::Clone for IChatMessageManager2Statics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IChatMessageManagerStatic(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IChatMessageManagerStatic {}
impl ::core::clone::Clone for IChatMessageManagerStatic {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IChatMessageManagerStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IChatMessageManagerStatics3 {}
impl ::core::clone::Clone for IChatMessageManagerStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IChatMessageNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IChatMessageNotificationTriggerDetails {}
impl ::core::clone::Clone for IChatMessageNotificationTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IChatMessageNotificationTriggerDetails2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IChatMessageNotificationTriggerDetails2 {}
impl ::core::clone::Clone for IChatMessageNotificationTriggerDetails2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IChatMessageReader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IChatMessageReader {}
impl ::core::clone::Clone for IChatMessageReader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IChatMessageReader2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IChatMessageReader2 {}
impl ::core::clone::Clone for IChatMessageReader2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IChatMessageStore(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IChatMessageStore {}
impl ::core::clone::Clone for IChatMessageStore {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IChatMessageStore2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IChatMessageStore2 {}
impl ::core::clone::Clone for IChatMessageStore2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IChatMessageStore3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IChatMessageStore3 {}
impl ::core::clone::Clone for IChatMessageStore3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IChatMessageStoreChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IChatMessageStoreChangedEventArgs {}
impl ::core::clone::Clone for IChatMessageStoreChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IChatMessageTransport(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IChatMessageTransport {}
impl ::core::clone::Clone for IChatMessageTransport {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IChatMessageTransport2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IChatMessageTransport2 {}
impl ::core::clone::Clone for IChatMessageTransport2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IChatMessageTransportConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IChatMessageTransportConfiguration {}
impl ::core::clone::Clone for IChatMessageTransportConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IChatMessageValidationResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IChatMessageValidationResult {}
impl ::core::clone::Clone for IChatMessageValidationResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IChatQueryOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IChatQueryOptions {}
impl ::core::clone::Clone for IChatQueryOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IChatRecipientDeliveryInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IChatRecipientDeliveryInfo {}
impl ::core::clone::Clone for IChatRecipientDeliveryInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IChatSearchReader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IChatSearchReader {}
impl ::core::clone::Clone for IChatSearchReader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IChatSyncConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IChatSyncConfiguration {}
impl ::core::clone::Clone for IChatSyncConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IChatSyncManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IChatSyncManager {}
impl ::core::clone::Clone for IChatSyncManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRcsEndUserMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRcsEndUserMessage {}
impl ::core::clone::Clone for IRcsEndUserMessage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRcsEndUserMessageAction(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRcsEndUserMessageAction {}
impl ::core::clone::Clone for IRcsEndUserMessageAction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRcsEndUserMessageAvailableEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRcsEndUserMessageAvailableEventArgs {}
impl ::core::clone::Clone for IRcsEndUserMessageAvailableEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRcsEndUserMessageAvailableTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRcsEndUserMessageAvailableTriggerDetails {}
impl ::core::clone::Clone for IRcsEndUserMessageAvailableTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRcsEndUserMessageManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRcsEndUserMessageManager {}
impl ::core::clone::Clone for IRcsEndUserMessageManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRcsManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRcsManagerStatics {}
impl ::core::clone::Clone for IRcsManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRcsManagerStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRcsManagerStatics2 {}
impl ::core::clone::Clone for IRcsManagerStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRcsServiceKindSupportedChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRcsServiceKindSupportedChangedEventArgs {}
impl ::core::clone::Clone for IRcsServiceKindSupportedChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRcsTransport(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRcsTransport {}
impl ::core::clone::Clone for IRcsTransport {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRcsTransportConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRcsTransportConfiguration {}
impl ::core::clone::Clone for IRcsTransportConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRemoteParticipantComposingChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRemoteParticipantComposingChangedEventArgs {}
impl ::core::clone::Clone for IRemoteParticipantComposingChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RcsEndUserMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RcsEndUserMessage {}
impl ::core::clone::Clone for RcsEndUserMessage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RcsEndUserMessageAction(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RcsEndUserMessageAction {}
impl ::core::clone::Clone for RcsEndUserMessageAction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RcsEndUserMessageAvailableEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RcsEndUserMessageAvailableEventArgs {}
impl ::core::clone::Clone for RcsEndUserMessageAvailableEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RcsEndUserMessageAvailableTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RcsEndUserMessageAvailableTriggerDetails {}
impl ::core::clone::Clone for RcsEndUserMessageAvailableTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RcsEndUserMessageManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RcsEndUserMessageManager {}
impl ::core::clone::Clone for RcsEndUserMessageManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RcsServiceKind(pub i32);
impl RcsServiceKind {
    pub const Chat: Self = Self(0i32);
    pub const GroupChat: Self = Self(1i32);
    pub const FileTransfer: Self = Self(2i32);
    pub const Capability: Self = Self(3i32);
}
impl ::core::marker::Copy for RcsServiceKind {}
impl ::core::clone::Clone for RcsServiceKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RcsServiceKindSupportedChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RcsServiceKindSupportedChangedEventArgs {}
impl ::core::clone::Clone for RcsServiceKindSupportedChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RcsTransport(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RcsTransport {}
impl ::core::clone::Clone for RcsTransport {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RcsTransportConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RcsTransportConfiguration {}
impl ::core::clone::Clone for RcsTransportConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RemoteParticipantComposingChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RemoteParticipantComposingChangedEventArgs {}
impl ::core::clone::Clone for RemoteParticipantComposingChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
