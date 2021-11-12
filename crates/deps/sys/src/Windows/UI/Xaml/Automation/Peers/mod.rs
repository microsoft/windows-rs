#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AccessibilityView(pub i32);
impl AccessibilityView {
    pub const Raw: AccessibilityView = AccessibilityView(0i32);
    pub const Control: AccessibilityView = AccessibilityView(1i32);
    pub const Content: AccessibilityView = AccessibilityView(2i32);
}
#[repr(transparent)]
pub struct AppBarAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppBarButtonAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppBarToggleButtonAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AutoSuggestBoxAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AutomationControlType(pub i32);
impl AutomationControlType {
    pub const Button: AutomationControlType = AutomationControlType(0i32);
    pub const Calendar: AutomationControlType = AutomationControlType(1i32);
    pub const CheckBox: AutomationControlType = AutomationControlType(2i32);
    pub const ComboBox: AutomationControlType = AutomationControlType(3i32);
    pub const Edit: AutomationControlType = AutomationControlType(4i32);
    pub const Hyperlink: AutomationControlType = AutomationControlType(5i32);
    pub const Image: AutomationControlType = AutomationControlType(6i32);
    pub const ListItem: AutomationControlType = AutomationControlType(7i32);
    pub const List: AutomationControlType = AutomationControlType(8i32);
    pub const Menu: AutomationControlType = AutomationControlType(9i32);
    pub const MenuBar: AutomationControlType = AutomationControlType(10i32);
    pub const MenuItem: AutomationControlType = AutomationControlType(11i32);
    pub const ProgressBar: AutomationControlType = AutomationControlType(12i32);
    pub const RadioButton: AutomationControlType = AutomationControlType(13i32);
    pub const ScrollBar: AutomationControlType = AutomationControlType(14i32);
    pub const Slider: AutomationControlType = AutomationControlType(15i32);
    pub const Spinner: AutomationControlType = AutomationControlType(16i32);
    pub const StatusBar: AutomationControlType = AutomationControlType(17i32);
    pub const Tab: AutomationControlType = AutomationControlType(18i32);
    pub const TabItem: AutomationControlType = AutomationControlType(19i32);
    pub const Text: AutomationControlType = AutomationControlType(20i32);
    pub const ToolBar: AutomationControlType = AutomationControlType(21i32);
    pub const ToolTip: AutomationControlType = AutomationControlType(22i32);
    pub const Tree: AutomationControlType = AutomationControlType(23i32);
    pub const TreeItem: AutomationControlType = AutomationControlType(24i32);
    pub const Custom: AutomationControlType = AutomationControlType(25i32);
    pub const Group: AutomationControlType = AutomationControlType(26i32);
    pub const Thumb: AutomationControlType = AutomationControlType(27i32);
    pub const DataGrid: AutomationControlType = AutomationControlType(28i32);
    pub const DataItem: AutomationControlType = AutomationControlType(29i32);
    pub const Document: AutomationControlType = AutomationControlType(30i32);
    pub const SplitButton: AutomationControlType = AutomationControlType(31i32);
    pub const Window: AutomationControlType = AutomationControlType(32i32);
    pub const Pane: AutomationControlType = AutomationControlType(33i32);
    pub const Header: AutomationControlType = AutomationControlType(34i32);
    pub const HeaderItem: AutomationControlType = AutomationControlType(35i32);
    pub const Table: AutomationControlType = AutomationControlType(36i32);
    pub const TitleBar: AutomationControlType = AutomationControlType(37i32);
    pub const Separator: AutomationControlType = AutomationControlType(38i32);
    pub const SemanticZoom: AutomationControlType = AutomationControlType(39i32);
    pub const AppBar: AutomationControlType = AutomationControlType(40i32);
}
#[repr(transparent)]
pub struct AutomationEvents(pub i32);
impl AutomationEvents {
    pub const ToolTipOpened: AutomationEvents = AutomationEvents(0i32);
    pub const ToolTipClosed: AutomationEvents = AutomationEvents(1i32);
    pub const MenuOpened: AutomationEvents = AutomationEvents(2i32);
    pub const MenuClosed: AutomationEvents = AutomationEvents(3i32);
    pub const AutomationFocusChanged: AutomationEvents = AutomationEvents(4i32);
    pub const InvokePatternOnInvoked: AutomationEvents = AutomationEvents(5i32);
    pub const SelectionItemPatternOnElementAddedToSelection: AutomationEvents = AutomationEvents(6i32);
    pub const SelectionItemPatternOnElementRemovedFromSelection: AutomationEvents = AutomationEvents(7i32);
    pub const SelectionItemPatternOnElementSelected: AutomationEvents = AutomationEvents(8i32);
    pub const SelectionPatternOnInvalidated: AutomationEvents = AutomationEvents(9i32);
    pub const TextPatternOnTextSelectionChanged: AutomationEvents = AutomationEvents(10i32);
    pub const TextPatternOnTextChanged: AutomationEvents = AutomationEvents(11i32);
    pub const AsyncContentLoaded: AutomationEvents = AutomationEvents(12i32);
    pub const PropertyChanged: AutomationEvents = AutomationEvents(13i32);
    pub const StructureChanged: AutomationEvents = AutomationEvents(14i32);
    pub const DragStart: AutomationEvents = AutomationEvents(15i32);
    pub const DragCancel: AutomationEvents = AutomationEvents(16i32);
    pub const DragComplete: AutomationEvents = AutomationEvents(17i32);
    pub const DragEnter: AutomationEvents = AutomationEvents(18i32);
    pub const DragLeave: AutomationEvents = AutomationEvents(19i32);
    pub const Dropped: AutomationEvents = AutomationEvents(20i32);
    pub const LiveRegionChanged: AutomationEvents = AutomationEvents(21i32);
    pub const InputReachedTarget: AutomationEvents = AutomationEvents(22i32);
    pub const InputReachedOtherElement: AutomationEvents = AutomationEvents(23i32);
    pub const InputDiscarded: AutomationEvents = AutomationEvents(24i32);
    pub const WindowClosed: AutomationEvents = AutomationEvents(25i32);
    pub const WindowOpened: AutomationEvents = AutomationEvents(26i32);
    pub const ConversionTargetChanged: AutomationEvents = AutomationEvents(27i32);
    pub const TextEditTextChanged: AutomationEvents = AutomationEvents(28i32);
    pub const LayoutInvalidated: AutomationEvents = AutomationEvents(29i32);
}
#[repr(transparent)]
pub struct AutomationHeadingLevel(pub i32);
impl AutomationHeadingLevel {
    pub const None: AutomationHeadingLevel = AutomationHeadingLevel(0i32);
    pub const Level1: AutomationHeadingLevel = AutomationHeadingLevel(1i32);
    pub const Level2: AutomationHeadingLevel = AutomationHeadingLevel(2i32);
    pub const Level3: AutomationHeadingLevel = AutomationHeadingLevel(3i32);
    pub const Level4: AutomationHeadingLevel = AutomationHeadingLevel(4i32);
    pub const Level5: AutomationHeadingLevel = AutomationHeadingLevel(5i32);
    pub const Level6: AutomationHeadingLevel = AutomationHeadingLevel(6i32);
    pub const Level7: AutomationHeadingLevel = AutomationHeadingLevel(7i32);
    pub const Level8: AutomationHeadingLevel = AutomationHeadingLevel(8i32);
    pub const Level9: AutomationHeadingLevel = AutomationHeadingLevel(9i32);
}
#[repr(transparent)]
pub struct AutomationLandmarkType(pub i32);
impl AutomationLandmarkType {
    pub const None: AutomationLandmarkType = AutomationLandmarkType(0i32);
    pub const Custom: AutomationLandmarkType = AutomationLandmarkType(1i32);
    pub const Form: AutomationLandmarkType = AutomationLandmarkType(2i32);
    pub const Main: AutomationLandmarkType = AutomationLandmarkType(3i32);
    pub const Navigation: AutomationLandmarkType = AutomationLandmarkType(4i32);
    pub const Search: AutomationLandmarkType = AutomationLandmarkType(5i32);
}
#[repr(transparent)]
pub struct AutomationLiveSetting(pub i32);
impl AutomationLiveSetting {
    pub const Off: AutomationLiveSetting = AutomationLiveSetting(0i32);
    pub const Polite: AutomationLiveSetting = AutomationLiveSetting(1i32);
    pub const Assertive: AutomationLiveSetting = AutomationLiveSetting(2i32);
}
#[repr(transparent)]
pub struct AutomationNavigationDirection(pub i32);
impl AutomationNavigationDirection {
    pub const Parent: AutomationNavigationDirection = AutomationNavigationDirection(0i32);
    pub const NextSibling: AutomationNavigationDirection = AutomationNavigationDirection(1i32);
    pub const PreviousSibling: AutomationNavigationDirection = AutomationNavigationDirection(2i32);
    pub const FirstChild: AutomationNavigationDirection = AutomationNavigationDirection(3i32);
    pub const LastChild: AutomationNavigationDirection = AutomationNavigationDirection(4i32);
}
#[repr(transparent)]
pub struct AutomationNotificationKind(pub i32);
impl AutomationNotificationKind {
    pub const ItemAdded: AutomationNotificationKind = AutomationNotificationKind(0i32);
    pub const ItemRemoved: AutomationNotificationKind = AutomationNotificationKind(1i32);
    pub const ActionCompleted: AutomationNotificationKind = AutomationNotificationKind(2i32);
    pub const ActionAborted: AutomationNotificationKind = AutomationNotificationKind(3i32);
    pub const Other: AutomationNotificationKind = AutomationNotificationKind(4i32);
}
#[repr(transparent)]
pub struct AutomationNotificationProcessing(pub i32);
impl AutomationNotificationProcessing {
    pub const ImportantAll: AutomationNotificationProcessing = AutomationNotificationProcessing(0i32);
    pub const ImportantMostRecent: AutomationNotificationProcessing = AutomationNotificationProcessing(1i32);
    pub const All: AutomationNotificationProcessing = AutomationNotificationProcessing(2i32);
    pub const MostRecent: AutomationNotificationProcessing = AutomationNotificationProcessing(3i32);
    pub const CurrentThenMostRecent: AutomationNotificationProcessing = AutomationNotificationProcessing(4i32);
}
#[repr(transparent)]
pub struct AutomationOrientation(pub i32);
impl AutomationOrientation {
    pub const None: AutomationOrientation = AutomationOrientation(0i32);
    pub const Horizontal: AutomationOrientation = AutomationOrientation(1i32);
    pub const Vertical: AutomationOrientation = AutomationOrientation(2i32);
}
#[repr(transparent)]
pub struct AutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AutomationPeerAnnotation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AutomationStructureChangeType(pub i32);
impl AutomationStructureChangeType {
    pub const ChildAdded: AutomationStructureChangeType = AutomationStructureChangeType(0i32);
    pub const ChildRemoved: AutomationStructureChangeType = AutomationStructureChangeType(1i32);
    pub const ChildrenInvalidated: AutomationStructureChangeType = AutomationStructureChangeType(2i32);
    pub const ChildrenBulkAdded: AutomationStructureChangeType = AutomationStructureChangeType(3i32);
    pub const ChildrenBulkRemoved: AutomationStructureChangeType = AutomationStructureChangeType(4i32);
    pub const ChildrenReordered: AutomationStructureChangeType = AutomationStructureChangeType(5i32);
}
#[repr(transparent)]
pub struct ButtonAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ButtonBaseAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CalendarDatePickerAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CaptureElementAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CheckBoxAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ColorPickerSliderAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ColorSpectrumAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ComboBoxAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ComboBoxItemAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ComboBoxItemDataAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DatePickerAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DatePickerFlyoutPresenterAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FlipViewAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FlipViewItemAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FlipViewItemDataAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FlyoutPresenterAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FrameworkElementAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GridViewAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GridViewHeaderItemAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GridViewItemAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GridViewItemDataAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GroupItemAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HubAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HubSectionAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HyperlinkButtonAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBarAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBarAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBarButtonAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBarButtonAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBarToggleButtonAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBarToggleButtonAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutoSuggestBoxAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutoSuggestBoxAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutomationPeer2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutomationPeer3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutomationPeer4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutomationPeer5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutomationPeer6(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutomationPeer7(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutomationPeer8(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutomationPeer9(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutomationPeerAnnotation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutomationPeerAnnotationFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutomationPeerAnnotationStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutomationPeerOverrides(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutomationPeerOverrides2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutomationPeerOverrides3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutomationPeerOverrides4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutomationPeerOverrides5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutomationPeerOverrides6(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutomationPeerOverrides8(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutomationPeerOverrides9(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutomationPeerProtected(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutomationPeerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutomationPeerStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IButtonAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IButtonAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IButtonBaseAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IButtonBaseAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICalendarDatePickerAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICalendarDatePickerAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICaptureElementAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICaptureElementAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICheckBoxAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICheckBoxAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IColorPickerSliderAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IColorPickerSliderAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IColorSpectrumAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IColorSpectrumAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComboBoxAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComboBoxAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComboBoxItemAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComboBoxItemAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComboBoxItemDataAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComboBoxItemDataAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDatePickerAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDatePickerAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDatePickerFlyoutPresenterAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFlipViewAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFlipViewAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFlipViewItemAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFlipViewItemAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFlipViewItemDataAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFlipViewItemDataAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFlyoutPresenterAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFlyoutPresenterAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFrameworkElementAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFrameworkElementAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFrameworkElementAutomationPeerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGridViewAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGridViewAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGridViewHeaderItemAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGridViewHeaderItemAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGridViewItemAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGridViewItemAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGridViewItemDataAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGridViewItemDataAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGroupItemAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGroupItemAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHubAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHubAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHubSectionAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHubSectionAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHyperlinkButtonAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHyperlinkButtonAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IImageAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IImageAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkToolbarAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IItemAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IItemAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IItemsControlAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IItemsControlAutomationPeer2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IItemsControlAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IItemsControlAutomationPeerOverrides2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListBoxAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListBoxAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListBoxItemAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListBoxItemAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListBoxItemDataAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListBoxItemDataAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListPickerFlyoutPresenterAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListViewAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListViewAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListViewBaseAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListViewBaseAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListViewBaseHeaderItemAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListViewBaseHeaderItemAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListViewHeaderItemAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListViewHeaderItemAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListViewItemAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListViewItemAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListViewItemDataAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListViewItemDataAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILoopingSelectorAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILoopingSelectorItemAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILoopingSelectorItemDataAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapControlAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaElementAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaElementAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaPlayerElementAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaPlayerElementAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaTransportControlsAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaTransportControlsAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMenuBarAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMenuBarAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMenuBarItemAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMenuBarItemAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMenuFlyoutItemAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMenuFlyoutItemAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMenuFlyoutPresenterAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMenuFlyoutPresenterAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INavigationViewItemAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INavigationViewItemAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPasswordBoxAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPasswordBoxAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPersonPictureAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPersonPictureAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPickerFlyoutPresenterAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPivotAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPivotAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPivotItemAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPivotItemAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPivotItemDataAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPivotItemDataAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProgressBarAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProgressBarAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProgressRingAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProgressRingAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRadioButtonAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRadioButtonAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRangeBaseAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRangeBaseAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRatingControlAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRatingControlAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRepeatButtonAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRepeatButtonAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRichEditBoxAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRichEditBoxAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRichTextBlockAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRichTextBlockAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRichTextBlockOverflowAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRichTextBlockOverflowAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScrollBarAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScrollBarAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScrollViewerAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScrollViewerAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISearchBoxAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISearchBoxAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISelectorAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISelectorAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISelectorItemAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISelectorItemAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISemanticZoomAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISemanticZoomAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISettingsFlyoutAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISettingsFlyoutAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISliderAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISliderAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextBlockAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextBlockAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextBoxAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextBoxAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IThumbAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IThumbAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimePickerAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimePickerAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimePickerFlyoutPresenterAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToggleButtonAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToggleButtonAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToggleMenuFlyoutItemAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToggleMenuFlyoutItemAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToggleSwitchAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToggleSwitchAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITreeViewItemAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITreeViewItemAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITreeViewListAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITreeViewListAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ImageAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkToolbarAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ItemAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ItemsControlAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ListBoxAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ListBoxItemAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ListBoxItemDataAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ListPickerFlyoutPresenterAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ListViewAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ListViewBaseAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ListViewBaseHeaderItemAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ListViewHeaderItemAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ListViewItemAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ListViewItemDataAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LoopingSelectorAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LoopingSelectorItemAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LoopingSelectorItemDataAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapControlAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaElementAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaPlayerElementAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaTransportControlsAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MenuBarAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MenuBarItemAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MenuFlyoutItemAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MenuFlyoutPresenterAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NavigationViewItemAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PasswordBoxAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PatternInterface(pub i32);
impl PatternInterface {
    pub const Invoke: PatternInterface = PatternInterface(0i32);
    pub const Selection: PatternInterface = PatternInterface(1i32);
    pub const Value: PatternInterface = PatternInterface(2i32);
    pub const RangeValue: PatternInterface = PatternInterface(3i32);
    pub const Scroll: PatternInterface = PatternInterface(4i32);
    pub const ScrollItem: PatternInterface = PatternInterface(5i32);
    pub const ExpandCollapse: PatternInterface = PatternInterface(6i32);
    pub const Grid: PatternInterface = PatternInterface(7i32);
    pub const GridItem: PatternInterface = PatternInterface(8i32);
    pub const MultipleView: PatternInterface = PatternInterface(9i32);
    pub const Window: PatternInterface = PatternInterface(10i32);
    pub const SelectionItem: PatternInterface = PatternInterface(11i32);
    pub const Dock: PatternInterface = PatternInterface(12i32);
    pub const Table: PatternInterface = PatternInterface(13i32);
    pub const TableItem: PatternInterface = PatternInterface(14i32);
    pub const Toggle: PatternInterface = PatternInterface(15i32);
    pub const Transform: PatternInterface = PatternInterface(16i32);
    pub const Text: PatternInterface = PatternInterface(17i32);
    pub const ItemContainer: PatternInterface = PatternInterface(18i32);
    pub const VirtualizedItem: PatternInterface = PatternInterface(19i32);
    pub const Text2: PatternInterface = PatternInterface(20i32);
    pub const TextChild: PatternInterface = PatternInterface(21i32);
    pub const TextRange: PatternInterface = PatternInterface(22i32);
    pub const Annotation: PatternInterface = PatternInterface(23i32);
    pub const Drag: PatternInterface = PatternInterface(24i32);
    pub const DropTarget: PatternInterface = PatternInterface(25i32);
    pub const ObjectModel: PatternInterface = PatternInterface(26i32);
    pub const Spreadsheet: PatternInterface = PatternInterface(27i32);
    pub const SpreadsheetItem: PatternInterface = PatternInterface(28i32);
    pub const Styles: PatternInterface = PatternInterface(29i32);
    pub const Transform2: PatternInterface = PatternInterface(30i32);
    pub const SynchronizedInput: PatternInterface = PatternInterface(31i32);
    pub const TextEdit: PatternInterface = PatternInterface(32i32);
    pub const CustomNavigation: PatternInterface = PatternInterface(33i32);
}
#[repr(transparent)]
pub struct PersonPictureAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PickerFlyoutPresenterAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PivotAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PivotItemAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PivotItemDataAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ProgressBarAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ProgressRingAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RadioButtonAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RangeBaseAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RatingControlAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct RawElementProviderRuntimeId(i32);
#[repr(transparent)]
pub struct RepeatButtonAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RichEditBoxAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RichTextBlockAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RichTextBlockOverflowAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ScrollBarAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ScrollViewerAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SearchBoxAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SelectorAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SelectorItemAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SemanticZoomAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SettingsFlyoutAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SliderAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TextBlockAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TextBoxAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ThumbAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TimePickerAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TimePickerFlyoutPresenterAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ToggleButtonAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ToggleMenuFlyoutItemAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ToggleSwitchAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TreeViewItemAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TreeViewListAutomationPeer(pub *mut ::core::ffi::c_void);
