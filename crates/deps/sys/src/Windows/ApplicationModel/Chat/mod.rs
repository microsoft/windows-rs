#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ChatCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ChatCapabilitiesManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ChatConversation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ChatConversationReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ChatConversationThreadingInfo(pub *mut ::core::ffi::c_void);
pub struct ChatConversationThreadingKind(i32);
pub struct ChatItemKind(i32);
#[repr(transparent)]
pub struct ChatMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ChatMessageAttachment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ChatMessageBlocking(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ChatMessageChange(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ChatMessageChangeReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ChatMessageChangeTracker(pub *mut ::core::ffi::c_void);
pub struct ChatMessageChangeType(i32);
#[repr(transparent)]
pub struct ChatMessageChangedDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ChatMessageChangedEventArgs(pub *mut ::core::ffi::c_void);
pub struct ChatMessageKind(i32);
#[repr(transparent)]
pub struct ChatMessageManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ChatMessageNotificationTriggerDetails(pub *mut ::core::ffi::c_void);
pub struct ChatMessageOperatorKind(i32);
#[repr(transparent)]
pub struct ChatMessageReader(pub *mut ::core::ffi::c_void);
pub struct ChatMessageStatus(i32);
#[repr(transparent)]
pub struct ChatMessageStore(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ChatMessageStoreChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ChatMessageTransport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ChatMessageTransportConfiguration(pub *mut ::core::ffi::c_void);
pub struct ChatMessageTransportKind(i32);
#[repr(transparent)]
pub struct ChatMessageValidationResult(pub *mut ::core::ffi::c_void);
pub struct ChatMessageValidationStatus(i32);
#[repr(transparent)]
pub struct ChatQueryOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ChatRecipientDeliveryInfo(pub *mut ::core::ffi::c_void);
pub struct ChatRestoreHistorySpan(i32);
#[repr(transparent)]
pub struct ChatSearchReader(pub *mut ::core::ffi::c_void);
pub struct ChatStoreChangedEventKind(i32);
#[repr(transparent)]
pub struct ChatSyncConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ChatSyncManager(pub *mut ::core::ffi::c_void);
pub struct ChatTransportErrorCodeCategory(i32);
pub struct ChatTransportInterpretedErrorCode(i32);
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
pub struct RcsManager(pub *mut ::core::ffi::c_void);
pub struct RcsServiceKind(i32);
#[repr(transparent)]
pub struct RcsServiceKindSupportedChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RcsTransport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RcsTransportConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RemoteParticipantComposingChangedEventArgs(pub *mut ::core::ffi::c_void);
