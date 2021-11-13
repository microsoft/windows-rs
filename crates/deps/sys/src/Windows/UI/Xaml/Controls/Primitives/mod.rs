#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AnimationDirection(pub i32);
impl AnimationDirection {
    pub const Left: Self = Self(0i32);
    pub const Top: Self = Self(1i32);
    pub const Right: Self = Self(2i32);
    pub const Bottom: Self = Self(3i32);
}
impl ::core::marker::Copy for AnimationDirection {}
impl ::core::clone::Clone for AnimationDirection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppBarButtonTemplateSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppBarButtonTemplateSettings {}
impl ::core::clone::Clone for AppBarButtonTemplateSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppBarTemplateSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppBarTemplateSettings {}
impl ::core::clone::Clone for AppBarTemplateSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppBarToggleButtonTemplateSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppBarToggleButtonTemplateSettings {}
impl ::core::clone::Clone for AppBarToggleButtonTemplateSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ButtonBase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ButtonBase {}
impl ::core::clone::Clone for ButtonBase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CalendarPanel(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CalendarPanel {}
impl ::core::clone::Clone for CalendarPanel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CalendarViewTemplateSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CalendarViewTemplateSettings {}
impl ::core::clone::Clone for CalendarViewTemplateSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CarouselPanel(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CarouselPanel {}
impl ::core::clone::Clone for CarouselPanel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ColorPickerSlider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ColorPickerSlider {}
impl ::core::clone::Clone for ColorPickerSlider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ColorSpectrum(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ColorSpectrum {}
impl ::core::clone::Clone for ColorSpectrum {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ComboBoxTemplateSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ComboBoxTemplateSettings {}
impl ::core::clone::Clone for ComboBoxTemplateSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CommandBarFlyoutCommandBar(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CommandBarFlyoutCommandBar {}
impl ::core::clone::Clone for CommandBarFlyoutCommandBar {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CommandBarFlyoutCommandBarTemplateSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CommandBarFlyoutCommandBarTemplateSettings {}
impl ::core::clone::Clone for CommandBarFlyoutCommandBarTemplateSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CommandBarTemplateSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CommandBarTemplateSettings {}
impl ::core::clone::Clone for CommandBarTemplateSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ComponentResourceLocation(pub i32);
impl ComponentResourceLocation {
    pub const Application: Self = Self(0i32);
    pub const Nested: Self = Self(1i32);
}
impl ::core::marker::Copy for ComponentResourceLocation {}
impl ::core::clone::Clone for ComponentResourceLocation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DragCompletedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DragCompletedEventArgs {}
impl ::core::clone::Clone for DragCompletedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DragCompletedEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DragCompletedEventHandler {}
impl ::core::clone::Clone for DragCompletedEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DragDeltaEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DragDeltaEventArgs {}
impl ::core::clone::Clone for DragDeltaEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DragDeltaEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DragDeltaEventHandler {}
impl ::core::clone::Clone for DragDeltaEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DragStartedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DragStartedEventArgs {}
impl ::core::clone::Clone for DragStartedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DragStartedEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DragStartedEventHandler {}
impl ::core::clone::Clone for DragStartedEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EdgeTransitionLocation(pub i32);
impl EdgeTransitionLocation {
    pub const Left: Self = Self(0i32);
    pub const Top: Self = Self(1i32);
    pub const Right: Self = Self(2i32);
    pub const Bottom: Self = Self(3i32);
}
impl ::core::marker::Copy for EdgeTransitionLocation {}
impl ::core::clone::Clone for EdgeTransitionLocation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FlyoutBase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FlyoutBase {}
impl ::core::clone::Clone for FlyoutBase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FlyoutBaseClosingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FlyoutBaseClosingEventArgs {}
impl ::core::clone::Clone for FlyoutBaseClosingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FlyoutPlacementMode(pub i32);
impl FlyoutPlacementMode {
    pub const Top: Self = Self(0i32);
    pub const Bottom: Self = Self(1i32);
    pub const Left: Self = Self(2i32);
    pub const Right: Self = Self(3i32);
    pub const Full: Self = Self(4i32);
    pub const TopEdgeAlignedLeft: Self = Self(5i32);
    pub const TopEdgeAlignedRight: Self = Self(6i32);
    pub const BottomEdgeAlignedLeft: Self = Self(7i32);
    pub const BottomEdgeAlignedRight: Self = Self(8i32);
    pub const LeftEdgeAlignedTop: Self = Self(9i32);
    pub const LeftEdgeAlignedBottom: Self = Self(10i32);
    pub const RightEdgeAlignedTop: Self = Self(11i32);
    pub const RightEdgeAlignedBottom: Self = Self(12i32);
    pub const Auto: Self = Self(13i32);
}
impl ::core::marker::Copy for FlyoutPlacementMode {}
impl ::core::clone::Clone for FlyoutPlacementMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FlyoutShowMode(pub i32);
impl FlyoutShowMode {
    pub const Auto: Self = Self(0i32);
    pub const Standard: Self = Self(1i32);
    pub const Transient: Self = Self(2i32);
    pub const TransientWithDismissOnPointerMoveAway: Self = Self(3i32);
}
impl ::core::marker::Copy for FlyoutShowMode {}
impl ::core::clone::Clone for FlyoutShowMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FlyoutShowOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FlyoutShowOptions {}
impl ::core::clone::Clone for FlyoutShowOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GeneratorDirection(pub i32);
impl GeneratorDirection {
    pub const Forward: Self = Self(0i32);
    pub const Backward: Self = Self(1i32);
}
impl ::core::marker::Copy for GeneratorDirection {}
impl ::core::clone::Clone for GeneratorDirection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct GeneratorPosition {
    pub Index: i32,
    pub Offset: i32,
}
impl ::core::marker::Copy for GeneratorPosition {}
impl ::core::clone::Clone for GeneratorPosition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GeneratorPositionHelper(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GeneratorPositionHelper {}
impl ::core::clone::Clone for GeneratorPositionHelper {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GridViewItemPresenter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GridViewItemPresenter {}
impl ::core::clone::Clone for GridViewItemPresenter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GridViewItemTemplateSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GridViewItemTemplateSettings {}
impl ::core::clone::Clone for GridViewItemTemplateSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GroupHeaderPlacement(pub i32);
impl GroupHeaderPlacement {
    pub const Top: Self = Self(0i32);
    pub const Left: Self = Self(1i32);
}
impl ::core::marker::Copy for GroupHeaderPlacement {}
impl ::core::clone::Clone for GroupHeaderPlacement {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBarButtonTemplateSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBarButtonTemplateSettings {}
impl ::core::clone::Clone for IAppBarButtonTemplateSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBarTemplateSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBarTemplateSettings {}
impl ::core::clone::Clone for IAppBarTemplateSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBarTemplateSettings2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBarTemplateSettings2 {}
impl ::core::clone::Clone for IAppBarTemplateSettings2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBarToggleButtonTemplateSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBarToggleButtonTemplateSettings {}
impl ::core::clone::Clone for IAppBarToggleButtonTemplateSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IButtonBase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IButtonBase {}
impl ::core::clone::Clone for IButtonBase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IButtonBaseFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IButtonBaseFactory {}
impl ::core::clone::Clone for IButtonBaseFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IButtonBaseStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IButtonBaseStatics {}
impl ::core::clone::Clone for IButtonBaseStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICalendarPanel(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICalendarPanel {}
impl ::core::clone::Clone for ICalendarPanel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICalendarViewTemplateSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICalendarViewTemplateSettings {}
impl ::core::clone::Clone for ICalendarViewTemplateSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICarouselPanel(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICarouselPanel {}
impl ::core::clone::Clone for ICarouselPanel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICarouselPanelFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICarouselPanelFactory {}
impl ::core::clone::Clone for ICarouselPanelFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IColorPickerSlider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IColorPickerSlider {}
impl ::core::clone::Clone for IColorPickerSlider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IColorPickerSliderFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IColorPickerSliderFactory {}
impl ::core::clone::Clone for IColorPickerSliderFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IColorPickerSliderStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IColorPickerSliderStatics {}
impl ::core::clone::Clone for IColorPickerSliderStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IColorSpectrum(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IColorSpectrum {}
impl ::core::clone::Clone for IColorSpectrum {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IColorSpectrumFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IColorSpectrumFactory {}
impl ::core::clone::Clone for IColorSpectrumFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IColorSpectrumStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IColorSpectrumStatics {}
impl ::core::clone::Clone for IColorSpectrumStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComboBoxTemplateSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComboBoxTemplateSettings {}
impl ::core::clone::Clone for IComboBoxTemplateSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComboBoxTemplateSettings2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComboBoxTemplateSettings2 {}
impl ::core::clone::Clone for IComboBoxTemplateSettings2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICommandBarFlyoutCommandBar(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICommandBarFlyoutCommandBar {}
impl ::core::clone::Clone for ICommandBarFlyoutCommandBar {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICommandBarFlyoutCommandBarFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICommandBarFlyoutCommandBarFactory {}
impl ::core::clone::Clone for ICommandBarFlyoutCommandBarFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICommandBarFlyoutCommandBarTemplateSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICommandBarFlyoutCommandBarTemplateSettings {}
impl ::core::clone::Clone for ICommandBarFlyoutCommandBarTemplateSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICommandBarTemplateSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICommandBarTemplateSettings {}
impl ::core::clone::Clone for ICommandBarTemplateSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICommandBarTemplateSettings2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICommandBarTemplateSettings2 {}
impl ::core::clone::Clone for ICommandBarTemplateSettings2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICommandBarTemplateSettings3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICommandBarTemplateSettings3 {}
impl ::core::clone::Clone for ICommandBarTemplateSettings3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICommandBarTemplateSettings4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICommandBarTemplateSettings4 {}
impl ::core::clone::Clone for ICommandBarTemplateSettings4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDragCompletedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDragCompletedEventArgs {}
impl ::core::clone::Clone for IDragCompletedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDragCompletedEventArgsFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDragCompletedEventArgsFactory {}
impl ::core::clone::Clone for IDragCompletedEventArgsFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDragDeltaEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDragDeltaEventArgs {}
impl ::core::clone::Clone for IDragDeltaEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDragDeltaEventArgsFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDragDeltaEventArgsFactory {}
impl ::core::clone::Clone for IDragDeltaEventArgsFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDragStartedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDragStartedEventArgs {}
impl ::core::clone::Clone for IDragStartedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDragStartedEventArgsFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDragStartedEventArgsFactory {}
impl ::core::clone::Clone for IDragStartedEventArgsFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFlyoutBase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFlyoutBase {}
impl ::core::clone::Clone for IFlyoutBase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFlyoutBase2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFlyoutBase2 {}
impl ::core::clone::Clone for IFlyoutBase2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFlyoutBase3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFlyoutBase3 {}
impl ::core::clone::Clone for IFlyoutBase3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFlyoutBase4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFlyoutBase4 {}
impl ::core::clone::Clone for IFlyoutBase4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFlyoutBase5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFlyoutBase5 {}
impl ::core::clone::Clone for IFlyoutBase5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFlyoutBase6(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFlyoutBase6 {}
impl ::core::clone::Clone for IFlyoutBase6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFlyoutBaseClosingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFlyoutBaseClosingEventArgs {}
impl ::core::clone::Clone for IFlyoutBaseClosingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFlyoutBaseFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFlyoutBaseFactory {}
impl ::core::clone::Clone for IFlyoutBaseFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFlyoutBaseOverrides(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFlyoutBaseOverrides {}
impl ::core::clone::Clone for IFlyoutBaseOverrides {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFlyoutBaseOverrides4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFlyoutBaseOverrides4 {}
impl ::core::clone::Clone for IFlyoutBaseOverrides4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFlyoutBaseStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFlyoutBaseStatics {}
impl ::core::clone::Clone for IFlyoutBaseStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFlyoutBaseStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFlyoutBaseStatics2 {}
impl ::core::clone::Clone for IFlyoutBaseStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFlyoutBaseStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFlyoutBaseStatics3 {}
impl ::core::clone::Clone for IFlyoutBaseStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFlyoutBaseStatics5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFlyoutBaseStatics5 {}
impl ::core::clone::Clone for IFlyoutBaseStatics5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFlyoutBaseStatics6(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFlyoutBaseStatics6 {}
impl ::core::clone::Clone for IFlyoutBaseStatics6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFlyoutShowOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFlyoutShowOptions {}
impl ::core::clone::Clone for IFlyoutShowOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFlyoutShowOptionsFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFlyoutShowOptionsFactory {}
impl ::core::clone::Clone for IFlyoutShowOptionsFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGeneratorPositionHelper(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGeneratorPositionHelper {}
impl ::core::clone::Clone for IGeneratorPositionHelper {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGeneratorPositionHelperStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGeneratorPositionHelperStatics {}
impl ::core::clone::Clone for IGeneratorPositionHelperStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGridViewItemPresenter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGridViewItemPresenter {}
impl ::core::clone::Clone for IGridViewItemPresenter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGridViewItemPresenterFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGridViewItemPresenterFactory {}
impl ::core::clone::Clone for IGridViewItemPresenterFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGridViewItemPresenterStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGridViewItemPresenterStatics {}
impl ::core::clone::Clone for IGridViewItemPresenterStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGridViewItemTemplateSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGridViewItemTemplateSettings {}
impl ::core::clone::Clone for IGridViewItemTemplateSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IItemsChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IItemsChangedEventArgs {}
impl ::core::clone::Clone for IItemsChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IJumpListItemBackgroundConverter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IJumpListItemBackgroundConverter {}
impl ::core::clone::Clone for IJumpListItemBackgroundConverter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IJumpListItemBackgroundConverterStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IJumpListItemBackgroundConverterStatics {}
impl ::core::clone::Clone for IJumpListItemBackgroundConverterStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IJumpListItemForegroundConverter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IJumpListItemForegroundConverter {}
impl ::core::clone::Clone for IJumpListItemForegroundConverter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IJumpListItemForegroundConverterStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IJumpListItemForegroundConverterStatics {}
impl ::core::clone::Clone for IJumpListItemForegroundConverterStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILayoutInformation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILayoutInformation {}
impl ::core::clone::Clone for ILayoutInformation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILayoutInformationStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILayoutInformationStatics {}
impl ::core::clone::Clone for ILayoutInformationStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILayoutInformationStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILayoutInformationStatics2 {}
impl ::core::clone::Clone for ILayoutInformationStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListViewItemPresenter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListViewItemPresenter {}
impl ::core::clone::Clone for IListViewItemPresenter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListViewItemPresenter2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListViewItemPresenter2 {}
impl ::core::clone::Clone for IListViewItemPresenter2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListViewItemPresenter3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListViewItemPresenter3 {}
impl ::core::clone::Clone for IListViewItemPresenter3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListViewItemPresenter4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListViewItemPresenter4 {}
impl ::core::clone::Clone for IListViewItemPresenter4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListViewItemPresenterFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListViewItemPresenterFactory {}
impl ::core::clone::Clone for IListViewItemPresenterFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListViewItemPresenterStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListViewItemPresenterStatics {}
impl ::core::clone::Clone for IListViewItemPresenterStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListViewItemPresenterStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListViewItemPresenterStatics2 {}
impl ::core::clone::Clone for IListViewItemPresenterStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListViewItemPresenterStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListViewItemPresenterStatics3 {}
impl ::core::clone::Clone for IListViewItemPresenterStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListViewItemPresenterStatics4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListViewItemPresenterStatics4 {}
impl ::core::clone::Clone for IListViewItemPresenterStatics4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListViewItemTemplateSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListViewItemTemplateSettings {}
impl ::core::clone::Clone for IListViewItemTemplateSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILoopingSelector(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILoopingSelector {}
impl ::core::clone::Clone for ILoopingSelector {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILoopingSelectorItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILoopingSelectorItem {}
impl ::core::clone::Clone for ILoopingSelectorItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILoopingSelectorPanel(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILoopingSelectorPanel {}
impl ::core::clone::Clone for ILoopingSelectorPanel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILoopingSelectorStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILoopingSelectorStatics {}
impl ::core::clone::Clone for ILoopingSelectorStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMenuFlyoutItemTemplateSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMenuFlyoutItemTemplateSettings {}
impl ::core::clone::Clone for IMenuFlyoutItemTemplateSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMenuFlyoutPresenterTemplateSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMenuFlyoutPresenterTemplateSettings {}
impl ::core::clone::Clone for IMenuFlyoutPresenterTemplateSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INavigationViewItemPresenter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INavigationViewItemPresenter {}
impl ::core::clone::Clone for INavigationViewItemPresenter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INavigationViewItemPresenterFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INavigationViewItemPresenterFactory {}
impl ::core::clone::Clone for INavigationViewItemPresenterFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INavigationViewItemPresenterStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INavigationViewItemPresenterStatics {}
impl ::core::clone::Clone for INavigationViewItemPresenterStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOrientedVirtualizingPanel(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOrientedVirtualizingPanel {}
impl ::core::clone::Clone for IOrientedVirtualizingPanel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOrientedVirtualizingPanelFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOrientedVirtualizingPanelFactory {}
impl ::core::clone::Clone for IOrientedVirtualizingPanelFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPickerFlyoutBase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPickerFlyoutBase {}
impl ::core::clone::Clone for IPickerFlyoutBase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPickerFlyoutBaseFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPickerFlyoutBaseFactory {}
impl ::core::clone::Clone for IPickerFlyoutBaseFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPickerFlyoutBaseOverrides(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPickerFlyoutBaseOverrides {}
impl ::core::clone::Clone for IPickerFlyoutBaseOverrides {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPickerFlyoutBaseStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPickerFlyoutBaseStatics {}
impl ::core::clone::Clone for IPickerFlyoutBaseStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPivotHeaderItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPivotHeaderItem {}
impl ::core::clone::Clone for IPivotHeaderItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPivotHeaderItemFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPivotHeaderItemFactory {}
impl ::core::clone::Clone for IPivotHeaderItemFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPivotHeaderPanel(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPivotHeaderPanel {}
impl ::core::clone::Clone for IPivotHeaderPanel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPivotPanel(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPivotPanel {}
impl ::core::clone::Clone for IPivotPanel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPopup(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPopup {}
impl ::core::clone::Clone for IPopup {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPopup2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPopup2 {}
impl ::core::clone::Clone for IPopup2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPopup3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPopup3 {}
impl ::core::clone::Clone for IPopup3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPopup4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPopup4 {}
impl ::core::clone::Clone for IPopup4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPopupStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPopupStatics {}
impl ::core::clone::Clone for IPopupStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPopupStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPopupStatics2 {}
impl ::core::clone::Clone for IPopupStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPopupStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPopupStatics3 {}
impl ::core::clone::Clone for IPopupStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPopupStatics4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPopupStatics4 {}
impl ::core::clone::Clone for IPopupStatics4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProgressBarTemplateSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProgressBarTemplateSettings {}
impl ::core::clone::Clone for IProgressBarTemplateSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProgressRingTemplateSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProgressRingTemplateSettings {}
impl ::core::clone::Clone for IProgressRingTemplateSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRangeBase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRangeBase {}
impl ::core::clone::Clone for IRangeBase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRangeBaseFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRangeBaseFactory {}
impl ::core::clone::Clone for IRangeBaseFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRangeBaseOverrides(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRangeBaseOverrides {}
impl ::core::clone::Clone for IRangeBaseOverrides {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRangeBaseStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRangeBaseStatics {}
impl ::core::clone::Clone for IRangeBaseStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRangeBaseValueChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRangeBaseValueChangedEventArgs {}
impl ::core::clone::Clone for IRangeBaseValueChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRepeatButton(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRepeatButton {}
impl ::core::clone::Clone for IRepeatButton {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRepeatButtonStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRepeatButtonStatics {}
impl ::core::clone::Clone for IRepeatButtonStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IScrollBar(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IScrollBar {}
impl ::core::clone::Clone for IScrollBar {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IScrollBarStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IScrollBarStatics {}
impl ::core::clone::Clone for IScrollBarStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IScrollEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IScrollEventArgs {}
impl ::core::clone::Clone for IScrollEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IScrollSnapPointsInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IScrollSnapPointsInfo {}
impl ::core::clone::Clone for IScrollSnapPointsInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISelector(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISelector {}
impl ::core::clone::Clone for ISelector {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISelectorFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISelectorFactory {}
impl ::core::clone::Clone for ISelectorFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISelectorItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISelectorItem {}
impl ::core::clone::Clone for ISelectorItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISelectorItemFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISelectorItemFactory {}
impl ::core::clone::Clone for ISelectorItemFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISelectorItemStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISelectorItemStatics {}
impl ::core::clone::Clone for ISelectorItemStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISelectorStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISelectorStatics {}
impl ::core::clone::Clone for ISelectorStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISettingsFlyoutTemplateSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISettingsFlyoutTemplateSettings {}
impl ::core::clone::Clone for ISettingsFlyoutTemplateSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISplitViewTemplateSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISplitViewTemplateSettings {}
impl ::core::clone::Clone for ISplitViewTemplateSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IThumb(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IThumb {}
impl ::core::clone::Clone for IThumb {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IThumbStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IThumbStatics {}
impl ::core::clone::Clone for IThumbStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITickBar(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITickBar {}
impl ::core::clone::Clone for ITickBar {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITickBarStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITickBarStatics {}
impl ::core::clone::Clone for ITickBarStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToggleButton(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToggleButton {}
impl ::core::clone::Clone for IToggleButton {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToggleButtonFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToggleButtonFactory {}
impl ::core::clone::Clone for IToggleButtonFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToggleButtonOverrides(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToggleButtonOverrides {}
impl ::core::clone::Clone for IToggleButtonOverrides {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToggleButtonStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToggleButtonStatics {}
impl ::core::clone::Clone for IToggleButtonStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToggleSwitchTemplateSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToggleSwitchTemplateSettings {}
impl ::core::clone::Clone for IToggleSwitchTemplateSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToolTipTemplateSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToolTipTemplateSettings {}
impl ::core::clone::Clone for IToolTipTemplateSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ItemsChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ItemsChangedEventArgs {}
impl ::core::clone::Clone for ItemsChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ItemsChangedEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ItemsChangedEventHandler {}
impl ::core::clone::Clone for ItemsChangedEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct JumpListItemBackgroundConverter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for JumpListItemBackgroundConverter {}
impl ::core::clone::Clone for JumpListItemBackgroundConverter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct JumpListItemForegroundConverter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for JumpListItemForegroundConverter {}
impl ::core::clone::Clone for JumpListItemForegroundConverter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LayoutInformation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LayoutInformation {}
impl ::core::clone::Clone for LayoutInformation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ListViewItemPresenter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ListViewItemPresenter {}
impl ::core::clone::Clone for ListViewItemPresenter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ListViewItemPresenterCheckMode(pub i32);
impl ListViewItemPresenterCheckMode {
    pub const Inline: Self = Self(0i32);
    pub const Overlay: Self = Self(1i32);
}
impl ::core::marker::Copy for ListViewItemPresenterCheckMode {}
impl ::core::clone::Clone for ListViewItemPresenterCheckMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ListViewItemPresenterSelectionIndicatorMode(pub i32);
impl ListViewItemPresenterSelectionIndicatorMode {
    pub const Inline: Self = Self(0i32);
    pub const Overlay: Self = Self(1i32);
}
impl ::core::marker::Copy for ListViewItemPresenterSelectionIndicatorMode {}
impl ::core::clone::Clone for ListViewItemPresenterSelectionIndicatorMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ListViewItemTemplateSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ListViewItemTemplateSettings {}
impl ::core::clone::Clone for ListViewItemTemplateSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LoopingSelector(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LoopingSelector {}
impl ::core::clone::Clone for LoopingSelector {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LoopingSelectorItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LoopingSelectorItem {}
impl ::core::clone::Clone for LoopingSelectorItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LoopingSelectorPanel(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LoopingSelectorPanel {}
impl ::core::clone::Clone for LoopingSelectorPanel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MenuFlyoutItemTemplateSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MenuFlyoutItemTemplateSettings {}
impl ::core::clone::Clone for MenuFlyoutItemTemplateSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MenuFlyoutPresenterTemplateSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MenuFlyoutPresenterTemplateSettings {}
impl ::core::clone::Clone for MenuFlyoutPresenterTemplateSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NavigationViewItemPresenter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for NavigationViewItemPresenter {}
impl ::core::clone::Clone for NavigationViewItemPresenter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct OrientedVirtualizingPanel(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for OrientedVirtualizingPanel {}
impl ::core::clone::Clone for OrientedVirtualizingPanel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PickerFlyoutBase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PickerFlyoutBase {}
impl ::core::clone::Clone for PickerFlyoutBase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PivotHeaderItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PivotHeaderItem {}
impl ::core::clone::Clone for PivotHeaderItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PivotHeaderPanel(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PivotHeaderPanel {}
impl ::core::clone::Clone for PivotHeaderPanel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PivotPanel(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PivotPanel {}
impl ::core::clone::Clone for PivotPanel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PlacementMode(pub i32);
impl PlacementMode {
    pub const Bottom: Self = Self(2i32);
    pub const Left: Self = Self(9i32);
    pub const Mouse: Self = Self(7i32);
    pub const Right: Self = Self(4i32);
    pub const Top: Self = Self(10i32);
}
impl ::core::marker::Copy for PlacementMode {}
impl ::core::clone::Clone for PlacementMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Popup(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Popup {}
impl ::core::clone::Clone for Popup {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PopupPlacementMode(pub i32);
impl PopupPlacementMode {
    pub const Auto: Self = Self(0i32);
    pub const Top: Self = Self(1i32);
    pub const Bottom: Self = Self(2i32);
    pub const Left: Self = Self(3i32);
    pub const Right: Self = Self(4i32);
    pub const TopEdgeAlignedLeft: Self = Self(5i32);
    pub const TopEdgeAlignedRight: Self = Self(6i32);
    pub const BottomEdgeAlignedLeft: Self = Self(7i32);
    pub const BottomEdgeAlignedRight: Self = Self(8i32);
    pub const LeftEdgeAlignedTop: Self = Self(9i32);
    pub const LeftEdgeAlignedBottom: Self = Self(10i32);
    pub const RightEdgeAlignedTop: Self = Self(11i32);
    pub const RightEdgeAlignedBottom: Self = Self(12i32);
}
impl ::core::marker::Copy for PopupPlacementMode {}
impl ::core::clone::Clone for PopupPlacementMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ProgressBarTemplateSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ProgressBarTemplateSettings {}
impl ::core::clone::Clone for ProgressBarTemplateSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ProgressRingTemplateSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ProgressRingTemplateSettings {}
impl ::core::clone::Clone for ProgressRingTemplateSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RangeBase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RangeBase {}
impl ::core::clone::Clone for RangeBase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RangeBaseValueChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RangeBaseValueChangedEventArgs {}
impl ::core::clone::Clone for RangeBaseValueChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RangeBaseValueChangedEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RangeBaseValueChangedEventHandler {}
impl ::core::clone::Clone for RangeBaseValueChangedEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RepeatButton(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RepeatButton {}
impl ::core::clone::Clone for RepeatButton {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ScrollBar(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ScrollBar {}
impl ::core::clone::Clone for ScrollBar {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ScrollEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ScrollEventArgs {}
impl ::core::clone::Clone for ScrollEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ScrollEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ScrollEventHandler {}
impl ::core::clone::Clone for ScrollEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ScrollEventType(pub i32);
impl ScrollEventType {
    pub const SmallDecrement: Self = Self(0i32);
    pub const SmallIncrement: Self = Self(1i32);
    pub const LargeDecrement: Self = Self(2i32);
    pub const LargeIncrement: Self = Self(3i32);
    pub const ThumbPosition: Self = Self(4i32);
    pub const ThumbTrack: Self = Self(5i32);
    pub const First: Self = Self(6i32);
    pub const Last: Self = Self(7i32);
    pub const EndScroll: Self = Self(8i32);
}
impl ::core::marker::Copy for ScrollEventType {}
impl ::core::clone::Clone for ScrollEventType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ScrollingIndicatorMode(pub i32);
impl ScrollingIndicatorMode {
    pub const None: Self = Self(0i32);
    pub const TouchIndicator: Self = Self(1i32);
    pub const MouseIndicator: Self = Self(2i32);
}
impl ::core::marker::Copy for ScrollingIndicatorMode {}
impl ::core::clone::Clone for ScrollingIndicatorMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Selector(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Selector {}
impl ::core::clone::Clone for Selector {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SelectorItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SelectorItem {}
impl ::core::clone::Clone for SelectorItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SettingsFlyoutTemplateSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SettingsFlyoutTemplateSettings {}
impl ::core::clone::Clone for SettingsFlyoutTemplateSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SliderSnapsTo(pub i32);
impl SliderSnapsTo {
    pub const StepValues: Self = Self(0i32);
    pub const Ticks: Self = Self(1i32);
}
impl ::core::marker::Copy for SliderSnapsTo {}
impl ::core::clone::Clone for SliderSnapsTo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SnapPointsAlignment(pub i32);
impl SnapPointsAlignment {
    pub const Near: Self = Self(0i32);
    pub const Center: Self = Self(1i32);
    pub const Far: Self = Self(2i32);
}
impl ::core::marker::Copy for SnapPointsAlignment {}
impl ::core::clone::Clone for SnapPointsAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SplitViewTemplateSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SplitViewTemplateSettings {}
impl ::core::clone::Clone for SplitViewTemplateSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Thumb(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Thumb {}
impl ::core::clone::Clone for Thumb {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TickBar(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TickBar {}
impl ::core::clone::Clone for TickBar {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TickPlacement(pub i32);
impl TickPlacement {
    pub const None: Self = Self(0i32);
    pub const TopLeft: Self = Self(1i32);
    pub const BottomRight: Self = Self(2i32);
    pub const Outside: Self = Self(3i32);
    pub const Inline: Self = Self(4i32);
}
impl ::core::marker::Copy for TickPlacement {}
impl ::core::clone::Clone for TickPlacement {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ToggleButton(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ToggleButton {}
impl ::core::clone::Clone for ToggleButton {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ToggleSwitchTemplateSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ToggleSwitchTemplateSettings {}
impl ::core::clone::Clone for ToggleSwitchTemplateSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ToolTipTemplateSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ToolTipTemplateSettings {}
impl ::core::clone::Clone for ToolTipTemplateSettings {
    fn clone(&self) -> Self {
        *self
    }
}
