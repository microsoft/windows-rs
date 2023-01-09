impl ::core::default::Default for ACCESSTIMEOUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ACCESSTIMEOUT {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwFlags == other.dwFlags && self.iTimeOutMSec == other.iTimeOutMSec
    }
}
impl ::core::cmp::Eq for ACCESSTIMEOUT {}
impl ::core::fmt::Debug for ACCESSTIMEOUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACCESSTIMEOUT").field("cbSize", &self.cbSize).field("dwFlags", &self.dwFlags).field("iTimeOutMSec", &self.iTimeOutMSec).finish()
    }
}
impl ::core::default::Default for ACC_UTILITY_STATE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ACC_UTILITY_STATE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ACC_UTILITY_STATE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for ACC_UTILITY_STATE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for ACC_UTILITY_STATE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for ACC_UTILITY_STATE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for ACC_UTILITY_STATE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for ACC_UTILITY_STATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for ActiveEnd {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ActiveEnd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ActiveEnd").field(&self.0).finish()
    }
}
impl ::core::default::Default for AnimationStyle {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AnimationStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AnimationStyle").field(&self.0).finish()
    }
}
impl ::core::default::Default for AnnoScope {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AnnoScope {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AnnoScope").field(&self.0).finish()
    }
}
impl ::core::default::Default for AsyncContentLoadedState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AsyncContentLoadedState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncContentLoadedState").field(&self.0).finish()
    }
}
impl ::core::default::Default for AutomationElementMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AutomationElementMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationElementMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for AutomationIdentifierType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AutomationIdentifierType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationIdentifierType").field(&self.0).finish()
    }
}
impl ::core::default::Default for BulletStyle {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BulletStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BulletStyle").field(&self.0).finish()
    }
}
impl ::core::default::Default for CapStyle {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CapStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CapStyle").field(&self.0).finish()
    }
}
impl ::core::default::Default for CaretBidiMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CaretBidiMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CaretBidiMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for CaretPosition {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CaretPosition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CaretPosition").field(&self.0).finish()
    }
}
impl ::core::default::Default for CoalesceEventsOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CoalesceEventsOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoalesceEventsOptions").field(&self.0).finish()
    }
}
impl ::core::default::Default for ConditionType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ConditionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConditionType").field(&self.0).finish()
    }
}
impl ::core::default::Default for ConnectionRecoveryBehaviorOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ConnectionRecoveryBehaviorOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConnectionRecoveryBehaviorOptions").field(&self.0).finish()
    }
}
impl ::core::default::Default for DockPosition {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DockPosition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DockPosition").field(&self.0).finish()
    }
}
impl ::core::default::Default for EventArgsType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EventArgsType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EventArgsType").field(&self.0).finish()
    }
}
impl ::core::default::Default for ExpandCollapseState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ExpandCollapseState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExpandCollapseState").field(&self.0).finish()
    }
}
impl ::core::default::Default for ExtendedProperty {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ExtendedProperty {
    fn eq(&self, other: &Self) -> bool {
        self.PropertyName == other.PropertyName && self.PropertyValue == other.PropertyValue
    }
}
impl ::core::cmp::Eq for ExtendedProperty {}
impl ::core::fmt::Debug for ExtendedProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ExtendedProperty").field("PropertyName", &self.PropertyName).field("PropertyValue", &self.PropertyValue).finish()
    }
}
impl ::core::default::Default for FILTERKEYS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILTERKEYS {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwFlags == other.dwFlags && self.iWaitMSec == other.iWaitMSec && self.iDelayMSec == other.iDelayMSec && self.iRepeatMSec == other.iRepeatMSec && self.iBounceMSec == other.iBounceMSec
    }
}
impl ::core::cmp::Eq for FILTERKEYS {}
impl ::core::fmt::Debug for FILTERKEYS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILTERKEYS").field("cbSize", &self.cbSize).field("dwFlags", &self.dwFlags).field("iWaitMSec", &self.iWaitMSec).field("iDelayMSec", &self.iDelayMSec).field("iRepeatMSec", &self.iRepeatMSec).field("iBounceMSec", &self.iBounceMSec).finish()
    }
}
impl ::core::default::Default for FillType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FillType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FillType").field(&self.0).finish()
    }
}
impl ::core::default::Default for FlowDirections {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FlowDirections {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FlowDirections").field(&self.0).finish()
    }
}
impl ::core::default::Default for HIGHCONTRASTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HIGHCONTRASTA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwFlags == other.dwFlags && self.lpszDefaultScheme == other.lpszDefaultScheme
    }
}
impl ::core::cmp::Eq for HIGHCONTRASTA {}
impl ::core::fmt::Debug for HIGHCONTRASTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HIGHCONTRASTA").field("cbSize", &self.cbSize).field("dwFlags", &self.dwFlags).field("lpszDefaultScheme", &self.lpszDefaultScheme).finish()
    }
}
impl ::core::default::Default for HIGHCONTRASTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HIGHCONTRASTW {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwFlags == other.dwFlags && self.lpszDefaultScheme == other.lpszDefaultScheme
    }
}
impl ::core::cmp::Eq for HIGHCONTRASTW {}
impl ::core::fmt::Debug for HIGHCONTRASTW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HIGHCONTRASTW").field("cbSize", &self.cbSize).field("dwFlags", &self.dwFlags).field("lpszDefaultScheme", &self.lpszDefaultScheme).finish()
    }
}
impl ::core::default::Default for HIGHCONTRASTW_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HIGHCONTRASTW_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HIGHCONTRASTW_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for HIGHCONTRASTW_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for HIGHCONTRASTW_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for HIGHCONTRASTW_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for HIGHCONTRASTW_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for HIGHCONTRASTW_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for HorizontalTextAlignment {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HorizontalTextAlignment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HorizontalTextAlignment").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAccIdentity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAccIdentity {}
impl ::core::fmt::Debug for IAccIdentity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAccIdentity").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAccPropServer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAccPropServer {}
impl ::core::fmt::Debug for IAccPropServer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAccPropServer").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAccPropServices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAccPropServices {}
impl ::core::fmt::Debug for IAccPropServices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAccPropServices").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAccessible {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAccessible {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAccessible {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAccessible").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAccessibleEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAccessibleEx {}
impl ::core::fmt::Debug for IAccessibleEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAccessibleEx").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAccessibleHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAccessibleHandler {}
impl ::core::fmt::Debug for IAccessibleHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAccessibleHandler").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAccessibleHostingElementProviders {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAccessibleHostingElementProviders {}
impl ::core::fmt::Debug for IAccessibleHostingElementProviders {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAccessibleHostingElementProviders").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAccessibleWindowlessSite {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAccessibleWindowlessSite {}
impl ::core::fmt::Debug for IAccessibleWindowlessSite {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAccessibleWindowlessSite").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAnnotationProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAnnotationProvider {}
impl ::core::fmt::Debug for IAnnotationProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAnnotationProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ICustomNavigationProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICustomNavigationProvider {}
impl ::core::fmt::Debug for ICustomNavigationProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICustomNavigationProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDockProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDockProvider {}
impl ::core::fmt::Debug for IDockProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDockProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDragProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDragProvider {}
impl ::core::fmt::Debug for IDragProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDragProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDropTargetProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDropTargetProvider {}
impl ::core::fmt::Debug for IDropTargetProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDropTargetProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IExpandCollapseProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IExpandCollapseProvider {}
impl ::core::fmt::Debug for IExpandCollapseProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IExpandCollapseProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IGridItemProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGridItemProvider {}
impl ::core::fmt::Debug for IGridItemProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGridItemProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IGridProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGridProvider {}
impl ::core::fmt::Debug for IGridProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGridProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IInvokeProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInvokeProvider {}
impl ::core::fmt::Debug for IInvokeProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInvokeProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IItemContainerProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IItemContainerProvider {}
impl ::core::fmt::Debug for IItemContainerProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IItemContainerProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ILegacyIAccessibleProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILegacyIAccessibleProvider {}
impl ::core::fmt::Debug for ILegacyIAccessibleProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILegacyIAccessibleProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMultipleViewProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMultipleViewProvider {}
impl ::core::fmt::Debug for IMultipleViewProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMultipleViewProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IObjectModelProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IObjectModelProvider {}
impl ::core::fmt::Debug for IObjectModelProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IObjectModelProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IProxyProviderWinEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProxyProviderWinEventHandler {}
impl ::core::fmt::Debug for IProxyProviderWinEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProxyProviderWinEventHandler").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IProxyProviderWinEventSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProxyProviderWinEventSink {}
impl ::core::fmt::Debug for IProxyProviderWinEventSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProxyProviderWinEventSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRangeValueProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRangeValueProvider {}
impl ::core::fmt::Debug for IRangeValueProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRangeValueProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRawElementProviderAdviseEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRawElementProviderAdviseEvents {}
impl ::core::fmt::Debug for IRawElementProviderAdviseEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRawElementProviderAdviseEvents").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRawElementProviderFragment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRawElementProviderFragment {}
impl ::core::fmt::Debug for IRawElementProviderFragment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRawElementProviderFragment").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRawElementProviderFragmentRoot {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRawElementProviderFragmentRoot {}
impl ::core::fmt::Debug for IRawElementProviderFragmentRoot {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRawElementProviderFragmentRoot").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRawElementProviderHostingAccessibles {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRawElementProviderHostingAccessibles {}
impl ::core::fmt::Debug for IRawElementProviderHostingAccessibles {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRawElementProviderHostingAccessibles").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRawElementProviderHwndOverride {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRawElementProviderHwndOverride {}
impl ::core::fmt::Debug for IRawElementProviderHwndOverride {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRawElementProviderHwndOverride").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRawElementProviderSimple {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRawElementProviderSimple {}
impl ::core::fmt::Debug for IRawElementProviderSimple {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRawElementProviderSimple").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRawElementProviderSimple2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRawElementProviderSimple2 {}
impl ::core::fmt::Debug for IRawElementProviderSimple2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRawElementProviderSimple2").field(&self.0).finish()
    }
}
impl IRawElementProviderSimple2 {
    pub unsafe fn ProviderOptions(&self) -> ::windows::core::Result<ProviderOptions> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ProviderOptions)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPatternProvider(&self, patternid: UIA_PATTERN_ID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPatternProvider)(::windows::core::Vtable::as_raw(self), patternid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetPropertyValue(&self, propertyid: UIA_PROPERTY_ID) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPropertyValue)(::windows::core::Vtable::as_raw(self), propertyid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn HostRawElementProvider(&self) -> ::windows::core::Result<IRawElementProviderSimple> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.HostRawElementProvider)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IRawElementProviderSimple3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRawElementProviderSimple3 {}
impl ::core::fmt::Debug for IRawElementProviderSimple3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRawElementProviderSimple3").field(&self.0).finish()
    }
}
impl IRawElementProviderSimple3 {
    pub unsafe fn ProviderOptions(&self) -> ::windows::core::Result<ProviderOptions> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.ProviderOptions)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPatternProvider(&self, patternid: UIA_PATTERN_ID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetPatternProvider)(::windows::core::Vtable::as_raw(self), patternid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetPropertyValue(&self, propertyid: UIA_PROPERTY_ID) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetPropertyValue)(::windows::core::Vtable::as_raw(self), propertyid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn HostRawElementProvider(&self) -> ::windows::core::Result<IRawElementProviderSimple> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.HostRawElementProvider)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ShowContextMenu(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ShowContextMenu)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
impl ::core::cmp::PartialEq for IRawElementProviderWindowlessSite {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRawElementProviderWindowlessSite {}
impl ::core::fmt::Debug for IRawElementProviderWindowlessSite {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRawElementProviderWindowlessSite").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRichEditUiaInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRichEditUiaInformation {}
impl ::core::fmt::Debug for IRichEditUiaInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRichEditUiaInformation").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRicheditWindowlessAccessibility {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRicheditWindowlessAccessibility {}
impl ::core::fmt::Debug for IRicheditWindowlessAccessibility {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRicheditWindowlessAccessibility").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IScrollItemProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IScrollItemProvider {}
impl ::core::fmt::Debug for IScrollItemProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IScrollItemProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IScrollProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IScrollProvider {}
impl ::core::fmt::Debug for IScrollProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IScrollProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISelectionItemProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISelectionItemProvider {}
impl ::core::fmt::Debug for ISelectionItemProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISelectionItemProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISelectionProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISelectionProvider {}
impl ::core::fmt::Debug for ISelectionProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISelectionProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISelectionProvider2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISelectionProvider2 {}
impl ::core::fmt::Debug for ISelectionProvider2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISelectionProvider2").field(&self.0).finish()
    }
}
impl ISelectionProvider2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSelection(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSelection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CanSelectMultiple(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CanSelectMultiple)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSelectionRequired(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsSelectionRequired)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ISpreadsheetItemProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpreadsheetItemProvider {}
impl ::core::fmt::Debug for ISpreadsheetItemProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpreadsheetItemProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISpreadsheetProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpreadsheetProvider {}
impl ::core::fmt::Debug for ISpreadsheetProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpreadsheetProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IStylesProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStylesProvider {}
impl ::core::fmt::Debug for IStylesProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStylesProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISynchronizedInputProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISynchronizedInputProvider {}
impl ::core::fmt::Debug for ISynchronizedInputProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISynchronizedInputProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITableItemProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITableItemProvider {}
impl ::core::fmt::Debug for ITableItemProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITableItemProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITableProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITableProvider {}
impl ::core::fmt::Debug for ITableProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITableProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITextChildProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextChildProvider {}
impl ::core::fmt::Debug for ITextChildProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextChildProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITextEditProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextEditProvider {}
impl ::core::fmt::Debug for ITextEditProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextEditProvider").field(&self.0).finish()
    }
}
impl ITextEditProvider {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSelection(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSelection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetVisibleRanges(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetVisibleRanges)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RangeFromChild<P0>(&self, childelement: P0) -> ::windows::core::Result<ITextRangeProvider>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IRawElementProviderSimple>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.RangeFromChild)(::windows::core::Vtable::as_raw(self), childelement.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RangeFromPoint(&self, point: UiaPoint) -> ::windows::core::Result<ITextRangeProvider> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.RangeFromPoint)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(point), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DocumentRange(&self) -> ::windows::core::Result<ITextRangeProvider> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.DocumentRange)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SupportedTextSelection(&self) -> ::windows::core::Result<SupportedTextSelection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SupportedTextSelection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ITextProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextProvider {}
impl ::core::fmt::Debug for ITextProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITextProvider2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextProvider2 {}
impl ::core::fmt::Debug for ITextProvider2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextProvider2").field(&self.0).finish()
    }
}
impl ITextProvider2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSelection(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSelection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetVisibleRanges(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetVisibleRanges)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RangeFromChild<P0>(&self, childelement: P0) -> ::windows::core::Result<ITextRangeProvider>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IRawElementProviderSimple>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.RangeFromChild)(::windows::core::Vtable::as_raw(self), childelement.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RangeFromPoint(&self, point: UiaPoint) -> ::windows::core::Result<ITextRangeProvider> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.RangeFromPoint)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(point), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DocumentRange(&self) -> ::windows::core::Result<ITextRangeProvider> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.DocumentRange)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SupportedTextSelection(&self) -> ::windows::core::Result<SupportedTextSelection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SupportedTextSelection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ITextRangeProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextRangeProvider {}
impl ::core::fmt::Debug for ITextRangeProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextRangeProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITextRangeProvider2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextRangeProvider2 {}
impl ::core::fmt::Debug for ITextRangeProvider2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextRangeProvider2").field(&self.0).finish()
    }
}
impl ITextRangeProvider2 {
    pub unsafe fn Clone(&self) -> ::windows::core::Result<ITextRangeProvider> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Compare<P0>(&self, range: P0) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITextRangeProvider>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Compare)(::windows::core::Vtable::as_raw(self), range.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CompareEndpoints<P0>(&self, endpoint: TextPatternRangeEndpoint, targetrange: P0, targetendpoint: TextPatternRangeEndpoint) -> ::windows::core::Result<i32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITextRangeProvider>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CompareEndpoints)(::windows::core::Vtable::as_raw(self), endpoint, targetrange.into().abi(), targetendpoint, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ExpandToEnclosingUnit(&self, unit: TextUnit) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ExpandToEnclosingUnit)(::windows::core::Vtable::as_raw(self), unit).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn FindAttribute<P0>(&self, attributeid: UIA_TEXTATTRIBUTE_ID, val: super::super::System::Com::VARIANT, backward: P0) -> ::windows::core::Result<ITextRangeProvider>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.FindAttribute)(::windows::core::Vtable::as_raw(self), attributeid, ::core::mem::transmute(val), backward.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindText<P0, P1>(&self, text: &::windows::core::BSTR, backward: P0, ignorecase: P1) -> ::windows::core::Result<ITextRangeProvider>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.FindText)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(text), backward.into(), ignorecase.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetAttributeValue(&self, attributeid: UIA_TEXTATTRIBUTE_ID) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetAttributeValue)(::windows::core::Vtable::as_raw(self), attributeid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetBoundingRectangles(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetBoundingRectangles)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetEnclosingElement(&self) -> ::windows::core::Result<IRawElementProviderSimple> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetEnclosingElement)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetText(&self, maxlength: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetText)(::windows::core::Vtable::as_raw(self), maxlength, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Move(&self, unit: TextUnit, count: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Move)(::windows::core::Vtable::as_raw(self), unit, count, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn MoveEndpointByUnit(&self, endpoint: TextPatternRangeEndpoint, unit: TextUnit, count: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.MoveEndpointByUnit)(::windows::core::Vtable::as_raw(self), endpoint, unit, count, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn MoveEndpointByRange<P0>(&self, endpoint: TextPatternRangeEndpoint, targetrange: P0, targetendpoint: TextPatternRangeEndpoint) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITextRangeProvider>>,
    {
        (::windows::core::Vtable::vtable(self).base__.MoveEndpointByRange)(::windows::core::Vtable::as_raw(self), endpoint, targetrange.into().abi(), targetendpoint).ok()
    }
    pub unsafe fn Select(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Select)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn AddToSelection(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.AddToSelection)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn RemoveFromSelection(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.RemoveFromSelection)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ScrollIntoView<P0>(&self, aligntotop: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.ScrollIntoView)(::windows::core::Vtable::as_raw(self), aligntotop.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetChildren(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetChildren)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IToggleProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IToggleProvider {}
impl ::core::fmt::Debug for IToggleProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IToggleProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITransformProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransformProvider {}
impl ::core::fmt::Debug for ITransformProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransformProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITransformProvider2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransformProvider2 {}
impl ::core::fmt::Debug for ITransformProvider2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransformProvider2").field(&self.0).finish()
    }
}
impl ITransformProvider2 {
    pub unsafe fn Move(&self, x: f64, y: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Move)(::windows::core::Vtable::as_raw(self), x, y).ok()
    }
    pub unsafe fn Resize(&self, width: f64, height: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Resize)(::windows::core::Vtable::as_raw(self), width, height).ok()
    }
    pub unsafe fn Rotate(&self, degrees: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Rotate)(::windows::core::Vtable::as_raw(self), degrees).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CanMove(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CanMove)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CanResize(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CanResize)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CanRotate(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CanRotate)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IUIAutomation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomation {}
impl ::core::fmt::Debug for IUIAutomation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomation").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIAutomation2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomation2 {}
impl ::core::fmt::Debug for IUIAutomation2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomation2").field(&self.0).finish()
    }
}
impl IUIAutomation2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CompareElements<P0, P1>(&self, el1: P0, el2: P1) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CompareElements)(::windows::core::Vtable::as_raw(self), el1.into().abi(), el2.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CompareRuntimeIds(&self, runtimeid1: *const super::super::System::Com::SAFEARRAY, runtimeid2: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CompareRuntimeIds)(::windows::core::Vtable::as_raw(self), runtimeid1, runtimeid2, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetRootElement(&self) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetRootElement)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ElementFromHandle<P0>(&self, hwnd: P0) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ElementFromHandle)(::windows::core::Vtable::as_raw(self), hwnd.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ElementFromPoint(&self, pt: super::super::Foundation::POINT) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ElementFromPoint)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pt), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFocusedElement(&self) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFocusedElement)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetRootElementBuildCache<P0>(&self, cacherequest: P0) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetRootElementBuildCache)(::windows::core::Vtable::as_raw(self), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ElementFromHandleBuildCache<P0, P1>(&self, hwnd: P0, cacherequest: P1) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ElementFromHandleBuildCache)(::windows::core::Vtable::as_raw(self), hwnd.into(), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ElementFromPointBuildCache<P0>(&self, pt: super::super::Foundation::POINT, cacherequest: P0) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ElementFromPointBuildCache)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pt), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFocusedElementBuildCache<P0>(&self, cacherequest: P0) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFocusedElementBuildCache)(::windows::core::Vtable::as_raw(self), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateTreeWalker<P0>(&self, pcondition: P0) -> ::windows::core::Result<IUIAutomationTreeWalker>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateTreeWalker)(::windows::core::Vtable::as_raw(self), pcondition.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ControlViewWalker(&self) -> ::windows::core::Result<IUIAutomationTreeWalker> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ControlViewWalker)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ContentViewWalker(&self) -> ::windows::core::Result<IUIAutomationTreeWalker> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ContentViewWalker)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RawViewWalker(&self) -> ::windows::core::Result<IUIAutomationTreeWalker> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.RawViewWalker)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RawViewCondition(&self) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.RawViewCondition)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ControlViewCondition(&self) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ControlViewCondition)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ContentViewCondition(&self) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ContentViewCondition)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCacheRequest(&self) -> ::windows::core::Result<IUIAutomationCacheRequest> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateCacheRequest)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateTrueCondition(&self) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateTrueCondition)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateFalseCondition(&self) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateFalseCondition)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreatePropertyCondition(&self, propertyid: UIA_PROPERTY_ID, value: super::super::System::Com::VARIANT) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreatePropertyCondition)(::windows::core::Vtable::as_raw(self), propertyid, ::core::mem::transmute(value), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreatePropertyConditionEx(&self, propertyid: UIA_PROPERTY_ID, value: super::super::System::Com::VARIANT, flags: PropertyConditionFlags) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreatePropertyConditionEx)(::windows::core::Vtable::as_raw(self), propertyid, ::core::mem::transmute(value), flags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateAndCondition<P0, P1>(&self, condition1: P0, condition2: P1) -> ::windows::core::Result<IUIAutomationCondition>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateAndCondition)(::windows::core::Vtable::as_raw(self), condition1.into().abi(), condition2.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateAndConditionFromArray(&self, conditions: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateAndConditionFromArray)(::windows::core::Vtable::as_raw(self), conditions, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateAndConditionFromNativeArray(&self, conditions: &[IUIAutomationCondition]) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateAndConditionFromNativeArray)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(conditions.as_ptr()), conditions.len() as _, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateOrCondition<P0, P1>(&self, condition1: P0, condition2: P1) -> ::windows::core::Result<IUIAutomationCondition>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateOrCondition)(::windows::core::Vtable::as_raw(self), condition1.into().abi(), condition2.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateOrConditionFromArray(&self, conditions: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateOrConditionFromArray)(::windows::core::Vtable::as_raw(self), conditions, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateOrConditionFromNativeArray(&self, conditions: &[IUIAutomationCondition]) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateOrConditionFromNativeArray)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(conditions.as_ptr()), conditions.len() as _, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateNotCondition<P0>(&self, condition: P0) -> ::windows::core::Result<IUIAutomationCondition>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateNotCondition)(::windows::core::Vtable::as_raw(self), condition.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AddAutomationEventHandler<P0, P1, P2>(&self, eventid: UIA_EVENT_ID, element: P0, scope: TreeScope, cacherequest: P1, handler: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
        P2: ::std::convert::Into<::windows::core::InParam<IUIAutomationEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).base__.AddAutomationEventHandler)(::windows::core::Vtable::as_raw(self), eventid, element.into().abi(), scope, cacherequest.into().abi(), handler.into().abi()).ok()
    }
    pub unsafe fn RemoveAutomationEventHandler<P0, P1>(&self, eventid: UIA_EVENT_ID, element: P0, handler: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).base__.RemoveAutomationEventHandler)(::windows::core::Vtable::as_raw(self), eventid, element.into().abi(), handler.into().abi()).ok()
    }
    pub unsafe fn AddPropertyChangedEventHandlerNativeArray<P0, P1, P2>(&self, element: P0, scope: TreeScope, cacherequest: P1, handler: P2, propertyarray: &[UIA_PROPERTY_ID]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
        P2: ::std::convert::Into<::windows::core::InParam<IUIAutomationPropertyChangedEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).base__.AddPropertyChangedEventHandlerNativeArray)(::windows::core::Vtable::as_raw(self), element.into().abi(), scope, cacherequest.into().abi(), handler.into().abi(), ::core::mem::transmute(propertyarray.as_ptr()), propertyarray.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddPropertyChangedEventHandler<P0, P1, P2>(&self, element: P0, scope: TreeScope, cacherequest: P1, handler: P2, propertyarray: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
        P2: ::std::convert::Into<::windows::core::InParam<IUIAutomationPropertyChangedEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).base__.AddPropertyChangedEventHandler)(::windows::core::Vtable::as_raw(self), element.into().abi(), scope, cacherequest.into().abi(), handler.into().abi(), propertyarray).ok()
    }
    pub unsafe fn RemovePropertyChangedEventHandler<P0, P1>(&self, element: P0, handler: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationPropertyChangedEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).base__.RemovePropertyChangedEventHandler)(::windows::core::Vtable::as_raw(self), element.into().abi(), handler.into().abi()).ok()
    }
    pub unsafe fn AddStructureChangedEventHandler<P0, P1, P2>(&self, element: P0, scope: TreeScope, cacherequest: P1, handler: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
        P2: ::std::convert::Into<::windows::core::InParam<IUIAutomationStructureChangedEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).base__.AddStructureChangedEventHandler)(::windows::core::Vtable::as_raw(self), element.into().abi(), scope, cacherequest.into().abi(), handler.into().abi()).ok()
    }
    pub unsafe fn RemoveStructureChangedEventHandler<P0, P1>(&self, element: P0, handler: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationStructureChangedEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).base__.RemoveStructureChangedEventHandler)(::windows::core::Vtable::as_raw(self), element.into().abi(), handler.into().abi()).ok()
    }
    pub unsafe fn AddFocusChangedEventHandler<P0, P1>(&self, cacherequest: P0, handler: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationFocusChangedEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).base__.AddFocusChangedEventHandler)(::windows::core::Vtable::as_raw(self), cacherequest.into().abi(), handler.into().abi()).ok()
    }
    pub unsafe fn RemoveFocusChangedEventHandler<P0>(&self, handler: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationFocusChangedEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).base__.RemoveFocusChangedEventHandler)(::windows::core::Vtable::as_raw(self), handler.into().abi()).ok()
    }
    pub unsafe fn RemoveAllEventHandlers(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.RemoveAllEventHandlers)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IntNativeArrayToSafeArray(&self, array: &[i32]) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IntNativeArrayToSafeArray)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(array.as_ptr()), array.len() as _, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IntSafeArrayToNativeArray(&self, intarray: *const super::super::System::Com::SAFEARRAY, array: *mut *mut i32, arraycount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.IntSafeArrayToNativeArray)(::windows::core::Vtable::as_raw(self), intarray, array, arraycount).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn RectToVariant(&self, rc: super::super::Foundation::RECT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.RectToVariant)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(rc), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn VariantToRect(&self, var: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.VariantToRect)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(var), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SafeArrayToRectNativeArray(&self, rects: *const super::super::System::Com::SAFEARRAY, rectarray: *mut *mut super::super::Foundation::RECT, rectarraycount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SafeArrayToRectNativeArray)(::windows::core::Vtable::as_raw(self), rects, rectarray, rectarraycount).ok()
    }
    pub unsafe fn CreateProxyFactoryEntry<P0>(&self, factory: P0) -> ::windows::core::Result<IUIAutomationProxyFactoryEntry>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationProxyFactory>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateProxyFactoryEntry)(::windows::core::Vtable::as_raw(self), factory.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ProxyFactoryMapping(&self) -> ::windows::core::Result<IUIAutomationProxyFactoryMapping> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ProxyFactoryMapping)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPropertyProgrammaticName(&self, property: UIA_PROPERTY_ID) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPropertyProgrammaticName)(::windows::core::Vtable::as_raw(self), property, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPatternProgrammaticName(&self, pattern: UIA_PATTERN_ID) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPatternProgrammaticName)(::windows::core::Vtable::as_raw(self), pattern, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PollForPotentialSupportedPatterns<P0>(&self, pelement: P0, patternids: *mut *mut super::super::System::Com::SAFEARRAY, patternnames: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
    {
        (::windows::core::Vtable::vtable(self).base__.PollForPotentialSupportedPatterns)(::windows::core::Vtable::as_raw(self), pelement.into().abi(), patternids, patternnames).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PollForPotentialSupportedProperties<P0>(&self, pelement: P0, propertyids: *mut *mut super::super::System::Com::SAFEARRAY, propertynames: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
    {
        (::windows::core::Vtable::vtable(self).base__.PollForPotentialSupportedProperties)(::windows::core::Vtable::as_raw(self), pelement.into().abi(), propertyids, propertynames).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CheckNotSupported(&self, value: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CheckNotSupported)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(value), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ReservedNotSupportedValue(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ReservedNotSupportedValue)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ReservedMixedAttributeValue(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ReservedMixedAttributeValue)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ElementFromIAccessible<P0>(&self, accessible: P0, childid: i32) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IAccessible>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ElementFromIAccessible)(::windows::core::Vtable::as_raw(self), accessible.into().abi(), childid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ElementFromIAccessibleBuildCache<P0, P1>(&self, accessible: P0, childid: i32, cacherequest: P1) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IAccessible>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ElementFromIAccessibleBuildCache)(::windows::core::Vtable::as_raw(self), accessible.into().abi(), childid, cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IUIAutomation3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomation3 {}
impl ::core::fmt::Debug for IUIAutomation3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomation3").field(&self.0).finish()
    }
}
impl IUIAutomation3 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CompareElements<P0, P1>(&self, el1: P0, el2: P1) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CompareElements)(::windows::core::Vtable::as_raw(self), el1.into().abi(), el2.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CompareRuntimeIds(&self, runtimeid1: *const super::super::System::Com::SAFEARRAY, runtimeid2: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CompareRuntimeIds)(::windows::core::Vtable::as_raw(self), runtimeid1, runtimeid2, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetRootElement(&self) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetRootElement)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ElementFromHandle<P0>(&self, hwnd: P0) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.ElementFromHandle)(::windows::core::Vtable::as_raw(self), hwnd.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ElementFromPoint(&self, pt: super::super::Foundation::POINT) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.ElementFromPoint)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pt), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFocusedElement(&self) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFocusedElement)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetRootElementBuildCache<P0>(&self, cacherequest: P0) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetRootElementBuildCache)(::windows::core::Vtable::as_raw(self), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ElementFromHandleBuildCache<P0, P1>(&self, hwnd: P0, cacherequest: P1) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.ElementFromHandleBuildCache)(::windows::core::Vtable::as_raw(self), hwnd.into(), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ElementFromPointBuildCache<P0>(&self, pt: super::super::Foundation::POINT, cacherequest: P0) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.ElementFromPointBuildCache)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pt), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFocusedElementBuildCache<P0>(&self, cacherequest: P0) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFocusedElementBuildCache)(::windows::core::Vtable::as_raw(self), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateTreeWalker<P0>(&self, pcondition: P0) -> ::windows::core::Result<IUIAutomationTreeWalker>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateTreeWalker)(::windows::core::Vtable::as_raw(self), pcondition.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ControlViewWalker(&self) -> ::windows::core::Result<IUIAutomationTreeWalker> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.ControlViewWalker)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ContentViewWalker(&self) -> ::windows::core::Result<IUIAutomationTreeWalker> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.ContentViewWalker)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RawViewWalker(&self) -> ::windows::core::Result<IUIAutomationTreeWalker> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.RawViewWalker)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RawViewCondition(&self) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.RawViewCondition)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ControlViewCondition(&self) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.ControlViewCondition)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ContentViewCondition(&self) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.ContentViewCondition)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCacheRequest(&self) -> ::windows::core::Result<IUIAutomationCacheRequest> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateCacheRequest)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateTrueCondition(&self) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateTrueCondition)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateFalseCondition(&self) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateFalseCondition)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreatePropertyCondition(&self, propertyid: UIA_PROPERTY_ID, value: super::super::System::Com::VARIANT) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreatePropertyCondition)(::windows::core::Vtable::as_raw(self), propertyid, ::core::mem::transmute(value), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreatePropertyConditionEx(&self, propertyid: UIA_PROPERTY_ID, value: super::super::System::Com::VARIANT, flags: PropertyConditionFlags) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreatePropertyConditionEx)(::windows::core::Vtable::as_raw(self), propertyid, ::core::mem::transmute(value), flags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateAndCondition<P0, P1>(&self, condition1: P0, condition2: P1) -> ::windows::core::Result<IUIAutomationCondition>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateAndCondition)(::windows::core::Vtable::as_raw(self), condition1.into().abi(), condition2.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateAndConditionFromArray(&self, conditions: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateAndConditionFromArray)(::windows::core::Vtable::as_raw(self), conditions, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateAndConditionFromNativeArray(&self, conditions: &[IUIAutomationCondition]) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateAndConditionFromNativeArray)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(conditions.as_ptr()), conditions.len() as _, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateOrCondition<P0, P1>(&self, condition1: P0, condition2: P1) -> ::windows::core::Result<IUIAutomationCondition>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateOrCondition)(::windows::core::Vtable::as_raw(self), condition1.into().abi(), condition2.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateOrConditionFromArray(&self, conditions: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateOrConditionFromArray)(::windows::core::Vtable::as_raw(self), conditions, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateOrConditionFromNativeArray(&self, conditions: &[IUIAutomationCondition]) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateOrConditionFromNativeArray)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(conditions.as_ptr()), conditions.len() as _, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateNotCondition<P0>(&self, condition: P0) -> ::windows::core::Result<IUIAutomationCondition>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateNotCondition)(::windows::core::Vtable::as_raw(self), condition.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AddAutomationEventHandler<P0, P1, P2>(&self, eventid: UIA_EVENT_ID, element: P0, scope: TreeScope, cacherequest: P1, handler: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
        P2: ::std::convert::Into<::windows::core::InParam<IUIAutomationEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.AddAutomationEventHandler)(::windows::core::Vtable::as_raw(self), eventid, element.into().abi(), scope, cacherequest.into().abi(), handler.into().abi()).ok()
    }
    pub unsafe fn RemoveAutomationEventHandler<P0, P1>(&self, eventid: UIA_EVENT_ID, element: P0, handler: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.RemoveAutomationEventHandler)(::windows::core::Vtable::as_raw(self), eventid, element.into().abi(), handler.into().abi()).ok()
    }
    pub unsafe fn AddPropertyChangedEventHandlerNativeArray<P0, P1, P2>(&self, element: P0, scope: TreeScope, cacherequest: P1, handler: P2, propertyarray: &[UIA_PROPERTY_ID]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
        P2: ::std::convert::Into<::windows::core::InParam<IUIAutomationPropertyChangedEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.AddPropertyChangedEventHandlerNativeArray)(::windows::core::Vtable::as_raw(self), element.into().abi(), scope, cacherequest.into().abi(), handler.into().abi(), ::core::mem::transmute(propertyarray.as_ptr()), propertyarray.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddPropertyChangedEventHandler<P0, P1, P2>(&self, element: P0, scope: TreeScope, cacherequest: P1, handler: P2, propertyarray: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
        P2: ::std::convert::Into<::windows::core::InParam<IUIAutomationPropertyChangedEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.AddPropertyChangedEventHandler)(::windows::core::Vtable::as_raw(self), element.into().abi(), scope, cacherequest.into().abi(), handler.into().abi(), propertyarray).ok()
    }
    pub unsafe fn RemovePropertyChangedEventHandler<P0, P1>(&self, element: P0, handler: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationPropertyChangedEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.RemovePropertyChangedEventHandler)(::windows::core::Vtable::as_raw(self), element.into().abi(), handler.into().abi()).ok()
    }
    pub unsafe fn AddStructureChangedEventHandler<P0, P1, P2>(&self, element: P0, scope: TreeScope, cacherequest: P1, handler: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
        P2: ::std::convert::Into<::windows::core::InParam<IUIAutomationStructureChangedEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.AddStructureChangedEventHandler)(::windows::core::Vtable::as_raw(self), element.into().abi(), scope, cacherequest.into().abi(), handler.into().abi()).ok()
    }
    pub unsafe fn RemoveStructureChangedEventHandler<P0, P1>(&self, element: P0, handler: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationStructureChangedEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.RemoveStructureChangedEventHandler)(::windows::core::Vtable::as_raw(self), element.into().abi(), handler.into().abi()).ok()
    }
    pub unsafe fn AddFocusChangedEventHandler<P0, P1>(&self, cacherequest: P0, handler: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationFocusChangedEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.AddFocusChangedEventHandler)(::windows::core::Vtable::as_raw(self), cacherequest.into().abi(), handler.into().abi()).ok()
    }
    pub unsafe fn RemoveFocusChangedEventHandler<P0>(&self, handler: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationFocusChangedEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.RemoveFocusChangedEventHandler)(::windows::core::Vtable::as_raw(self), handler.into().abi()).ok()
    }
    pub unsafe fn RemoveAllEventHandlers(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.RemoveAllEventHandlers)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IntNativeArrayToSafeArray(&self, array: &[i32]) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.IntNativeArrayToSafeArray)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(array.as_ptr()), array.len() as _, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IntSafeArrayToNativeArray(&self, intarray: *const super::super::System::Com::SAFEARRAY, array: *mut *mut i32, arraycount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.IntSafeArrayToNativeArray)(::windows::core::Vtable::as_raw(self), intarray, array, arraycount).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn RectToVariant(&self, rc: super::super::Foundation::RECT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.RectToVariant)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(rc), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn VariantToRect(&self, var: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.VariantToRect)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(var), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SafeArrayToRectNativeArray(&self, rects: *const super::super::System::Com::SAFEARRAY, rectarray: *mut *mut super::super::Foundation::RECT, rectarraycount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SafeArrayToRectNativeArray)(::windows::core::Vtable::as_raw(self), rects, rectarray, rectarraycount).ok()
    }
    pub unsafe fn CreateProxyFactoryEntry<P0>(&self, factory: P0) -> ::windows::core::Result<IUIAutomationProxyFactoryEntry>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationProxyFactory>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateProxyFactoryEntry)(::windows::core::Vtable::as_raw(self), factory.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ProxyFactoryMapping(&self) -> ::windows::core::Result<IUIAutomationProxyFactoryMapping> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.ProxyFactoryMapping)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPropertyProgrammaticName(&self, property: UIA_PROPERTY_ID) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetPropertyProgrammaticName)(::windows::core::Vtable::as_raw(self), property, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPatternProgrammaticName(&self, pattern: UIA_PATTERN_ID) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetPatternProgrammaticName)(::windows::core::Vtable::as_raw(self), pattern, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PollForPotentialSupportedPatterns<P0>(&self, pelement: P0, patternids: *mut *mut super::super::System::Com::SAFEARRAY, patternnames: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.PollForPotentialSupportedPatterns)(::windows::core::Vtable::as_raw(self), pelement.into().abi(), patternids, patternnames).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PollForPotentialSupportedProperties<P0>(&self, pelement: P0, propertyids: *mut *mut super::super::System::Com::SAFEARRAY, propertynames: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.PollForPotentialSupportedProperties)(::windows::core::Vtable::as_raw(self), pelement.into().abi(), propertyids, propertynames).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CheckNotSupported(&self, value: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CheckNotSupported)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(value), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ReservedNotSupportedValue(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.ReservedNotSupportedValue)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ReservedMixedAttributeValue(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.ReservedMixedAttributeValue)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ElementFromIAccessible<P0>(&self, accessible: P0, childid: i32) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IAccessible>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.ElementFromIAccessible)(::windows::core::Vtable::as_raw(self), accessible.into().abi(), childid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ElementFromIAccessibleBuildCache<P0, P1>(&self, accessible: P0, childid: i32, cacherequest: P1) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IAccessible>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.ElementFromIAccessibleBuildCache)(::windows::core::Vtable::as_raw(self), accessible.into().abi(), childid, cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AutoSetFocus(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.AutoSetFocus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAutoSetFocus<P0>(&self, autosetfocus: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetAutoSetFocus)(::windows::core::Vtable::as_raw(self), autosetfocus.into()).ok()
    }
    pub unsafe fn ConnectionTimeout(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ConnectionTimeout)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetConnectionTimeout(&self, timeout: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetConnectionTimeout)(::windows::core::Vtable::as_raw(self), timeout).ok()
    }
    pub unsafe fn TransactionTimeout(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.TransactionTimeout)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetTransactionTimeout(&self, timeout: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetTransactionTimeout)(::windows::core::Vtable::as_raw(self), timeout).ok()
    }
}
impl ::core::cmp::PartialEq for IUIAutomation4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomation4 {}
impl ::core::fmt::Debug for IUIAutomation4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomation4").field(&self.0).finish()
    }
}
impl IUIAutomation4 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CompareElements<P0, P1>(&self, el1: P0, el2: P1) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CompareElements)(::windows::core::Vtable::as_raw(self), el1.into().abi(), el2.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CompareRuntimeIds(&self, runtimeid1: *const super::super::System::Com::SAFEARRAY, runtimeid2: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CompareRuntimeIds)(::windows::core::Vtable::as_raw(self), runtimeid1, runtimeid2, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetRootElement(&self) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetRootElement)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ElementFromHandle<P0>(&self, hwnd: P0) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ElementFromHandle)(::windows::core::Vtable::as_raw(self), hwnd.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ElementFromPoint(&self, pt: super::super::Foundation::POINT) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ElementFromPoint)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pt), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFocusedElement(&self) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetFocusedElement)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetRootElementBuildCache<P0>(&self, cacherequest: P0) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetRootElementBuildCache)(::windows::core::Vtable::as_raw(self), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ElementFromHandleBuildCache<P0, P1>(&self, hwnd: P0, cacherequest: P1) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ElementFromHandleBuildCache)(::windows::core::Vtable::as_raw(self), hwnd.into(), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ElementFromPointBuildCache<P0>(&self, pt: super::super::Foundation::POINT, cacherequest: P0) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ElementFromPointBuildCache)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pt), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFocusedElementBuildCache<P0>(&self, cacherequest: P0) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetFocusedElementBuildCache)(::windows::core::Vtable::as_raw(self), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateTreeWalker<P0>(&self, pcondition: P0) -> ::windows::core::Result<IUIAutomationTreeWalker>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateTreeWalker)(::windows::core::Vtable::as_raw(self), pcondition.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ControlViewWalker(&self) -> ::windows::core::Result<IUIAutomationTreeWalker> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ControlViewWalker)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ContentViewWalker(&self) -> ::windows::core::Result<IUIAutomationTreeWalker> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ContentViewWalker)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RawViewWalker(&self) -> ::windows::core::Result<IUIAutomationTreeWalker> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.RawViewWalker)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RawViewCondition(&self) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.RawViewCondition)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ControlViewCondition(&self) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ControlViewCondition)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ContentViewCondition(&self) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ContentViewCondition)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCacheRequest(&self) -> ::windows::core::Result<IUIAutomationCacheRequest> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateCacheRequest)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateTrueCondition(&self) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateTrueCondition)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateFalseCondition(&self) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateFalseCondition)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreatePropertyCondition(&self, propertyid: UIA_PROPERTY_ID, value: super::super::System::Com::VARIANT) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreatePropertyCondition)(::windows::core::Vtable::as_raw(self), propertyid, ::core::mem::transmute(value), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreatePropertyConditionEx(&self, propertyid: UIA_PROPERTY_ID, value: super::super::System::Com::VARIANT, flags: PropertyConditionFlags) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreatePropertyConditionEx)(::windows::core::Vtable::as_raw(self), propertyid, ::core::mem::transmute(value), flags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateAndCondition<P0, P1>(&self, condition1: P0, condition2: P1) -> ::windows::core::Result<IUIAutomationCondition>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateAndCondition)(::windows::core::Vtable::as_raw(self), condition1.into().abi(), condition2.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateAndConditionFromArray(&self, conditions: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateAndConditionFromArray)(::windows::core::Vtable::as_raw(self), conditions, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateAndConditionFromNativeArray(&self, conditions: &[IUIAutomationCondition]) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateAndConditionFromNativeArray)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(conditions.as_ptr()), conditions.len() as _, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateOrCondition<P0, P1>(&self, condition1: P0, condition2: P1) -> ::windows::core::Result<IUIAutomationCondition>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateOrCondition)(::windows::core::Vtable::as_raw(self), condition1.into().abi(), condition2.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateOrConditionFromArray(&self, conditions: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateOrConditionFromArray)(::windows::core::Vtable::as_raw(self), conditions, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateOrConditionFromNativeArray(&self, conditions: &[IUIAutomationCondition]) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateOrConditionFromNativeArray)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(conditions.as_ptr()), conditions.len() as _, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateNotCondition<P0>(&self, condition: P0) -> ::windows::core::Result<IUIAutomationCondition>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateNotCondition)(::windows::core::Vtable::as_raw(self), condition.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AddAutomationEventHandler<P0, P1, P2>(&self, eventid: UIA_EVENT_ID, element: P0, scope: TreeScope, cacherequest: P1, handler: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
        P2: ::std::convert::Into<::windows::core::InParam<IUIAutomationEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.AddAutomationEventHandler)(::windows::core::Vtable::as_raw(self), eventid, element.into().abi(), scope, cacherequest.into().abi(), handler.into().abi()).ok()
    }
    pub unsafe fn RemoveAutomationEventHandler<P0, P1>(&self, eventid: UIA_EVENT_ID, element: P0, handler: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.RemoveAutomationEventHandler)(::windows::core::Vtable::as_raw(self), eventid, element.into().abi(), handler.into().abi()).ok()
    }
    pub unsafe fn AddPropertyChangedEventHandlerNativeArray<P0, P1, P2>(&self, element: P0, scope: TreeScope, cacherequest: P1, handler: P2, propertyarray: &[UIA_PROPERTY_ID]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
        P2: ::std::convert::Into<::windows::core::InParam<IUIAutomationPropertyChangedEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.AddPropertyChangedEventHandlerNativeArray)(::windows::core::Vtable::as_raw(self), element.into().abi(), scope, cacherequest.into().abi(), handler.into().abi(), ::core::mem::transmute(propertyarray.as_ptr()), propertyarray.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddPropertyChangedEventHandler<P0, P1, P2>(&self, element: P0, scope: TreeScope, cacherequest: P1, handler: P2, propertyarray: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
        P2: ::std::convert::Into<::windows::core::InParam<IUIAutomationPropertyChangedEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.AddPropertyChangedEventHandler)(::windows::core::Vtable::as_raw(self), element.into().abi(), scope, cacherequest.into().abi(), handler.into().abi(), propertyarray).ok()
    }
    pub unsafe fn RemovePropertyChangedEventHandler<P0, P1>(&self, element: P0, handler: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationPropertyChangedEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.RemovePropertyChangedEventHandler)(::windows::core::Vtable::as_raw(self), element.into().abi(), handler.into().abi()).ok()
    }
    pub unsafe fn AddStructureChangedEventHandler<P0, P1, P2>(&self, element: P0, scope: TreeScope, cacherequest: P1, handler: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
        P2: ::std::convert::Into<::windows::core::InParam<IUIAutomationStructureChangedEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.AddStructureChangedEventHandler)(::windows::core::Vtable::as_raw(self), element.into().abi(), scope, cacherequest.into().abi(), handler.into().abi()).ok()
    }
    pub unsafe fn RemoveStructureChangedEventHandler<P0, P1>(&self, element: P0, handler: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationStructureChangedEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.RemoveStructureChangedEventHandler)(::windows::core::Vtable::as_raw(self), element.into().abi(), handler.into().abi()).ok()
    }
    pub unsafe fn AddFocusChangedEventHandler<P0, P1>(&self, cacherequest: P0, handler: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationFocusChangedEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.AddFocusChangedEventHandler)(::windows::core::Vtable::as_raw(self), cacherequest.into().abi(), handler.into().abi()).ok()
    }
    pub unsafe fn RemoveFocusChangedEventHandler<P0>(&self, handler: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationFocusChangedEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.RemoveFocusChangedEventHandler)(::windows::core::Vtable::as_raw(self), handler.into().abi()).ok()
    }
    pub unsafe fn RemoveAllEventHandlers(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.RemoveAllEventHandlers)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IntNativeArrayToSafeArray(&self, array: &[i32]) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.IntNativeArrayToSafeArray)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(array.as_ptr()), array.len() as _, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IntSafeArrayToNativeArray(&self, intarray: *const super::super::System::Com::SAFEARRAY, array: *mut *mut i32, arraycount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.IntSafeArrayToNativeArray)(::windows::core::Vtable::as_raw(self), intarray, array, arraycount).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn RectToVariant(&self, rc: super::super::Foundation::RECT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.RectToVariant)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(rc), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn VariantToRect(&self, var: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.VariantToRect)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(var), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SafeArrayToRectNativeArray(&self, rects: *const super::super::System::Com::SAFEARRAY, rectarray: *mut *mut super::super::Foundation::RECT, rectarraycount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SafeArrayToRectNativeArray)(::windows::core::Vtable::as_raw(self), rects, rectarray, rectarraycount).ok()
    }
    pub unsafe fn CreateProxyFactoryEntry<P0>(&self, factory: P0) -> ::windows::core::Result<IUIAutomationProxyFactoryEntry>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationProxyFactory>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateProxyFactoryEntry)(::windows::core::Vtable::as_raw(self), factory.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ProxyFactoryMapping(&self) -> ::windows::core::Result<IUIAutomationProxyFactoryMapping> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ProxyFactoryMapping)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPropertyProgrammaticName(&self, property: UIA_PROPERTY_ID) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPropertyProgrammaticName)(::windows::core::Vtable::as_raw(self), property, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPatternProgrammaticName(&self, pattern: UIA_PATTERN_ID) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPatternProgrammaticName)(::windows::core::Vtable::as_raw(self), pattern, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PollForPotentialSupportedPatterns<P0>(&self, pelement: P0, patternids: *mut *mut super::super::System::Com::SAFEARRAY, patternnames: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.PollForPotentialSupportedPatterns)(::windows::core::Vtable::as_raw(self), pelement.into().abi(), patternids, patternnames).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PollForPotentialSupportedProperties<P0>(&self, pelement: P0, propertyids: *mut *mut super::super::System::Com::SAFEARRAY, propertynames: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.PollForPotentialSupportedProperties)(::windows::core::Vtable::as_raw(self), pelement.into().abi(), propertyids, propertynames).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CheckNotSupported(&self, value: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CheckNotSupported)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(value), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ReservedNotSupportedValue(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ReservedNotSupportedValue)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ReservedMixedAttributeValue(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ReservedMixedAttributeValue)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ElementFromIAccessible<P0>(&self, accessible: P0, childid: i32) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IAccessible>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ElementFromIAccessible)(::windows::core::Vtable::as_raw(self), accessible.into().abi(), childid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ElementFromIAccessibleBuildCache<P0, P1>(&self, accessible: P0, childid: i32, cacherequest: P1) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IAccessible>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ElementFromIAccessibleBuildCache)(::windows::core::Vtable::as_raw(self), accessible.into().abi(), childid, cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AutoSetFocus(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.AutoSetFocus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAutoSetFocus<P0>(&self, autosetfocus: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetAutoSetFocus)(::windows::core::Vtable::as_raw(self), autosetfocus.into()).ok()
    }
    pub unsafe fn ConnectionTimeout(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.ConnectionTimeout)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetConnectionTimeout(&self, timeout: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetConnectionTimeout)(::windows::core::Vtable::as_raw(self), timeout).ok()
    }
    pub unsafe fn TransactionTimeout(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.TransactionTimeout)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetTransactionTimeout(&self, timeout: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetTransactionTimeout)(::windows::core::Vtable::as_raw(self), timeout).ok()
    }
    pub unsafe fn AddTextEditTextChangedEventHandler<P0, P1, P2>(&self, element: P0, scope: TreeScope, texteditchangetype: TextEditChangeType, cacherequest: P1, handler: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
        P2: ::std::convert::Into<::windows::core::InParam<IUIAutomationTextEditTextChangedEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).base__.AddTextEditTextChangedEventHandler)(::windows::core::Vtable::as_raw(self), element.into().abi(), scope, texteditchangetype, cacherequest.into().abi(), handler.into().abi()).ok()
    }
    pub unsafe fn RemoveTextEditTextChangedEventHandler<P0, P1>(&self, element: P0, handler: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationTextEditTextChangedEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).base__.RemoveTextEditTextChangedEventHandler)(::windows::core::Vtable::as_raw(self), element.into().abi(), handler.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IUIAutomation5 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomation5 {}
impl ::core::fmt::Debug for IUIAutomation5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomation5").field(&self.0).finish()
    }
}
impl IUIAutomation5 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CompareElements<P0, P1>(&self, el1: P0, el2: P1) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CompareElements)(::windows::core::Vtable::as_raw(self), el1.into().abi(), el2.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CompareRuntimeIds(&self, runtimeid1: *const super::super::System::Com::SAFEARRAY, runtimeid2: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CompareRuntimeIds)(::windows::core::Vtable::as_raw(self), runtimeid1, runtimeid2, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetRootElement(&self) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetRootElement)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ElementFromHandle<P0>(&self, hwnd: P0) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.ElementFromHandle)(::windows::core::Vtable::as_raw(self), hwnd.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ElementFromPoint(&self, pt: super::super::Foundation::POINT) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.ElementFromPoint)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pt), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFocusedElement(&self) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetFocusedElement)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetRootElementBuildCache<P0>(&self, cacherequest: P0) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetRootElementBuildCache)(::windows::core::Vtable::as_raw(self), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ElementFromHandleBuildCache<P0, P1>(&self, hwnd: P0, cacherequest: P1) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.ElementFromHandleBuildCache)(::windows::core::Vtable::as_raw(self), hwnd.into(), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ElementFromPointBuildCache<P0>(&self, pt: super::super::Foundation::POINT, cacherequest: P0) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.ElementFromPointBuildCache)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pt), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFocusedElementBuildCache<P0>(&self, cacherequest: P0) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetFocusedElementBuildCache)(::windows::core::Vtable::as_raw(self), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateTreeWalker<P0>(&self, pcondition: P0) -> ::windows::core::Result<IUIAutomationTreeWalker>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateTreeWalker)(::windows::core::Vtable::as_raw(self), pcondition.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ControlViewWalker(&self) -> ::windows::core::Result<IUIAutomationTreeWalker> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.ControlViewWalker)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ContentViewWalker(&self) -> ::windows::core::Result<IUIAutomationTreeWalker> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.ContentViewWalker)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RawViewWalker(&self) -> ::windows::core::Result<IUIAutomationTreeWalker> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.RawViewWalker)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RawViewCondition(&self) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.RawViewCondition)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ControlViewCondition(&self) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.ControlViewCondition)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ContentViewCondition(&self) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.ContentViewCondition)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCacheRequest(&self) -> ::windows::core::Result<IUIAutomationCacheRequest> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateCacheRequest)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateTrueCondition(&self) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateTrueCondition)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateFalseCondition(&self) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateFalseCondition)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreatePropertyCondition(&self, propertyid: UIA_PROPERTY_ID, value: super::super::System::Com::VARIANT) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreatePropertyCondition)(::windows::core::Vtable::as_raw(self), propertyid, ::core::mem::transmute(value), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreatePropertyConditionEx(&self, propertyid: UIA_PROPERTY_ID, value: super::super::System::Com::VARIANT, flags: PropertyConditionFlags) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreatePropertyConditionEx)(::windows::core::Vtable::as_raw(self), propertyid, ::core::mem::transmute(value), flags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateAndCondition<P0, P1>(&self, condition1: P0, condition2: P1) -> ::windows::core::Result<IUIAutomationCondition>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateAndCondition)(::windows::core::Vtable::as_raw(self), condition1.into().abi(), condition2.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateAndConditionFromArray(&self, conditions: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateAndConditionFromArray)(::windows::core::Vtable::as_raw(self), conditions, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateAndConditionFromNativeArray(&self, conditions: &[IUIAutomationCondition]) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateAndConditionFromNativeArray)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(conditions.as_ptr()), conditions.len() as _, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateOrCondition<P0, P1>(&self, condition1: P0, condition2: P1) -> ::windows::core::Result<IUIAutomationCondition>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateOrCondition)(::windows::core::Vtable::as_raw(self), condition1.into().abi(), condition2.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateOrConditionFromArray(&self, conditions: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateOrConditionFromArray)(::windows::core::Vtable::as_raw(self), conditions, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateOrConditionFromNativeArray(&self, conditions: &[IUIAutomationCondition]) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateOrConditionFromNativeArray)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(conditions.as_ptr()), conditions.len() as _, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateNotCondition<P0>(&self, condition: P0) -> ::windows::core::Result<IUIAutomationCondition>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateNotCondition)(::windows::core::Vtable::as_raw(self), condition.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AddAutomationEventHandler<P0, P1, P2>(&self, eventid: UIA_EVENT_ID, element: P0, scope: TreeScope, cacherequest: P1, handler: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
        P2: ::std::convert::Into<::windows::core::InParam<IUIAutomationEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.AddAutomationEventHandler)(::windows::core::Vtable::as_raw(self), eventid, element.into().abi(), scope, cacherequest.into().abi(), handler.into().abi()).ok()
    }
    pub unsafe fn RemoveAutomationEventHandler<P0, P1>(&self, eventid: UIA_EVENT_ID, element: P0, handler: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.RemoveAutomationEventHandler)(::windows::core::Vtable::as_raw(self), eventid, element.into().abi(), handler.into().abi()).ok()
    }
    pub unsafe fn AddPropertyChangedEventHandlerNativeArray<P0, P1, P2>(&self, element: P0, scope: TreeScope, cacherequest: P1, handler: P2, propertyarray: &[UIA_PROPERTY_ID]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
        P2: ::std::convert::Into<::windows::core::InParam<IUIAutomationPropertyChangedEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.AddPropertyChangedEventHandlerNativeArray)(::windows::core::Vtable::as_raw(self), element.into().abi(), scope, cacherequest.into().abi(), handler.into().abi(), ::core::mem::transmute(propertyarray.as_ptr()), propertyarray.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddPropertyChangedEventHandler<P0, P1, P2>(&self, element: P0, scope: TreeScope, cacherequest: P1, handler: P2, propertyarray: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
        P2: ::std::convert::Into<::windows::core::InParam<IUIAutomationPropertyChangedEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.AddPropertyChangedEventHandler)(::windows::core::Vtable::as_raw(self), element.into().abi(), scope, cacherequest.into().abi(), handler.into().abi(), propertyarray).ok()
    }
    pub unsafe fn RemovePropertyChangedEventHandler<P0, P1>(&self, element: P0, handler: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationPropertyChangedEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.RemovePropertyChangedEventHandler)(::windows::core::Vtable::as_raw(self), element.into().abi(), handler.into().abi()).ok()
    }
    pub unsafe fn AddStructureChangedEventHandler<P0, P1, P2>(&self, element: P0, scope: TreeScope, cacherequest: P1, handler: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
        P2: ::std::convert::Into<::windows::core::InParam<IUIAutomationStructureChangedEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.AddStructureChangedEventHandler)(::windows::core::Vtable::as_raw(self), element.into().abi(), scope, cacherequest.into().abi(), handler.into().abi()).ok()
    }
    pub unsafe fn RemoveStructureChangedEventHandler<P0, P1>(&self, element: P0, handler: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationStructureChangedEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.RemoveStructureChangedEventHandler)(::windows::core::Vtable::as_raw(self), element.into().abi(), handler.into().abi()).ok()
    }
    pub unsafe fn AddFocusChangedEventHandler<P0, P1>(&self, cacherequest: P0, handler: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationFocusChangedEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.AddFocusChangedEventHandler)(::windows::core::Vtable::as_raw(self), cacherequest.into().abi(), handler.into().abi()).ok()
    }
    pub unsafe fn RemoveFocusChangedEventHandler<P0>(&self, handler: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationFocusChangedEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.RemoveFocusChangedEventHandler)(::windows::core::Vtable::as_raw(self), handler.into().abi()).ok()
    }
    pub unsafe fn RemoveAllEventHandlers(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.RemoveAllEventHandlers)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IntNativeArrayToSafeArray(&self, array: &[i32]) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.IntNativeArrayToSafeArray)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(array.as_ptr()), array.len() as _, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IntSafeArrayToNativeArray(&self, intarray: *const super::super::System::Com::SAFEARRAY, array: *mut *mut i32, arraycount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.IntSafeArrayToNativeArray)(::windows::core::Vtable::as_raw(self), intarray, array, arraycount).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn RectToVariant(&self, rc: super::super::Foundation::RECT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.RectToVariant)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(rc), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn VariantToRect(&self, var: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.VariantToRect)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(var), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SafeArrayToRectNativeArray(&self, rects: *const super::super::System::Com::SAFEARRAY, rectarray: *mut *mut super::super::Foundation::RECT, rectarraycount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SafeArrayToRectNativeArray)(::windows::core::Vtable::as_raw(self), rects, rectarray, rectarraycount).ok()
    }
    pub unsafe fn CreateProxyFactoryEntry<P0>(&self, factory: P0) -> ::windows::core::Result<IUIAutomationProxyFactoryEntry>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationProxyFactory>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateProxyFactoryEntry)(::windows::core::Vtable::as_raw(self), factory.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ProxyFactoryMapping(&self) -> ::windows::core::Result<IUIAutomationProxyFactoryMapping> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.ProxyFactoryMapping)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPropertyProgrammaticName(&self, property: UIA_PROPERTY_ID) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetPropertyProgrammaticName)(::windows::core::Vtable::as_raw(self), property, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPatternProgrammaticName(&self, pattern: UIA_PATTERN_ID) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetPatternProgrammaticName)(::windows::core::Vtable::as_raw(self), pattern, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PollForPotentialSupportedPatterns<P0>(&self, pelement: P0, patternids: *mut *mut super::super::System::Com::SAFEARRAY, patternnames: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.PollForPotentialSupportedPatterns)(::windows::core::Vtable::as_raw(self), pelement.into().abi(), patternids, patternnames).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PollForPotentialSupportedProperties<P0>(&self, pelement: P0, propertyids: *mut *mut super::super::System::Com::SAFEARRAY, propertynames: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.PollForPotentialSupportedProperties)(::windows::core::Vtable::as_raw(self), pelement.into().abi(), propertyids, propertynames).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CheckNotSupported(&self, value: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CheckNotSupported)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(value), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ReservedNotSupportedValue(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.ReservedNotSupportedValue)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ReservedMixedAttributeValue(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.ReservedMixedAttributeValue)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ElementFromIAccessible<P0>(&self, accessible: P0, childid: i32) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IAccessible>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.ElementFromIAccessible)(::windows::core::Vtable::as_raw(self), accessible.into().abi(), childid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ElementFromIAccessibleBuildCache<P0, P1>(&self, accessible: P0, childid: i32, cacherequest: P1) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IAccessible>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.ElementFromIAccessibleBuildCache)(::windows::core::Vtable::as_raw(self), accessible.into().abi(), childid, cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AutoSetFocus(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.AutoSetFocus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAutoSetFocus<P0>(&self, autosetfocus: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetAutoSetFocus)(::windows::core::Vtable::as_raw(self), autosetfocus.into()).ok()
    }
    pub unsafe fn ConnectionTimeout(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ConnectionTimeout)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetConnectionTimeout(&self, timeout: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetConnectionTimeout)(::windows::core::Vtable::as_raw(self), timeout).ok()
    }
    pub unsafe fn TransactionTimeout(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.TransactionTimeout)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetTransactionTimeout(&self, timeout: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetTransactionTimeout)(::windows::core::Vtable::as_raw(self), timeout).ok()
    }
    pub unsafe fn AddTextEditTextChangedEventHandler<P0, P1, P2>(&self, element: P0, scope: TreeScope, texteditchangetype: TextEditChangeType, cacherequest: P1, handler: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
        P2: ::std::convert::Into<::windows::core::InParam<IUIAutomationTextEditTextChangedEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.AddTextEditTextChangedEventHandler)(::windows::core::Vtable::as_raw(self), element.into().abi(), scope, texteditchangetype, cacherequest.into().abi(), handler.into().abi()).ok()
    }
    pub unsafe fn RemoveTextEditTextChangedEventHandler<P0, P1>(&self, element: P0, handler: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationTextEditTextChangedEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.RemoveTextEditTextChangedEventHandler)(::windows::core::Vtable::as_raw(self), element.into().abi(), handler.into().abi()).ok()
    }
    pub unsafe fn AddChangesEventHandler<P0, P1, P2>(&self, element: P0, scope: TreeScope, changetypes: &[i32], pcacherequest: P1, handler: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
        P2: ::std::convert::Into<::windows::core::InParam<IUIAutomationChangesEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).base__.AddChangesEventHandler)(::windows::core::Vtable::as_raw(self), element.into().abi(), scope, ::core::mem::transmute(changetypes.as_ptr()), changetypes.len() as _, pcacherequest.into().abi(), handler.into().abi()).ok()
    }
    pub unsafe fn RemoveChangesEventHandler<P0, P1>(&self, element: P0, handler: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationChangesEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).base__.RemoveChangesEventHandler)(::windows::core::Vtable::as_raw(self), element.into().abi(), handler.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IUIAutomation6 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomation6 {}
impl ::core::fmt::Debug for IUIAutomation6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomation6").field(&self.0).finish()
    }
}
impl IUIAutomation6 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CompareElements<P0, P1>(&self, el1: P0, el2: P1) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CompareElements)(::windows::core::Vtable::as_raw(self), el1.into().abi(), el2.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CompareRuntimeIds(&self, runtimeid1: *const super::super::System::Com::SAFEARRAY, runtimeid2: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CompareRuntimeIds)(::windows::core::Vtable::as_raw(self), runtimeid1, runtimeid2, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetRootElement(&self) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetRootElement)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ElementFromHandle<P0>(&self, hwnd: P0) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.ElementFromHandle)(::windows::core::Vtable::as_raw(self), hwnd.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ElementFromPoint(&self, pt: super::super::Foundation::POINT) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.ElementFromPoint)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pt), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFocusedElement(&self) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetFocusedElement)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetRootElementBuildCache<P0>(&self, cacherequest: P0) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetRootElementBuildCache)(::windows::core::Vtable::as_raw(self), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ElementFromHandleBuildCache<P0, P1>(&self, hwnd: P0, cacherequest: P1) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.ElementFromHandleBuildCache)(::windows::core::Vtable::as_raw(self), hwnd.into(), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ElementFromPointBuildCache<P0>(&self, pt: super::super::Foundation::POINT, cacherequest: P0) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.ElementFromPointBuildCache)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pt), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFocusedElementBuildCache<P0>(&self, cacherequest: P0) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetFocusedElementBuildCache)(::windows::core::Vtable::as_raw(self), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateTreeWalker<P0>(&self, pcondition: P0) -> ::windows::core::Result<IUIAutomationTreeWalker>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateTreeWalker)(::windows::core::Vtable::as_raw(self), pcondition.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ControlViewWalker(&self) -> ::windows::core::Result<IUIAutomationTreeWalker> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.ControlViewWalker)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ContentViewWalker(&self) -> ::windows::core::Result<IUIAutomationTreeWalker> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.ContentViewWalker)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RawViewWalker(&self) -> ::windows::core::Result<IUIAutomationTreeWalker> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.RawViewWalker)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RawViewCondition(&self) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.RawViewCondition)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ControlViewCondition(&self) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.ControlViewCondition)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ContentViewCondition(&self) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.ContentViewCondition)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCacheRequest(&self) -> ::windows::core::Result<IUIAutomationCacheRequest> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateCacheRequest)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateTrueCondition(&self) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateTrueCondition)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateFalseCondition(&self) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateFalseCondition)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreatePropertyCondition(&self, propertyid: UIA_PROPERTY_ID, value: super::super::System::Com::VARIANT) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreatePropertyCondition)(::windows::core::Vtable::as_raw(self), propertyid, ::core::mem::transmute(value), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreatePropertyConditionEx(&self, propertyid: UIA_PROPERTY_ID, value: super::super::System::Com::VARIANT, flags: PropertyConditionFlags) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreatePropertyConditionEx)(::windows::core::Vtable::as_raw(self), propertyid, ::core::mem::transmute(value), flags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateAndCondition<P0, P1>(&self, condition1: P0, condition2: P1) -> ::windows::core::Result<IUIAutomationCondition>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateAndCondition)(::windows::core::Vtable::as_raw(self), condition1.into().abi(), condition2.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateAndConditionFromArray(&self, conditions: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateAndConditionFromArray)(::windows::core::Vtable::as_raw(self), conditions, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateAndConditionFromNativeArray(&self, conditions: &[IUIAutomationCondition]) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateAndConditionFromNativeArray)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(conditions.as_ptr()), conditions.len() as _, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateOrCondition<P0, P1>(&self, condition1: P0, condition2: P1) -> ::windows::core::Result<IUIAutomationCondition>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateOrCondition)(::windows::core::Vtable::as_raw(self), condition1.into().abi(), condition2.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateOrConditionFromArray(&self, conditions: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateOrConditionFromArray)(::windows::core::Vtable::as_raw(self), conditions, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateOrConditionFromNativeArray(&self, conditions: &[IUIAutomationCondition]) -> ::windows::core::Result<IUIAutomationCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateOrConditionFromNativeArray)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(conditions.as_ptr()), conditions.len() as _, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateNotCondition<P0>(&self, condition: P0) -> ::windows::core::Result<IUIAutomationCondition>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateNotCondition)(::windows::core::Vtable::as_raw(self), condition.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AddAutomationEventHandler<P0, P1, P2>(&self, eventid: UIA_EVENT_ID, element: P0, scope: TreeScope, cacherequest: P1, handler: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
        P2: ::std::convert::Into<::windows::core::InParam<IUIAutomationEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.AddAutomationEventHandler)(::windows::core::Vtable::as_raw(self), eventid, element.into().abi(), scope, cacherequest.into().abi(), handler.into().abi()).ok()
    }
    pub unsafe fn RemoveAutomationEventHandler<P0, P1>(&self, eventid: UIA_EVENT_ID, element: P0, handler: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.RemoveAutomationEventHandler)(::windows::core::Vtable::as_raw(self), eventid, element.into().abi(), handler.into().abi()).ok()
    }
    pub unsafe fn AddPropertyChangedEventHandlerNativeArray<P0, P1, P2>(&self, element: P0, scope: TreeScope, cacherequest: P1, handler: P2, propertyarray: &[UIA_PROPERTY_ID]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
        P2: ::std::convert::Into<::windows::core::InParam<IUIAutomationPropertyChangedEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.AddPropertyChangedEventHandlerNativeArray)(::windows::core::Vtable::as_raw(self), element.into().abi(), scope, cacherequest.into().abi(), handler.into().abi(), ::core::mem::transmute(propertyarray.as_ptr()), propertyarray.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddPropertyChangedEventHandler<P0, P1, P2>(&self, element: P0, scope: TreeScope, cacherequest: P1, handler: P2, propertyarray: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
        P2: ::std::convert::Into<::windows::core::InParam<IUIAutomationPropertyChangedEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.AddPropertyChangedEventHandler)(::windows::core::Vtable::as_raw(self), element.into().abi(), scope, cacherequest.into().abi(), handler.into().abi(), propertyarray).ok()
    }
    pub unsafe fn RemovePropertyChangedEventHandler<P0, P1>(&self, element: P0, handler: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationPropertyChangedEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.RemovePropertyChangedEventHandler)(::windows::core::Vtable::as_raw(self), element.into().abi(), handler.into().abi()).ok()
    }
    pub unsafe fn AddStructureChangedEventHandler<P0, P1, P2>(&self, element: P0, scope: TreeScope, cacherequest: P1, handler: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
        P2: ::std::convert::Into<::windows::core::InParam<IUIAutomationStructureChangedEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.AddStructureChangedEventHandler)(::windows::core::Vtable::as_raw(self), element.into().abi(), scope, cacherequest.into().abi(), handler.into().abi()).ok()
    }
    pub unsafe fn RemoveStructureChangedEventHandler<P0, P1>(&self, element: P0, handler: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationStructureChangedEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.RemoveStructureChangedEventHandler)(::windows::core::Vtable::as_raw(self), element.into().abi(), handler.into().abi()).ok()
    }
    pub unsafe fn AddFocusChangedEventHandler<P0, P1>(&self, cacherequest: P0, handler: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationFocusChangedEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.AddFocusChangedEventHandler)(::windows::core::Vtable::as_raw(self), cacherequest.into().abi(), handler.into().abi()).ok()
    }
    pub unsafe fn RemoveFocusChangedEventHandler<P0>(&self, handler: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationFocusChangedEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.RemoveFocusChangedEventHandler)(::windows::core::Vtable::as_raw(self), handler.into().abi()).ok()
    }
    pub unsafe fn RemoveAllEventHandlers(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.RemoveAllEventHandlers)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IntNativeArrayToSafeArray(&self, array: &[i32]) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.IntNativeArrayToSafeArray)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(array.as_ptr()), array.len() as _, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IntSafeArrayToNativeArray(&self, intarray: *const super::super::System::Com::SAFEARRAY, array: *mut *mut i32, arraycount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.IntSafeArrayToNativeArray)(::windows::core::Vtable::as_raw(self), intarray, array, arraycount).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn RectToVariant(&self, rc: super::super::Foundation::RECT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.RectToVariant)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(rc), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn VariantToRect(&self, var: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.VariantToRect)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(var), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SafeArrayToRectNativeArray(&self, rects: *const super::super::System::Com::SAFEARRAY, rectarray: *mut *mut super::super::Foundation::RECT, rectarraycount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SafeArrayToRectNativeArray)(::windows::core::Vtable::as_raw(self), rects, rectarray, rectarraycount).ok()
    }
    pub unsafe fn CreateProxyFactoryEntry<P0>(&self, factory: P0) -> ::windows::core::Result<IUIAutomationProxyFactoryEntry>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationProxyFactory>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateProxyFactoryEntry)(::windows::core::Vtable::as_raw(self), factory.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ProxyFactoryMapping(&self) -> ::windows::core::Result<IUIAutomationProxyFactoryMapping> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.ProxyFactoryMapping)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPropertyProgrammaticName(&self, property: UIA_PROPERTY_ID) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetPropertyProgrammaticName)(::windows::core::Vtable::as_raw(self), property, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPatternProgrammaticName(&self, pattern: UIA_PATTERN_ID) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetPatternProgrammaticName)(::windows::core::Vtable::as_raw(self), pattern, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PollForPotentialSupportedPatterns<P0>(&self, pelement: P0, patternids: *mut *mut super::super::System::Com::SAFEARRAY, patternnames: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.PollForPotentialSupportedPatterns)(::windows::core::Vtable::as_raw(self), pelement.into().abi(), patternids, patternnames).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PollForPotentialSupportedProperties<P0>(&self, pelement: P0, propertyids: *mut *mut super::super::System::Com::SAFEARRAY, propertynames: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.PollForPotentialSupportedProperties)(::windows::core::Vtable::as_raw(self), pelement.into().abi(), propertyids, propertynames).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CheckNotSupported(&self, value: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CheckNotSupported)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(value), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ReservedNotSupportedValue(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.ReservedNotSupportedValue)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ReservedMixedAttributeValue(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.ReservedMixedAttributeValue)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ElementFromIAccessible<P0>(&self, accessible: P0, childid: i32) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IAccessible>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.ElementFromIAccessible)(::windows::core::Vtable::as_raw(self), accessible.into().abi(), childid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ElementFromIAccessibleBuildCache<P0, P1>(&self, accessible: P0, childid: i32, cacherequest: P1) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IAccessible>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.ElementFromIAccessibleBuildCache)(::windows::core::Vtable::as_raw(self), accessible.into().abi(), childid, cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AutoSetFocus(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.AutoSetFocus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAutoSetFocus<P0>(&self, autosetfocus: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetAutoSetFocus)(::windows::core::Vtable::as_raw(self), autosetfocus.into()).ok()
    }
    pub unsafe fn ConnectionTimeout(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.ConnectionTimeout)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetConnectionTimeout(&self, timeout: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetConnectionTimeout)(::windows::core::Vtable::as_raw(self), timeout).ok()
    }
    pub unsafe fn TransactionTimeout(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.TransactionTimeout)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetTransactionTimeout(&self, timeout: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetTransactionTimeout)(::windows::core::Vtable::as_raw(self), timeout).ok()
    }
    pub unsafe fn AddTextEditTextChangedEventHandler<P0, P1, P2>(&self, element: P0, scope: TreeScope, texteditchangetype: TextEditChangeType, cacherequest: P1, handler: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
        P2: ::std::convert::Into<::windows::core::InParam<IUIAutomationTextEditTextChangedEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.AddTextEditTextChangedEventHandler)(::windows::core::Vtable::as_raw(self), element.into().abi(), scope, texteditchangetype, cacherequest.into().abi(), handler.into().abi()).ok()
    }
    pub unsafe fn RemoveTextEditTextChangedEventHandler<P0, P1>(&self, element: P0, handler: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationTextEditTextChangedEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.RemoveTextEditTextChangedEventHandler)(::windows::core::Vtable::as_raw(self), element.into().abi(), handler.into().abi()).ok()
    }
    pub unsafe fn AddChangesEventHandler<P0, P1, P2>(&self, element: P0, scope: TreeScope, changetypes: &[i32], pcacherequest: P1, handler: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
        P2: ::std::convert::Into<::windows::core::InParam<IUIAutomationChangesEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.AddChangesEventHandler)(::windows::core::Vtable::as_raw(self), element.into().abi(), scope, ::core::mem::transmute(changetypes.as_ptr()), changetypes.len() as _, pcacherequest.into().abi(), handler.into().abi()).ok()
    }
    pub unsafe fn RemoveChangesEventHandler<P0, P1>(&self, element: P0, handler: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationChangesEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.RemoveChangesEventHandler)(::windows::core::Vtable::as_raw(self), element.into().abi(), handler.into().abi()).ok()
    }
    pub unsafe fn AddNotificationEventHandler<P0, P1, P2>(&self, element: P0, scope: TreeScope, cacherequest: P1, handler: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
        P2: ::std::convert::Into<::windows::core::InParam<IUIAutomationNotificationEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).base__.AddNotificationEventHandler)(::windows::core::Vtable::as_raw(self), element.into().abi(), scope, cacherequest.into().abi(), handler.into().abi()).ok()
    }
    pub unsafe fn RemoveNotificationEventHandler<P0, P1>(&self, element: P0, handler: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationNotificationEventHandler>>,
    {
        (::windows::core::Vtable::vtable(self).base__.RemoveNotificationEventHandler)(::windows::core::Vtable::as_raw(self), element.into().abi(), handler.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationActiveTextPositionChangedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationActiveTextPositionChangedEventHandler {}
impl ::core::fmt::Debug for IUIAutomationActiveTextPositionChangedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationActiveTextPositionChangedEventHandler").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationAndCondition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationAndCondition {}
impl ::core::fmt::Debug for IUIAutomationAndCondition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationAndCondition").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationAnnotationPattern {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationAnnotationPattern {}
impl ::core::fmt::Debug for IUIAutomationAnnotationPattern {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationAnnotationPattern").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationBoolCondition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationBoolCondition {}
impl ::core::fmt::Debug for IUIAutomationBoolCondition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationBoolCondition").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationCacheRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationCacheRequest {}
impl ::core::fmt::Debug for IUIAutomationCacheRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationCacheRequest").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationChangesEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationChangesEventHandler {}
impl ::core::fmt::Debug for IUIAutomationChangesEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationChangesEventHandler").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationCondition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationCondition {}
impl ::core::fmt::Debug for IUIAutomationCondition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationCondition").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationCustomNavigationPattern {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationCustomNavigationPattern {}
impl ::core::fmt::Debug for IUIAutomationCustomNavigationPattern {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationCustomNavigationPattern").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationDockPattern {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationDockPattern {}
impl ::core::fmt::Debug for IUIAutomationDockPattern {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationDockPattern").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationDragPattern {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationDragPattern {}
impl ::core::fmt::Debug for IUIAutomationDragPattern {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationDragPattern").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationDropTargetPattern {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationDropTargetPattern {}
impl ::core::fmt::Debug for IUIAutomationDropTargetPattern {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationDropTargetPattern").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationElement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationElement {}
impl ::core::fmt::Debug for IUIAutomationElement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationElement").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationElement2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationElement2 {}
impl ::core::fmt::Debug for IUIAutomationElement2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationElement2").field(&self.0).finish()
    }
}
impl IUIAutomationElement2 {
    pub unsafe fn SetFocus(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetFocus)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRuntimeId(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetRuntimeId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindFirst<P0>(&self, scope: TreeScope, condition: P0) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.FindFirst)(::windows::core::Vtable::as_raw(self), scope, condition.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindAll<P0>(&self, scope: TreeScope, condition: P0) -> ::windows::core::Result<IUIAutomationElementArray>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.FindAll)(::windows::core::Vtable::as_raw(self), scope, condition.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindFirstBuildCache<P0, P1>(&self, scope: TreeScope, condition: P0, cacherequest: P1) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.FindFirstBuildCache)(::windows::core::Vtable::as_raw(self), scope, condition.into().abi(), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindAllBuildCache<P0, P1>(&self, scope: TreeScope, condition: P0, cacherequest: P1) -> ::windows::core::Result<IUIAutomationElementArray>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.FindAllBuildCache)(::windows::core::Vtable::as_raw(self), scope, condition.into().abi(), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn BuildUpdatedCache<P0>(&self, cacherequest: P0) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.BuildUpdatedCache)(::windows::core::Vtable::as_raw(self), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetCurrentPropertyValue(&self, propertyid: UIA_PROPERTY_ID) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCurrentPropertyValue)(::windows::core::Vtable::as_raw(self), propertyid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetCurrentPropertyValueEx<P0>(&self, propertyid: UIA_PROPERTY_ID, ignoredefaultvalue: P0) -> ::windows::core::Result<super::super::System::Com::VARIANT>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCurrentPropertyValueEx)(::windows::core::Vtable::as_raw(self), propertyid, ignoredefaultvalue.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetCachedPropertyValue(&self, propertyid: UIA_PROPERTY_ID) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCachedPropertyValue)(::windows::core::Vtable::as_raw(self), propertyid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetCachedPropertyValueEx<P0>(&self, propertyid: UIA_PROPERTY_ID, ignoredefaultvalue: P0) -> ::windows::core::Result<super::super::System::Com::VARIANT>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCachedPropertyValueEx)(::windows::core::Vtable::as_raw(self), propertyid, ignoredefaultvalue.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCurrentPatternAs<T>(&self, patternid: UIA_PATTERN_ID) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCurrentPatternAs)(::windows::core::Vtable::as_raw(self), patternid, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCachedPatternAs<T>(&self, patternid: UIA_PATTERN_ID) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCachedPatternAs)(::windows::core::Vtable::as_raw(self), patternid, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCurrentPattern(&self, patternid: UIA_PATTERN_ID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCurrentPattern)(::windows::core::Vtable::as_raw(self), patternid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCachedPattern(&self, patternid: UIA_PATTERN_ID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCachedPattern)(::windows::core::Vtable::as_raw(self), patternid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCachedParent(&self) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCachedParent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCachedChildren(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCachedChildren)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentProcessId(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CurrentProcessId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentControlType(&self) -> ::windows::core::Result<UIA_CONTROLTYPE_ID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CurrentControlType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentLocalizedControlType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CurrentLocalizedControlType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CurrentName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentAcceleratorKey(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CurrentAcceleratorKey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentAccessKey(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CurrentAccessKey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentHasKeyboardFocus(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CurrentHasKeyboardFocus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsKeyboardFocusable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CurrentIsKeyboardFocusable)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsEnabled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CurrentIsEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentAutomationId(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CurrentAutomationId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentClassName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CurrentClassName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentHelpText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CurrentHelpText)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentCulture(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CurrentCulture)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsControlElement(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CurrentIsControlElement)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsContentElement(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CurrentIsContentElement)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsPassword(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CurrentIsPassword)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentNativeWindowHandle(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CurrentNativeWindowHandle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentItemType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CurrentItemType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsOffscreen(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CurrentIsOffscreen)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentOrientation(&self) -> ::windows::core::Result<OrientationType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CurrentOrientation)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentFrameworkId(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CurrentFrameworkId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsRequiredForForm(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CurrentIsRequiredForForm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentItemStatus(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CurrentItemStatus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentBoundingRectangle(&self) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CurrentBoundingRectangle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentLabeledBy(&self) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CurrentLabeledBy)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentAriaRole(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CurrentAriaRole)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentAriaProperties(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CurrentAriaProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsDataValidForForm(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CurrentIsDataValidForForm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentControllerFor(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CurrentControllerFor)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentDescribedBy(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CurrentDescribedBy)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentFlowsTo(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CurrentFlowsTo)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentProviderDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CurrentProviderDescription)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedProcessId(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CachedProcessId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedControlType(&self) -> ::windows::core::Result<UIA_CONTROLTYPE_ID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CachedControlType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedLocalizedControlType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CachedLocalizedControlType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CachedName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedAcceleratorKey(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CachedAcceleratorKey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedAccessKey(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CachedAccessKey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedHasKeyboardFocus(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CachedHasKeyboardFocus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsKeyboardFocusable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CachedIsKeyboardFocusable)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsEnabled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CachedIsEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedAutomationId(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CachedAutomationId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedClassName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CachedClassName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedHelpText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CachedHelpText)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedCulture(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CachedCulture)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsControlElement(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CachedIsControlElement)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsContentElement(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CachedIsContentElement)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsPassword(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CachedIsPassword)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedNativeWindowHandle(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CachedNativeWindowHandle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedItemType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CachedItemType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsOffscreen(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CachedIsOffscreen)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedOrientation(&self) -> ::windows::core::Result<OrientationType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CachedOrientation)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedFrameworkId(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CachedFrameworkId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsRequiredForForm(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CachedIsRequiredForForm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedItemStatus(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CachedItemStatus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedBoundingRectangle(&self) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CachedBoundingRectangle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedLabeledBy(&self) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CachedLabeledBy)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedAriaRole(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CachedAriaRole)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedAriaProperties(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CachedAriaProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsDataValidForForm(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CachedIsDataValidForForm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedControllerFor(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CachedControllerFor)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedDescribedBy(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CachedDescribedBy)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedFlowsTo(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CachedFlowsTo)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedProviderDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CachedProviderDescription)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetClickablePoint(&self, clickable: *mut super::super::Foundation::POINT, gotclickable: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetClickablePoint)(::windows::core::Vtable::as_raw(self), clickable, gotclickable).ok()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationElement3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationElement3 {}
impl ::core::fmt::Debug for IUIAutomationElement3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationElement3").field(&self.0).finish()
    }
}
impl IUIAutomationElement3 {
    pub unsafe fn SetFocus(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetFocus)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRuntimeId(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetRuntimeId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindFirst<P0>(&self, scope: TreeScope, condition: P0) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.FindFirst)(::windows::core::Vtable::as_raw(self), scope, condition.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindAll<P0>(&self, scope: TreeScope, condition: P0) -> ::windows::core::Result<IUIAutomationElementArray>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.FindAll)(::windows::core::Vtable::as_raw(self), scope, condition.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindFirstBuildCache<P0, P1>(&self, scope: TreeScope, condition: P0, cacherequest: P1) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.FindFirstBuildCache)(::windows::core::Vtable::as_raw(self), scope, condition.into().abi(), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindAllBuildCache<P0, P1>(&self, scope: TreeScope, condition: P0, cacherequest: P1) -> ::windows::core::Result<IUIAutomationElementArray>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.FindAllBuildCache)(::windows::core::Vtable::as_raw(self), scope, condition.into().abi(), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn BuildUpdatedCache<P0>(&self, cacherequest: P0) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.BuildUpdatedCache)(::windows::core::Vtable::as_raw(self), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetCurrentPropertyValue(&self, propertyid: UIA_PROPERTY_ID) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetCurrentPropertyValue)(::windows::core::Vtable::as_raw(self), propertyid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetCurrentPropertyValueEx<P0>(&self, propertyid: UIA_PROPERTY_ID, ignoredefaultvalue: P0) -> ::windows::core::Result<super::super::System::Com::VARIANT>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetCurrentPropertyValueEx)(::windows::core::Vtable::as_raw(self), propertyid, ignoredefaultvalue.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetCachedPropertyValue(&self, propertyid: UIA_PROPERTY_ID) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetCachedPropertyValue)(::windows::core::Vtable::as_raw(self), propertyid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetCachedPropertyValueEx<P0>(&self, propertyid: UIA_PROPERTY_ID, ignoredefaultvalue: P0) -> ::windows::core::Result<super::super::System::Com::VARIANT>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetCachedPropertyValueEx)(::windows::core::Vtable::as_raw(self), propertyid, ignoredefaultvalue.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCurrentPatternAs<T>(&self, patternid: UIA_PATTERN_ID) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetCurrentPatternAs)(::windows::core::Vtable::as_raw(self), patternid, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCachedPatternAs<T>(&self, patternid: UIA_PATTERN_ID) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetCachedPatternAs)(::windows::core::Vtable::as_raw(self), patternid, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCurrentPattern(&self, patternid: UIA_PATTERN_ID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetCurrentPattern)(::windows::core::Vtable::as_raw(self), patternid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCachedPattern(&self, patternid: UIA_PATTERN_ID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetCachedPattern)(::windows::core::Vtable::as_raw(self), patternid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCachedParent(&self) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetCachedParent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCachedChildren(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetCachedChildren)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentProcessId(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CurrentProcessId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentControlType(&self) -> ::windows::core::Result<UIA_CONTROLTYPE_ID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CurrentControlType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentLocalizedControlType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CurrentLocalizedControlType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CurrentName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentAcceleratorKey(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CurrentAcceleratorKey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentAccessKey(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CurrentAccessKey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentHasKeyboardFocus(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CurrentHasKeyboardFocus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsKeyboardFocusable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CurrentIsKeyboardFocusable)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsEnabled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CurrentIsEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentAutomationId(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CurrentAutomationId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentClassName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CurrentClassName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentHelpText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CurrentHelpText)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentCulture(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CurrentCulture)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsControlElement(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CurrentIsControlElement)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsContentElement(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CurrentIsContentElement)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsPassword(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CurrentIsPassword)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentNativeWindowHandle(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CurrentNativeWindowHandle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentItemType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CurrentItemType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsOffscreen(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CurrentIsOffscreen)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentOrientation(&self) -> ::windows::core::Result<OrientationType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CurrentOrientation)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentFrameworkId(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CurrentFrameworkId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsRequiredForForm(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CurrentIsRequiredForForm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentItemStatus(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CurrentItemStatus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentBoundingRectangle(&self) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CurrentBoundingRectangle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentLabeledBy(&self) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CurrentLabeledBy)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentAriaRole(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CurrentAriaRole)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentAriaProperties(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CurrentAriaProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsDataValidForForm(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CurrentIsDataValidForForm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentControllerFor(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CurrentControllerFor)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentDescribedBy(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CurrentDescribedBy)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentFlowsTo(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CurrentFlowsTo)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentProviderDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CurrentProviderDescription)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedProcessId(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CachedProcessId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedControlType(&self) -> ::windows::core::Result<UIA_CONTROLTYPE_ID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CachedControlType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedLocalizedControlType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CachedLocalizedControlType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CachedName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedAcceleratorKey(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CachedAcceleratorKey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedAccessKey(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CachedAccessKey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedHasKeyboardFocus(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CachedHasKeyboardFocus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsKeyboardFocusable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CachedIsKeyboardFocusable)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsEnabled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CachedIsEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedAutomationId(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CachedAutomationId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedClassName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CachedClassName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedHelpText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CachedHelpText)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedCulture(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CachedCulture)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsControlElement(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CachedIsControlElement)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsContentElement(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CachedIsContentElement)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsPassword(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CachedIsPassword)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedNativeWindowHandle(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CachedNativeWindowHandle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedItemType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CachedItemType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsOffscreen(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CachedIsOffscreen)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedOrientation(&self) -> ::windows::core::Result<OrientationType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CachedOrientation)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedFrameworkId(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CachedFrameworkId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsRequiredForForm(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CachedIsRequiredForForm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedItemStatus(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CachedItemStatus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedBoundingRectangle(&self) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CachedBoundingRectangle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedLabeledBy(&self) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CachedLabeledBy)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedAriaRole(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CachedAriaRole)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedAriaProperties(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CachedAriaProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsDataValidForForm(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CachedIsDataValidForForm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedControllerFor(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CachedControllerFor)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedDescribedBy(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CachedDescribedBy)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedFlowsTo(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CachedFlowsTo)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedProviderDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CachedProviderDescription)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetClickablePoint(&self, clickable: *mut super::super::Foundation::POINT, gotclickable: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetClickablePoint)(::windows::core::Vtable::as_raw(self), clickable, gotclickable).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentOptimizeForVisualContent(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CurrentOptimizeForVisualContent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedOptimizeForVisualContent(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CachedOptimizeForVisualContent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentLiveSetting(&self) -> ::windows::core::Result<LiveSetting> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CurrentLiveSetting)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedLiveSetting(&self) -> ::windows::core::Result<LiveSetting> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CachedLiveSetting)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentFlowsFrom(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CurrentFlowsFrom)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedFlowsFrom(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CachedFlowsFrom)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IUIAutomationElement4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationElement4 {}
impl ::core::fmt::Debug for IUIAutomationElement4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationElement4").field(&self.0).finish()
    }
}
impl IUIAutomationElement4 {
    pub unsafe fn SetFocus(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetFocus)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRuntimeId(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetRuntimeId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindFirst<P0>(&self, scope: TreeScope, condition: P0) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.FindFirst)(::windows::core::Vtable::as_raw(self), scope, condition.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindAll<P0>(&self, scope: TreeScope, condition: P0) -> ::windows::core::Result<IUIAutomationElementArray>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.FindAll)(::windows::core::Vtable::as_raw(self), scope, condition.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindFirstBuildCache<P0, P1>(&self, scope: TreeScope, condition: P0, cacherequest: P1) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.FindFirstBuildCache)(::windows::core::Vtable::as_raw(self), scope, condition.into().abi(), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindAllBuildCache<P0, P1>(&self, scope: TreeScope, condition: P0, cacherequest: P1) -> ::windows::core::Result<IUIAutomationElementArray>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.FindAllBuildCache)(::windows::core::Vtable::as_raw(self), scope, condition.into().abi(), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn BuildUpdatedCache<P0>(&self, cacherequest: P0) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.BuildUpdatedCache)(::windows::core::Vtable::as_raw(self), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetCurrentPropertyValue(&self, propertyid: UIA_PROPERTY_ID) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetCurrentPropertyValue)(::windows::core::Vtable::as_raw(self), propertyid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetCurrentPropertyValueEx<P0>(&self, propertyid: UIA_PROPERTY_ID, ignoredefaultvalue: P0) -> ::windows::core::Result<super::super::System::Com::VARIANT>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetCurrentPropertyValueEx)(::windows::core::Vtable::as_raw(self), propertyid, ignoredefaultvalue.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetCachedPropertyValue(&self, propertyid: UIA_PROPERTY_ID) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetCachedPropertyValue)(::windows::core::Vtable::as_raw(self), propertyid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetCachedPropertyValueEx<P0>(&self, propertyid: UIA_PROPERTY_ID, ignoredefaultvalue: P0) -> ::windows::core::Result<super::super::System::Com::VARIANT>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetCachedPropertyValueEx)(::windows::core::Vtable::as_raw(self), propertyid, ignoredefaultvalue.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCurrentPatternAs<T>(&self, patternid: UIA_PATTERN_ID) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetCurrentPatternAs)(::windows::core::Vtable::as_raw(self), patternid, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCachedPatternAs<T>(&self, patternid: UIA_PATTERN_ID) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetCachedPatternAs)(::windows::core::Vtable::as_raw(self), patternid, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCurrentPattern(&self, patternid: UIA_PATTERN_ID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetCurrentPattern)(::windows::core::Vtable::as_raw(self), patternid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCachedPattern(&self, patternid: UIA_PATTERN_ID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetCachedPattern)(::windows::core::Vtable::as_raw(self), patternid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCachedParent(&self) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetCachedParent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCachedChildren(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetCachedChildren)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentProcessId(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CurrentProcessId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentControlType(&self) -> ::windows::core::Result<UIA_CONTROLTYPE_ID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CurrentControlType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentLocalizedControlType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CurrentLocalizedControlType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CurrentName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentAcceleratorKey(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CurrentAcceleratorKey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentAccessKey(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CurrentAccessKey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentHasKeyboardFocus(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CurrentHasKeyboardFocus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsKeyboardFocusable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CurrentIsKeyboardFocusable)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsEnabled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CurrentIsEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentAutomationId(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CurrentAutomationId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentClassName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CurrentClassName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentHelpText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CurrentHelpText)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentCulture(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CurrentCulture)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsControlElement(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CurrentIsControlElement)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsContentElement(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CurrentIsContentElement)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsPassword(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CurrentIsPassword)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentNativeWindowHandle(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CurrentNativeWindowHandle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentItemType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CurrentItemType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsOffscreen(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CurrentIsOffscreen)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentOrientation(&self) -> ::windows::core::Result<OrientationType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CurrentOrientation)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentFrameworkId(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CurrentFrameworkId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsRequiredForForm(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CurrentIsRequiredForForm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentItemStatus(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CurrentItemStatus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentBoundingRectangle(&self) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CurrentBoundingRectangle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentLabeledBy(&self) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CurrentLabeledBy)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentAriaRole(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CurrentAriaRole)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentAriaProperties(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CurrentAriaProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsDataValidForForm(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CurrentIsDataValidForForm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentControllerFor(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CurrentControllerFor)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentDescribedBy(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CurrentDescribedBy)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentFlowsTo(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CurrentFlowsTo)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentProviderDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CurrentProviderDescription)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedProcessId(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CachedProcessId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedControlType(&self) -> ::windows::core::Result<UIA_CONTROLTYPE_ID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CachedControlType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedLocalizedControlType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CachedLocalizedControlType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CachedName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedAcceleratorKey(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CachedAcceleratorKey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedAccessKey(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CachedAccessKey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedHasKeyboardFocus(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CachedHasKeyboardFocus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsKeyboardFocusable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CachedIsKeyboardFocusable)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsEnabled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CachedIsEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedAutomationId(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CachedAutomationId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedClassName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CachedClassName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedHelpText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CachedHelpText)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedCulture(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CachedCulture)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsControlElement(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CachedIsControlElement)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsContentElement(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CachedIsContentElement)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsPassword(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CachedIsPassword)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedNativeWindowHandle(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CachedNativeWindowHandle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedItemType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CachedItemType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsOffscreen(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CachedIsOffscreen)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedOrientation(&self) -> ::windows::core::Result<OrientationType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CachedOrientation)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedFrameworkId(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CachedFrameworkId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsRequiredForForm(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CachedIsRequiredForForm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedItemStatus(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CachedItemStatus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedBoundingRectangle(&self) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CachedBoundingRectangle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedLabeledBy(&self) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CachedLabeledBy)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedAriaRole(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CachedAriaRole)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedAriaProperties(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CachedAriaProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsDataValidForForm(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CachedIsDataValidForForm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedControllerFor(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CachedControllerFor)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedDescribedBy(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CachedDescribedBy)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedFlowsTo(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CachedFlowsTo)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedProviderDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CachedProviderDescription)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetClickablePoint(&self, clickable: *mut super::super::Foundation::POINT, gotclickable: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetClickablePoint)(::windows::core::Vtable::as_raw(self), clickable, gotclickable).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentOptimizeForVisualContent(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CurrentOptimizeForVisualContent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedOptimizeForVisualContent(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CachedOptimizeForVisualContent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentLiveSetting(&self) -> ::windows::core::Result<LiveSetting> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CurrentLiveSetting)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedLiveSetting(&self) -> ::windows::core::Result<LiveSetting> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CachedLiveSetting)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentFlowsFrom(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CurrentFlowsFrom)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedFlowsFrom(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CachedFlowsFrom)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ShowContextMenu(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ShowContextMenu)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsPeripheral(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CurrentIsPeripheral)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsPeripheral(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CachedIsPeripheral)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IUIAutomationElement5 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationElement5 {}
impl ::core::fmt::Debug for IUIAutomationElement5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationElement5").field(&self.0).finish()
    }
}
impl IUIAutomationElement5 {
    pub unsafe fn SetFocus(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetFocus)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRuntimeId(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetRuntimeId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindFirst<P0>(&self, scope: TreeScope, condition: P0) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.FindFirst)(::windows::core::Vtable::as_raw(self), scope, condition.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindAll<P0>(&self, scope: TreeScope, condition: P0) -> ::windows::core::Result<IUIAutomationElementArray>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.FindAll)(::windows::core::Vtable::as_raw(self), scope, condition.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindFirstBuildCache<P0, P1>(&self, scope: TreeScope, condition: P0, cacherequest: P1) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.FindFirstBuildCache)(::windows::core::Vtable::as_raw(self), scope, condition.into().abi(), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindAllBuildCache<P0, P1>(&self, scope: TreeScope, condition: P0, cacherequest: P1) -> ::windows::core::Result<IUIAutomationElementArray>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.FindAllBuildCache)(::windows::core::Vtable::as_raw(self), scope, condition.into().abi(), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn BuildUpdatedCache<P0>(&self, cacherequest: P0) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.BuildUpdatedCache)(::windows::core::Vtable::as_raw(self), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetCurrentPropertyValue(&self, propertyid: UIA_PROPERTY_ID) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetCurrentPropertyValue)(::windows::core::Vtable::as_raw(self), propertyid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetCurrentPropertyValueEx<P0>(&self, propertyid: UIA_PROPERTY_ID, ignoredefaultvalue: P0) -> ::windows::core::Result<super::super::System::Com::VARIANT>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetCurrentPropertyValueEx)(::windows::core::Vtable::as_raw(self), propertyid, ignoredefaultvalue.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetCachedPropertyValue(&self, propertyid: UIA_PROPERTY_ID) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetCachedPropertyValue)(::windows::core::Vtable::as_raw(self), propertyid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetCachedPropertyValueEx<P0>(&self, propertyid: UIA_PROPERTY_ID, ignoredefaultvalue: P0) -> ::windows::core::Result<super::super::System::Com::VARIANT>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetCachedPropertyValueEx)(::windows::core::Vtable::as_raw(self), propertyid, ignoredefaultvalue.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCurrentPatternAs<T>(&self, patternid: UIA_PATTERN_ID) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetCurrentPatternAs)(::windows::core::Vtable::as_raw(self), patternid, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCachedPatternAs<T>(&self, patternid: UIA_PATTERN_ID) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetCachedPatternAs)(::windows::core::Vtable::as_raw(self), patternid, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCurrentPattern(&self, patternid: UIA_PATTERN_ID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetCurrentPattern)(::windows::core::Vtable::as_raw(self), patternid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCachedPattern(&self, patternid: UIA_PATTERN_ID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetCachedPattern)(::windows::core::Vtable::as_raw(self), patternid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCachedParent(&self) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetCachedParent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCachedChildren(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetCachedChildren)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentProcessId(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CurrentProcessId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentControlType(&self) -> ::windows::core::Result<UIA_CONTROLTYPE_ID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CurrentControlType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentLocalizedControlType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CurrentLocalizedControlType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CurrentName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentAcceleratorKey(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CurrentAcceleratorKey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentAccessKey(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CurrentAccessKey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentHasKeyboardFocus(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CurrentHasKeyboardFocus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsKeyboardFocusable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CurrentIsKeyboardFocusable)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsEnabled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CurrentIsEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentAutomationId(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CurrentAutomationId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentClassName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CurrentClassName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentHelpText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CurrentHelpText)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentCulture(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CurrentCulture)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsControlElement(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CurrentIsControlElement)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsContentElement(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CurrentIsContentElement)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsPassword(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CurrentIsPassword)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentNativeWindowHandle(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CurrentNativeWindowHandle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentItemType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CurrentItemType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsOffscreen(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CurrentIsOffscreen)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentOrientation(&self) -> ::windows::core::Result<OrientationType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CurrentOrientation)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentFrameworkId(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CurrentFrameworkId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsRequiredForForm(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CurrentIsRequiredForForm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentItemStatus(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CurrentItemStatus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentBoundingRectangle(&self) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CurrentBoundingRectangle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentLabeledBy(&self) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CurrentLabeledBy)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentAriaRole(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CurrentAriaRole)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentAriaProperties(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CurrentAriaProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsDataValidForForm(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CurrentIsDataValidForForm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentControllerFor(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CurrentControllerFor)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentDescribedBy(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CurrentDescribedBy)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentFlowsTo(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CurrentFlowsTo)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentProviderDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CurrentProviderDescription)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedProcessId(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CachedProcessId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedControlType(&self) -> ::windows::core::Result<UIA_CONTROLTYPE_ID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CachedControlType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedLocalizedControlType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CachedLocalizedControlType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CachedName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedAcceleratorKey(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CachedAcceleratorKey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedAccessKey(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CachedAccessKey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedHasKeyboardFocus(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CachedHasKeyboardFocus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsKeyboardFocusable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CachedIsKeyboardFocusable)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsEnabled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CachedIsEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedAutomationId(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CachedAutomationId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedClassName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CachedClassName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedHelpText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CachedHelpText)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedCulture(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CachedCulture)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsControlElement(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CachedIsControlElement)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsContentElement(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CachedIsContentElement)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsPassword(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CachedIsPassword)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedNativeWindowHandle(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CachedNativeWindowHandle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedItemType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CachedItemType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsOffscreen(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CachedIsOffscreen)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedOrientation(&self) -> ::windows::core::Result<OrientationType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CachedOrientation)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedFrameworkId(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CachedFrameworkId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsRequiredForForm(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CachedIsRequiredForForm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedItemStatus(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CachedItemStatus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedBoundingRectangle(&self) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CachedBoundingRectangle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedLabeledBy(&self) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CachedLabeledBy)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedAriaRole(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CachedAriaRole)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedAriaProperties(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CachedAriaProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsDataValidForForm(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CachedIsDataValidForForm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedControllerFor(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CachedControllerFor)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedDescribedBy(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CachedDescribedBy)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedFlowsTo(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CachedFlowsTo)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedProviderDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CachedProviderDescription)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetClickablePoint(&self, clickable: *mut super::super::Foundation::POINT, gotclickable: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetClickablePoint)(::windows::core::Vtable::as_raw(self), clickable, gotclickable).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentOptimizeForVisualContent(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CurrentOptimizeForVisualContent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedOptimizeForVisualContent(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CachedOptimizeForVisualContent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentLiveSetting(&self) -> ::windows::core::Result<LiveSetting> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CurrentLiveSetting)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedLiveSetting(&self) -> ::windows::core::Result<LiveSetting> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CachedLiveSetting)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentFlowsFrom(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CurrentFlowsFrom)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedFlowsFrom(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CachedFlowsFrom)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ShowContextMenu(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.ShowContextMenu)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsPeripheral(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CurrentIsPeripheral)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsPeripheral(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CachedIsPeripheral)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentPositionInSet(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CurrentPositionInSet)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentSizeOfSet(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CurrentSizeOfSet)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentLevel(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CurrentLevel)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CurrentAnnotationTypes(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CurrentAnnotationTypes)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentAnnotationObjects(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CurrentAnnotationObjects)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedPositionInSet(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CachedPositionInSet)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedSizeOfSet(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CachedSizeOfSet)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedLevel(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CachedLevel)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CachedAnnotationTypes(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CachedAnnotationTypes)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedAnnotationObjects(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CachedAnnotationObjects)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IUIAutomationElement6 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationElement6 {}
impl ::core::fmt::Debug for IUIAutomationElement6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationElement6").field(&self.0).finish()
    }
}
impl IUIAutomationElement6 {
    pub unsafe fn SetFocus(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetFocus)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRuntimeId(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetRuntimeId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindFirst<P0>(&self, scope: TreeScope, condition: P0) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.FindFirst)(::windows::core::Vtable::as_raw(self), scope, condition.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindAll<P0>(&self, scope: TreeScope, condition: P0) -> ::windows::core::Result<IUIAutomationElementArray>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.FindAll)(::windows::core::Vtable::as_raw(self), scope, condition.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindFirstBuildCache<P0, P1>(&self, scope: TreeScope, condition: P0, cacherequest: P1) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.FindFirstBuildCache)(::windows::core::Vtable::as_raw(self), scope, condition.into().abi(), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindAllBuildCache<P0, P1>(&self, scope: TreeScope, condition: P0, cacherequest: P1) -> ::windows::core::Result<IUIAutomationElementArray>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.FindAllBuildCache)(::windows::core::Vtable::as_raw(self), scope, condition.into().abi(), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn BuildUpdatedCache<P0>(&self, cacherequest: P0) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.BuildUpdatedCache)(::windows::core::Vtable::as_raw(self), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetCurrentPropertyValue(&self, propertyid: UIA_PROPERTY_ID) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetCurrentPropertyValue)(::windows::core::Vtable::as_raw(self), propertyid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetCurrentPropertyValueEx<P0>(&self, propertyid: UIA_PROPERTY_ID, ignoredefaultvalue: P0) -> ::windows::core::Result<super::super::System::Com::VARIANT>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetCurrentPropertyValueEx)(::windows::core::Vtable::as_raw(self), propertyid, ignoredefaultvalue.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetCachedPropertyValue(&self, propertyid: UIA_PROPERTY_ID) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetCachedPropertyValue)(::windows::core::Vtable::as_raw(self), propertyid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetCachedPropertyValueEx<P0>(&self, propertyid: UIA_PROPERTY_ID, ignoredefaultvalue: P0) -> ::windows::core::Result<super::super::System::Com::VARIANT>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetCachedPropertyValueEx)(::windows::core::Vtable::as_raw(self), propertyid, ignoredefaultvalue.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCurrentPatternAs<T>(&self, patternid: UIA_PATTERN_ID) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetCurrentPatternAs)(::windows::core::Vtable::as_raw(self), patternid, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCachedPatternAs<T>(&self, patternid: UIA_PATTERN_ID) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetCachedPatternAs)(::windows::core::Vtable::as_raw(self), patternid, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCurrentPattern(&self, patternid: UIA_PATTERN_ID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetCurrentPattern)(::windows::core::Vtable::as_raw(self), patternid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCachedPattern(&self, patternid: UIA_PATTERN_ID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetCachedPattern)(::windows::core::Vtable::as_raw(self), patternid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCachedParent(&self) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetCachedParent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCachedChildren(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetCachedChildren)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentProcessId(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CurrentProcessId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentControlType(&self) -> ::windows::core::Result<UIA_CONTROLTYPE_ID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CurrentControlType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentLocalizedControlType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CurrentLocalizedControlType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CurrentName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentAcceleratorKey(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CurrentAcceleratorKey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentAccessKey(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CurrentAccessKey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentHasKeyboardFocus(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CurrentHasKeyboardFocus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsKeyboardFocusable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CurrentIsKeyboardFocusable)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsEnabled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CurrentIsEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentAutomationId(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CurrentAutomationId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentClassName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CurrentClassName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentHelpText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CurrentHelpText)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentCulture(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CurrentCulture)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsControlElement(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CurrentIsControlElement)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsContentElement(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CurrentIsContentElement)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsPassword(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CurrentIsPassword)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentNativeWindowHandle(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CurrentNativeWindowHandle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentItemType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CurrentItemType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsOffscreen(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CurrentIsOffscreen)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentOrientation(&self) -> ::windows::core::Result<OrientationType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CurrentOrientation)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentFrameworkId(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CurrentFrameworkId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsRequiredForForm(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CurrentIsRequiredForForm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentItemStatus(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CurrentItemStatus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentBoundingRectangle(&self) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CurrentBoundingRectangle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentLabeledBy(&self) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CurrentLabeledBy)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentAriaRole(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CurrentAriaRole)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentAriaProperties(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CurrentAriaProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsDataValidForForm(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CurrentIsDataValidForForm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentControllerFor(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CurrentControllerFor)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentDescribedBy(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CurrentDescribedBy)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentFlowsTo(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CurrentFlowsTo)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentProviderDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CurrentProviderDescription)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedProcessId(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CachedProcessId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedControlType(&self) -> ::windows::core::Result<UIA_CONTROLTYPE_ID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CachedControlType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedLocalizedControlType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CachedLocalizedControlType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CachedName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedAcceleratorKey(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CachedAcceleratorKey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedAccessKey(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CachedAccessKey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedHasKeyboardFocus(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CachedHasKeyboardFocus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsKeyboardFocusable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CachedIsKeyboardFocusable)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsEnabled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CachedIsEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedAutomationId(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CachedAutomationId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedClassName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CachedClassName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedHelpText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CachedHelpText)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedCulture(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CachedCulture)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsControlElement(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CachedIsControlElement)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsContentElement(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CachedIsContentElement)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsPassword(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CachedIsPassword)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedNativeWindowHandle(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CachedNativeWindowHandle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedItemType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CachedItemType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsOffscreen(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CachedIsOffscreen)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedOrientation(&self) -> ::windows::core::Result<OrientationType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CachedOrientation)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedFrameworkId(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CachedFrameworkId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsRequiredForForm(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CachedIsRequiredForForm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedItemStatus(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CachedItemStatus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedBoundingRectangle(&self) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CachedBoundingRectangle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedLabeledBy(&self) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CachedLabeledBy)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedAriaRole(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CachedAriaRole)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedAriaProperties(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CachedAriaProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsDataValidForForm(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CachedIsDataValidForForm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedControllerFor(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CachedControllerFor)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedDescribedBy(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CachedDescribedBy)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedFlowsTo(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CachedFlowsTo)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedProviderDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CachedProviderDescription)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetClickablePoint(&self, clickable: *mut super::super::Foundation::POINT, gotclickable: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetClickablePoint)(::windows::core::Vtable::as_raw(self), clickable, gotclickable).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentOptimizeForVisualContent(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CurrentOptimizeForVisualContent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedOptimizeForVisualContent(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CachedOptimizeForVisualContent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentLiveSetting(&self) -> ::windows::core::Result<LiveSetting> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CurrentLiveSetting)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedLiveSetting(&self) -> ::windows::core::Result<LiveSetting> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CachedLiveSetting)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentFlowsFrom(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CurrentFlowsFrom)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedFlowsFrom(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CachedFlowsFrom)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ShowContextMenu(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ShowContextMenu)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsPeripheral(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CurrentIsPeripheral)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsPeripheral(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CachedIsPeripheral)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentPositionInSet(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CurrentPositionInSet)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentSizeOfSet(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CurrentSizeOfSet)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentLevel(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CurrentLevel)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CurrentAnnotationTypes(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CurrentAnnotationTypes)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentAnnotationObjects(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CurrentAnnotationObjects)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedPositionInSet(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CachedPositionInSet)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedSizeOfSet(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CachedSizeOfSet)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedLevel(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CachedLevel)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CachedAnnotationTypes(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CachedAnnotationTypes)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedAnnotationObjects(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CachedAnnotationObjects)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentLandmarkType(&self) -> ::windows::core::Result<UIA_LANDMARKTYPE_ID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CurrentLandmarkType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentLocalizedLandmarkType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CurrentLocalizedLandmarkType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedLandmarkType(&self) -> ::windows::core::Result<UIA_LANDMARKTYPE_ID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CachedLandmarkType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedLocalizedLandmarkType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CachedLocalizedLandmarkType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IUIAutomationElement7 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationElement7 {}
impl ::core::fmt::Debug for IUIAutomationElement7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationElement7").field(&self.0).finish()
    }
}
impl IUIAutomationElement7 {
    pub unsafe fn SetFocus(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.SetFocus)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRuntimeId(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetRuntimeId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindFirst<P0>(&self, scope: TreeScope, condition: P0) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.FindFirst)(::windows::core::Vtable::as_raw(self), scope, condition.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindAll<P0>(&self, scope: TreeScope, condition: P0) -> ::windows::core::Result<IUIAutomationElementArray>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.FindAll)(::windows::core::Vtable::as_raw(self), scope, condition.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindFirstBuildCache<P0, P1>(&self, scope: TreeScope, condition: P0, cacherequest: P1) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.FindFirstBuildCache)(::windows::core::Vtable::as_raw(self), scope, condition.into().abi(), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindAllBuildCache<P0, P1>(&self, scope: TreeScope, condition: P0, cacherequest: P1) -> ::windows::core::Result<IUIAutomationElementArray>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.FindAllBuildCache)(::windows::core::Vtable::as_raw(self), scope, condition.into().abi(), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn BuildUpdatedCache<P0>(&self, cacherequest: P0) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.BuildUpdatedCache)(::windows::core::Vtable::as_raw(self), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetCurrentPropertyValue(&self, propertyid: UIA_PROPERTY_ID) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetCurrentPropertyValue)(::windows::core::Vtable::as_raw(self), propertyid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetCurrentPropertyValueEx<P0>(&self, propertyid: UIA_PROPERTY_ID, ignoredefaultvalue: P0) -> ::windows::core::Result<super::super::System::Com::VARIANT>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetCurrentPropertyValueEx)(::windows::core::Vtable::as_raw(self), propertyid, ignoredefaultvalue.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetCachedPropertyValue(&self, propertyid: UIA_PROPERTY_ID) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetCachedPropertyValue)(::windows::core::Vtable::as_raw(self), propertyid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetCachedPropertyValueEx<P0>(&self, propertyid: UIA_PROPERTY_ID, ignoredefaultvalue: P0) -> ::windows::core::Result<super::super::System::Com::VARIANT>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetCachedPropertyValueEx)(::windows::core::Vtable::as_raw(self), propertyid, ignoredefaultvalue.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCurrentPatternAs<T>(&self, patternid: UIA_PATTERN_ID) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetCurrentPatternAs)(::windows::core::Vtable::as_raw(self), patternid, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCachedPatternAs<T>(&self, patternid: UIA_PATTERN_ID) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetCachedPatternAs)(::windows::core::Vtable::as_raw(self), patternid, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCurrentPattern(&self, patternid: UIA_PATTERN_ID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetCurrentPattern)(::windows::core::Vtable::as_raw(self), patternid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCachedPattern(&self, patternid: UIA_PATTERN_ID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetCachedPattern)(::windows::core::Vtable::as_raw(self), patternid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCachedParent(&self) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetCachedParent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCachedChildren(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetCachedChildren)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentProcessId(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CurrentProcessId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentControlType(&self) -> ::windows::core::Result<UIA_CONTROLTYPE_ID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CurrentControlType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentLocalizedControlType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CurrentLocalizedControlType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CurrentName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentAcceleratorKey(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CurrentAcceleratorKey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentAccessKey(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CurrentAccessKey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentHasKeyboardFocus(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CurrentHasKeyboardFocus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsKeyboardFocusable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CurrentIsKeyboardFocusable)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsEnabled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CurrentIsEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentAutomationId(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CurrentAutomationId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentClassName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CurrentClassName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentHelpText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CurrentHelpText)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentCulture(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CurrentCulture)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsControlElement(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CurrentIsControlElement)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsContentElement(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CurrentIsContentElement)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsPassword(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CurrentIsPassword)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentNativeWindowHandle(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CurrentNativeWindowHandle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentItemType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CurrentItemType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsOffscreen(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CurrentIsOffscreen)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentOrientation(&self) -> ::windows::core::Result<OrientationType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CurrentOrientation)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentFrameworkId(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CurrentFrameworkId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsRequiredForForm(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CurrentIsRequiredForForm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentItemStatus(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CurrentItemStatus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentBoundingRectangle(&self) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CurrentBoundingRectangle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentLabeledBy(&self) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CurrentLabeledBy)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentAriaRole(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CurrentAriaRole)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentAriaProperties(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CurrentAriaProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsDataValidForForm(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CurrentIsDataValidForForm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentControllerFor(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CurrentControllerFor)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentDescribedBy(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CurrentDescribedBy)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentFlowsTo(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CurrentFlowsTo)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentProviderDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CurrentProviderDescription)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedProcessId(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CachedProcessId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedControlType(&self) -> ::windows::core::Result<UIA_CONTROLTYPE_ID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CachedControlType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedLocalizedControlType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CachedLocalizedControlType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CachedName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedAcceleratorKey(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CachedAcceleratorKey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedAccessKey(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CachedAccessKey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedHasKeyboardFocus(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CachedHasKeyboardFocus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsKeyboardFocusable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CachedIsKeyboardFocusable)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsEnabled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CachedIsEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedAutomationId(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CachedAutomationId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedClassName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CachedClassName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedHelpText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CachedHelpText)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedCulture(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CachedCulture)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsControlElement(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CachedIsControlElement)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsContentElement(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CachedIsContentElement)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsPassword(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CachedIsPassword)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedNativeWindowHandle(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CachedNativeWindowHandle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedItemType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CachedItemType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsOffscreen(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CachedIsOffscreen)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedOrientation(&self) -> ::windows::core::Result<OrientationType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CachedOrientation)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedFrameworkId(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CachedFrameworkId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsRequiredForForm(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CachedIsRequiredForForm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedItemStatus(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CachedItemStatus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedBoundingRectangle(&self) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CachedBoundingRectangle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedLabeledBy(&self) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CachedLabeledBy)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedAriaRole(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CachedAriaRole)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedAriaProperties(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CachedAriaProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsDataValidForForm(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CachedIsDataValidForForm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedControllerFor(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CachedControllerFor)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedDescribedBy(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CachedDescribedBy)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedFlowsTo(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CachedFlowsTo)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedProviderDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CachedProviderDescription)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetClickablePoint(&self, clickable: *mut super::super::Foundation::POINT, gotclickable: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetClickablePoint)(::windows::core::Vtable::as_raw(self), clickable, gotclickable).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentOptimizeForVisualContent(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CurrentOptimizeForVisualContent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedOptimizeForVisualContent(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CachedOptimizeForVisualContent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentLiveSetting(&self) -> ::windows::core::Result<LiveSetting> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CurrentLiveSetting)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedLiveSetting(&self) -> ::windows::core::Result<LiveSetting> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CachedLiveSetting)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentFlowsFrom(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CurrentFlowsFrom)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedFlowsFrom(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CachedFlowsFrom)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ShowContextMenu(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.ShowContextMenu)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsPeripheral(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CurrentIsPeripheral)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsPeripheral(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CachedIsPeripheral)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentPositionInSet(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CurrentPositionInSet)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentSizeOfSet(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CurrentSizeOfSet)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentLevel(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CurrentLevel)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CurrentAnnotationTypes(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CurrentAnnotationTypes)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentAnnotationObjects(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CurrentAnnotationObjects)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedPositionInSet(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CachedPositionInSet)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedSizeOfSet(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CachedSizeOfSet)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedLevel(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CachedLevel)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CachedAnnotationTypes(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CachedAnnotationTypes)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedAnnotationObjects(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CachedAnnotationObjects)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentLandmarkType(&self) -> ::windows::core::Result<UIA_LANDMARKTYPE_ID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CurrentLandmarkType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentLocalizedLandmarkType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CurrentLocalizedLandmarkType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedLandmarkType(&self) -> ::windows::core::Result<UIA_LANDMARKTYPE_ID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CachedLandmarkType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedLocalizedLandmarkType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CachedLocalizedLandmarkType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentFullDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CurrentFullDescription)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedFullDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CachedFullDescription)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IUIAutomationElement8 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationElement8 {}
impl ::core::fmt::Debug for IUIAutomationElement8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationElement8").field(&self.0).finish()
    }
}
impl IUIAutomationElement8 {
    pub unsafe fn SetFocus(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.SetFocus)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRuntimeId(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetRuntimeId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindFirst<P0>(&self, scope: TreeScope, condition: P0) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.FindFirst)(::windows::core::Vtable::as_raw(self), scope, condition.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindAll<P0>(&self, scope: TreeScope, condition: P0) -> ::windows::core::Result<IUIAutomationElementArray>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.FindAll)(::windows::core::Vtable::as_raw(self), scope, condition.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindFirstBuildCache<P0, P1>(&self, scope: TreeScope, condition: P0, cacherequest: P1) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.FindFirstBuildCache)(::windows::core::Vtable::as_raw(self), scope, condition.into().abi(), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindAllBuildCache<P0, P1>(&self, scope: TreeScope, condition: P0, cacherequest: P1) -> ::windows::core::Result<IUIAutomationElementArray>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.FindAllBuildCache)(::windows::core::Vtable::as_raw(self), scope, condition.into().abi(), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn BuildUpdatedCache<P0>(&self, cacherequest: P0) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.BuildUpdatedCache)(::windows::core::Vtable::as_raw(self), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetCurrentPropertyValue(&self, propertyid: UIA_PROPERTY_ID) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetCurrentPropertyValue)(::windows::core::Vtable::as_raw(self), propertyid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetCurrentPropertyValueEx<P0>(&self, propertyid: UIA_PROPERTY_ID, ignoredefaultvalue: P0) -> ::windows::core::Result<super::super::System::Com::VARIANT>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetCurrentPropertyValueEx)(::windows::core::Vtable::as_raw(self), propertyid, ignoredefaultvalue.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetCachedPropertyValue(&self, propertyid: UIA_PROPERTY_ID) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetCachedPropertyValue)(::windows::core::Vtable::as_raw(self), propertyid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetCachedPropertyValueEx<P0>(&self, propertyid: UIA_PROPERTY_ID, ignoredefaultvalue: P0) -> ::windows::core::Result<super::super::System::Com::VARIANT>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetCachedPropertyValueEx)(::windows::core::Vtable::as_raw(self), propertyid, ignoredefaultvalue.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCurrentPatternAs<T>(&self, patternid: UIA_PATTERN_ID) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetCurrentPatternAs)(::windows::core::Vtable::as_raw(self), patternid, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCachedPatternAs<T>(&self, patternid: UIA_PATTERN_ID) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetCachedPatternAs)(::windows::core::Vtable::as_raw(self), patternid, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCurrentPattern(&self, patternid: UIA_PATTERN_ID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetCurrentPattern)(::windows::core::Vtable::as_raw(self), patternid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCachedPattern(&self, patternid: UIA_PATTERN_ID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetCachedPattern)(::windows::core::Vtable::as_raw(self), patternid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCachedParent(&self) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetCachedParent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCachedChildren(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetCachedChildren)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentProcessId(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CurrentProcessId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentControlType(&self) -> ::windows::core::Result<UIA_CONTROLTYPE_ID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CurrentControlType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentLocalizedControlType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CurrentLocalizedControlType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CurrentName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentAcceleratorKey(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CurrentAcceleratorKey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentAccessKey(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CurrentAccessKey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentHasKeyboardFocus(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CurrentHasKeyboardFocus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsKeyboardFocusable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CurrentIsKeyboardFocusable)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsEnabled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CurrentIsEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentAutomationId(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CurrentAutomationId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentClassName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CurrentClassName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentHelpText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CurrentHelpText)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentCulture(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CurrentCulture)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsControlElement(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CurrentIsControlElement)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsContentElement(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CurrentIsContentElement)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsPassword(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CurrentIsPassword)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentNativeWindowHandle(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CurrentNativeWindowHandle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentItemType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CurrentItemType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsOffscreen(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CurrentIsOffscreen)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentOrientation(&self) -> ::windows::core::Result<OrientationType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CurrentOrientation)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentFrameworkId(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CurrentFrameworkId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsRequiredForForm(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CurrentIsRequiredForForm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentItemStatus(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CurrentItemStatus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentBoundingRectangle(&self) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CurrentBoundingRectangle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentLabeledBy(&self) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CurrentLabeledBy)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentAriaRole(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CurrentAriaRole)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentAriaProperties(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CurrentAriaProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsDataValidForForm(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CurrentIsDataValidForForm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentControllerFor(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CurrentControllerFor)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentDescribedBy(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CurrentDescribedBy)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentFlowsTo(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CurrentFlowsTo)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentProviderDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CurrentProviderDescription)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedProcessId(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CachedProcessId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedControlType(&self) -> ::windows::core::Result<UIA_CONTROLTYPE_ID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CachedControlType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedLocalizedControlType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CachedLocalizedControlType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CachedName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedAcceleratorKey(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CachedAcceleratorKey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedAccessKey(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CachedAccessKey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedHasKeyboardFocus(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CachedHasKeyboardFocus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsKeyboardFocusable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CachedIsKeyboardFocusable)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsEnabled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CachedIsEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedAutomationId(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CachedAutomationId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedClassName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CachedClassName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedHelpText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CachedHelpText)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedCulture(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CachedCulture)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsControlElement(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CachedIsControlElement)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsContentElement(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CachedIsContentElement)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsPassword(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CachedIsPassword)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedNativeWindowHandle(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CachedNativeWindowHandle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedItemType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CachedItemType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsOffscreen(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CachedIsOffscreen)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedOrientation(&self) -> ::windows::core::Result<OrientationType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CachedOrientation)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedFrameworkId(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CachedFrameworkId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsRequiredForForm(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CachedIsRequiredForForm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedItemStatus(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CachedItemStatus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedBoundingRectangle(&self) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CachedBoundingRectangle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedLabeledBy(&self) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CachedLabeledBy)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedAriaRole(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CachedAriaRole)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedAriaProperties(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CachedAriaProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsDataValidForForm(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CachedIsDataValidForForm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedControllerFor(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CachedControllerFor)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedDescribedBy(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CachedDescribedBy)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedFlowsTo(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CachedFlowsTo)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedProviderDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CachedProviderDescription)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetClickablePoint(&self, clickable: *mut super::super::Foundation::POINT, gotclickable: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetClickablePoint)(::windows::core::Vtable::as_raw(self), clickable, gotclickable).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentOptimizeForVisualContent(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CurrentOptimizeForVisualContent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedOptimizeForVisualContent(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CachedOptimizeForVisualContent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentLiveSetting(&self) -> ::windows::core::Result<LiveSetting> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CurrentLiveSetting)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedLiveSetting(&self) -> ::windows::core::Result<LiveSetting> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CachedLiveSetting)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentFlowsFrom(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CurrentFlowsFrom)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedFlowsFrom(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CachedFlowsFrom)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ShowContextMenu(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.ShowContextMenu)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsPeripheral(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CurrentIsPeripheral)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsPeripheral(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CachedIsPeripheral)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentPositionInSet(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CurrentPositionInSet)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentSizeOfSet(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CurrentSizeOfSet)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentLevel(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CurrentLevel)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CurrentAnnotationTypes(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CurrentAnnotationTypes)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentAnnotationObjects(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CurrentAnnotationObjects)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedPositionInSet(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CachedPositionInSet)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedSizeOfSet(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CachedSizeOfSet)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedLevel(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CachedLevel)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CachedAnnotationTypes(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CachedAnnotationTypes)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedAnnotationObjects(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CachedAnnotationObjects)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentLandmarkType(&self) -> ::windows::core::Result<UIA_LANDMARKTYPE_ID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CurrentLandmarkType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentLocalizedLandmarkType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CurrentLocalizedLandmarkType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedLandmarkType(&self) -> ::windows::core::Result<UIA_LANDMARKTYPE_ID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CachedLandmarkType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedLocalizedLandmarkType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CachedLocalizedLandmarkType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentFullDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CurrentFullDescription)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedFullDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CachedFullDescription)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindFirstWithOptions<P0, P1>(&self, scope: TreeScope, condition: P0, traversaloptions: TreeTraversalOptions, root: P1) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.FindFirstWithOptions)(::windows::core::Vtable::as_raw(self), scope, condition.into().abi(), traversaloptions, root.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindAllWithOptions<P0, P1>(&self, scope: TreeScope, condition: P0, traversaloptions: TreeTraversalOptions, root: P1) -> ::windows::core::Result<IUIAutomationElementArray>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.FindAllWithOptions)(::windows::core::Vtable::as_raw(self), scope, condition.into().abi(), traversaloptions, root.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindFirstWithOptionsBuildCache<P0, P1, P2>(&self, scope: TreeScope, condition: P0, cacherequest: P1, traversaloptions: TreeTraversalOptions, root: P2) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
        P2: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.FindFirstWithOptionsBuildCache)(::windows::core::Vtable::as_raw(self), scope, condition.into().abi(), cacherequest.into().abi(), traversaloptions, root.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindAllWithOptionsBuildCache<P0, P1, P2>(&self, scope: TreeScope, condition: P0, cacherequest: P1, traversaloptions: TreeTraversalOptions, root: P2) -> ::windows::core::Result<IUIAutomationElementArray>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
        P2: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.FindAllWithOptionsBuildCache)(::windows::core::Vtable::as_raw(self), scope, condition.into().abi(), cacherequest.into().abi(), traversaloptions, root.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetCurrentMetadataValue(&self, targetid: i32, metadataid: UIA_METADATA_ID) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCurrentMetadataValue)(::windows::core::Vtable::as_raw(self), targetid, metadataid, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IUIAutomationElement9 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationElement9 {}
impl ::core::fmt::Debug for IUIAutomationElement9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationElement9").field(&self.0).finish()
    }
}
impl IUIAutomationElement9 {
    pub unsafe fn SetFocus(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.SetFocus)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRuntimeId(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.GetRuntimeId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindFirst<P0>(&self, scope: TreeScope, condition: P0) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.FindFirst)(::windows::core::Vtable::as_raw(self), scope, condition.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindAll<P0>(&self, scope: TreeScope, condition: P0) -> ::windows::core::Result<IUIAutomationElementArray>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.FindAll)(::windows::core::Vtable::as_raw(self), scope, condition.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindFirstBuildCache<P0, P1>(&self, scope: TreeScope, condition: P0, cacherequest: P1) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.FindFirstBuildCache)(::windows::core::Vtable::as_raw(self), scope, condition.into().abi(), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindAllBuildCache<P0, P1>(&self, scope: TreeScope, condition: P0, cacherequest: P1) -> ::windows::core::Result<IUIAutomationElementArray>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.FindAllBuildCache)(::windows::core::Vtable::as_raw(self), scope, condition.into().abi(), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn BuildUpdatedCache<P0>(&self, cacherequest: P0) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.BuildUpdatedCache)(::windows::core::Vtable::as_raw(self), cacherequest.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetCurrentPropertyValue(&self, propertyid: UIA_PROPERTY_ID) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.GetCurrentPropertyValue)(::windows::core::Vtable::as_raw(self), propertyid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetCurrentPropertyValueEx<P0>(&self, propertyid: UIA_PROPERTY_ID, ignoredefaultvalue: P0) -> ::windows::core::Result<super::super::System::Com::VARIANT>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.GetCurrentPropertyValueEx)(::windows::core::Vtable::as_raw(self), propertyid, ignoredefaultvalue.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetCachedPropertyValue(&self, propertyid: UIA_PROPERTY_ID) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.GetCachedPropertyValue)(::windows::core::Vtable::as_raw(self), propertyid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetCachedPropertyValueEx<P0>(&self, propertyid: UIA_PROPERTY_ID, ignoredefaultvalue: P0) -> ::windows::core::Result<super::super::System::Com::VARIANT>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.GetCachedPropertyValueEx)(::windows::core::Vtable::as_raw(self), propertyid, ignoredefaultvalue.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCurrentPatternAs<T>(&self, patternid: UIA_PATTERN_ID) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.GetCurrentPatternAs)(::windows::core::Vtable::as_raw(self), patternid, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCachedPatternAs<T>(&self, patternid: UIA_PATTERN_ID) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.GetCachedPatternAs)(::windows::core::Vtable::as_raw(self), patternid, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCurrentPattern(&self, patternid: UIA_PATTERN_ID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.GetCurrentPattern)(::windows::core::Vtable::as_raw(self), patternid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCachedPattern(&self, patternid: UIA_PATTERN_ID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.GetCachedPattern)(::windows::core::Vtable::as_raw(self), patternid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCachedParent(&self) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.GetCachedParent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCachedChildren(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.GetCachedChildren)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentProcessId(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CurrentProcessId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentControlType(&self) -> ::windows::core::Result<UIA_CONTROLTYPE_ID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CurrentControlType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentLocalizedControlType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CurrentLocalizedControlType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CurrentName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentAcceleratorKey(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CurrentAcceleratorKey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentAccessKey(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CurrentAccessKey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentHasKeyboardFocus(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CurrentHasKeyboardFocus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsKeyboardFocusable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CurrentIsKeyboardFocusable)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsEnabled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CurrentIsEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentAutomationId(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CurrentAutomationId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentClassName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CurrentClassName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentHelpText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CurrentHelpText)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentCulture(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CurrentCulture)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsControlElement(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CurrentIsControlElement)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsContentElement(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CurrentIsContentElement)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsPassword(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CurrentIsPassword)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentNativeWindowHandle(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CurrentNativeWindowHandle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentItemType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CurrentItemType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsOffscreen(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CurrentIsOffscreen)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentOrientation(&self) -> ::windows::core::Result<OrientationType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CurrentOrientation)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentFrameworkId(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CurrentFrameworkId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsRequiredForForm(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CurrentIsRequiredForForm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentItemStatus(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CurrentItemStatus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentBoundingRectangle(&self) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CurrentBoundingRectangle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentLabeledBy(&self) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CurrentLabeledBy)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentAriaRole(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CurrentAriaRole)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentAriaProperties(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CurrentAriaProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsDataValidForForm(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CurrentIsDataValidForForm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentControllerFor(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CurrentControllerFor)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentDescribedBy(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CurrentDescribedBy)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentFlowsTo(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CurrentFlowsTo)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentProviderDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CurrentProviderDescription)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedProcessId(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CachedProcessId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedControlType(&self) -> ::windows::core::Result<UIA_CONTROLTYPE_ID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CachedControlType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedLocalizedControlType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CachedLocalizedControlType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CachedName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedAcceleratorKey(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CachedAcceleratorKey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedAccessKey(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CachedAccessKey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedHasKeyboardFocus(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CachedHasKeyboardFocus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsKeyboardFocusable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CachedIsKeyboardFocusable)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsEnabled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CachedIsEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedAutomationId(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CachedAutomationId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedClassName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CachedClassName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedHelpText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CachedHelpText)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedCulture(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CachedCulture)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsControlElement(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CachedIsControlElement)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsContentElement(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CachedIsContentElement)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsPassword(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CachedIsPassword)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedNativeWindowHandle(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CachedNativeWindowHandle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedItemType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CachedItemType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsOffscreen(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CachedIsOffscreen)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedOrientation(&self) -> ::windows::core::Result<OrientationType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CachedOrientation)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedFrameworkId(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CachedFrameworkId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsRequiredForForm(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CachedIsRequiredForForm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedItemStatus(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CachedItemStatus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedBoundingRectangle(&self) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CachedBoundingRectangle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedLabeledBy(&self) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CachedLabeledBy)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedAriaRole(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CachedAriaRole)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedAriaProperties(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CachedAriaProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsDataValidForForm(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CachedIsDataValidForForm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedControllerFor(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CachedControllerFor)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedDescribedBy(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CachedDescribedBy)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedFlowsTo(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CachedFlowsTo)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedProviderDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CachedProviderDescription)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetClickablePoint(&self, clickable: *mut super::super::Foundation::POINT, gotclickable: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.GetClickablePoint)(::windows::core::Vtable::as_raw(self), clickable, gotclickable).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentOptimizeForVisualContent(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CurrentOptimizeForVisualContent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedOptimizeForVisualContent(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CachedOptimizeForVisualContent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentLiveSetting(&self) -> ::windows::core::Result<LiveSetting> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CurrentLiveSetting)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedLiveSetting(&self) -> ::windows::core::Result<LiveSetting> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CachedLiveSetting)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentFlowsFrom(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CurrentFlowsFrom)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedFlowsFrom(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CachedFlowsFrom)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ShowContextMenu(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.ShowContextMenu)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsPeripheral(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CurrentIsPeripheral)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsPeripheral(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CachedIsPeripheral)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentPositionInSet(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CurrentPositionInSet)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentSizeOfSet(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CurrentSizeOfSet)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentLevel(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CurrentLevel)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CurrentAnnotationTypes(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CurrentAnnotationTypes)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentAnnotationObjects(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CurrentAnnotationObjects)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedPositionInSet(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CachedPositionInSet)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedSizeOfSet(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CachedSizeOfSet)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedLevel(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CachedLevel)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CachedAnnotationTypes(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CachedAnnotationTypes)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedAnnotationObjects(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CachedAnnotationObjects)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentLandmarkType(&self) -> ::windows::core::Result<UIA_LANDMARKTYPE_ID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CurrentLandmarkType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentLocalizedLandmarkType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CurrentLocalizedLandmarkType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedLandmarkType(&self) -> ::windows::core::Result<UIA_LANDMARKTYPE_ID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CachedLandmarkType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedLocalizedLandmarkType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CachedLocalizedLandmarkType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentFullDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CurrentFullDescription)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedFullDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CachedFullDescription)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindFirstWithOptions<P0, P1>(&self, scope: TreeScope, condition: P0, traversaloptions: TreeTraversalOptions, root: P1) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.FindFirstWithOptions)(::windows::core::Vtable::as_raw(self), scope, condition.into().abi(), traversaloptions, root.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindAllWithOptions<P0, P1>(&self, scope: TreeScope, condition: P0, traversaloptions: TreeTraversalOptions, root: P1) -> ::windows::core::Result<IUIAutomationElementArray>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.FindAllWithOptions)(::windows::core::Vtable::as_raw(self), scope, condition.into().abi(), traversaloptions, root.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindFirstWithOptionsBuildCache<P0, P1, P2>(&self, scope: TreeScope, condition: P0, cacherequest: P1, traversaloptions: TreeTraversalOptions, root: P2) -> ::windows::core::Result<IUIAutomationElement>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
        P2: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.FindFirstWithOptionsBuildCache)(::windows::core::Vtable::as_raw(self), scope, condition.into().abi(), cacherequest.into().abi(), traversaloptions, root.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindAllWithOptionsBuildCache<P0, P1, P2>(&self, scope: TreeScope, condition: P0, cacherequest: P1, traversaloptions: TreeTraversalOptions, root: P2) -> ::windows::core::Result<IUIAutomationElementArray>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationCondition>>,
        P1: ::std::convert::Into<::windows::core::InParam<IUIAutomationCacheRequest>>,
        P2: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.FindAllWithOptionsBuildCache)(::windows::core::Vtable::as_raw(self), scope, condition.into().abi(), cacherequest.into().abi(), traversaloptions, root.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetCurrentMetadataValue(&self, targetid: i32, metadataid: UIA_METADATA_ID) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetCurrentMetadataValue)(::windows::core::Vtable::as_raw(self), targetid, metadataid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CurrentHeadingLevel(&self) -> ::windows::core::Result<UIA_HEADINGLEVEL_ID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CurrentHeadingLevel)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CachedHeadingLevel(&self) -> ::windows::core::Result<UIA_HEADINGLEVEL_ID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CachedHeadingLevel)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IUIAutomationElementArray {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationElementArray {}
impl ::core::fmt::Debug for IUIAutomationElementArray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationElementArray").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationEventHandler {}
impl ::core::fmt::Debug for IUIAutomationEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationEventHandler").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationEventHandlerGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationEventHandlerGroup {}
impl ::core::fmt::Debug for IUIAutomationEventHandlerGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationEventHandlerGroup").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationExpandCollapsePattern {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationExpandCollapsePattern {}
impl ::core::fmt::Debug for IUIAutomationExpandCollapsePattern {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationExpandCollapsePattern").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationFocusChangedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationFocusChangedEventHandler {}
impl ::core::fmt::Debug for IUIAutomationFocusChangedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationFocusChangedEventHandler").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationGridItemPattern {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationGridItemPattern {}
impl ::core::fmt::Debug for IUIAutomationGridItemPattern {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationGridItemPattern").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationGridPattern {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationGridPattern {}
impl ::core::fmt::Debug for IUIAutomationGridPattern {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationGridPattern").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationInvokePattern {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationInvokePattern {}
impl ::core::fmt::Debug for IUIAutomationInvokePattern {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationInvokePattern").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationItemContainerPattern {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationItemContainerPattern {}
impl ::core::fmt::Debug for IUIAutomationItemContainerPattern {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationItemContainerPattern").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationLegacyIAccessiblePattern {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationLegacyIAccessiblePattern {}
impl ::core::fmt::Debug for IUIAutomationLegacyIAccessiblePattern {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationLegacyIAccessiblePattern").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationMultipleViewPattern {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationMultipleViewPattern {}
impl ::core::fmt::Debug for IUIAutomationMultipleViewPattern {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationMultipleViewPattern").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationNotCondition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationNotCondition {}
impl ::core::fmt::Debug for IUIAutomationNotCondition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationNotCondition").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationNotificationEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationNotificationEventHandler {}
impl ::core::fmt::Debug for IUIAutomationNotificationEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationNotificationEventHandler").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationObjectModelPattern {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationObjectModelPattern {}
impl ::core::fmt::Debug for IUIAutomationObjectModelPattern {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationObjectModelPattern").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationOrCondition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationOrCondition {}
impl ::core::fmt::Debug for IUIAutomationOrCondition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationOrCondition").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationPatternHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationPatternHandler {}
impl ::core::fmt::Debug for IUIAutomationPatternHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationPatternHandler").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationPatternInstance {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationPatternInstance {}
impl ::core::fmt::Debug for IUIAutomationPatternInstance {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationPatternInstance").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationPropertyChangedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationPropertyChangedEventHandler {}
impl ::core::fmt::Debug for IUIAutomationPropertyChangedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationPropertyChangedEventHandler").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationPropertyCondition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationPropertyCondition {}
impl ::core::fmt::Debug for IUIAutomationPropertyCondition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationPropertyCondition").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationProxyFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationProxyFactory {}
impl ::core::fmt::Debug for IUIAutomationProxyFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationProxyFactory").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationProxyFactoryEntry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationProxyFactoryEntry {}
impl ::core::fmt::Debug for IUIAutomationProxyFactoryEntry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationProxyFactoryEntry").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationProxyFactoryMapping {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationProxyFactoryMapping {}
impl ::core::fmt::Debug for IUIAutomationProxyFactoryMapping {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationProxyFactoryMapping").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationRangeValuePattern {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationRangeValuePattern {}
impl ::core::fmt::Debug for IUIAutomationRangeValuePattern {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationRangeValuePattern").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationRegistrar {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationRegistrar {}
impl ::core::fmt::Debug for IUIAutomationRegistrar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationRegistrar").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationScrollItemPattern {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationScrollItemPattern {}
impl ::core::fmt::Debug for IUIAutomationScrollItemPattern {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationScrollItemPattern").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationScrollPattern {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationScrollPattern {}
impl ::core::fmt::Debug for IUIAutomationScrollPattern {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationScrollPattern").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationSelectionItemPattern {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationSelectionItemPattern {}
impl ::core::fmt::Debug for IUIAutomationSelectionItemPattern {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationSelectionItemPattern").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationSelectionPattern {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationSelectionPattern {}
impl ::core::fmt::Debug for IUIAutomationSelectionPattern {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationSelectionPattern").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationSelectionPattern2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationSelectionPattern2 {}
impl ::core::fmt::Debug for IUIAutomationSelectionPattern2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationSelectionPattern2").field(&self.0).finish()
    }
}
impl IUIAutomationSelectionPattern2 {
    pub unsafe fn GetCurrentSelection(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCurrentSelection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentCanSelectMultiple(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CurrentCanSelectMultiple)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentIsSelectionRequired(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CurrentIsSelectionRequired)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCachedSelection(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCachedSelection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedCanSelectMultiple(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CachedCanSelectMultiple)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedIsSelectionRequired(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CachedIsSelectionRequired)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IUIAutomationSpreadsheetItemPattern {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationSpreadsheetItemPattern {}
impl ::core::fmt::Debug for IUIAutomationSpreadsheetItemPattern {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationSpreadsheetItemPattern").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationSpreadsheetPattern {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationSpreadsheetPattern {}
impl ::core::fmt::Debug for IUIAutomationSpreadsheetPattern {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationSpreadsheetPattern").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationStructureChangedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationStructureChangedEventHandler {}
impl ::core::fmt::Debug for IUIAutomationStructureChangedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationStructureChangedEventHandler").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationStylesPattern {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationStylesPattern {}
impl ::core::fmt::Debug for IUIAutomationStylesPattern {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationStylesPattern").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationSynchronizedInputPattern {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationSynchronizedInputPattern {}
impl ::core::fmt::Debug for IUIAutomationSynchronizedInputPattern {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationSynchronizedInputPattern").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationTableItemPattern {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationTableItemPattern {}
impl ::core::fmt::Debug for IUIAutomationTableItemPattern {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationTableItemPattern").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationTablePattern {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationTablePattern {}
impl ::core::fmt::Debug for IUIAutomationTablePattern {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationTablePattern").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationTextChildPattern {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationTextChildPattern {}
impl ::core::fmt::Debug for IUIAutomationTextChildPattern {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationTextChildPattern").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationTextEditPattern {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationTextEditPattern {}
impl ::core::fmt::Debug for IUIAutomationTextEditPattern {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationTextEditPattern").field(&self.0).finish()
    }
}
impl IUIAutomationTextEditPattern {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RangeFromPoint(&self, pt: super::super::Foundation::POINT) -> ::windows::core::Result<IUIAutomationTextRange> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.RangeFromPoint)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pt), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RangeFromChild<P0>(&self, child: P0) -> ::windows::core::Result<IUIAutomationTextRange>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.RangeFromChild)(::windows::core::Vtable::as_raw(self), child.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSelection(&self) -> ::windows::core::Result<IUIAutomationTextRangeArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSelection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetVisibleRanges(&self) -> ::windows::core::Result<IUIAutomationTextRangeArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetVisibleRanges)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DocumentRange(&self) -> ::windows::core::Result<IUIAutomationTextRange> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.DocumentRange)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SupportedTextSelection(&self) -> ::windows::core::Result<SupportedTextSelection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SupportedTextSelection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IUIAutomationTextEditTextChangedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationTextEditTextChangedEventHandler {}
impl ::core::fmt::Debug for IUIAutomationTextEditTextChangedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationTextEditTextChangedEventHandler").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationTextPattern {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationTextPattern {}
impl ::core::fmt::Debug for IUIAutomationTextPattern {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationTextPattern").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationTextPattern2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationTextPattern2 {}
impl ::core::fmt::Debug for IUIAutomationTextPattern2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationTextPattern2").field(&self.0).finish()
    }
}
impl IUIAutomationTextPattern2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RangeFromPoint(&self, pt: super::super::Foundation::POINT) -> ::windows::core::Result<IUIAutomationTextRange> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.RangeFromPoint)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pt), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RangeFromChild<P0>(&self, child: P0) -> ::windows::core::Result<IUIAutomationTextRange>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationElement>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.RangeFromChild)(::windows::core::Vtable::as_raw(self), child.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSelection(&self) -> ::windows::core::Result<IUIAutomationTextRangeArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSelection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetVisibleRanges(&self) -> ::windows::core::Result<IUIAutomationTextRangeArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetVisibleRanges)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DocumentRange(&self) -> ::windows::core::Result<IUIAutomationTextRange> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.DocumentRange)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SupportedTextSelection(&self) -> ::windows::core::Result<SupportedTextSelection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SupportedTextSelection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IUIAutomationTextRange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationTextRange {}
impl ::core::fmt::Debug for IUIAutomationTextRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationTextRange").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationTextRange2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationTextRange2 {}
impl ::core::fmt::Debug for IUIAutomationTextRange2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationTextRange2").field(&self.0).finish()
    }
}
impl IUIAutomationTextRange2 {
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IUIAutomationTextRange> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Compare<P0>(&self, range: P0) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationTextRange>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Compare)(::windows::core::Vtable::as_raw(self), range.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CompareEndpoints<P0>(&self, srcendpoint: TextPatternRangeEndpoint, range: P0, targetendpoint: TextPatternRangeEndpoint) -> ::windows::core::Result<i32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationTextRange>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CompareEndpoints)(::windows::core::Vtable::as_raw(self), srcendpoint, range.into().abi(), targetendpoint, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ExpandToEnclosingUnit(&self, textunit: TextUnit) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ExpandToEnclosingUnit)(::windows::core::Vtable::as_raw(self), textunit).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn FindAttribute<P0>(&self, attr: UIA_TEXTATTRIBUTE_ID, val: super::super::System::Com::VARIANT, backward: P0) -> ::windows::core::Result<IUIAutomationTextRange>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.FindAttribute)(::windows::core::Vtable::as_raw(self), attr, ::core::mem::transmute(val), backward.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindText<P0, P1>(&self, text: &::windows::core::BSTR, backward: P0, ignorecase: P1) -> ::windows::core::Result<IUIAutomationTextRange>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.FindText)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(text), backward.into(), ignorecase.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetAttributeValue(&self, attr: UIA_TEXTATTRIBUTE_ID) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetAttributeValue)(::windows::core::Vtable::as_raw(self), attr, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetBoundingRectangles(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetBoundingRectangles)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetEnclosingElement(&self) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetEnclosingElement)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetText(&self, maxlength: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetText)(::windows::core::Vtable::as_raw(self), maxlength, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Move(&self, unit: TextUnit, count: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Move)(::windows::core::Vtable::as_raw(self), unit, count, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn MoveEndpointByUnit(&self, endpoint: TextPatternRangeEndpoint, unit: TextUnit, count: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.MoveEndpointByUnit)(::windows::core::Vtable::as_raw(self), endpoint, unit, count, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn MoveEndpointByRange<P0>(&self, srcendpoint: TextPatternRangeEndpoint, range: P0, targetendpoint: TextPatternRangeEndpoint) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationTextRange>>,
    {
        (::windows::core::Vtable::vtable(self).base__.MoveEndpointByRange)(::windows::core::Vtable::as_raw(self), srcendpoint, range.into().abi(), targetendpoint).ok()
    }
    pub unsafe fn Select(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Select)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn AddToSelection(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.AddToSelection)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn RemoveFromSelection(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.RemoveFromSelection)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ScrollIntoView<P0>(&self, aligntotop: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.ScrollIntoView)(::windows::core::Vtable::as_raw(self), aligntotop.into()).ok()
    }
    pub unsafe fn GetChildren(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetChildren)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IUIAutomationTextRange3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationTextRange3 {}
impl ::core::fmt::Debug for IUIAutomationTextRange3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationTextRange3").field(&self.0).finish()
    }
}
impl IUIAutomationTextRange3 {
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IUIAutomationTextRange> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Compare<P0>(&self, range: P0) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationTextRange>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Compare)(::windows::core::Vtable::as_raw(self), range.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CompareEndpoints<P0>(&self, srcendpoint: TextPatternRangeEndpoint, range: P0, targetendpoint: TextPatternRangeEndpoint) -> ::windows::core::Result<i32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationTextRange>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CompareEndpoints)(::windows::core::Vtable::as_raw(self), srcendpoint, range.into().abi(), targetendpoint, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ExpandToEnclosingUnit(&self, textunit: TextUnit) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.ExpandToEnclosingUnit)(::windows::core::Vtable::as_raw(self), textunit).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn FindAttribute<P0>(&self, attr: UIA_TEXTATTRIBUTE_ID, val: super::super::System::Com::VARIANT, backward: P0) -> ::windows::core::Result<IUIAutomationTextRange>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.FindAttribute)(::windows::core::Vtable::as_raw(self), attr, ::core::mem::transmute(val), backward.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindText<P0, P1>(&self, text: &::windows::core::BSTR, backward: P0, ignorecase: P1) -> ::windows::core::Result<IUIAutomationTextRange>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.FindText)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(text), backward.into(), ignorecase.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetAttributeValue(&self, attr: UIA_TEXTATTRIBUTE_ID) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetAttributeValue)(::windows::core::Vtable::as_raw(self), attr, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetBoundingRectangles(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetBoundingRectangles)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetEnclosingElement(&self) -> ::windows::core::Result<IUIAutomationElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetEnclosingElement)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetText(&self, maxlength: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetText)(::windows::core::Vtable::as_raw(self), maxlength, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Move(&self, unit: TextUnit, count: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Move)(::windows::core::Vtable::as_raw(self), unit, count, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn MoveEndpointByUnit(&self, endpoint: TextPatternRangeEndpoint, unit: TextUnit, count: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.MoveEndpointByUnit)(::windows::core::Vtable::as_raw(self), endpoint, unit, count, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn MoveEndpointByRange<P0>(&self, srcendpoint: TextPatternRangeEndpoint, range: P0, targetendpoint: TextPatternRangeEndpoint) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUIAutomationTextRange>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.MoveEndpointByRange)(::windows::core::Vtable::as_raw(self), srcendpoint, range.into().abi(), targetendpoint).ok()
    }
    pub unsafe fn Select(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Select)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn AddToSelection(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.AddToSelection)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn RemoveFromSelection(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.RemoveFromSelection)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ScrollIntoView<P0>(&self, aligntotop: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.ScrollIntoView)(::windows::core::Vtable::as_raw(self), aligntotop.into()).ok()
    }
    pub unsafe fn GetChildren(&self) -> ::windows::core::Result<IUIAutomationElementArray> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetChildren)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ShowContextMenu(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ShowContextMenu)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationTextRangeArray {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationTextRangeArray {}
impl ::core::fmt::Debug for IUIAutomationTextRangeArray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationTextRangeArray").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationTogglePattern {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationTogglePattern {}
impl ::core::fmt::Debug for IUIAutomationTogglePattern {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationTogglePattern").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationTransformPattern {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationTransformPattern {}
impl ::core::fmt::Debug for IUIAutomationTransformPattern {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationTransformPattern").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationTransformPattern2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationTransformPattern2 {}
impl ::core::fmt::Debug for IUIAutomationTransformPattern2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationTransformPattern2").field(&self.0).finish()
    }
}
impl IUIAutomationTransformPattern2 {
    pub unsafe fn Move(&self, x: f64, y: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Move)(::windows::core::Vtable::as_raw(self), x, y).ok()
    }
    pub unsafe fn Resize(&self, width: f64, height: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Resize)(::windows::core::Vtable::as_raw(self), width, height).ok()
    }
    pub unsafe fn Rotate(&self, degrees: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Rotate)(::windows::core::Vtable::as_raw(self), degrees).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentCanMove(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CurrentCanMove)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentCanResize(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CurrentCanResize)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentCanRotate(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CurrentCanRotate)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedCanMove(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CachedCanMove)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedCanResize(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CachedCanResize)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CachedCanRotate(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CachedCanRotate)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IUIAutomationTreeWalker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationTreeWalker {}
impl ::core::fmt::Debug for IUIAutomationTreeWalker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationTreeWalker").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationValuePattern {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationValuePattern {}
impl ::core::fmt::Debug for IUIAutomationValuePattern {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationValuePattern").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationVirtualizedItemPattern {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationVirtualizedItemPattern {}
impl ::core::fmt::Debug for IUIAutomationVirtualizedItemPattern {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationVirtualizedItemPattern").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUIAutomationWindowPattern {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIAutomationWindowPattern {}
impl ::core::fmt::Debug for IUIAutomationWindowPattern {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIAutomationWindowPattern").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IValueProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IValueProvider {}
impl ::core::fmt::Debug for IValueProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IValueProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IVirtualizedItemProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVirtualizedItemProvider {}
impl ::core::fmt::Debug for IVirtualizedItemProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVirtualizedItemProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWindowProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWindowProvider {}
impl ::core::fmt::Debug for IWindowProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWindowProvider").field(&self.0).finish()
    }
}
impl ::core::default::Default for LiveSetting {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LiveSetting {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LiveSetting").field(&self.0).finish()
    }
}
impl ::core::default::Default for MOUSEKEYS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MOUSEKEYS {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwFlags == other.dwFlags && self.iMaxSpeed == other.iMaxSpeed && self.iTimeToMaxSpeed == other.iTimeToMaxSpeed && self.iCtrlSpeed == other.iCtrlSpeed && self.dwReserved1 == other.dwReserved1 && self.dwReserved2 == other.dwReserved2
    }
}
impl ::core::cmp::Eq for MOUSEKEYS {}
impl ::core::fmt::Debug for MOUSEKEYS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MOUSEKEYS").field("cbSize", &self.cbSize).field("dwFlags", &self.dwFlags).field("iMaxSpeed", &self.iMaxSpeed).field("iTimeToMaxSpeed", &self.iTimeToMaxSpeed).field("iCtrlSpeed", &self.iCtrlSpeed).field("dwReserved1", &self.dwReserved1).field("dwReserved2", &self.dwReserved2).finish()
    }
}
impl ::core::default::Default for MSAAMENUINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MSAAMENUINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwMSAASignature == other.dwMSAASignature && self.cchWText == other.cchWText && self.pszWText == other.pszWText
    }
}
impl ::core::cmp::Eq for MSAAMENUINFO {}
impl ::core::fmt::Debug for MSAAMENUINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MSAAMENUINFO").field("dwMSAASignature", &self.dwMSAASignature).field("cchWText", &self.cchWText).field("pszWText", &self.pszWText).finish()
    }
}
impl ::core::default::Default for NavigateDirection {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NavigateDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NavigateDirection").field(&self.0).finish()
    }
}
impl ::core::default::Default for NormalizeState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NormalizeState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NormalizeState").field(&self.0).finish()
    }
}
impl ::core::default::Default for NotificationKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NotificationKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NotificationKind").field(&self.0).finish()
    }
}
impl ::core::default::Default for NotificationProcessing {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NotificationProcessing {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NotificationProcessing").field(&self.0).finish()
    }
}
impl ::core::default::Default for OrientationType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OrientationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OrientationType").field(&self.0).finish()
    }
}
impl ::core::default::Default for OutlineStyles {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OutlineStyles {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OutlineStyles").field(&self.0).finish()
    }
}
impl ::core::default::Default for PropertyConditionFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PropertyConditionFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PropertyConditionFlags").field(&self.0).finish()
    }
}
impl ::core::default::Default for ProviderOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ProviderOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProviderOptions").field(&self.0).finish()
    }
}
impl ::core::default::Default for ProviderType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ProviderType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProviderType").field(&self.0).finish()
    }
}
impl ::core::default::Default for RowOrColumnMajor {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RowOrColumnMajor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RowOrColumnMajor").field(&self.0).finish()
    }
}
impl ::core::default::Default for SERIALKEYSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERIALKEYSA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwFlags == other.dwFlags && self.lpszActivePort == other.lpszActivePort && self.lpszPort == other.lpszPort && self.iBaudRate == other.iBaudRate && self.iPortState == other.iPortState && self.iActive == other.iActive
    }
}
impl ::core::cmp::Eq for SERIALKEYSA {}
impl ::core::fmt::Debug for SERIALKEYSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERIALKEYSA").field("cbSize", &self.cbSize).field("dwFlags", &self.dwFlags).field("lpszActivePort", &self.lpszActivePort).field("lpszPort", &self.lpszPort).field("iBaudRate", &self.iBaudRate).field("iPortState", &self.iPortState).field("iActive", &self.iActive).finish()
    }
}
impl ::core::default::Default for SERIALKEYSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERIALKEYSW {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwFlags == other.dwFlags && self.lpszActivePort == other.lpszActivePort && self.lpszPort == other.lpszPort && self.iBaudRate == other.iBaudRate && self.iPortState == other.iPortState && self.iActive == other.iActive
    }
}
impl ::core::cmp::Eq for SERIALKEYSW {}
impl ::core::fmt::Debug for SERIALKEYSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERIALKEYSW").field("cbSize", &self.cbSize).field("dwFlags", &self.dwFlags).field("lpszActivePort", &self.lpszActivePort).field("lpszPort", &self.lpszPort).field("iBaudRate", &self.iBaudRate).field("iPortState", &self.iPortState).field("iActive", &self.iActive).finish()
    }
}
impl ::core::default::Default for SERIALKEYS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SERIALKEYS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SERIALKEYS_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for SERIALKEYS_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SERIALKEYS_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SERIALKEYS_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SERIALKEYS_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SERIALKEYS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for SOUNDSENTRYA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SOUNDSENTRYA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwFlags == other.dwFlags && self.iFSTextEffect == other.iFSTextEffect && self.iFSTextEffectMSec == other.iFSTextEffectMSec && self.iFSTextEffectColorBits == other.iFSTextEffectColorBits && self.iFSGrafEffect == other.iFSGrafEffect && self.iFSGrafEffectMSec == other.iFSGrafEffectMSec && self.iFSGrafEffectColor == other.iFSGrafEffectColor && self.iWindowsEffect == other.iWindowsEffect && self.iWindowsEffectMSec == other.iWindowsEffectMSec && self.lpszWindowsEffectDLL == other.lpszWindowsEffectDLL && self.iWindowsEffectOrdinal == other.iWindowsEffectOrdinal
    }
}
impl ::core::cmp::Eq for SOUNDSENTRYA {}
impl ::core::fmt::Debug for SOUNDSENTRYA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOUNDSENTRYA")
            .field("cbSize", &self.cbSize)
            .field("dwFlags", &self.dwFlags)
            .field("iFSTextEffect", &self.iFSTextEffect)
            .field("iFSTextEffectMSec", &self.iFSTextEffectMSec)
            .field("iFSTextEffectColorBits", &self.iFSTextEffectColorBits)
            .field("iFSGrafEffect", &self.iFSGrafEffect)
            .field("iFSGrafEffectMSec", &self.iFSGrafEffectMSec)
            .field("iFSGrafEffectColor", &self.iFSGrafEffectColor)
            .field("iWindowsEffect", &self.iWindowsEffect)
            .field("iWindowsEffectMSec", &self.iWindowsEffectMSec)
            .field("lpszWindowsEffectDLL", &self.lpszWindowsEffectDLL)
            .field("iWindowsEffectOrdinal", &self.iWindowsEffectOrdinal)
            .finish()
    }
}
impl ::core::default::Default for SOUNDSENTRYW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SOUNDSENTRYW {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwFlags == other.dwFlags && self.iFSTextEffect == other.iFSTextEffect && self.iFSTextEffectMSec == other.iFSTextEffectMSec && self.iFSTextEffectColorBits == other.iFSTextEffectColorBits && self.iFSGrafEffect == other.iFSGrafEffect && self.iFSGrafEffectMSec == other.iFSGrafEffectMSec && self.iFSGrafEffectColor == other.iFSGrafEffectColor && self.iWindowsEffect == other.iWindowsEffect && self.iWindowsEffectMSec == other.iWindowsEffectMSec && self.lpszWindowsEffectDLL == other.lpszWindowsEffectDLL && self.iWindowsEffectOrdinal == other.iWindowsEffectOrdinal
    }
}
impl ::core::cmp::Eq for SOUNDSENTRYW {}
impl ::core::fmt::Debug for SOUNDSENTRYW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOUNDSENTRYW")
            .field("cbSize", &self.cbSize)
            .field("dwFlags", &self.dwFlags)
            .field("iFSTextEffect", &self.iFSTextEffect)
            .field("iFSTextEffectMSec", &self.iFSTextEffectMSec)
            .field("iFSTextEffectColorBits", &self.iFSTextEffectColorBits)
            .field("iFSGrafEffect", &self.iFSGrafEffect)
            .field("iFSGrafEffectMSec", &self.iFSGrafEffectMSec)
            .field("iFSGrafEffectColor", &self.iFSGrafEffectColor)
            .field("iWindowsEffect", &self.iWindowsEffect)
            .field("iWindowsEffectMSec", &self.iWindowsEffectMSec)
            .field("lpszWindowsEffectDLL", &self.lpszWindowsEffectDLL)
            .field("iWindowsEffectOrdinal", &self.iWindowsEffectOrdinal)
            .finish()
    }
}
impl ::core::default::Default for SOUNDSENTRY_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SOUNDSENTRY_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SOUNDSENTRY_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for SOUNDSENTRY_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SOUNDSENTRY_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SOUNDSENTRY_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SOUNDSENTRY_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SOUNDSENTRY_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for SOUNDSENTRY_TEXT_EFFECT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SOUNDSENTRY_TEXT_EFFECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SOUNDSENTRY_TEXT_EFFECT").field(&self.0).finish()
    }
}
impl ::core::default::Default for SOUNDSENTRY_WINDOWS_EFFECT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SOUNDSENTRY_WINDOWS_EFFECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SOUNDSENTRY_WINDOWS_EFFECT").field(&self.0).finish()
    }
}
impl ::core::default::Default for SOUND_SENTRY_GRAPHICS_EFFECT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SOUND_SENTRY_GRAPHICS_EFFECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SOUND_SENTRY_GRAPHICS_EFFECT").field(&self.0).finish()
    }
}
impl ::core::default::Default for STICKYKEYS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STICKYKEYS {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for STICKYKEYS {}
impl ::core::fmt::Debug for STICKYKEYS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STICKYKEYS").field("cbSize", &self.cbSize).field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::core::default::Default for STICKYKEYS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STICKYKEYS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STICKYKEYS_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for STICKYKEYS_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for STICKYKEYS_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for STICKYKEYS_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for STICKYKEYS_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for STICKYKEYS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for SayAsInterpretAs {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SayAsInterpretAs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SayAsInterpretAs").field(&self.0).finish()
    }
}
impl ::core::default::Default for ScrollAmount {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ScrollAmount {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ScrollAmount").field(&self.0).finish()
    }
}
impl ::core::default::Default for StructureChangeType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for StructureChangeType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StructureChangeType").field(&self.0).finish()
    }
}
impl ::core::default::Default for SupportedTextSelection {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SupportedTextSelection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SupportedTextSelection").field(&self.0).finish()
    }
}
impl ::core::default::Default for SynchronizedInputType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SynchronizedInputType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SynchronizedInputType").field(&self.0).finish()
    }
}
impl ::core::default::Default for TOGGLEKEYS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TOGGLEKEYS {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for TOGGLEKEYS {}
impl ::core::fmt::Debug for TOGGLEKEYS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TOGGLEKEYS").field("cbSize", &self.cbSize).field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::core::default::Default for TextDecorationLineStyle {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TextDecorationLineStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TextDecorationLineStyle").field(&self.0).finish()
    }
}
impl ::core::default::Default for TextEditChangeType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TextEditChangeType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TextEditChangeType").field(&self.0).finish()
    }
}
impl ::core::default::Default for TextPatternRangeEndpoint {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TextPatternRangeEndpoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TextPatternRangeEndpoint").field(&self.0).finish()
    }
}
impl ::core::default::Default for TextUnit {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TextUnit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TextUnit").field(&self.0).finish()
    }
}
impl ::core::default::Default for ToggleState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ToggleState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToggleState").field(&self.0).finish()
    }
}
impl ::core::default::Default for TreeScope {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TreeScope {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TreeScope").field(&self.0).finish()
    }
}
impl ::core::default::Default for TreeTraversalOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TreeTraversalOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TreeTraversalOptions").field(&self.0).finish()
    }
}
impl ::core::default::Default for UIA_ANNOTATIONTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UIA_ANNOTATIONTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UIA_ANNOTATIONTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for UIA_CHANGE_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UIA_CHANGE_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UIA_CHANGE_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for UIA_CONTROLTYPE_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UIA_CONTROLTYPE_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UIA_CONTROLTYPE_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for UIA_EVENT_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UIA_EVENT_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UIA_EVENT_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for UIA_HEADINGLEVEL_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UIA_HEADINGLEVEL_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UIA_HEADINGLEVEL_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for UIA_LANDMARKTYPE_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UIA_LANDMARKTYPE_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UIA_LANDMARKTYPE_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for UIA_METADATA_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UIA_METADATA_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UIA_METADATA_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for UIA_PATTERN_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UIA_PATTERN_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UIA_PATTERN_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for UIA_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UIA_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UIA_PROPERTY_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for UIA_STYLE_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UIA_STYLE_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UIA_STYLE_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for UIA_TEXTATTRIBUTE_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UIA_TEXTATTRIBUTE_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UIA_TEXTATTRIBUTE_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for UIAutomationEventInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for UIAutomationEventInfo {
    fn eq(&self, other: &Self) -> bool {
        self.guid == other.guid && self.pProgrammaticName == other.pProgrammaticName
    }
}
impl ::core::cmp::Eq for UIAutomationEventInfo {}
impl ::core::fmt::Debug for UIAutomationEventInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UIAutomationEventInfo").field("guid", &self.guid).field("pProgrammaticName", &self.pProgrammaticName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for UIAutomationMethodInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for UIAutomationMethodInfo {
    fn eq(&self, other: &Self) -> bool {
        self.pProgrammaticName == other.pProgrammaticName && self.doSetFocus == other.doSetFocus && self.cInParameters == other.cInParameters && self.cOutParameters == other.cOutParameters && self.pParameterTypes == other.pParameterTypes && self.pParameterNames == other.pParameterNames
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for UIAutomationMethodInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for UIAutomationMethodInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UIAutomationMethodInfo").field("pProgrammaticName", &self.pProgrammaticName).field("doSetFocus", &self.doSetFocus).field("cInParameters", &self.cInParameters).field("cOutParameters", &self.cOutParameters).field("pParameterTypes", &self.pParameterTypes).field("pParameterNames", &self.pParameterNames).finish()
    }
}
impl ::core::default::Default for UIAutomationParameter {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for UIAutomationParameter {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type && self.pData == other.pData
    }
}
impl ::core::cmp::Eq for UIAutomationParameter {}
impl ::core::fmt::Debug for UIAutomationParameter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UIAutomationParameter").field("type", &self.r#type).field("pData", &self.pData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for UIAutomationPatternInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for UIAutomationPatternInfo {
    fn eq(&self, other: &Self) -> bool {
        self.guid == other.guid && self.pProgrammaticName == other.pProgrammaticName && self.providerInterfaceId == other.providerInterfaceId && self.clientInterfaceId == other.clientInterfaceId && self.cProperties == other.cProperties && self.pProperties == other.pProperties && self.cMethods == other.cMethods && self.pMethods == other.pMethods && self.cEvents == other.cEvents && self.pEvents == other.pEvents && self.pPatternHandler == other.pPatternHandler
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for UIAutomationPatternInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for UIAutomationPatternInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UIAutomationPatternInfo")
            .field("guid", &self.guid)
            .field("pProgrammaticName", &self.pProgrammaticName)
            .field("providerInterfaceId", &self.providerInterfaceId)
            .field("clientInterfaceId", &self.clientInterfaceId)
            .field("cProperties", &self.cProperties)
            .field("pProperties", &self.pProperties)
            .field("cMethods", &self.cMethods)
            .field("pMethods", &self.pMethods)
            .field("cEvents", &self.cEvents)
            .field("pEvents", &self.pEvents)
            .field("pPatternHandler", &self.pPatternHandler)
            .finish()
    }
}
impl ::core::default::Default for UIAutomationPropertyInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for UIAutomationPropertyInfo {
    fn eq(&self, other: &Self) -> bool {
        self.guid == other.guid && self.pProgrammaticName == other.pProgrammaticName && self.r#type == other.r#type
    }
}
impl ::core::cmp::Eq for UIAutomationPropertyInfo {}
impl ::core::fmt::Debug for UIAutomationPropertyInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UIAutomationPropertyInfo").field("guid", &self.guid).field("pProgrammaticName", &self.pProgrammaticName).field("type", &self.r#type).finish()
    }
}
impl ::core::default::Default for UIAutomationType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UIAutomationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UIAutomationType").field(&self.0).finish()
    }
}
impl ::core::default::Default for UiaAndOrCondition {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for UiaAndOrCondition {
    fn eq(&self, other: &Self) -> bool {
        self.ConditionType == other.ConditionType && self.ppConditions == other.ppConditions && self.cConditions == other.cConditions
    }
}
impl ::core::cmp::Eq for UiaAndOrCondition {}
impl ::core::fmt::Debug for UiaAndOrCondition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UiaAndOrCondition").field("ConditionType", &self.ConditionType).field("ppConditions", &self.ppConditions).field("cConditions", &self.cConditions).finish()
    }
}
impl ::core::default::Default for UiaAsyncContentLoadedEventArgs {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for UiaAsyncContentLoadedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.EventId == other.EventId && self.AsyncContentLoadedState == other.AsyncContentLoadedState && self.PercentComplete == other.PercentComplete
    }
}
impl ::core::cmp::Eq for UiaAsyncContentLoadedEventArgs {}
impl ::core::fmt::Debug for UiaAsyncContentLoadedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UiaAsyncContentLoadedEventArgs").field("Type", &self.Type).field("EventId", &self.EventId).field("AsyncContentLoadedState", &self.AsyncContentLoadedState).field("PercentComplete", &self.PercentComplete).finish()
    }
}
impl ::core::default::Default for UiaCacheRequest {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for UiaCacheRequest {
    fn eq(&self, other: &Self) -> bool {
        self.pViewCondition == other.pViewCondition && self.Scope == other.Scope && self.pProperties == other.pProperties && self.cProperties == other.cProperties && self.pPatterns == other.pPatterns && self.cPatterns == other.cPatterns && self.automationElementMode == other.automationElementMode
    }
}
impl ::core::cmp::Eq for UiaCacheRequest {}
impl ::core::fmt::Debug for UiaCacheRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UiaCacheRequest").field("pViewCondition", &self.pViewCondition).field("Scope", &self.Scope).field("pProperties", &self.pProperties).field("cProperties", &self.cProperties).field("pPatterns", &self.pPatterns).field("cPatterns", &self.cPatterns).field("automationElementMode", &self.automationElementMode).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::default::Default for UiaChangeInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::default::Default for UiaChangesEventArgs {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for UiaChangesEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.EventId == other.EventId && self.EventIdCount == other.EventIdCount && self.pUiaChanges == other.pUiaChanges
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for UiaChangesEventArgs {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::fmt::Debug for UiaChangesEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UiaChangesEventArgs").field("Type", &self.Type).field("EventId", &self.EventId).field("EventIdCount", &self.EventIdCount).field("pUiaChanges", &self.pUiaChanges).finish()
    }
}
impl ::core::default::Default for UiaCondition {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for UiaCondition {
    fn eq(&self, other: &Self) -> bool {
        self.ConditionType == other.ConditionType
    }
}
impl ::core::cmp::Eq for UiaCondition {}
impl ::core::fmt::Debug for UiaCondition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UiaCondition").field("ConditionType", &self.ConditionType).finish()
    }
}
impl ::core::default::Default for UiaEventArgs {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for UiaEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.EventId == other.EventId
    }
}
impl ::core::cmp::Eq for UiaEventArgs {}
impl ::core::fmt::Debug for UiaEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UiaEventArgs").field("Type", &self.Type).field("EventId", &self.EventId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for UiaFindParams {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for UiaFindParams {
    fn eq(&self, other: &Self) -> bool {
        self.MaxDepth == other.MaxDepth && self.FindFirst == other.FindFirst && self.ExcludeRoot == other.ExcludeRoot && self.pFindCondition == other.pFindCondition
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for UiaFindParams {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for UiaFindParams {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UiaFindParams").field("MaxDepth", &self.MaxDepth).field("FindFirst", &self.FindFirst).field("ExcludeRoot", &self.ExcludeRoot).field("pFindCondition", &self.pFindCondition).finish()
    }
}
impl ::core::default::Default for UiaNotCondition {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for UiaNotCondition {
    fn eq(&self, other: &Self) -> bool {
        self.ConditionType == other.ConditionType && self.pCondition == other.pCondition
    }
}
impl ::core::cmp::Eq for UiaNotCondition {}
impl ::core::fmt::Debug for UiaNotCondition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UiaNotCondition").field("ConditionType", &self.ConditionType).field("pCondition", &self.pCondition).finish()
    }
}
impl ::core::default::Default for UiaPoint {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for UiaPoint {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl ::core::cmp::Eq for UiaPoint {}
impl ::core::fmt::Debug for UiaPoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UiaPoint").field("x", &self.x).field("y", &self.y).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::default::Default for UiaPropertyChangedEventArgs {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::default::Default for UiaPropertyCondition {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for UiaRect {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for UiaRect {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left && self.top == other.top && self.width == other.width && self.height == other.height
    }
}
impl ::core::cmp::Eq for UiaRect {}
impl ::core::fmt::Debug for UiaRect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UiaRect").field("left", &self.left).field("top", &self.top).field("width", &self.width).field("height", &self.height).finish()
    }
}
impl ::core::default::Default for UiaStructureChangedEventArgs {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for UiaStructureChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.EventId == other.EventId && self.StructureChangeType == other.StructureChangeType && self.pRuntimeId == other.pRuntimeId && self.cRuntimeIdLen == other.cRuntimeIdLen
    }
}
impl ::core::cmp::Eq for UiaStructureChangedEventArgs {}
impl ::core::fmt::Debug for UiaStructureChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UiaStructureChangedEventArgs").field("Type", &self.Type).field("EventId", &self.EventId).field("StructureChangeType", &self.StructureChangeType).field("pRuntimeId", &self.pRuntimeId).field("cRuntimeIdLen", &self.cRuntimeIdLen).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for UiaTextEditTextChangedEventArgs {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for UiaTextEditTextChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.EventId == other.EventId && self.TextEditChangeType == other.TextEditChangeType && self.pTextChange == other.pTextChange
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for UiaTextEditTextChangedEventArgs {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for UiaTextEditTextChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UiaTextEditTextChangedEventArgs").field("Type", &self.Type).field("EventId", &self.EventId).field("TextEditChangeType", &self.TextEditChangeType).field("pTextChange", &self.pTextChange).finish()
    }
}
impl ::core::default::Default for UiaWindowClosedEventArgs {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for UiaWindowClosedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.EventId == other.EventId && self.pRuntimeId == other.pRuntimeId && self.cRuntimeIdLen == other.cRuntimeIdLen
    }
}
impl ::core::cmp::Eq for UiaWindowClosedEventArgs {}
impl ::core::fmt::Debug for UiaWindowClosedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UiaWindowClosedEventArgs").field("Type", &self.Type).field("EventId", &self.EventId).field("pRuntimeId", &self.pRuntimeId).field("cRuntimeIdLen", &self.cRuntimeIdLen).finish()
    }
}
impl ::core::default::Default for VisualEffects {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VisualEffects {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VisualEffects").field(&self.0).finish()
    }
}
impl ::core::default::Default for WindowInteractionState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WindowInteractionState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowInteractionState").field(&self.0).finish()
    }
}
impl ::core::default::Default for WindowVisualState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WindowVisualState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowVisualState").field(&self.0).finish()
    }
}
impl ::core::default::Default for ZoomUnit {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ZoomUnit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ZoomUnit").field(&self.0).finish()
    }
}
