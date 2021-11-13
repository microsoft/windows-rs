#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AccessibilityView(pub i32);
impl AccessibilityView {
    pub const Raw: Self = Self(0i32);
    pub const Control: Self = Self(1i32);
    pub const Content: Self = Self(2i32);
}
impl ::core::marker::Copy for AccessibilityView {}
impl ::core::clone::Clone for AccessibilityView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppBarAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppBarAutomationPeer {}
impl ::core::clone::Clone for AppBarAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppBarButtonAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppBarButtonAutomationPeer {}
impl ::core::clone::Clone for AppBarButtonAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppBarToggleButtonAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppBarToggleButtonAutomationPeer {}
impl ::core::clone::Clone for AppBarToggleButtonAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AutoSuggestBoxAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AutoSuggestBoxAutomationPeer {}
impl ::core::clone::Clone for AutoSuggestBoxAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AutomationControlType(pub i32);
impl AutomationControlType {
    pub const Button: Self = Self(0i32);
    pub const Calendar: Self = Self(1i32);
    pub const CheckBox: Self = Self(2i32);
    pub const ComboBox: Self = Self(3i32);
    pub const Edit: Self = Self(4i32);
    pub const Hyperlink: Self = Self(5i32);
    pub const Image: Self = Self(6i32);
    pub const ListItem: Self = Self(7i32);
    pub const List: Self = Self(8i32);
    pub const Menu: Self = Self(9i32);
    pub const MenuBar: Self = Self(10i32);
    pub const MenuItem: Self = Self(11i32);
    pub const ProgressBar: Self = Self(12i32);
    pub const RadioButton: Self = Self(13i32);
    pub const ScrollBar: Self = Self(14i32);
    pub const Slider: Self = Self(15i32);
    pub const Spinner: Self = Self(16i32);
    pub const StatusBar: Self = Self(17i32);
    pub const Tab: Self = Self(18i32);
    pub const TabItem: Self = Self(19i32);
    pub const Text: Self = Self(20i32);
    pub const ToolBar: Self = Self(21i32);
    pub const ToolTip: Self = Self(22i32);
    pub const Tree: Self = Self(23i32);
    pub const TreeItem: Self = Self(24i32);
    pub const Custom: Self = Self(25i32);
    pub const Group: Self = Self(26i32);
    pub const Thumb: Self = Self(27i32);
    pub const DataGrid: Self = Self(28i32);
    pub const DataItem: Self = Self(29i32);
    pub const Document: Self = Self(30i32);
    pub const SplitButton: Self = Self(31i32);
    pub const Window: Self = Self(32i32);
    pub const Pane: Self = Self(33i32);
    pub const Header: Self = Self(34i32);
    pub const HeaderItem: Self = Self(35i32);
    pub const Table: Self = Self(36i32);
    pub const TitleBar: Self = Self(37i32);
    pub const Separator: Self = Self(38i32);
    pub const SemanticZoom: Self = Self(39i32);
    pub const AppBar: Self = Self(40i32);
}
impl ::core::marker::Copy for AutomationControlType {}
impl ::core::clone::Clone for AutomationControlType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AutomationEvents(pub i32);
impl AutomationEvents {
    pub const ToolTipOpened: Self = Self(0i32);
    pub const ToolTipClosed: Self = Self(1i32);
    pub const MenuOpened: Self = Self(2i32);
    pub const MenuClosed: Self = Self(3i32);
    pub const AutomationFocusChanged: Self = Self(4i32);
    pub const InvokePatternOnInvoked: Self = Self(5i32);
    pub const SelectionItemPatternOnElementAddedToSelection: Self = Self(6i32);
    pub const SelectionItemPatternOnElementRemovedFromSelection: Self = Self(7i32);
    pub const SelectionItemPatternOnElementSelected: Self = Self(8i32);
    pub const SelectionPatternOnInvalidated: Self = Self(9i32);
    pub const TextPatternOnTextSelectionChanged: Self = Self(10i32);
    pub const TextPatternOnTextChanged: Self = Self(11i32);
    pub const AsyncContentLoaded: Self = Self(12i32);
    pub const PropertyChanged: Self = Self(13i32);
    pub const StructureChanged: Self = Self(14i32);
    pub const DragStart: Self = Self(15i32);
    pub const DragCancel: Self = Self(16i32);
    pub const DragComplete: Self = Self(17i32);
    pub const DragEnter: Self = Self(18i32);
    pub const DragLeave: Self = Self(19i32);
    pub const Dropped: Self = Self(20i32);
    pub const LiveRegionChanged: Self = Self(21i32);
    pub const InputReachedTarget: Self = Self(22i32);
    pub const InputReachedOtherElement: Self = Self(23i32);
    pub const InputDiscarded: Self = Self(24i32);
    pub const WindowClosed: Self = Self(25i32);
    pub const WindowOpened: Self = Self(26i32);
    pub const ConversionTargetChanged: Self = Self(27i32);
    pub const TextEditTextChanged: Self = Self(28i32);
    pub const LayoutInvalidated: Self = Self(29i32);
}
impl ::core::marker::Copy for AutomationEvents {}
impl ::core::clone::Clone for AutomationEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AutomationHeadingLevel(pub i32);
impl AutomationHeadingLevel {
    pub const None: Self = Self(0i32);
    pub const Level1: Self = Self(1i32);
    pub const Level2: Self = Self(2i32);
    pub const Level3: Self = Self(3i32);
    pub const Level4: Self = Self(4i32);
    pub const Level5: Self = Self(5i32);
    pub const Level6: Self = Self(6i32);
    pub const Level7: Self = Self(7i32);
    pub const Level8: Self = Self(8i32);
    pub const Level9: Self = Self(9i32);
}
impl ::core::marker::Copy for AutomationHeadingLevel {}
impl ::core::clone::Clone for AutomationHeadingLevel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AutomationLandmarkType(pub i32);
impl AutomationLandmarkType {
    pub const None: Self = Self(0i32);
    pub const Custom: Self = Self(1i32);
    pub const Form: Self = Self(2i32);
    pub const Main: Self = Self(3i32);
    pub const Navigation: Self = Self(4i32);
    pub const Search: Self = Self(5i32);
}
impl ::core::marker::Copy for AutomationLandmarkType {}
impl ::core::clone::Clone for AutomationLandmarkType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AutomationLiveSetting(pub i32);
impl AutomationLiveSetting {
    pub const Off: Self = Self(0i32);
    pub const Polite: Self = Self(1i32);
    pub const Assertive: Self = Self(2i32);
}
impl ::core::marker::Copy for AutomationLiveSetting {}
impl ::core::clone::Clone for AutomationLiveSetting {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AutomationNavigationDirection(pub i32);
impl AutomationNavigationDirection {
    pub const Parent: Self = Self(0i32);
    pub const NextSibling: Self = Self(1i32);
    pub const PreviousSibling: Self = Self(2i32);
    pub const FirstChild: Self = Self(3i32);
    pub const LastChild: Self = Self(4i32);
}
impl ::core::marker::Copy for AutomationNavigationDirection {}
impl ::core::clone::Clone for AutomationNavigationDirection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AutomationNotificationKind(pub i32);
impl AutomationNotificationKind {
    pub const ItemAdded: Self = Self(0i32);
    pub const ItemRemoved: Self = Self(1i32);
    pub const ActionCompleted: Self = Self(2i32);
    pub const ActionAborted: Self = Self(3i32);
    pub const Other: Self = Self(4i32);
}
impl ::core::marker::Copy for AutomationNotificationKind {}
impl ::core::clone::Clone for AutomationNotificationKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AutomationNotificationProcessing(pub i32);
impl AutomationNotificationProcessing {
    pub const ImportantAll: Self = Self(0i32);
    pub const ImportantMostRecent: Self = Self(1i32);
    pub const All: Self = Self(2i32);
    pub const MostRecent: Self = Self(3i32);
    pub const CurrentThenMostRecent: Self = Self(4i32);
}
impl ::core::marker::Copy for AutomationNotificationProcessing {}
impl ::core::clone::Clone for AutomationNotificationProcessing {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AutomationOrientation(pub i32);
impl AutomationOrientation {
    pub const None: Self = Self(0i32);
    pub const Horizontal: Self = Self(1i32);
    pub const Vertical: Self = Self(2i32);
}
impl ::core::marker::Copy for AutomationOrientation {}
impl ::core::clone::Clone for AutomationOrientation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AutomationPeer {}
impl ::core::clone::Clone for AutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AutomationPeerAnnotation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AutomationPeerAnnotation {}
impl ::core::clone::Clone for AutomationPeerAnnotation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AutomationStructureChangeType(pub i32);
impl AutomationStructureChangeType {
    pub const ChildAdded: Self = Self(0i32);
    pub const ChildRemoved: Self = Self(1i32);
    pub const ChildrenInvalidated: Self = Self(2i32);
    pub const ChildrenBulkAdded: Self = Self(3i32);
    pub const ChildrenBulkRemoved: Self = Self(4i32);
    pub const ChildrenReordered: Self = Self(5i32);
}
impl ::core::marker::Copy for AutomationStructureChangeType {}
impl ::core::clone::Clone for AutomationStructureChangeType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ButtonAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ButtonAutomationPeer {}
impl ::core::clone::Clone for ButtonAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ButtonBaseAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ButtonBaseAutomationPeer {}
impl ::core::clone::Clone for ButtonBaseAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CalendarDatePickerAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CalendarDatePickerAutomationPeer {}
impl ::core::clone::Clone for CalendarDatePickerAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CaptureElementAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CaptureElementAutomationPeer {}
impl ::core::clone::Clone for CaptureElementAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CheckBoxAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CheckBoxAutomationPeer {}
impl ::core::clone::Clone for CheckBoxAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ColorPickerSliderAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ColorPickerSliderAutomationPeer {}
impl ::core::clone::Clone for ColorPickerSliderAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ColorSpectrumAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ColorSpectrumAutomationPeer {}
impl ::core::clone::Clone for ColorSpectrumAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ComboBoxAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ComboBoxAutomationPeer {}
impl ::core::clone::Clone for ComboBoxAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ComboBoxItemAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ComboBoxItemAutomationPeer {}
impl ::core::clone::Clone for ComboBoxItemAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ComboBoxItemDataAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ComboBoxItemDataAutomationPeer {}
impl ::core::clone::Clone for ComboBoxItemDataAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DatePickerAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DatePickerAutomationPeer {}
impl ::core::clone::Clone for DatePickerAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DatePickerFlyoutPresenterAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DatePickerFlyoutPresenterAutomationPeer {}
impl ::core::clone::Clone for DatePickerFlyoutPresenterAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FlipViewAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FlipViewAutomationPeer {}
impl ::core::clone::Clone for FlipViewAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FlipViewItemAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FlipViewItemAutomationPeer {}
impl ::core::clone::Clone for FlipViewItemAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FlipViewItemDataAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FlipViewItemDataAutomationPeer {}
impl ::core::clone::Clone for FlipViewItemDataAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FlyoutPresenterAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FlyoutPresenterAutomationPeer {}
impl ::core::clone::Clone for FlyoutPresenterAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FrameworkElementAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FrameworkElementAutomationPeer {}
impl ::core::clone::Clone for FrameworkElementAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GridViewAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GridViewAutomationPeer {}
impl ::core::clone::Clone for GridViewAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GridViewHeaderItemAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GridViewHeaderItemAutomationPeer {}
impl ::core::clone::Clone for GridViewHeaderItemAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GridViewItemAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GridViewItemAutomationPeer {}
impl ::core::clone::Clone for GridViewItemAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GridViewItemDataAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GridViewItemDataAutomationPeer {}
impl ::core::clone::Clone for GridViewItemDataAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GroupItemAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GroupItemAutomationPeer {}
impl ::core::clone::Clone for GroupItemAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HubAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HubAutomationPeer {}
impl ::core::clone::Clone for HubAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HubSectionAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HubSectionAutomationPeer {}
impl ::core::clone::Clone for HubSectionAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HyperlinkButtonAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HyperlinkButtonAutomationPeer {}
impl ::core::clone::Clone for HyperlinkButtonAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBarAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBarAutomationPeer {}
impl ::core::clone::Clone for IAppBarAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBarAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBarAutomationPeerFactory {}
impl ::core::clone::Clone for IAppBarAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBarButtonAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBarButtonAutomationPeer {}
impl ::core::clone::Clone for IAppBarButtonAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBarButtonAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBarButtonAutomationPeerFactory {}
impl ::core::clone::Clone for IAppBarButtonAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBarToggleButtonAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBarToggleButtonAutomationPeer {}
impl ::core::clone::Clone for IAppBarToggleButtonAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBarToggleButtonAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBarToggleButtonAutomationPeerFactory {}
impl ::core::clone::Clone for IAppBarToggleButtonAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAutoSuggestBoxAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAutoSuggestBoxAutomationPeer {}
impl ::core::clone::Clone for IAutoSuggestBoxAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAutoSuggestBoxAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAutoSuggestBoxAutomationPeerFactory {}
impl ::core::clone::Clone for IAutoSuggestBoxAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAutomationPeer {}
impl ::core::clone::Clone for IAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAutomationPeer2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAutomationPeer2 {}
impl ::core::clone::Clone for IAutomationPeer2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAutomationPeer3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAutomationPeer3 {}
impl ::core::clone::Clone for IAutomationPeer3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAutomationPeer4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAutomationPeer4 {}
impl ::core::clone::Clone for IAutomationPeer4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAutomationPeer5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAutomationPeer5 {}
impl ::core::clone::Clone for IAutomationPeer5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAutomationPeer6(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAutomationPeer6 {}
impl ::core::clone::Clone for IAutomationPeer6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAutomationPeer7(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAutomationPeer7 {}
impl ::core::clone::Clone for IAutomationPeer7 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAutomationPeer8(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAutomationPeer8 {}
impl ::core::clone::Clone for IAutomationPeer8 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAutomationPeer9(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAutomationPeer9 {}
impl ::core::clone::Clone for IAutomationPeer9 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAutomationPeerAnnotation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAutomationPeerAnnotation {}
impl ::core::clone::Clone for IAutomationPeerAnnotation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAutomationPeerAnnotationFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAutomationPeerAnnotationFactory {}
impl ::core::clone::Clone for IAutomationPeerAnnotationFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAutomationPeerAnnotationStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAutomationPeerAnnotationStatics {}
impl ::core::clone::Clone for IAutomationPeerAnnotationStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAutomationPeerFactory {}
impl ::core::clone::Clone for IAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAutomationPeerOverrides(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAutomationPeerOverrides {}
impl ::core::clone::Clone for IAutomationPeerOverrides {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAutomationPeerOverrides2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAutomationPeerOverrides2 {}
impl ::core::clone::Clone for IAutomationPeerOverrides2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAutomationPeerOverrides3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAutomationPeerOverrides3 {}
impl ::core::clone::Clone for IAutomationPeerOverrides3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAutomationPeerOverrides4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAutomationPeerOverrides4 {}
impl ::core::clone::Clone for IAutomationPeerOverrides4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAutomationPeerOverrides5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAutomationPeerOverrides5 {}
impl ::core::clone::Clone for IAutomationPeerOverrides5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAutomationPeerOverrides6(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAutomationPeerOverrides6 {}
impl ::core::clone::Clone for IAutomationPeerOverrides6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAutomationPeerOverrides8(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAutomationPeerOverrides8 {}
impl ::core::clone::Clone for IAutomationPeerOverrides8 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAutomationPeerOverrides9(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAutomationPeerOverrides9 {}
impl ::core::clone::Clone for IAutomationPeerOverrides9 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAutomationPeerProtected(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAutomationPeerProtected {}
impl ::core::clone::Clone for IAutomationPeerProtected {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAutomationPeerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAutomationPeerStatics {}
impl ::core::clone::Clone for IAutomationPeerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAutomationPeerStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAutomationPeerStatics3 {}
impl ::core::clone::Clone for IAutomationPeerStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IButtonAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IButtonAutomationPeer {}
impl ::core::clone::Clone for IButtonAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IButtonAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IButtonAutomationPeerFactory {}
impl ::core::clone::Clone for IButtonAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IButtonBaseAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IButtonBaseAutomationPeer {}
impl ::core::clone::Clone for IButtonBaseAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IButtonBaseAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IButtonBaseAutomationPeerFactory {}
impl ::core::clone::Clone for IButtonBaseAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICalendarDatePickerAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICalendarDatePickerAutomationPeer {}
impl ::core::clone::Clone for ICalendarDatePickerAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICalendarDatePickerAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICalendarDatePickerAutomationPeerFactory {}
impl ::core::clone::Clone for ICalendarDatePickerAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICaptureElementAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICaptureElementAutomationPeer {}
impl ::core::clone::Clone for ICaptureElementAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICaptureElementAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICaptureElementAutomationPeerFactory {}
impl ::core::clone::Clone for ICaptureElementAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICheckBoxAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICheckBoxAutomationPeer {}
impl ::core::clone::Clone for ICheckBoxAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICheckBoxAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICheckBoxAutomationPeerFactory {}
impl ::core::clone::Clone for ICheckBoxAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IColorPickerSliderAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IColorPickerSliderAutomationPeer {}
impl ::core::clone::Clone for IColorPickerSliderAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IColorPickerSliderAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IColorPickerSliderAutomationPeerFactory {}
impl ::core::clone::Clone for IColorPickerSliderAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IColorSpectrumAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IColorSpectrumAutomationPeer {}
impl ::core::clone::Clone for IColorSpectrumAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IColorSpectrumAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IColorSpectrumAutomationPeerFactory {}
impl ::core::clone::Clone for IColorSpectrumAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComboBoxAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComboBoxAutomationPeer {}
impl ::core::clone::Clone for IComboBoxAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComboBoxAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComboBoxAutomationPeerFactory {}
impl ::core::clone::Clone for IComboBoxAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComboBoxItemAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComboBoxItemAutomationPeer {}
impl ::core::clone::Clone for IComboBoxItemAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComboBoxItemAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComboBoxItemAutomationPeerFactory {}
impl ::core::clone::Clone for IComboBoxItemAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComboBoxItemDataAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComboBoxItemDataAutomationPeer {}
impl ::core::clone::Clone for IComboBoxItemDataAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComboBoxItemDataAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComboBoxItemDataAutomationPeerFactory {}
impl ::core::clone::Clone for IComboBoxItemDataAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDatePickerAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDatePickerAutomationPeer {}
impl ::core::clone::Clone for IDatePickerAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDatePickerAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDatePickerAutomationPeerFactory {}
impl ::core::clone::Clone for IDatePickerAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDatePickerFlyoutPresenterAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDatePickerFlyoutPresenterAutomationPeer {}
impl ::core::clone::Clone for IDatePickerFlyoutPresenterAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFlipViewAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFlipViewAutomationPeer {}
impl ::core::clone::Clone for IFlipViewAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFlipViewAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFlipViewAutomationPeerFactory {}
impl ::core::clone::Clone for IFlipViewAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFlipViewItemAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFlipViewItemAutomationPeer {}
impl ::core::clone::Clone for IFlipViewItemAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFlipViewItemAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFlipViewItemAutomationPeerFactory {}
impl ::core::clone::Clone for IFlipViewItemAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFlipViewItemDataAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFlipViewItemDataAutomationPeer {}
impl ::core::clone::Clone for IFlipViewItemDataAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFlipViewItemDataAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFlipViewItemDataAutomationPeerFactory {}
impl ::core::clone::Clone for IFlipViewItemDataAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFlyoutPresenterAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFlyoutPresenterAutomationPeer {}
impl ::core::clone::Clone for IFlyoutPresenterAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFlyoutPresenterAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFlyoutPresenterAutomationPeerFactory {}
impl ::core::clone::Clone for IFlyoutPresenterAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFrameworkElementAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFrameworkElementAutomationPeer {}
impl ::core::clone::Clone for IFrameworkElementAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFrameworkElementAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFrameworkElementAutomationPeerFactory {}
impl ::core::clone::Clone for IFrameworkElementAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFrameworkElementAutomationPeerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFrameworkElementAutomationPeerStatics {}
impl ::core::clone::Clone for IFrameworkElementAutomationPeerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGridViewAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGridViewAutomationPeer {}
impl ::core::clone::Clone for IGridViewAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGridViewAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGridViewAutomationPeerFactory {}
impl ::core::clone::Clone for IGridViewAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGridViewHeaderItemAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGridViewHeaderItemAutomationPeer {}
impl ::core::clone::Clone for IGridViewHeaderItemAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGridViewHeaderItemAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGridViewHeaderItemAutomationPeerFactory {}
impl ::core::clone::Clone for IGridViewHeaderItemAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGridViewItemAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGridViewItemAutomationPeer {}
impl ::core::clone::Clone for IGridViewItemAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGridViewItemAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGridViewItemAutomationPeerFactory {}
impl ::core::clone::Clone for IGridViewItemAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGridViewItemDataAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGridViewItemDataAutomationPeer {}
impl ::core::clone::Clone for IGridViewItemDataAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGridViewItemDataAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGridViewItemDataAutomationPeerFactory {}
impl ::core::clone::Clone for IGridViewItemDataAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGroupItemAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGroupItemAutomationPeer {}
impl ::core::clone::Clone for IGroupItemAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGroupItemAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGroupItemAutomationPeerFactory {}
impl ::core::clone::Clone for IGroupItemAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHubAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHubAutomationPeer {}
impl ::core::clone::Clone for IHubAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHubAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHubAutomationPeerFactory {}
impl ::core::clone::Clone for IHubAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHubSectionAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHubSectionAutomationPeer {}
impl ::core::clone::Clone for IHubSectionAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHubSectionAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHubSectionAutomationPeerFactory {}
impl ::core::clone::Clone for IHubSectionAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHyperlinkButtonAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHyperlinkButtonAutomationPeer {}
impl ::core::clone::Clone for IHyperlinkButtonAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHyperlinkButtonAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHyperlinkButtonAutomationPeerFactory {}
impl ::core::clone::Clone for IHyperlinkButtonAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IImageAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IImageAutomationPeer {}
impl ::core::clone::Clone for IImageAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IImageAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IImageAutomationPeerFactory {}
impl ::core::clone::Clone for IImageAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkToolbarAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkToolbarAutomationPeer {}
impl ::core::clone::Clone for IInkToolbarAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IItemAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IItemAutomationPeer {}
impl ::core::clone::Clone for IItemAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IItemAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IItemAutomationPeerFactory {}
impl ::core::clone::Clone for IItemAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IItemsControlAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IItemsControlAutomationPeer {}
impl ::core::clone::Clone for IItemsControlAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IItemsControlAutomationPeer2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IItemsControlAutomationPeer2 {}
impl ::core::clone::Clone for IItemsControlAutomationPeer2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IItemsControlAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IItemsControlAutomationPeerFactory {}
impl ::core::clone::Clone for IItemsControlAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IItemsControlAutomationPeerOverrides2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IItemsControlAutomationPeerOverrides2 {}
impl ::core::clone::Clone for IItemsControlAutomationPeerOverrides2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListBoxAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListBoxAutomationPeer {}
impl ::core::clone::Clone for IListBoxAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListBoxAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListBoxAutomationPeerFactory {}
impl ::core::clone::Clone for IListBoxAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListBoxItemAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListBoxItemAutomationPeer {}
impl ::core::clone::Clone for IListBoxItemAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListBoxItemAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListBoxItemAutomationPeerFactory {}
impl ::core::clone::Clone for IListBoxItemAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListBoxItemDataAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListBoxItemDataAutomationPeer {}
impl ::core::clone::Clone for IListBoxItemDataAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListBoxItemDataAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListBoxItemDataAutomationPeerFactory {}
impl ::core::clone::Clone for IListBoxItemDataAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListPickerFlyoutPresenterAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListPickerFlyoutPresenterAutomationPeer {}
impl ::core::clone::Clone for IListPickerFlyoutPresenterAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListViewAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListViewAutomationPeer {}
impl ::core::clone::Clone for IListViewAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListViewAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListViewAutomationPeerFactory {}
impl ::core::clone::Clone for IListViewAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListViewBaseAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListViewBaseAutomationPeer {}
impl ::core::clone::Clone for IListViewBaseAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListViewBaseAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListViewBaseAutomationPeerFactory {}
impl ::core::clone::Clone for IListViewBaseAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListViewBaseHeaderItemAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListViewBaseHeaderItemAutomationPeer {}
impl ::core::clone::Clone for IListViewBaseHeaderItemAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListViewBaseHeaderItemAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListViewBaseHeaderItemAutomationPeerFactory {}
impl ::core::clone::Clone for IListViewBaseHeaderItemAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListViewHeaderItemAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListViewHeaderItemAutomationPeer {}
impl ::core::clone::Clone for IListViewHeaderItemAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListViewHeaderItemAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListViewHeaderItemAutomationPeerFactory {}
impl ::core::clone::Clone for IListViewHeaderItemAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListViewItemAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListViewItemAutomationPeer {}
impl ::core::clone::Clone for IListViewItemAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListViewItemAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListViewItemAutomationPeerFactory {}
impl ::core::clone::Clone for IListViewItemAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListViewItemDataAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListViewItemDataAutomationPeer {}
impl ::core::clone::Clone for IListViewItemDataAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListViewItemDataAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListViewItemDataAutomationPeerFactory {}
impl ::core::clone::Clone for IListViewItemDataAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILoopingSelectorAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILoopingSelectorAutomationPeer {}
impl ::core::clone::Clone for ILoopingSelectorAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILoopingSelectorItemAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILoopingSelectorItemAutomationPeer {}
impl ::core::clone::Clone for ILoopingSelectorItemAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILoopingSelectorItemDataAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILoopingSelectorItemDataAutomationPeer {}
impl ::core::clone::Clone for ILoopingSelectorItemDataAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMapControlAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMapControlAutomationPeer {}
impl ::core::clone::Clone for IMapControlAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaElementAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaElementAutomationPeer {}
impl ::core::clone::Clone for IMediaElementAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaElementAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaElementAutomationPeerFactory {}
impl ::core::clone::Clone for IMediaElementAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaPlayerElementAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaPlayerElementAutomationPeer {}
impl ::core::clone::Clone for IMediaPlayerElementAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaPlayerElementAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaPlayerElementAutomationPeerFactory {}
impl ::core::clone::Clone for IMediaPlayerElementAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaTransportControlsAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaTransportControlsAutomationPeer {}
impl ::core::clone::Clone for IMediaTransportControlsAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaTransportControlsAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaTransportControlsAutomationPeerFactory {}
impl ::core::clone::Clone for IMediaTransportControlsAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMenuBarAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMenuBarAutomationPeer {}
impl ::core::clone::Clone for IMenuBarAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMenuBarAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMenuBarAutomationPeerFactory {}
impl ::core::clone::Clone for IMenuBarAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMenuBarItemAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMenuBarItemAutomationPeer {}
impl ::core::clone::Clone for IMenuBarItemAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMenuBarItemAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMenuBarItemAutomationPeerFactory {}
impl ::core::clone::Clone for IMenuBarItemAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMenuFlyoutItemAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMenuFlyoutItemAutomationPeer {}
impl ::core::clone::Clone for IMenuFlyoutItemAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMenuFlyoutItemAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMenuFlyoutItemAutomationPeerFactory {}
impl ::core::clone::Clone for IMenuFlyoutItemAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMenuFlyoutPresenterAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMenuFlyoutPresenterAutomationPeer {}
impl ::core::clone::Clone for IMenuFlyoutPresenterAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMenuFlyoutPresenterAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMenuFlyoutPresenterAutomationPeerFactory {}
impl ::core::clone::Clone for IMenuFlyoutPresenterAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INavigationViewItemAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INavigationViewItemAutomationPeer {}
impl ::core::clone::Clone for INavigationViewItemAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INavigationViewItemAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INavigationViewItemAutomationPeerFactory {}
impl ::core::clone::Clone for INavigationViewItemAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPasswordBoxAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPasswordBoxAutomationPeer {}
impl ::core::clone::Clone for IPasswordBoxAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPasswordBoxAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPasswordBoxAutomationPeerFactory {}
impl ::core::clone::Clone for IPasswordBoxAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPersonPictureAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPersonPictureAutomationPeer {}
impl ::core::clone::Clone for IPersonPictureAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPersonPictureAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPersonPictureAutomationPeerFactory {}
impl ::core::clone::Clone for IPersonPictureAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPickerFlyoutPresenterAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPickerFlyoutPresenterAutomationPeer {}
impl ::core::clone::Clone for IPickerFlyoutPresenterAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPivotAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPivotAutomationPeer {}
impl ::core::clone::Clone for IPivotAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPivotAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPivotAutomationPeerFactory {}
impl ::core::clone::Clone for IPivotAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPivotItemAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPivotItemAutomationPeer {}
impl ::core::clone::Clone for IPivotItemAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPivotItemAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPivotItemAutomationPeerFactory {}
impl ::core::clone::Clone for IPivotItemAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPivotItemDataAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPivotItemDataAutomationPeer {}
impl ::core::clone::Clone for IPivotItemDataAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPivotItemDataAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPivotItemDataAutomationPeerFactory {}
impl ::core::clone::Clone for IPivotItemDataAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProgressBarAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProgressBarAutomationPeer {}
impl ::core::clone::Clone for IProgressBarAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProgressBarAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProgressBarAutomationPeerFactory {}
impl ::core::clone::Clone for IProgressBarAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProgressRingAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProgressRingAutomationPeer {}
impl ::core::clone::Clone for IProgressRingAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProgressRingAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProgressRingAutomationPeerFactory {}
impl ::core::clone::Clone for IProgressRingAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRadioButtonAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRadioButtonAutomationPeer {}
impl ::core::clone::Clone for IRadioButtonAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRadioButtonAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRadioButtonAutomationPeerFactory {}
impl ::core::clone::Clone for IRadioButtonAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRangeBaseAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRangeBaseAutomationPeer {}
impl ::core::clone::Clone for IRangeBaseAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRangeBaseAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRangeBaseAutomationPeerFactory {}
impl ::core::clone::Clone for IRangeBaseAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRatingControlAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRatingControlAutomationPeer {}
impl ::core::clone::Clone for IRatingControlAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRatingControlAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRatingControlAutomationPeerFactory {}
impl ::core::clone::Clone for IRatingControlAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRepeatButtonAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRepeatButtonAutomationPeer {}
impl ::core::clone::Clone for IRepeatButtonAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRepeatButtonAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRepeatButtonAutomationPeerFactory {}
impl ::core::clone::Clone for IRepeatButtonAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRichEditBoxAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRichEditBoxAutomationPeer {}
impl ::core::clone::Clone for IRichEditBoxAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRichEditBoxAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRichEditBoxAutomationPeerFactory {}
impl ::core::clone::Clone for IRichEditBoxAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRichTextBlockAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRichTextBlockAutomationPeer {}
impl ::core::clone::Clone for IRichTextBlockAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRichTextBlockAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRichTextBlockAutomationPeerFactory {}
impl ::core::clone::Clone for IRichTextBlockAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRichTextBlockOverflowAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRichTextBlockOverflowAutomationPeer {}
impl ::core::clone::Clone for IRichTextBlockOverflowAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRichTextBlockOverflowAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRichTextBlockOverflowAutomationPeerFactory {}
impl ::core::clone::Clone for IRichTextBlockOverflowAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IScrollBarAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IScrollBarAutomationPeer {}
impl ::core::clone::Clone for IScrollBarAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IScrollBarAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IScrollBarAutomationPeerFactory {}
impl ::core::clone::Clone for IScrollBarAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IScrollViewerAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IScrollViewerAutomationPeer {}
impl ::core::clone::Clone for IScrollViewerAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IScrollViewerAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IScrollViewerAutomationPeerFactory {}
impl ::core::clone::Clone for IScrollViewerAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISearchBoxAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISearchBoxAutomationPeer {}
impl ::core::clone::Clone for ISearchBoxAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISearchBoxAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISearchBoxAutomationPeerFactory {}
impl ::core::clone::Clone for ISearchBoxAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISelectorAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISelectorAutomationPeer {}
impl ::core::clone::Clone for ISelectorAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISelectorAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISelectorAutomationPeerFactory {}
impl ::core::clone::Clone for ISelectorAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISelectorItemAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISelectorItemAutomationPeer {}
impl ::core::clone::Clone for ISelectorItemAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISelectorItemAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISelectorItemAutomationPeerFactory {}
impl ::core::clone::Clone for ISelectorItemAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISemanticZoomAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISemanticZoomAutomationPeer {}
impl ::core::clone::Clone for ISemanticZoomAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISemanticZoomAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISemanticZoomAutomationPeerFactory {}
impl ::core::clone::Clone for ISemanticZoomAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISettingsFlyoutAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISettingsFlyoutAutomationPeer {}
impl ::core::clone::Clone for ISettingsFlyoutAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISettingsFlyoutAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISettingsFlyoutAutomationPeerFactory {}
impl ::core::clone::Clone for ISettingsFlyoutAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISliderAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISliderAutomationPeer {}
impl ::core::clone::Clone for ISliderAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISliderAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISliderAutomationPeerFactory {}
impl ::core::clone::Clone for ISliderAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextBlockAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextBlockAutomationPeer {}
impl ::core::clone::Clone for ITextBlockAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextBlockAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextBlockAutomationPeerFactory {}
impl ::core::clone::Clone for ITextBlockAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextBoxAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextBoxAutomationPeer {}
impl ::core::clone::Clone for ITextBoxAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextBoxAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextBoxAutomationPeerFactory {}
impl ::core::clone::Clone for ITextBoxAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IThumbAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IThumbAutomationPeer {}
impl ::core::clone::Clone for IThumbAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IThumbAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IThumbAutomationPeerFactory {}
impl ::core::clone::Clone for IThumbAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITimePickerAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITimePickerAutomationPeer {}
impl ::core::clone::Clone for ITimePickerAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITimePickerAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITimePickerAutomationPeerFactory {}
impl ::core::clone::Clone for ITimePickerAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITimePickerFlyoutPresenterAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITimePickerFlyoutPresenterAutomationPeer {}
impl ::core::clone::Clone for ITimePickerFlyoutPresenterAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToggleButtonAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToggleButtonAutomationPeer {}
impl ::core::clone::Clone for IToggleButtonAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToggleButtonAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToggleButtonAutomationPeerFactory {}
impl ::core::clone::Clone for IToggleButtonAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToggleMenuFlyoutItemAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToggleMenuFlyoutItemAutomationPeer {}
impl ::core::clone::Clone for IToggleMenuFlyoutItemAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToggleMenuFlyoutItemAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToggleMenuFlyoutItemAutomationPeerFactory {}
impl ::core::clone::Clone for IToggleMenuFlyoutItemAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToggleSwitchAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToggleSwitchAutomationPeer {}
impl ::core::clone::Clone for IToggleSwitchAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToggleSwitchAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToggleSwitchAutomationPeerFactory {}
impl ::core::clone::Clone for IToggleSwitchAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITreeViewItemAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITreeViewItemAutomationPeer {}
impl ::core::clone::Clone for ITreeViewItemAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITreeViewItemAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITreeViewItemAutomationPeerFactory {}
impl ::core::clone::Clone for ITreeViewItemAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITreeViewListAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITreeViewListAutomationPeer {}
impl ::core::clone::Clone for ITreeViewListAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITreeViewListAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITreeViewListAutomationPeerFactory {}
impl ::core::clone::Clone for ITreeViewListAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ImageAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ImageAutomationPeer {}
impl ::core::clone::Clone for ImageAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkToolbarAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for InkToolbarAutomationPeer {}
impl ::core::clone::Clone for InkToolbarAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ItemAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ItemAutomationPeer {}
impl ::core::clone::Clone for ItemAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ItemsControlAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ItemsControlAutomationPeer {}
impl ::core::clone::Clone for ItemsControlAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ListBoxAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ListBoxAutomationPeer {}
impl ::core::clone::Clone for ListBoxAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ListBoxItemAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ListBoxItemAutomationPeer {}
impl ::core::clone::Clone for ListBoxItemAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ListBoxItemDataAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ListBoxItemDataAutomationPeer {}
impl ::core::clone::Clone for ListBoxItemDataAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ListPickerFlyoutPresenterAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ListPickerFlyoutPresenterAutomationPeer {}
impl ::core::clone::Clone for ListPickerFlyoutPresenterAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ListViewAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ListViewAutomationPeer {}
impl ::core::clone::Clone for ListViewAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ListViewBaseAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ListViewBaseAutomationPeer {}
impl ::core::clone::Clone for ListViewBaseAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ListViewBaseHeaderItemAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ListViewBaseHeaderItemAutomationPeer {}
impl ::core::clone::Clone for ListViewBaseHeaderItemAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ListViewHeaderItemAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ListViewHeaderItemAutomationPeer {}
impl ::core::clone::Clone for ListViewHeaderItemAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ListViewItemAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ListViewItemAutomationPeer {}
impl ::core::clone::Clone for ListViewItemAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ListViewItemDataAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ListViewItemDataAutomationPeer {}
impl ::core::clone::Clone for ListViewItemDataAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LoopingSelectorAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LoopingSelectorAutomationPeer {}
impl ::core::clone::Clone for LoopingSelectorAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LoopingSelectorItemAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LoopingSelectorItemAutomationPeer {}
impl ::core::clone::Clone for LoopingSelectorItemAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LoopingSelectorItemDataAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LoopingSelectorItemDataAutomationPeer {}
impl ::core::clone::Clone for LoopingSelectorItemDataAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MapControlAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MapControlAutomationPeer {}
impl ::core::clone::Clone for MapControlAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaElementAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaElementAutomationPeer {}
impl ::core::clone::Clone for MediaElementAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaPlayerElementAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaPlayerElementAutomationPeer {}
impl ::core::clone::Clone for MediaPlayerElementAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaTransportControlsAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaTransportControlsAutomationPeer {}
impl ::core::clone::Clone for MediaTransportControlsAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MenuBarAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MenuBarAutomationPeer {}
impl ::core::clone::Clone for MenuBarAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MenuBarItemAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MenuBarItemAutomationPeer {}
impl ::core::clone::Clone for MenuBarItemAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MenuFlyoutItemAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MenuFlyoutItemAutomationPeer {}
impl ::core::clone::Clone for MenuFlyoutItemAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MenuFlyoutPresenterAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MenuFlyoutPresenterAutomationPeer {}
impl ::core::clone::Clone for MenuFlyoutPresenterAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NavigationViewItemAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for NavigationViewItemAutomationPeer {}
impl ::core::clone::Clone for NavigationViewItemAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PasswordBoxAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PasswordBoxAutomationPeer {}
impl ::core::clone::Clone for PasswordBoxAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PatternInterface(pub i32);
impl PatternInterface {
    pub const Invoke: Self = Self(0i32);
    pub const Selection: Self = Self(1i32);
    pub const Value: Self = Self(2i32);
    pub const RangeValue: Self = Self(3i32);
    pub const Scroll: Self = Self(4i32);
    pub const ScrollItem: Self = Self(5i32);
    pub const ExpandCollapse: Self = Self(6i32);
    pub const Grid: Self = Self(7i32);
    pub const GridItem: Self = Self(8i32);
    pub const MultipleView: Self = Self(9i32);
    pub const Window: Self = Self(10i32);
    pub const SelectionItem: Self = Self(11i32);
    pub const Dock: Self = Self(12i32);
    pub const Table: Self = Self(13i32);
    pub const TableItem: Self = Self(14i32);
    pub const Toggle: Self = Self(15i32);
    pub const Transform: Self = Self(16i32);
    pub const Text: Self = Self(17i32);
    pub const ItemContainer: Self = Self(18i32);
    pub const VirtualizedItem: Self = Self(19i32);
    pub const Text2: Self = Self(20i32);
    pub const TextChild: Self = Self(21i32);
    pub const TextRange: Self = Self(22i32);
    pub const Annotation: Self = Self(23i32);
    pub const Drag: Self = Self(24i32);
    pub const DropTarget: Self = Self(25i32);
    pub const ObjectModel: Self = Self(26i32);
    pub const Spreadsheet: Self = Self(27i32);
    pub const SpreadsheetItem: Self = Self(28i32);
    pub const Styles: Self = Self(29i32);
    pub const Transform2: Self = Self(30i32);
    pub const SynchronizedInput: Self = Self(31i32);
    pub const TextEdit: Self = Self(32i32);
    pub const CustomNavigation: Self = Self(33i32);
}
impl ::core::marker::Copy for PatternInterface {}
impl ::core::clone::Clone for PatternInterface {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PersonPictureAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PersonPictureAutomationPeer {}
impl ::core::clone::Clone for PersonPictureAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PickerFlyoutPresenterAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PickerFlyoutPresenterAutomationPeer {}
impl ::core::clone::Clone for PickerFlyoutPresenterAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PivotAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PivotAutomationPeer {}
impl ::core::clone::Clone for PivotAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PivotItemAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PivotItemAutomationPeer {}
impl ::core::clone::Clone for PivotItemAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PivotItemDataAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PivotItemDataAutomationPeer {}
impl ::core::clone::Clone for PivotItemDataAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ProgressBarAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ProgressBarAutomationPeer {}
impl ::core::clone::Clone for ProgressBarAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ProgressRingAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ProgressRingAutomationPeer {}
impl ::core::clone::Clone for ProgressRingAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RadioButtonAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RadioButtonAutomationPeer {}
impl ::core::clone::Clone for RadioButtonAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RangeBaseAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RangeBaseAutomationPeer {}
impl ::core::clone::Clone for RangeBaseAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RatingControlAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RatingControlAutomationPeer {}
impl ::core::clone::Clone for RatingControlAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct RawElementProviderRuntimeId {
    pub Part1: u32,
    pub Part2: u32,
}
impl ::core::marker::Copy for RawElementProviderRuntimeId {}
impl ::core::clone::Clone for RawElementProviderRuntimeId {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RepeatButtonAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RepeatButtonAutomationPeer {}
impl ::core::clone::Clone for RepeatButtonAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RichEditBoxAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RichEditBoxAutomationPeer {}
impl ::core::clone::Clone for RichEditBoxAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RichTextBlockAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RichTextBlockAutomationPeer {}
impl ::core::clone::Clone for RichTextBlockAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RichTextBlockOverflowAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RichTextBlockOverflowAutomationPeer {}
impl ::core::clone::Clone for RichTextBlockOverflowAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ScrollBarAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ScrollBarAutomationPeer {}
impl ::core::clone::Clone for ScrollBarAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ScrollViewerAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ScrollViewerAutomationPeer {}
impl ::core::clone::Clone for ScrollViewerAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SearchBoxAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SearchBoxAutomationPeer {}
impl ::core::clone::Clone for SearchBoxAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SelectorAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SelectorAutomationPeer {}
impl ::core::clone::Clone for SelectorAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SelectorItemAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SelectorItemAutomationPeer {}
impl ::core::clone::Clone for SelectorItemAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SemanticZoomAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SemanticZoomAutomationPeer {}
impl ::core::clone::Clone for SemanticZoomAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SettingsFlyoutAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SettingsFlyoutAutomationPeer {}
impl ::core::clone::Clone for SettingsFlyoutAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SliderAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SliderAutomationPeer {}
impl ::core::clone::Clone for SliderAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TextBlockAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TextBlockAutomationPeer {}
impl ::core::clone::Clone for TextBlockAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TextBoxAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TextBoxAutomationPeer {}
impl ::core::clone::Clone for TextBoxAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ThumbAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ThumbAutomationPeer {}
impl ::core::clone::Clone for ThumbAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TimePickerAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TimePickerAutomationPeer {}
impl ::core::clone::Clone for TimePickerAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TimePickerFlyoutPresenterAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TimePickerFlyoutPresenterAutomationPeer {}
impl ::core::clone::Clone for TimePickerFlyoutPresenterAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ToggleButtonAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ToggleButtonAutomationPeer {}
impl ::core::clone::Clone for ToggleButtonAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ToggleMenuFlyoutItemAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ToggleMenuFlyoutItemAutomationPeer {}
impl ::core::clone::Clone for ToggleMenuFlyoutItemAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ToggleSwitchAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ToggleSwitchAutomationPeer {}
impl ::core::clone::Clone for ToggleSwitchAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TreeViewItemAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TreeViewItemAutomationPeer {}
impl ::core::clone::Clone for TreeViewItemAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TreeViewListAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TreeViewListAutomationPeer {}
impl ::core::clone::Clone for TreeViewListAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
