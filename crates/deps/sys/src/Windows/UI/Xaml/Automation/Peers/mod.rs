#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AccessibilityView(pub i32);
impl AccessibilityView {
    pub const Raw: Self = Self(0i32);
    pub const Control: Self = Self(1i32);
    pub const Content: Self = Self(2i32);
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
#[repr(transparent)]
pub struct AutomationLiveSetting(pub i32);
impl AutomationLiveSetting {
    pub const Off: Self = Self(0i32);
    pub const Polite: Self = Self(1i32);
    pub const Assertive: Self = Self(2i32);
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
#[repr(transparent)]
pub struct AutomationNotificationKind(pub i32);
impl AutomationNotificationKind {
    pub const ItemAdded: Self = Self(0i32);
    pub const ItemRemoved: Self = Self(1i32);
    pub const ActionCompleted: Self = Self(2i32);
    pub const ActionAborted: Self = Self(3i32);
    pub const Other: Self = Self(4i32);
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
#[repr(transparent)]
pub struct AutomationOrientation(pub i32);
impl AutomationOrientation {
    pub const None: Self = Self(0i32);
    pub const Horizontal: Self = Self(1i32);
    pub const Vertical: Self = Self(2i32);
}
#[repr(transparent)]
pub struct AutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AutomationPeerAnnotation(pub *mut ::core::ffi::c_void);
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
