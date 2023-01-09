impl ::core::cmp::PartialEq for AccessibilitySettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AccessibilitySettings {}
impl ::core::fmt::Debug for AccessibilitySettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AccessibilitySettings").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ActivationViewSwitcher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ActivationViewSwitcher {}
impl ::core::fmt::Debug for ActivationViewSwitcher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ActivationViewSwitcher").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ApplicationView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ApplicationView {}
impl ::core::fmt::Debug for ApplicationView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ApplicationView").field(&self.0).finish()
    }
}
impl ::core::default::Default for ApplicationViewBoundsMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ApplicationViewBoundsMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ApplicationViewBoundsMode").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ApplicationViewConsolidatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ApplicationViewConsolidatedEventArgs {}
impl ::core::fmt::Debug for ApplicationViewConsolidatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ApplicationViewConsolidatedEventArgs").field(&self.0).finish()
    }
}
impl ::core::default::Default for ApplicationViewMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ApplicationViewMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ApplicationViewMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for ApplicationViewOrientation {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ApplicationViewOrientation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ApplicationViewOrientation").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ApplicationViewScaling {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ApplicationViewScaling {}
impl ::core::fmt::Debug for ApplicationViewScaling {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ApplicationViewScaling").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for ApplicationViewState {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for ApplicationViewState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ApplicationViewState").field(&self.0).finish()
    }
}
impl ::core::default::Default for ApplicationViewSwitchingOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ApplicationViewSwitchingOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ApplicationViewSwitchingOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for ApplicationViewSwitchingOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for ApplicationViewSwitchingOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for ApplicationViewSwitchingOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for ApplicationViewSwitchingOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for ApplicationViewSwitchingOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::cmp::PartialEq for ApplicationViewTitleBar {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ApplicationViewTitleBar {}
impl ::core::fmt::Debug for ApplicationViewTitleBar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ApplicationViewTitleBar").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ApplicationViewTransferContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ApplicationViewTransferContext {}
impl ::core::fmt::Debug for ApplicationViewTransferContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ApplicationViewTransferContext").field(&self.0).finish()
    }
}
impl ::core::default::Default for ApplicationViewWindowingMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ApplicationViewWindowingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ApplicationViewWindowingMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for FullScreenSystemOverlayMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FullScreenSystemOverlayMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FullScreenSystemOverlayMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for HandPreference {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HandPreference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HandPreference").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for InputPane {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InputPane {}
impl ::core::fmt::Debug for InputPane {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InputPane").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for InputPaneVisibilityEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InputPaneVisibilityEventArgs {}
impl ::core::fmt::Debug for InputPaneVisibilityEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InputPaneVisibilityEventArgs").field(&self.0).finish()
    }
}
impl ::core::default::Default for ScreenCaptureDisabledBehavior {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ScreenCaptureDisabledBehavior {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ScreenCaptureDisabledBehavior").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for StatusBar {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StatusBar {}
impl ::core::fmt::Debug for StatusBar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StatusBar").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for StatusBarProgressIndicator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StatusBarProgressIndicator {}
impl ::core::fmt::Debug for StatusBarProgressIndicator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StatusBarProgressIndicator").field(&self.0).finish()
    }
}
impl ::core::default::Default for UIColorType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UIColorType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UIColorType").field(&self.0).finish()
    }
}
impl ::core::default::Default for UIElementType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UIElementType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UIElementType").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for UISettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UISettings {}
impl ::core::fmt::Debug for UISettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UISettings").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for UISettingsAnimationsEnabledChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UISettingsAnimationsEnabledChangedEventArgs {}
impl ::core::fmt::Debug for UISettingsAnimationsEnabledChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UISettingsAnimationsEnabledChangedEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for UISettingsAutoHideScrollBarsChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UISettingsAutoHideScrollBarsChangedEventArgs {}
impl ::core::fmt::Debug for UISettingsAutoHideScrollBarsChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UISettingsAutoHideScrollBarsChangedEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for UISettingsMessageDurationChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UISettingsMessageDurationChangedEventArgs {}
impl ::core::fmt::Debug for UISettingsMessageDurationChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UISettingsMessageDurationChangedEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for UIViewSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UIViewSettings {}
impl ::core::fmt::Debug for UIViewSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UIViewSettings").field(&self.0).finish()
    }
}
impl ::core::default::Default for UserInteractionMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UserInteractionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserInteractionMode").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ViewModePreferences {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ViewModePreferences {}
impl ::core::fmt::Debug for ViewModePreferences {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ViewModePreferences").field(&self.0).finish()
    }
}
impl ::core::default::Default for ViewSizePreference {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ViewSizePreference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ViewSizePreference").field(&self.0).finish()
    }
}
