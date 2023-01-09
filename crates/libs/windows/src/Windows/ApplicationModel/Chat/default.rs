impl ::core::cmp::PartialEq for ChatCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatCapabilities {}
impl ::core::fmt::Debug for ChatCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatCapabilities").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ChatConversation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatConversation {}
impl ::core::fmt::Debug for ChatConversation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatConversation").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ChatConversationReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatConversationReader {}
impl ::core::fmt::Debug for ChatConversationReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatConversationReader").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ChatConversationThreadingInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatConversationThreadingInfo {}
impl ::core::fmt::Debug for ChatConversationThreadingInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatConversationThreadingInfo").field(&self.0).finish()
    }
}
impl ::core::default::Default for ChatConversationThreadingKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ChatConversationThreadingKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatConversationThreadingKind").field(&self.0).finish()
    }
}
impl ::core::default::Default for ChatItemKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ChatItemKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatItemKind").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ChatMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatMessage {}
impl ::core::fmt::Debug for ChatMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessage").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ChatMessageAttachment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatMessageAttachment {}
impl ::core::fmt::Debug for ChatMessageAttachment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageAttachment").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ChatMessageChange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatMessageChange {}
impl ::core::fmt::Debug for ChatMessageChange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageChange").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ChatMessageChangeReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatMessageChangeReader {}
impl ::core::fmt::Debug for ChatMessageChangeReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageChangeReader").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ChatMessageChangeTracker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatMessageChangeTracker {}
impl ::core::fmt::Debug for ChatMessageChangeTracker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageChangeTracker").field(&self.0).finish()
    }
}
impl ::core::default::Default for ChatMessageChangeType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ChatMessageChangeType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageChangeType").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ChatMessageChangedDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatMessageChangedDeferral {}
impl ::core::fmt::Debug for ChatMessageChangedDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageChangedDeferral").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ChatMessageChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatMessageChangedEventArgs {}
impl ::core::fmt::Debug for ChatMessageChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageChangedEventArgs").field(&self.0).finish()
    }
}
impl ::core::default::Default for ChatMessageKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ChatMessageKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageKind").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ChatMessageNotificationTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatMessageNotificationTriggerDetails {}
impl ::core::fmt::Debug for ChatMessageNotificationTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageNotificationTriggerDetails").field(&self.0).finish()
    }
}
impl ::core::default::Default for ChatMessageOperatorKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ChatMessageOperatorKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageOperatorKind").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ChatMessageReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatMessageReader {}
impl ::core::fmt::Debug for ChatMessageReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageReader").field(&self.0).finish()
    }
}
impl ::core::default::Default for ChatMessageStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ChatMessageStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageStatus").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ChatMessageStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatMessageStore {}
impl ::core::fmt::Debug for ChatMessageStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageStore").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ChatMessageStoreChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatMessageStoreChangedEventArgs {}
impl ::core::fmt::Debug for ChatMessageStoreChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageStoreChangedEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ChatMessageTransport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatMessageTransport {}
impl ::core::fmt::Debug for ChatMessageTransport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageTransport").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ChatMessageTransportConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatMessageTransportConfiguration {}
impl ::core::fmt::Debug for ChatMessageTransportConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageTransportConfiguration").field(&self.0).finish()
    }
}
impl ::core::default::Default for ChatMessageTransportKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ChatMessageTransportKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageTransportKind").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ChatMessageValidationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatMessageValidationResult {}
impl ::core::fmt::Debug for ChatMessageValidationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageValidationResult").field(&self.0).finish()
    }
}
impl ::core::default::Default for ChatMessageValidationStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ChatMessageValidationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageValidationStatus").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ChatQueryOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatQueryOptions {}
impl ::core::fmt::Debug for ChatQueryOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatQueryOptions").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ChatRecipientDeliveryInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatRecipientDeliveryInfo {}
impl ::core::fmt::Debug for ChatRecipientDeliveryInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatRecipientDeliveryInfo").field(&self.0).finish()
    }
}
impl ::core::default::Default for ChatRestoreHistorySpan {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ChatRestoreHistorySpan {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatRestoreHistorySpan").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ChatSearchReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatSearchReader {}
impl ::core::fmt::Debug for ChatSearchReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatSearchReader").field(&self.0).finish()
    }
}
impl ::core::default::Default for ChatStoreChangedEventKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ChatStoreChangedEventKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatStoreChangedEventKind").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ChatSyncConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatSyncConfiguration {}
impl ::core::fmt::Debug for ChatSyncConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatSyncConfiguration").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ChatSyncManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatSyncManager {}
impl ::core::fmt::Debug for ChatSyncManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatSyncManager").field(&self.0).finish()
    }
}
impl ::core::default::Default for ChatTransportErrorCodeCategory {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ChatTransportErrorCodeCategory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatTransportErrorCodeCategory").field(&self.0).finish()
    }
}
impl ::core::default::Default for ChatTransportInterpretedErrorCode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ChatTransportInterpretedErrorCode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatTransportInterpretedErrorCode").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IChatItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IChatItem {}
impl ::core::fmt::Debug for IChatItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IChatItem").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for RcsEndUserMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RcsEndUserMessage {}
impl ::core::fmt::Debug for RcsEndUserMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RcsEndUserMessage").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for RcsEndUserMessageAction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RcsEndUserMessageAction {}
impl ::core::fmt::Debug for RcsEndUserMessageAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RcsEndUserMessageAction").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for RcsEndUserMessageAvailableEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RcsEndUserMessageAvailableEventArgs {}
impl ::core::fmt::Debug for RcsEndUserMessageAvailableEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RcsEndUserMessageAvailableEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for RcsEndUserMessageAvailableTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RcsEndUserMessageAvailableTriggerDetails {}
impl ::core::fmt::Debug for RcsEndUserMessageAvailableTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RcsEndUserMessageAvailableTriggerDetails").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for RcsEndUserMessageManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RcsEndUserMessageManager {}
impl ::core::fmt::Debug for RcsEndUserMessageManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RcsEndUserMessageManager").field(&self.0).finish()
    }
}
impl ::core::default::Default for RcsServiceKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RcsServiceKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RcsServiceKind").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for RcsServiceKindSupportedChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RcsServiceKindSupportedChangedEventArgs {}
impl ::core::fmt::Debug for RcsServiceKindSupportedChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RcsServiceKindSupportedChangedEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for RcsTransport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RcsTransport {}
impl ::core::fmt::Debug for RcsTransport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RcsTransport").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for RcsTransportConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RcsTransportConfiguration {}
impl ::core::fmt::Debug for RcsTransportConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RcsTransportConfiguration").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for RemoteParticipantComposingChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteParticipantComposingChangedEventArgs {}
impl ::core::fmt::Debug for RemoteParticipantComposingChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteParticipantComposingChangedEventArgs").field(&self.0).finish()
    }
}
