impl ::core::cmp::PartialEq for CoreTextCompositionCompletedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreTextCompositionCompletedEventArgs {}
impl ::core::fmt::Debug for CoreTextCompositionCompletedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextCompositionCompletedEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for CoreTextCompositionSegment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreTextCompositionSegment {}
impl ::core::fmt::Debug for CoreTextCompositionSegment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextCompositionSegment").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for CoreTextCompositionStartedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreTextCompositionStartedEventArgs {}
impl ::core::fmt::Debug for CoreTextCompositionStartedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextCompositionStartedEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for CoreTextEditContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreTextEditContext {}
impl ::core::fmt::Debug for CoreTextEditContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextEditContext").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for CoreTextFormatUpdatingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreTextFormatUpdatingEventArgs {}
impl ::core::fmt::Debug for CoreTextFormatUpdatingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextFormatUpdatingEventArgs").field(&self.0).finish()
    }
}
impl ::core::default::Default for CoreTextFormatUpdatingReason {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CoreTextFormatUpdatingReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextFormatUpdatingReason").field(&self.0).finish()
    }
}
impl ::core::default::Default for CoreTextFormatUpdatingResult {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CoreTextFormatUpdatingResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextFormatUpdatingResult").field(&self.0).finish()
    }
}
impl ::core::default::Default for CoreTextInputPaneDisplayPolicy {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CoreTextInputPaneDisplayPolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextInputPaneDisplayPolicy").field(&self.0).finish()
    }
}
impl ::core::default::Default for CoreTextInputScope {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CoreTextInputScope {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextInputScope").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for CoreTextLayoutBounds {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreTextLayoutBounds {}
impl ::core::fmt::Debug for CoreTextLayoutBounds {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextLayoutBounds").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for CoreTextLayoutRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreTextLayoutRequest {}
impl ::core::fmt::Debug for CoreTextLayoutRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextLayoutRequest").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for CoreTextLayoutRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreTextLayoutRequestedEventArgs {}
impl ::core::fmt::Debug for CoreTextLayoutRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextLayoutRequestedEventArgs").field(&self.0).finish()
    }
}
impl ::core::default::Default for CoreTextRange {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CoreTextRange {
    fn eq(&self, other: &Self) -> bool {
        self.StartCaretPosition == other.StartCaretPosition && self.EndCaretPosition == other.EndCaretPosition
    }
}
impl ::core::cmp::Eq for CoreTextRange {}
impl ::core::fmt::Debug for CoreTextRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CoreTextRange").field("StartCaretPosition", &self.StartCaretPosition).field("EndCaretPosition", &self.EndCaretPosition).finish()
    }
}
impl ::core::cmp::PartialEq for CoreTextSelectionRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreTextSelectionRequest {}
impl ::core::fmt::Debug for CoreTextSelectionRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextSelectionRequest").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for CoreTextSelectionRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreTextSelectionRequestedEventArgs {}
impl ::core::fmt::Debug for CoreTextSelectionRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextSelectionRequestedEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for CoreTextSelectionUpdatingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreTextSelectionUpdatingEventArgs {}
impl ::core::fmt::Debug for CoreTextSelectionUpdatingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextSelectionUpdatingEventArgs").field(&self.0).finish()
    }
}
impl ::core::default::Default for CoreTextSelectionUpdatingResult {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CoreTextSelectionUpdatingResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextSelectionUpdatingResult").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for CoreTextServicesManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreTextServicesManager {}
impl ::core::fmt::Debug for CoreTextServicesManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextServicesManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for CoreTextTextRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreTextTextRequest {}
impl ::core::fmt::Debug for CoreTextTextRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextTextRequest").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for CoreTextTextRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreTextTextRequestedEventArgs {}
impl ::core::fmt::Debug for CoreTextTextRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextTextRequestedEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for CoreTextTextUpdatingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreTextTextUpdatingEventArgs {}
impl ::core::fmt::Debug for CoreTextTextUpdatingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextTextUpdatingEventArgs").field(&self.0).finish()
    }
}
impl ::core::default::Default for CoreTextTextUpdatingResult {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CoreTextTextUpdatingResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreTextTextUpdatingResult").field(&self.0).finish()
    }
}
