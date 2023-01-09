impl ::core::cmp::PartialEq for AcceleratorKeyEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AcceleratorKeyEventArgs {}
impl ::core::fmt::Debug for AcceleratorKeyEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AcceleratorKeyEventArgs").field(&self.0).finish()
    }
}
impl ::core::default::Default for AppViewBackButtonVisibility {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AppViewBackButtonVisibility {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppViewBackButtonVisibility").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for AutomationProviderRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AutomationProviderRequestedEventArgs {}
impl ::core::fmt::Debug for AutomationProviderRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationProviderRequestedEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for BackRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BackRequestedEventArgs {}
impl ::core::fmt::Debug for BackRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BackRequestedEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for CharacterReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CharacterReceivedEventArgs {}
impl ::core::fmt::Debug for CharacterReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CharacterReceivedEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ClosestInteractiveBoundsRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ClosestInteractiveBoundsRequestedEventArgs {}
impl ::core::fmt::Debug for ClosestInteractiveBoundsRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClosestInteractiveBoundsRequestedEventArgs").field(&self.0).finish()
    }
}
impl ::core::default::Default for CoreAcceleratorKeyEventType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CoreAcceleratorKeyEventType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreAcceleratorKeyEventType").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for CoreAcceleratorKeys {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreAcceleratorKeys {}
impl ::core::fmt::Debug for CoreAcceleratorKeys {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreAcceleratorKeys").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for CoreComponentInputSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreComponentInputSource {}
impl ::core::fmt::Debug for CoreComponentInputSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreComponentInputSource").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for CoreCursor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreCursor {}
impl ::core::fmt::Debug for CoreCursor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreCursor").field(&self.0).finish()
    }
}
impl ::core::default::Default for CoreCursorType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CoreCursorType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreCursorType").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for CoreDispatcher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreDispatcher {}
impl ::core::fmt::Debug for CoreDispatcher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreDispatcher").field(&self.0).finish()
    }
}
impl ::core::default::Default for CoreDispatcherPriority {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CoreDispatcherPriority {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreDispatcherPriority").field(&self.0).finish()
    }
}
impl ::core::default::Default for CoreIndependentInputFilters {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CoreIndependentInputFilters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreIndependentInputFilters").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CoreIndependentInputFilters {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CoreIndependentInputFilters {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CoreIndependentInputFilters {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CoreIndependentInputFilters {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CoreIndependentInputFilters {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::cmp::PartialEq for CoreIndependentInputSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreIndependentInputSource {}
impl ::core::fmt::Debug for CoreIndependentInputSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreIndependentInputSource").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for CoreIndependentInputSourceController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreIndependentInputSourceController {}
impl ::core::fmt::Debug for CoreIndependentInputSourceController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreIndependentInputSourceController").field(&self.0).finish()
    }
}
impl ::core::default::Default for CoreInputDeviceTypes {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CoreInputDeviceTypes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreInputDeviceTypes").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CoreInputDeviceTypes {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CoreInputDeviceTypes {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CoreInputDeviceTypes {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CoreInputDeviceTypes {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CoreInputDeviceTypes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CorePhysicalKeyStatus {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CorePhysicalKeyStatus {
    fn eq(&self, other: &Self) -> bool {
        self.RepeatCount == other.RepeatCount && self.ScanCode == other.ScanCode && self.IsExtendedKey == other.IsExtendedKey && self.IsMenuKeyDown == other.IsMenuKeyDown && self.WasKeyDown == other.WasKeyDown && self.IsKeyReleased == other.IsKeyReleased
    }
}
impl ::core::cmp::Eq for CorePhysicalKeyStatus {}
impl ::core::fmt::Debug for CorePhysicalKeyStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CorePhysicalKeyStatus").field("RepeatCount", &self.RepeatCount).field("ScanCode", &self.ScanCode).field("IsExtendedKey", &self.IsExtendedKey).field("IsMenuKeyDown", &self.IsMenuKeyDown).field("WasKeyDown", &self.WasKeyDown).field("IsKeyReleased", &self.IsKeyReleased).finish()
    }
}
impl ::core::default::Default for CoreProcessEventsOption {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CoreProcessEventsOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreProcessEventsOption").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation")]
impl ::core::default::Default for CoreProximityEvaluation {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::PartialEq for CoreProximityEvaluation {
    fn eq(&self, other: &Self) -> bool {
        self.Score == other.Score && self.AdjustedPoint == other.AdjustedPoint
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::Eq for CoreProximityEvaluation {}
#[cfg(feature = "Foundation")]
impl ::core::fmt::Debug for CoreProximityEvaluation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CoreProximityEvaluation").field("Score", &self.Score).field("AdjustedPoint", &self.AdjustedPoint).finish()
    }
}
impl ::core::default::Default for CoreProximityEvaluationScore {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CoreProximityEvaluationScore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreProximityEvaluationScore").field(&self.0).finish()
    }
}
impl ::core::default::Default for CoreVirtualKeyStates {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CoreVirtualKeyStates {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreVirtualKeyStates").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CoreVirtualKeyStates {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CoreVirtualKeyStates {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CoreVirtualKeyStates {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CoreVirtualKeyStates {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CoreVirtualKeyStates {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::cmp::PartialEq for CoreWindow {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWindow {}
impl ::core::fmt::Debug for CoreWindow {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWindow").field(&self.0).finish()
    }
}
impl ::core::default::Default for CoreWindowActivationMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CoreWindowActivationMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWindowActivationMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for CoreWindowActivationState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CoreWindowActivationState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWindowActivationState").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for CoreWindowDialog {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWindowDialog {}
impl ::core::fmt::Debug for CoreWindowDialog {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWindowDialog").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for CoreWindowEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWindowEventArgs {}
impl ::core::fmt::Debug for CoreWindowEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWindowEventArgs").field(&self.0).finish()
    }
}
impl ::core::default::Default for CoreWindowFlowDirection {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CoreWindowFlowDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWindowFlowDirection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for CoreWindowFlyout {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWindowFlyout {}
impl ::core::fmt::Debug for CoreWindowFlyout {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWindowFlyout").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for CoreWindowPopupShowingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWindowPopupShowingEventArgs {}
impl ::core::fmt::Debug for CoreWindowPopupShowingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWindowPopupShowingEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for CoreWindowResizeManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWindowResizeManager {}
impl ::core::fmt::Debug for CoreWindowResizeManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWindowResizeManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for DispatchedHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DispatchedHandler {}
impl ::core::fmt::Debug for DispatchedHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DispatchedHandler").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ICoreAcceleratorKeys {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICoreAcceleratorKeys {}
impl ::core::fmt::Debug for ICoreAcceleratorKeys {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICoreAcceleratorKeys").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ICoreInputSourceBase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICoreInputSourceBase {}
impl ::core::fmt::Debug for ICoreInputSourceBase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICoreInputSourceBase").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ICorePointerInputSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICorePointerInputSource {}
impl ::core::fmt::Debug for ICorePointerInputSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICorePointerInputSource").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ICorePointerInputSource2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICorePointerInputSource2 {}
impl ::core::fmt::Debug for ICorePointerInputSource2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICorePointerInputSource2").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ICorePointerRedirector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICorePointerRedirector {}
impl ::core::fmt::Debug for ICorePointerRedirector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICorePointerRedirector").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ICoreWindow {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICoreWindow {}
impl ::core::fmt::Debug for ICoreWindow {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICoreWindow").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ICoreWindowEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICoreWindowEventArgs {}
impl ::core::fmt::Debug for ICoreWindowEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICoreWindowEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IInitializeWithCoreWindow {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInitializeWithCoreWindow {}
impl ::core::fmt::Debug for IInitializeWithCoreWindow {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInitializeWithCoreWindow").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IdleDispatchedHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IdleDispatchedHandler {}
impl ::core::fmt::Debug for IdleDispatchedHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IdleDispatchedHandler").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IdleDispatchedHandlerArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IdleDispatchedHandlerArgs {}
impl ::core::fmt::Debug for IdleDispatchedHandlerArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IdleDispatchedHandlerArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for InputEnabledEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InputEnabledEventArgs {}
impl ::core::fmt::Debug for InputEnabledEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InputEnabledEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for KeyEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for KeyEventArgs {}
impl ::core::fmt::Debug for KeyEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for PointerEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PointerEventArgs {}
impl ::core::fmt::Debug for PointerEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PointerEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for SystemNavigationManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SystemNavigationManager {}
impl ::core::fmt::Debug for SystemNavigationManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemNavigationManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for TouchHitTestingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TouchHitTestingEventArgs {}
impl ::core::fmt::Debug for TouchHitTestingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TouchHitTestingEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for VisibilityChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VisibilityChangedEventArgs {}
impl ::core::fmt::Debug for VisibilityChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VisibilityChangedEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for WindowActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WindowActivatedEventArgs {}
impl ::core::fmt::Debug for WindowActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for WindowSizeChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WindowSizeChangedEventArgs {}
impl ::core::fmt::Debug for WindowSizeChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowSizeChangedEventArgs").field(&self.0).finish()
    }
}
