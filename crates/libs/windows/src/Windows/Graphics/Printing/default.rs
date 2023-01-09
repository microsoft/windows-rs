impl ::core::cmp::PartialEq for IPrintDocumentSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintDocumentSource {}
impl ::core::fmt::Debug for IPrintDocumentSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintDocumentSource").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPrintTaskOptionsCore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintTaskOptionsCore {}
impl ::core::fmt::Debug for IPrintTaskOptionsCore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintTaskOptionsCore").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPrintTaskOptionsCoreProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintTaskOptionsCoreProperties {}
impl ::core::fmt::Debug for IPrintTaskOptionsCoreProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintTaskOptionsCoreProperties").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPrintTaskOptionsCoreUIConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintTaskOptionsCoreUIConfiguration {}
impl ::core::fmt::Debug for IPrintTaskOptionsCoreUIConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintTaskOptionsCoreUIConfiguration").field(&self.0).finish()
    }
}
impl ::core::default::Default for PrintBinding {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PrintBinding {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintBinding").field(&self.0).finish()
    }
}
impl ::core::default::Default for PrintBordering {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PrintBordering {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintBordering").field(&self.0).finish()
    }
}
impl ::core::default::Default for PrintCollation {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PrintCollation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintCollation").field(&self.0).finish()
    }
}
impl ::core::default::Default for PrintColorMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PrintColorMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintColorMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for PrintDuplex {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PrintDuplex {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintDuplex").field(&self.0).finish()
    }
}
impl ::core::default::Default for PrintHolePunch {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PrintHolePunch {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintHolePunch").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for PrintManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintManager {}
impl ::core::fmt::Debug for PrintManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintManager").field(&self.0).finish()
    }
}
impl ::core::default::Default for PrintMediaSize {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PrintMediaSize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintMediaSize").field(&self.0).finish()
    }
}
impl ::core::default::Default for PrintMediaType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PrintMediaType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintMediaType").field(&self.0).finish()
    }
}
impl ::core::default::Default for PrintOrientation {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PrintOrientation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintOrientation").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation")]
impl ::core::default::Default for PrintPageDescription {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::PartialEq for PrintPageDescription {
    fn eq(&self, other: &Self) -> bool {
        self.PageSize == other.PageSize && self.ImageableRect == other.ImageableRect && self.DpiX == other.DpiX && self.DpiY == other.DpiY
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::Eq for PrintPageDescription {}
#[cfg(feature = "Foundation")]
impl ::core::fmt::Debug for PrintPageDescription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PrintPageDescription").field("PageSize", &self.PageSize).field("ImageableRect", &self.ImageableRect).field("DpiX", &self.DpiX).field("DpiY", &self.DpiY).finish()
    }
}
impl ::core::cmp::PartialEq for PrintPageInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintPageInfo {}
impl ::core::fmt::Debug for PrintPageInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintPageInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for PrintPageRange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintPageRange {}
impl ::core::fmt::Debug for PrintPageRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintPageRange").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for PrintPageRangeOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintPageRangeOptions {}
impl ::core::fmt::Debug for PrintPageRangeOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintPageRangeOptions").field(&self.0).finish()
    }
}
impl ::core::default::Default for PrintQuality {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PrintQuality {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintQuality").field(&self.0).finish()
    }
}
impl ::core::default::Default for PrintStaple {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PrintStaple {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintStaple").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for PrintTask {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintTask {}
impl ::core::fmt::Debug for PrintTask {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintTask").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for PrintTaskCompletedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintTaskCompletedEventArgs {}
impl ::core::fmt::Debug for PrintTaskCompletedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintTaskCompletedEventArgs").field(&self.0).finish()
    }
}
impl ::core::default::Default for PrintTaskCompletion {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PrintTaskCompletion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintTaskCompletion").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for PrintTaskOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintTaskOptions {}
impl ::core::fmt::Debug for PrintTaskOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintTaskOptions").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for PrintTaskProgressingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintTaskProgressingEventArgs {}
impl ::core::fmt::Debug for PrintTaskProgressingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintTaskProgressingEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for PrintTaskRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintTaskRequest {}
impl ::core::fmt::Debug for PrintTaskRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintTaskRequest").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for PrintTaskRequestedDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintTaskRequestedDeferral {}
impl ::core::fmt::Debug for PrintTaskRequestedDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintTaskRequestedDeferral").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for PrintTaskRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintTaskRequestedEventArgs {}
impl ::core::fmt::Debug for PrintTaskRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintTaskRequestedEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for PrintTaskSourceRequestedArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintTaskSourceRequestedArgs {}
impl ::core::fmt::Debug for PrintTaskSourceRequestedArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintTaskSourceRequestedArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for PrintTaskSourceRequestedDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintTaskSourceRequestedDeferral {}
impl ::core::fmt::Debug for PrintTaskSourceRequestedDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintTaskSourceRequestedDeferral").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for PrintTaskSourceRequestedHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintTaskSourceRequestedHandler {}
impl ::core::fmt::Debug for PrintTaskSourceRequestedHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintTaskSourceRequestedHandler").field(&self.0).finish()
    }
}
