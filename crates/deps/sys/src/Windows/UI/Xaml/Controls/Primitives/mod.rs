#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct AnimationDirection(i32);
#[repr(transparent)]
pub struct AppBarButtonTemplateSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppBarTemplateSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppBarToggleButtonTemplateSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ButtonBase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CalendarPanel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CalendarViewTemplateSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CarouselPanel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ColorPickerSlider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ColorSpectrum(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ComboBoxTemplateSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CommandBarFlyoutCommandBar(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CommandBarFlyoutCommandBarTemplateSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CommandBarTemplateSettings(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct ComponentResourceLocation(i32);
#[repr(transparent)]
pub struct DragCompletedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DragCompletedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DragDeltaEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DragDeltaEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DragStartedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DragStartedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct EdgeTransitionLocation(i32);
#[repr(transparent)]
pub struct FlyoutBase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FlyoutBaseClosingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct FlyoutPlacementMode(i32);
#[repr(C)]
pub struct FlyoutShowMode(i32);
#[repr(transparent)]
pub struct FlyoutShowOptions(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct GeneratorDirection(i32);
#[repr(C)]
pub struct GeneratorPosition(i32);
#[repr(transparent)]
pub struct GeneratorPositionHelper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GridViewItemPresenter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GridViewItemTemplateSettings(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct GroupHeaderPlacement(i32);
#[repr(transparent)]
pub struct IAppBarButtonTemplateSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBarTemplateSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBarTemplateSettings2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBarToggleButtonTemplateSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IButtonBase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IButtonBaseFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IButtonBaseStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICalendarPanel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICalendarViewTemplateSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICarouselPanel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICarouselPanelFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IColorPickerSlider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IColorPickerSliderFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IColorPickerSliderStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IColorSpectrum(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IColorSpectrumFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IColorSpectrumStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComboBoxTemplateSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComboBoxTemplateSettings2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICommandBarFlyoutCommandBar(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICommandBarFlyoutCommandBarFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICommandBarFlyoutCommandBarTemplateSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICommandBarTemplateSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICommandBarTemplateSettings2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICommandBarTemplateSettings3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICommandBarTemplateSettings4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDragCompletedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDragCompletedEventArgsFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDragDeltaEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDragDeltaEventArgsFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDragStartedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDragStartedEventArgsFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFlyoutBase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFlyoutBase2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFlyoutBase3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFlyoutBase4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFlyoutBase5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFlyoutBase6(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFlyoutBaseClosingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFlyoutBaseFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFlyoutBaseOverrides(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFlyoutBaseOverrides4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFlyoutBaseStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFlyoutBaseStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFlyoutBaseStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFlyoutBaseStatics5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFlyoutBaseStatics6(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFlyoutShowOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFlyoutShowOptionsFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGeneratorPositionHelper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGeneratorPositionHelperStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGridViewItemPresenter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGridViewItemPresenterFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGridViewItemPresenterStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGridViewItemTemplateSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IItemsChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IJumpListItemBackgroundConverter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IJumpListItemBackgroundConverterStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IJumpListItemForegroundConverter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IJumpListItemForegroundConverterStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILayoutInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILayoutInformationStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILayoutInformationStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListViewItemPresenter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListViewItemPresenter2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListViewItemPresenter3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListViewItemPresenter4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListViewItemPresenterFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListViewItemPresenterStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListViewItemPresenterStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListViewItemPresenterStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListViewItemPresenterStatics4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListViewItemTemplateSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILoopingSelector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILoopingSelectorItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILoopingSelectorPanel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILoopingSelectorStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMenuFlyoutItemTemplateSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMenuFlyoutPresenterTemplateSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INavigationViewItemPresenter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INavigationViewItemPresenterFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INavigationViewItemPresenterStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOrientedVirtualizingPanel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOrientedVirtualizingPanelFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPickerFlyoutBase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPickerFlyoutBaseFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPickerFlyoutBaseOverrides(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPickerFlyoutBaseStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPivotHeaderItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPivotHeaderItemFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPivotHeaderPanel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPivotPanel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPopup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPopup2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPopup3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPopup4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPopupStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPopupStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPopupStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPopupStatics4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProgressBarTemplateSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProgressRingTemplateSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRangeBase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRangeBaseFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRangeBaseOverrides(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRangeBaseStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRangeBaseValueChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRepeatButton(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRepeatButtonStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScrollBar(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScrollBarStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScrollEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScrollSnapPointsInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISelector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISelectorFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISelectorItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISelectorItemFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISelectorItemStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISelectorStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISettingsFlyoutTemplateSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISplitViewTemplateSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IThumb(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IThumbStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITickBar(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITickBarStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToggleButton(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToggleButtonFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToggleButtonOverrides(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToggleButtonStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToggleSwitchTemplateSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToolTipTemplateSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ItemsChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ItemsChangedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct JumpListItemBackgroundConverter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct JumpListItemForegroundConverter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LayoutInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ListViewItemPresenter(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct ListViewItemPresenterCheckMode(i32);
#[repr(C)]
pub struct ListViewItemPresenterSelectionIndicatorMode(i32);
#[repr(transparent)]
pub struct ListViewItemTemplateSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LoopingSelector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LoopingSelectorItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LoopingSelectorPanel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MenuFlyoutItemTemplateSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MenuFlyoutPresenterTemplateSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NavigationViewItemPresenter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct OrientedVirtualizingPanel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PickerFlyoutBase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PivotHeaderItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PivotHeaderPanel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PivotPanel(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PlacementMode(i32);
#[repr(transparent)]
pub struct Popup(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PopupPlacementMode(i32);
#[repr(transparent)]
pub struct ProgressBarTemplateSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ProgressRingTemplateSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RangeBase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RangeBaseValueChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RangeBaseValueChangedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RepeatButton(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ScrollBar(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ScrollEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ScrollEventHandler(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct ScrollEventType(i32);
#[repr(C)]
pub struct ScrollingIndicatorMode(i32);
#[repr(transparent)]
pub struct Selector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SelectorItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SettingsFlyoutTemplateSettings(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct SliderSnapsTo(i32);
#[repr(C)]
pub struct SnapPointsAlignment(i32);
#[repr(transparent)]
pub struct SplitViewTemplateSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Thumb(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TickBar(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct TickPlacement(i32);
#[repr(transparent)]
pub struct ToggleButton(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ToggleSwitchTemplateSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ToolTipTemplateSettings(pub *mut ::core::ffi::c_void);
