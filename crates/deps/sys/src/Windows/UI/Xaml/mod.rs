#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "UI_Xaml_Automation")]
pub mod Automation;
#[cfg(feature = "UI_Xaml_Controls")]
pub mod Controls;
#[cfg(feature = "UI_Xaml_Core")]
pub mod Core;
#[cfg(feature = "UI_Xaml_Data")]
pub mod Data;
#[cfg(feature = "UI_Xaml_Documents")]
pub mod Documents;
#[cfg(feature = "UI_Xaml_Hosting")]
pub mod Hosting;
#[cfg(feature = "UI_Xaml_Input")]
pub mod Input;
#[cfg(feature = "UI_Xaml_Interop")]
pub mod Interop;
#[cfg(feature = "UI_Xaml_Markup")]
pub mod Markup;
#[cfg(feature = "UI_Xaml_Media")]
pub mod Media;
#[cfg(feature = "UI_Xaml_Navigation")]
pub mod Navigation;
#[cfg(feature = "UI_Xaml_Printing")]
pub mod Printing;
#[cfg(feature = "UI_Xaml_Resources")]
pub mod Resources;
#[cfg(feature = "UI_Xaml_Shapes")]
pub mod Shapes;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AdaptiveTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AdaptiveTrigger {}
impl ::core::clone::Clone for AdaptiveTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Application(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Application {}
impl ::core::clone::Clone for Application {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ApplicationHighContrastAdjustment(pub u32);
impl ApplicationHighContrastAdjustment {
    pub const None: Self = Self(0u32);
    pub const Auto: Self = Self(4294967295u32);
}
impl ::core::marker::Copy for ApplicationHighContrastAdjustment {}
impl ::core::clone::Clone for ApplicationHighContrastAdjustment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ApplicationInitializationCallback(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ApplicationInitializationCallback {}
impl ::core::clone::Clone for ApplicationInitializationCallback {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ApplicationInitializationCallbackParams(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ApplicationInitializationCallbackParams {}
impl ::core::clone::Clone for ApplicationInitializationCallbackParams {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ApplicationRequiresPointerMode(pub i32);
impl ApplicationRequiresPointerMode {
    pub const Auto: Self = Self(0i32);
    pub const WhenRequested: Self = Self(1i32);
}
impl ::core::marker::Copy for ApplicationRequiresPointerMode {}
impl ::core::clone::Clone for ApplicationRequiresPointerMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ApplicationTheme(pub i32);
impl ApplicationTheme {
    pub const Light: Self = Self(0i32);
    pub const Dark: Self = Self(1i32);
}
impl ::core::marker::Copy for ApplicationTheme {}
impl ::core::clone::Clone for ApplicationTheme {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AutomationTextAttributesEnum(pub i32);
impl AutomationTextAttributesEnum {
    pub const AnimationStyleAttribute: Self = Self(40000i32);
    pub const BackgroundColorAttribute: Self = Self(40001i32);
    pub const BulletStyleAttribute: Self = Self(40002i32);
    pub const CapStyleAttribute: Self = Self(40003i32);
    pub const CultureAttribute: Self = Self(40004i32);
    pub const FontNameAttribute: Self = Self(40005i32);
    pub const FontSizeAttribute: Self = Self(40006i32);
    pub const FontWeightAttribute: Self = Self(40007i32);
    pub const ForegroundColorAttribute: Self = Self(40008i32);
    pub const HorizontalTextAlignmentAttribute: Self = Self(40009i32);
    pub const IndentationFirstLineAttribute: Self = Self(40010i32);
    pub const IndentationLeadingAttribute: Self = Self(40011i32);
    pub const IndentationTrailingAttribute: Self = Self(40012i32);
    pub const IsHiddenAttribute: Self = Self(40013i32);
    pub const IsItalicAttribute: Self = Self(40014i32);
    pub const IsReadOnlyAttribute: Self = Self(40015i32);
    pub const IsSubscriptAttribute: Self = Self(40016i32);
    pub const IsSuperscriptAttribute: Self = Self(40017i32);
    pub const MarginBottomAttribute: Self = Self(40018i32);
    pub const MarginLeadingAttribute: Self = Self(40019i32);
    pub const MarginTopAttribute: Self = Self(40020i32);
    pub const MarginTrailingAttribute: Self = Self(40021i32);
    pub const OutlineStylesAttribute: Self = Self(40022i32);
    pub const OverlineColorAttribute: Self = Self(40023i32);
    pub const OverlineStyleAttribute: Self = Self(40024i32);
    pub const StrikethroughColorAttribute: Self = Self(40025i32);
    pub const StrikethroughStyleAttribute: Self = Self(40026i32);
    pub const TabsAttribute: Self = Self(40027i32);
    pub const TextFlowDirectionsAttribute: Self = Self(40028i32);
    pub const UnderlineColorAttribute: Self = Self(40029i32);
    pub const UnderlineStyleAttribute: Self = Self(40030i32);
    pub const AnnotationTypesAttribute: Self = Self(40031i32);
    pub const AnnotationObjectsAttribute: Self = Self(40032i32);
    pub const StyleNameAttribute: Self = Self(40033i32);
    pub const StyleIdAttribute: Self = Self(40034i32);
    pub const LinkAttribute: Self = Self(40035i32);
    pub const IsActiveAttribute: Self = Self(40036i32);
    pub const SelectionActiveEndAttribute: Self = Self(40037i32);
    pub const CaretPositionAttribute: Self = Self(40038i32);
    pub const CaretBidiModeAttribute: Self = Self(40039i32);
}
impl ::core::marker::Copy for AutomationTextAttributesEnum {}
impl ::core::clone::Clone for AutomationTextAttributesEnum {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BindingFailedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BindingFailedEventArgs {}
impl ::core::clone::Clone for BindingFailedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BindingFailedEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BindingFailedEventHandler {}
impl ::core::clone::Clone for BindingFailedEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BringIntoViewOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BringIntoViewOptions {}
impl ::core::clone::Clone for BringIntoViewOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BringIntoViewRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BringIntoViewRequestedEventArgs {}
impl ::core::clone::Clone for BringIntoViewRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BrushTransition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BrushTransition {}
impl ::core::clone::Clone for BrushTransition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ColorPaletteResources(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ColorPaletteResources {}
impl ::core::clone::Clone for ColorPaletteResources {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CornerRadius {
    pub TopLeft: f64,
    pub TopRight: f64,
    pub BottomRight: f64,
    pub BottomLeft: f64,
}
impl ::core::marker::Copy for CornerRadius {}
impl ::core::clone::Clone for CornerRadius {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CornerRadiusHelper(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CornerRadiusHelper {}
impl ::core::clone::Clone for CornerRadiusHelper {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CreateDefaultValueCallback(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CreateDefaultValueCallback {}
impl ::core::clone::Clone for CreateDefaultValueCallback {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DataContextChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DataContextChangedEventArgs {}
impl ::core::clone::Clone for DataContextChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DataTemplate(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DataTemplate {}
impl ::core::clone::Clone for DataTemplate {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DataTemplateKey(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DataTemplateKey {}
impl ::core::clone::Clone for DataTemplateKey {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DebugSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DebugSettings {}
impl ::core::clone::Clone for DebugSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DependencyObject(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DependencyObject {}
impl ::core::clone::Clone for DependencyObject {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DependencyObjectCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DependencyObjectCollection {}
impl ::core::clone::Clone for DependencyObjectCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DependencyProperty(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DependencyProperty {}
impl ::core::clone::Clone for DependencyProperty {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DependencyPropertyChangedCallback(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DependencyPropertyChangedCallback {}
impl ::core::clone::Clone for DependencyPropertyChangedCallback {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DependencyPropertyChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DependencyPropertyChangedEventArgs {}
impl ::core::clone::Clone for DependencyPropertyChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DependencyPropertyChangedEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DependencyPropertyChangedEventHandler {}
impl ::core::clone::Clone for DependencyPropertyChangedEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DispatcherTimer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DispatcherTimer {}
impl ::core::clone::Clone for DispatcherTimer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DragEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DragEventArgs {}
impl ::core::clone::Clone for DragEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DragEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DragEventHandler {}
impl ::core::clone::Clone for DragEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DragOperationDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DragOperationDeferral {}
impl ::core::clone::Clone for DragOperationDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DragStartingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DragStartingEventArgs {}
impl ::core::clone::Clone for DragStartingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DragUI(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DragUI {}
impl ::core::clone::Clone for DragUI {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DragUIOverride(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DragUIOverride {}
impl ::core::clone::Clone for DragUIOverride {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DropCompletedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DropCompletedEventArgs {}
impl ::core::clone::Clone for DropCompletedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Foundation")]
pub struct Duration {
    pub TimeSpan: super::super::Foundation::TimeSpan,
    pub Type: DurationType,
}
#[cfg(feature = "Foundation")]
impl ::core::marker::Copy for Duration {}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for Duration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DurationHelper(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DurationHelper {}
impl ::core::clone::Clone for DurationHelper {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DurationType(pub i32);
impl DurationType {
    pub const Automatic: Self = Self(0i32);
    pub const TimeSpan: Self = Self(1i32);
    pub const Forever: Self = Self(2i32);
}
impl ::core::marker::Copy for DurationType {}
impl ::core::clone::Clone for DurationType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EffectiveViewportChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EffectiveViewportChangedEventArgs {}
impl ::core::clone::Clone for EffectiveViewportChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ElementFactoryGetArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ElementFactoryGetArgs {}
impl ::core::clone::Clone for ElementFactoryGetArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ElementFactoryRecycleArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ElementFactoryRecycleArgs {}
impl ::core::clone::Clone for ElementFactoryRecycleArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ElementHighContrastAdjustment(pub u32);
impl ElementHighContrastAdjustment {
    pub const None: Self = Self(0u32);
    pub const Application: Self = Self(2147483648u32);
    pub const Auto: Self = Self(4294967295u32);
}
impl ::core::marker::Copy for ElementHighContrastAdjustment {}
impl ::core::clone::Clone for ElementHighContrastAdjustment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ElementSoundKind(pub i32);
impl ElementSoundKind {
    pub const Focus: Self = Self(0i32);
    pub const Invoke: Self = Self(1i32);
    pub const Show: Self = Self(2i32);
    pub const Hide: Self = Self(3i32);
    pub const MovePrevious: Self = Self(4i32);
    pub const MoveNext: Self = Self(5i32);
    pub const GoBack: Self = Self(6i32);
}
impl ::core::marker::Copy for ElementSoundKind {}
impl ::core::clone::Clone for ElementSoundKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ElementSoundMode(pub i32);
impl ElementSoundMode {
    pub const Default: Self = Self(0i32);
    pub const FocusOnly: Self = Self(1i32);
    pub const Off: Self = Self(2i32);
}
impl ::core::marker::Copy for ElementSoundMode {}
impl ::core::clone::Clone for ElementSoundMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ElementSoundPlayer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ElementSoundPlayer {}
impl ::core::clone::Clone for ElementSoundPlayer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ElementSoundPlayerState(pub i32);
impl ElementSoundPlayerState {
    pub const Auto: Self = Self(0i32);
    pub const Off: Self = Self(1i32);
    pub const On: Self = Self(2i32);
}
impl ::core::marker::Copy for ElementSoundPlayerState {}
impl ::core::clone::Clone for ElementSoundPlayerState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ElementSpatialAudioMode(pub i32);
impl ElementSpatialAudioMode {
    pub const Auto: Self = Self(0i32);
    pub const Off: Self = Self(1i32);
    pub const On: Self = Self(2i32);
}
impl ::core::marker::Copy for ElementSpatialAudioMode {}
impl ::core::clone::Clone for ElementSpatialAudioMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ElementTheme(pub i32);
impl ElementTheme {
    pub const Default: Self = Self(0i32);
    pub const Light: Self = Self(1i32);
    pub const Dark: Self = Self(2i32);
}
impl ::core::marker::Copy for ElementTheme {}
impl ::core::clone::Clone for ElementTheme {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EnteredBackgroundEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EnteredBackgroundEventHandler {}
impl ::core::clone::Clone for EnteredBackgroundEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EventTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EventTrigger {}
impl ::core::clone::Clone for EventTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ExceptionRoutedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ExceptionRoutedEventArgs {}
impl ::core::clone::Clone for ExceptionRoutedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ExceptionRoutedEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ExceptionRoutedEventHandler {}
impl ::core::clone::Clone for ExceptionRoutedEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FlowDirection(pub i32);
impl FlowDirection {
    pub const LeftToRight: Self = Self(0i32);
    pub const RightToLeft: Self = Self(1i32);
}
impl ::core::marker::Copy for FlowDirection {}
impl ::core::clone::Clone for FlowDirection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FocusState(pub i32);
impl FocusState {
    pub const Unfocused: Self = Self(0i32);
    pub const Pointer: Self = Self(1i32);
    pub const Keyboard: Self = Self(2i32);
    pub const Programmatic: Self = Self(3i32);
}
impl ::core::marker::Copy for FocusState {}
impl ::core::clone::Clone for FocusState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FocusVisualKind(pub i32);
impl FocusVisualKind {
    pub const DottedLine: Self = Self(0i32);
    pub const HighVisibility: Self = Self(1i32);
    pub const Reveal: Self = Self(2i32);
}
impl ::core::marker::Copy for FocusVisualKind {}
impl ::core::clone::Clone for FocusVisualKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FontCapitals(pub i32);
impl FontCapitals {
    pub const Normal: Self = Self(0i32);
    pub const AllSmallCaps: Self = Self(1i32);
    pub const SmallCaps: Self = Self(2i32);
    pub const AllPetiteCaps: Self = Self(3i32);
    pub const PetiteCaps: Self = Self(4i32);
    pub const Unicase: Self = Self(5i32);
    pub const Titling: Self = Self(6i32);
}
impl ::core::marker::Copy for FontCapitals {}
impl ::core::clone::Clone for FontCapitals {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FontEastAsianLanguage(pub i32);
impl FontEastAsianLanguage {
    pub const Normal: Self = Self(0i32);
    pub const HojoKanji: Self = Self(1i32);
    pub const Jis04: Self = Self(2i32);
    pub const Jis78: Self = Self(3i32);
    pub const Jis83: Self = Self(4i32);
    pub const Jis90: Self = Self(5i32);
    pub const NlcKanji: Self = Self(6i32);
    pub const Simplified: Self = Self(7i32);
    pub const Traditional: Self = Self(8i32);
    pub const TraditionalNames: Self = Self(9i32);
}
impl ::core::marker::Copy for FontEastAsianLanguage {}
impl ::core::clone::Clone for FontEastAsianLanguage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FontEastAsianWidths(pub i32);
impl FontEastAsianWidths {
    pub const Normal: Self = Self(0i32);
    pub const Full: Self = Self(1i32);
    pub const Half: Self = Self(2i32);
    pub const Proportional: Self = Self(3i32);
    pub const Quarter: Self = Self(4i32);
    pub const Third: Self = Self(5i32);
}
impl ::core::marker::Copy for FontEastAsianWidths {}
impl ::core::clone::Clone for FontEastAsianWidths {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FontFraction(pub i32);
impl FontFraction {
    pub const Normal: Self = Self(0i32);
    pub const Stacked: Self = Self(1i32);
    pub const Slashed: Self = Self(2i32);
}
impl ::core::marker::Copy for FontFraction {}
impl ::core::clone::Clone for FontFraction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FontNumeralAlignment(pub i32);
impl FontNumeralAlignment {
    pub const Normal: Self = Self(0i32);
    pub const Proportional: Self = Self(1i32);
    pub const Tabular: Self = Self(2i32);
}
impl ::core::marker::Copy for FontNumeralAlignment {}
impl ::core::clone::Clone for FontNumeralAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FontNumeralStyle(pub i32);
impl FontNumeralStyle {
    pub const Normal: Self = Self(0i32);
    pub const Lining: Self = Self(1i32);
    pub const OldStyle: Self = Self(2i32);
}
impl ::core::marker::Copy for FontNumeralStyle {}
impl ::core::clone::Clone for FontNumeralStyle {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FontVariants(pub i32);
impl FontVariants {
    pub const Normal: Self = Self(0i32);
    pub const Superscript: Self = Self(1i32);
    pub const Subscript: Self = Self(2i32);
    pub const Ordinal: Self = Self(3i32);
    pub const Inferior: Self = Self(4i32);
    pub const Ruby: Self = Self(5i32);
}
impl ::core::marker::Copy for FontVariants {}
impl ::core::clone::Clone for FontVariants {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FrameworkElement(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FrameworkElement {}
impl ::core::clone::Clone for FrameworkElement {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FrameworkTemplate(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FrameworkTemplate {}
impl ::core::clone::Clone for FrameworkTemplate {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FrameworkView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FrameworkView {}
impl ::core::clone::Clone for FrameworkView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FrameworkViewSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FrameworkViewSource {}
impl ::core::clone::Clone for FrameworkViewSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct GridLength {
    pub Value: f64,
    pub GridUnitType: GridUnitType,
}
impl ::core::marker::Copy for GridLength {}
impl ::core::clone::Clone for GridLength {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GridLengthHelper(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GridLengthHelper {}
impl ::core::clone::Clone for GridLengthHelper {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GridUnitType(pub i32);
impl GridUnitType {
    pub const Auto: Self = Self(0i32);
    pub const Pixel: Self = Self(1i32);
    pub const Star: Self = Self(2i32);
}
impl ::core::marker::Copy for GridUnitType {}
impl ::core::clone::Clone for GridUnitType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HorizontalAlignment(pub i32);
impl HorizontalAlignment {
    pub const Left: Self = Self(0i32);
    pub const Center: Self = Self(1i32);
    pub const Right: Self = Self(2i32);
    pub const Stretch: Self = Self(3i32);
}
impl ::core::marker::Copy for HorizontalAlignment {}
impl ::core::clone::Clone for HorizontalAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAdaptiveTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAdaptiveTrigger {}
impl ::core::clone::Clone for IAdaptiveTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAdaptiveTriggerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAdaptiveTriggerFactory {}
impl ::core::clone::Clone for IAdaptiveTriggerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAdaptiveTriggerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAdaptiveTriggerStatics {}
impl ::core::clone::Clone for IAdaptiveTriggerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IApplication(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IApplication {}
impl ::core::clone::Clone for IApplication {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IApplication2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IApplication2 {}
impl ::core::clone::Clone for IApplication2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IApplication3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IApplication3 {}
impl ::core::clone::Clone for IApplication3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IApplicationFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IApplicationFactory {}
impl ::core::clone::Clone for IApplicationFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IApplicationInitializationCallbackParams(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IApplicationInitializationCallbackParams {}
impl ::core::clone::Clone for IApplicationInitializationCallbackParams {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IApplicationOverrides(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IApplicationOverrides {}
impl ::core::clone::Clone for IApplicationOverrides {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IApplicationOverrides2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IApplicationOverrides2 {}
impl ::core::clone::Clone for IApplicationOverrides2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IApplicationStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IApplicationStatics {}
impl ::core::clone::Clone for IApplicationStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBindingFailedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBindingFailedEventArgs {}
impl ::core::clone::Clone for IBindingFailedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBringIntoViewOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBringIntoViewOptions {}
impl ::core::clone::Clone for IBringIntoViewOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBringIntoViewOptions2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBringIntoViewOptions2 {}
impl ::core::clone::Clone for IBringIntoViewOptions2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBringIntoViewRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBringIntoViewRequestedEventArgs {}
impl ::core::clone::Clone for IBringIntoViewRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBrushTransition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBrushTransition {}
impl ::core::clone::Clone for IBrushTransition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBrushTransitionFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBrushTransitionFactory {}
impl ::core::clone::Clone for IBrushTransitionFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IColorPaletteResources(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IColorPaletteResources {}
impl ::core::clone::Clone for IColorPaletteResources {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IColorPaletteResourcesFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IColorPaletteResourcesFactory {}
impl ::core::clone::Clone for IColorPaletteResourcesFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICornerRadiusHelper(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICornerRadiusHelper {}
impl ::core::clone::Clone for ICornerRadiusHelper {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICornerRadiusHelperStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICornerRadiusHelperStatics {}
impl ::core::clone::Clone for ICornerRadiusHelperStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDataContextChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDataContextChangedEventArgs {}
impl ::core::clone::Clone for IDataContextChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDataTemplate(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDataTemplate {}
impl ::core::clone::Clone for IDataTemplate {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDataTemplateExtension(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDataTemplateExtension {}
impl ::core::clone::Clone for IDataTemplateExtension {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDataTemplateFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDataTemplateFactory {}
impl ::core::clone::Clone for IDataTemplateFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDataTemplateKey(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDataTemplateKey {}
impl ::core::clone::Clone for IDataTemplateKey {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDataTemplateKeyFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDataTemplateKeyFactory {}
impl ::core::clone::Clone for IDataTemplateKeyFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDataTemplateStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDataTemplateStatics2 {}
impl ::core::clone::Clone for IDataTemplateStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDebugSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDebugSettings {}
impl ::core::clone::Clone for IDebugSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDebugSettings2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDebugSettings2 {}
impl ::core::clone::Clone for IDebugSettings2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDebugSettings3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDebugSettings3 {}
impl ::core::clone::Clone for IDebugSettings3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDebugSettings4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDebugSettings4 {}
impl ::core::clone::Clone for IDebugSettings4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDependencyObject(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDependencyObject {}
impl ::core::clone::Clone for IDependencyObject {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDependencyObject2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDependencyObject2 {}
impl ::core::clone::Clone for IDependencyObject2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDependencyObjectCollectionFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDependencyObjectCollectionFactory {}
impl ::core::clone::Clone for IDependencyObjectCollectionFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDependencyObjectFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDependencyObjectFactory {}
impl ::core::clone::Clone for IDependencyObjectFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDependencyProperty(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDependencyProperty {}
impl ::core::clone::Clone for IDependencyProperty {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDependencyPropertyChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDependencyPropertyChangedEventArgs {}
impl ::core::clone::Clone for IDependencyPropertyChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDependencyPropertyStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDependencyPropertyStatics {}
impl ::core::clone::Clone for IDependencyPropertyStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDispatcherTimer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDispatcherTimer {}
impl ::core::clone::Clone for IDispatcherTimer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDispatcherTimerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDispatcherTimerFactory {}
impl ::core::clone::Clone for IDispatcherTimerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDragEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDragEventArgs {}
impl ::core::clone::Clone for IDragEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDragEventArgs2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDragEventArgs2 {}
impl ::core::clone::Clone for IDragEventArgs2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDragEventArgs3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDragEventArgs3 {}
impl ::core::clone::Clone for IDragEventArgs3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDragOperationDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDragOperationDeferral {}
impl ::core::clone::Clone for IDragOperationDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDragStartingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDragStartingEventArgs {}
impl ::core::clone::Clone for IDragStartingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDragStartingEventArgs2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDragStartingEventArgs2 {}
impl ::core::clone::Clone for IDragStartingEventArgs2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDragUI(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDragUI {}
impl ::core::clone::Clone for IDragUI {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDragUIOverride(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDragUIOverride {}
impl ::core::clone::Clone for IDragUIOverride {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDropCompletedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDropCompletedEventArgs {}
impl ::core::clone::Clone for IDropCompletedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDurationHelper(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDurationHelper {}
impl ::core::clone::Clone for IDurationHelper {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDurationHelperStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDurationHelperStatics {}
impl ::core::clone::Clone for IDurationHelperStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEffectiveViewportChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEffectiveViewportChangedEventArgs {}
impl ::core::clone::Clone for IEffectiveViewportChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IElementFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IElementFactory {}
impl ::core::clone::Clone for IElementFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IElementFactoryGetArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IElementFactoryGetArgs {}
impl ::core::clone::Clone for IElementFactoryGetArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IElementFactoryGetArgsFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IElementFactoryGetArgsFactory {}
impl ::core::clone::Clone for IElementFactoryGetArgsFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IElementFactoryRecycleArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IElementFactoryRecycleArgs {}
impl ::core::clone::Clone for IElementFactoryRecycleArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IElementFactoryRecycleArgsFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IElementFactoryRecycleArgsFactory {}
impl ::core::clone::Clone for IElementFactoryRecycleArgsFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IElementSoundPlayer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IElementSoundPlayer {}
impl ::core::clone::Clone for IElementSoundPlayer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IElementSoundPlayerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IElementSoundPlayerStatics {}
impl ::core::clone::Clone for IElementSoundPlayerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IElementSoundPlayerStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IElementSoundPlayerStatics2 {}
impl ::core::clone::Clone for IElementSoundPlayerStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEventTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEventTrigger {}
impl ::core::clone::Clone for IEventTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IExceptionRoutedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IExceptionRoutedEventArgs {}
impl ::core::clone::Clone for IExceptionRoutedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IExceptionRoutedEventArgsFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IExceptionRoutedEventArgsFactory {}
impl ::core::clone::Clone for IExceptionRoutedEventArgsFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFrameworkElement(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFrameworkElement {}
impl ::core::clone::Clone for IFrameworkElement {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFrameworkElement2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFrameworkElement2 {}
impl ::core::clone::Clone for IFrameworkElement2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFrameworkElement3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFrameworkElement3 {}
impl ::core::clone::Clone for IFrameworkElement3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFrameworkElement4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFrameworkElement4 {}
impl ::core::clone::Clone for IFrameworkElement4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFrameworkElement6(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFrameworkElement6 {}
impl ::core::clone::Clone for IFrameworkElement6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFrameworkElement7(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFrameworkElement7 {}
impl ::core::clone::Clone for IFrameworkElement7 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFrameworkElementFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFrameworkElementFactory {}
impl ::core::clone::Clone for IFrameworkElementFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFrameworkElementOverrides(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFrameworkElementOverrides {}
impl ::core::clone::Clone for IFrameworkElementOverrides {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFrameworkElementOverrides2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFrameworkElementOverrides2 {}
impl ::core::clone::Clone for IFrameworkElementOverrides2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFrameworkElementProtected7(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFrameworkElementProtected7 {}
impl ::core::clone::Clone for IFrameworkElementProtected7 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFrameworkElementStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFrameworkElementStatics {}
impl ::core::clone::Clone for IFrameworkElementStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFrameworkElementStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFrameworkElementStatics2 {}
impl ::core::clone::Clone for IFrameworkElementStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFrameworkElementStatics4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFrameworkElementStatics4 {}
impl ::core::clone::Clone for IFrameworkElementStatics4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFrameworkElementStatics5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFrameworkElementStatics5 {}
impl ::core::clone::Clone for IFrameworkElementStatics5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFrameworkElementStatics6(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFrameworkElementStatics6 {}
impl ::core::clone::Clone for IFrameworkElementStatics6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFrameworkTemplate(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFrameworkTemplate {}
impl ::core::clone::Clone for IFrameworkTemplate {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFrameworkTemplateFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFrameworkTemplateFactory {}
impl ::core::clone::Clone for IFrameworkTemplateFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFrameworkView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFrameworkView {}
impl ::core::clone::Clone for IFrameworkView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFrameworkViewSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFrameworkViewSource {}
impl ::core::clone::Clone for IFrameworkViewSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGridLengthHelper(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGridLengthHelper {}
impl ::core::clone::Clone for IGridLengthHelper {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGridLengthHelperStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGridLengthHelperStatics {}
impl ::core::clone::Clone for IGridLengthHelperStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaFailedRoutedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaFailedRoutedEventArgs {}
impl ::core::clone::Clone for IMediaFailedRoutedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPointHelper(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPointHelper {}
impl ::core::clone::Clone for IPointHelper {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPointHelperStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPointHelperStatics {}
impl ::core::clone::Clone for IPointHelperStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPropertyMetadata(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPropertyMetadata {}
impl ::core::clone::Clone for IPropertyMetadata {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPropertyMetadataFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPropertyMetadataFactory {}
impl ::core::clone::Clone for IPropertyMetadataFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPropertyMetadataStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPropertyMetadataStatics {}
impl ::core::clone::Clone for IPropertyMetadataStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPropertyPath(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPropertyPath {}
impl ::core::clone::Clone for IPropertyPath {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPropertyPathFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPropertyPathFactory {}
impl ::core::clone::Clone for IPropertyPathFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRectHelper(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRectHelper {}
impl ::core::clone::Clone for IRectHelper {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRectHelperStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRectHelperStatics {}
impl ::core::clone::Clone for IRectHelperStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IResourceDictionary(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IResourceDictionary {}
impl ::core::clone::Clone for IResourceDictionary {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IResourceDictionaryFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IResourceDictionaryFactory {}
impl ::core::clone::Clone for IResourceDictionaryFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRoutedEvent(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRoutedEvent {}
impl ::core::clone::Clone for IRoutedEvent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRoutedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRoutedEventArgs {}
impl ::core::clone::Clone for IRoutedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRoutedEventArgsFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRoutedEventArgsFactory {}
impl ::core::clone::Clone for IRoutedEventArgsFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IScalarTransition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IScalarTransition {}
impl ::core::clone::Clone for IScalarTransition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IScalarTransitionFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IScalarTransitionFactory {}
impl ::core::clone::Clone for IScalarTransitionFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISetter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISetter {}
impl ::core::clone::Clone for ISetter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISetter2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISetter2 {}
impl ::core::clone::Clone for ISetter2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISetterBase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISetterBase {}
impl ::core::clone::Clone for ISetterBase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISetterBaseCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISetterBaseCollection {}
impl ::core::clone::Clone for ISetterBaseCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISetterBaseFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISetterBaseFactory {}
impl ::core::clone::Clone for ISetterBaseFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISetterFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISetterFactory {}
impl ::core::clone::Clone for ISetterFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISizeChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISizeChangedEventArgs {}
impl ::core::clone::Clone for ISizeChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISizeHelper(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISizeHelper {}
impl ::core::clone::Clone for ISizeHelper {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISizeHelperStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISizeHelperStatics {}
impl ::core::clone::Clone for ISizeHelperStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStateTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStateTrigger {}
impl ::core::clone::Clone for IStateTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStateTriggerBase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStateTriggerBase {}
impl ::core::clone::Clone for IStateTriggerBase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStateTriggerBaseFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStateTriggerBaseFactory {}
impl ::core::clone::Clone for IStateTriggerBaseFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStateTriggerBaseProtected(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStateTriggerBaseProtected {}
impl ::core::clone::Clone for IStateTriggerBaseProtected {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStateTriggerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStateTriggerStatics {}
impl ::core::clone::Clone for IStateTriggerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStyle(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStyle {}
impl ::core::clone::Clone for IStyle {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStyleFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStyleFactory {}
impl ::core::clone::Clone for IStyleFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITargetPropertyPath(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITargetPropertyPath {}
impl ::core::clone::Clone for ITargetPropertyPath {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITargetPropertyPathFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITargetPropertyPathFactory {}
impl ::core::clone::Clone for ITargetPropertyPathFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IThicknessHelper(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IThicknessHelper {}
impl ::core::clone::Clone for IThicknessHelper {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IThicknessHelperStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IThicknessHelperStatics {}
impl ::core::clone::Clone for IThicknessHelperStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITriggerAction(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITriggerAction {}
impl ::core::clone::Clone for ITriggerAction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITriggerActionFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITriggerActionFactory {}
impl ::core::clone::Clone for ITriggerActionFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITriggerBase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITriggerBase {}
impl ::core::clone::Clone for ITriggerBase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITriggerBaseFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITriggerBaseFactory {}
impl ::core::clone::Clone for ITriggerBaseFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUIElement(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUIElement {}
impl ::core::clone::Clone for IUIElement {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUIElement10(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUIElement10 {}
impl ::core::clone::Clone for IUIElement10 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUIElement2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUIElement2 {}
impl ::core::clone::Clone for IUIElement2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUIElement3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUIElement3 {}
impl ::core::clone::Clone for IUIElement3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUIElement4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUIElement4 {}
impl ::core::clone::Clone for IUIElement4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUIElement5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUIElement5 {}
impl ::core::clone::Clone for IUIElement5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUIElement7(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUIElement7 {}
impl ::core::clone::Clone for IUIElement7 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUIElement8(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUIElement8 {}
impl ::core::clone::Clone for IUIElement8 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUIElement9(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUIElement9 {}
impl ::core::clone::Clone for IUIElement9 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUIElementFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUIElementFactory {}
impl ::core::clone::Clone for IUIElementFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUIElementOverrides(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUIElementOverrides {}
impl ::core::clone::Clone for IUIElementOverrides {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUIElementOverrides7(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUIElementOverrides7 {}
impl ::core::clone::Clone for IUIElementOverrides7 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUIElementOverrides8(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUIElementOverrides8 {}
impl ::core::clone::Clone for IUIElementOverrides8 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUIElementOverrides9(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUIElementOverrides9 {}
impl ::core::clone::Clone for IUIElementOverrides9 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUIElementStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUIElementStatics {}
impl ::core::clone::Clone for IUIElementStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUIElementStatics10(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUIElementStatics10 {}
impl ::core::clone::Clone for IUIElementStatics10 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUIElementStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUIElementStatics2 {}
impl ::core::clone::Clone for IUIElementStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUIElementStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUIElementStatics3 {}
impl ::core::clone::Clone for IUIElementStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUIElementStatics4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUIElementStatics4 {}
impl ::core::clone::Clone for IUIElementStatics4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUIElementStatics5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUIElementStatics5 {}
impl ::core::clone::Clone for IUIElementStatics5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUIElementStatics6(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUIElementStatics6 {}
impl ::core::clone::Clone for IUIElementStatics6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUIElementStatics7(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUIElementStatics7 {}
impl ::core::clone::Clone for IUIElementStatics7 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUIElementStatics8(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUIElementStatics8 {}
impl ::core::clone::Clone for IUIElementStatics8 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUIElementStatics9(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUIElementStatics9 {}
impl ::core::clone::Clone for IUIElementStatics9 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUIElementWeakCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUIElementWeakCollection {}
impl ::core::clone::Clone for IUIElementWeakCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUIElementWeakCollectionFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUIElementWeakCollectionFactory {}
impl ::core::clone::Clone for IUIElementWeakCollectionFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUnhandledExceptionEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUnhandledExceptionEventArgs {}
impl ::core::clone::Clone for IUnhandledExceptionEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVector3Transition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVector3Transition {}
impl ::core::clone::Clone for IVector3Transition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVector3TransitionFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVector3TransitionFactory {}
impl ::core::clone::Clone for IVector3TransitionFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVisualState(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVisualState {}
impl ::core::clone::Clone for IVisualState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVisualState2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVisualState2 {}
impl ::core::clone::Clone for IVisualState2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVisualStateChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVisualStateChangedEventArgs {}
impl ::core::clone::Clone for IVisualStateChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVisualStateGroup(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVisualStateGroup {}
impl ::core::clone::Clone for IVisualStateGroup {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVisualStateManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVisualStateManager {}
impl ::core::clone::Clone for IVisualStateManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVisualStateManagerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVisualStateManagerFactory {}
impl ::core::clone::Clone for IVisualStateManagerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVisualStateManagerOverrides(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVisualStateManagerOverrides {}
impl ::core::clone::Clone for IVisualStateManagerOverrides {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVisualStateManagerProtected(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVisualStateManagerProtected {}
impl ::core::clone::Clone for IVisualStateManagerProtected {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVisualStateManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVisualStateManagerStatics {}
impl ::core::clone::Clone for IVisualStateManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVisualTransition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVisualTransition {}
impl ::core::clone::Clone for IVisualTransition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVisualTransitionFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVisualTransitionFactory {}
impl ::core::clone::Clone for IVisualTransitionFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWindow(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWindow {}
impl ::core::clone::Clone for IWindow {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWindow2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWindow2 {}
impl ::core::clone::Clone for IWindow2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWindow3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWindow3 {}
impl ::core::clone::Clone for IWindow3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWindow4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWindow4 {}
impl ::core::clone::Clone for IWindow4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWindowCreatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWindowCreatedEventArgs {}
impl ::core::clone::Clone for IWindowCreatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWindowStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWindowStatics {}
impl ::core::clone::Clone for IWindowStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXamlRoot(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXamlRoot {}
impl ::core::clone::Clone for IXamlRoot {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXamlRootChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXamlRootChangedEventArgs {}
impl ::core::clone::Clone for IXamlRootChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LeavingBackgroundEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LeavingBackgroundEventHandler {}
impl ::core::clone::Clone for LeavingBackgroundEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LineStackingStrategy(pub i32);
impl LineStackingStrategy {
    pub const MaxHeight: Self = Self(0i32);
    pub const BlockLineHeight: Self = Self(1i32);
    pub const BaselineToBaseline: Self = Self(2i32);
}
impl ::core::marker::Copy for LineStackingStrategy {}
impl ::core::clone::Clone for LineStackingStrategy {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaFailedRoutedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaFailedRoutedEventArgs {}
impl ::core::clone::Clone for MediaFailedRoutedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct OpticalMarginAlignment(pub i32);
impl OpticalMarginAlignment {
    pub const None: Self = Self(0i32);
    pub const TrimSideBearings: Self = Self(1i32);
}
impl ::core::marker::Copy for OpticalMarginAlignment {}
impl ::core::clone::Clone for OpticalMarginAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PointHelper(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PointHelper {}
impl ::core::clone::Clone for PointHelper {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PropertyChangedCallback(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PropertyChangedCallback {}
impl ::core::clone::Clone for PropertyChangedCallback {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PropertyMetadata(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PropertyMetadata {}
impl ::core::clone::Clone for PropertyMetadata {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PropertyPath(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PropertyPath {}
impl ::core::clone::Clone for PropertyPath {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RectHelper(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RectHelper {}
impl ::core::clone::Clone for RectHelper {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ResourceDictionary(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ResourceDictionary {}
impl ::core::clone::Clone for ResourceDictionary {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RoutedEvent(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RoutedEvent {}
impl ::core::clone::Clone for RoutedEvent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RoutedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RoutedEventArgs {}
impl ::core::clone::Clone for RoutedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RoutedEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RoutedEventHandler {}
impl ::core::clone::Clone for RoutedEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ScalarTransition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ScalarTransition {}
impl ::core::clone::Clone for ScalarTransition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Setter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Setter {}
impl ::core::clone::Clone for Setter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SetterBase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SetterBase {}
impl ::core::clone::Clone for SetterBase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SetterBaseCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SetterBaseCollection {}
impl ::core::clone::Clone for SetterBaseCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SizeChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SizeChangedEventArgs {}
impl ::core::clone::Clone for SizeChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SizeChangedEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SizeChangedEventHandler {}
impl ::core::clone::Clone for SizeChangedEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SizeHelper(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SizeHelper {}
impl ::core::clone::Clone for SizeHelper {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StateTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for StateTrigger {}
impl ::core::clone::Clone for StateTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StateTriggerBase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for StateTriggerBase {}
impl ::core::clone::Clone for StateTriggerBase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Style(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Style {}
impl ::core::clone::Clone for Style {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SuspendingEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SuspendingEventHandler {}
impl ::core::clone::Clone for SuspendingEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TargetPropertyPath(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TargetPropertyPath {}
impl ::core::clone::Clone for TargetPropertyPath {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TextAlignment(pub i32);
impl TextAlignment {
    pub const Center: Self = Self(0i32);
    pub const Left: Self = Self(1i32);
    pub const Start: Self = Self(1i32);
    pub const Right: Self = Self(2i32);
    pub const End: Self = Self(2i32);
    pub const Justify: Self = Self(3i32);
    pub const DetectFromContent: Self = Self(4i32);
}
impl ::core::marker::Copy for TextAlignment {}
impl ::core::clone::Clone for TextAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TextLineBounds(pub i32);
impl TextLineBounds {
    pub const Full: Self = Self(0i32);
    pub const TrimToCapHeight: Self = Self(1i32);
    pub const TrimToBaseline: Self = Self(2i32);
    pub const Tight: Self = Self(3i32);
}
impl ::core::marker::Copy for TextLineBounds {}
impl ::core::clone::Clone for TextLineBounds {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TextReadingOrder(pub i32);
impl TextReadingOrder {
    pub const Default: Self = Self(0i32);
    pub const UseFlowDirection: Self = Self(0i32);
    pub const DetectFromContent: Self = Self(1i32);
}
impl ::core::marker::Copy for TextReadingOrder {}
impl ::core::clone::Clone for TextReadingOrder {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TextTrimming(pub i32);
impl TextTrimming {
    pub const None: Self = Self(0i32);
    pub const CharacterEllipsis: Self = Self(1i32);
    pub const WordEllipsis: Self = Self(2i32);
    pub const Clip: Self = Self(3i32);
}
impl ::core::marker::Copy for TextTrimming {}
impl ::core::clone::Clone for TextTrimming {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TextWrapping(pub i32);
impl TextWrapping {
    pub const NoWrap: Self = Self(1i32);
    pub const Wrap: Self = Self(2i32);
    pub const WrapWholeWords: Self = Self(3i32);
}
impl ::core::marker::Copy for TextWrapping {}
impl ::core::clone::Clone for TextWrapping {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct Thickness {
    pub Left: f64,
    pub Top: f64,
    pub Right: f64,
    pub Bottom: f64,
}
impl ::core::marker::Copy for Thickness {}
impl ::core::clone::Clone for Thickness {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ThicknessHelper(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ThicknessHelper {}
impl ::core::clone::Clone for ThicknessHelper {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TriggerAction(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TriggerAction {}
impl ::core::clone::Clone for TriggerAction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TriggerActionCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TriggerActionCollection {}
impl ::core::clone::Clone for TriggerActionCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TriggerBase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TriggerBase {}
impl ::core::clone::Clone for TriggerBase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TriggerCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TriggerCollection {}
impl ::core::clone::Clone for TriggerCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UIElement(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UIElement {}
impl ::core::clone::Clone for UIElement {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UIElementWeakCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UIElementWeakCollection {}
impl ::core::clone::Clone for UIElementWeakCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UnhandledExceptionEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UnhandledExceptionEventArgs {}
impl ::core::clone::Clone for UnhandledExceptionEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UnhandledExceptionEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UnhandledExceptionEventHandler {}
impl ::core::clone::Clone for UnhandledExceptionEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Vector3Transition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Vector3Transition {}
impl ::core::clone::Clone for Vector3Transition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Vector3TransitionComponents(pub u32);
impl Vector3TransitionComponents {
    pub const X: Self = Self(1u32);
    pub const Y: Self = Self(2u32);
    pub const Z: Self = Self(4u32);
}
impl ::core::marker::Copy for Vector3TransitionComponents {}
impl ::core::clone::Clone for Vector3TransitionComponents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VerticalAlignment(pub i32);
impl VerticalAlignment {
    pub const Top: Self = Self(0i32);
    pub const Center: Self = Self(1i32);
    pub const Bottom: Self = Self(2i32);
    pub const Stretch: Self = Self(3i32);
}
impl ::core::marker::Copy for VerticalAlignment {}
impl ::core::clone::Clone for VerticalAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Visibility(pub i32);
impl Visibility {
    pub const Visible: Self = Self(0i32);
    pub const Collapsed: Self = Self(1i32);
}
impl ::core::marker::Copy for Visibility {}
impl ::core::clone::Clone for Visibility {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VisualState(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VisualState {}
impl ::core::clone::Clone for VisualState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VisualStateChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VisualStateChangedEventArgs {}
impl ::core::clone::Clone for VisualStateChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VisualStateChangedEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VisualStateChangedEventHandler {}
impl ::core::clone::Clone for VisualStateChangedEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VisualStateGroup(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VisualStateGroup {}
impl ::core::clone::Clone for VisualStateGroup {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VisualStateManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VisualStateManager {}
impl ::core::clone::Clone for VisualStateManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VisualTransition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VisualTransition {}
impl ::core::clone::Clone for VisualTransition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Window(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Window {}
impl ::core::clone::Clone for Window {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WindowActivatedEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WindowActivatedEventHandler {}
impl ::core::clone::Clone for WindowActivatedEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WindowClosedEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WindowClosedEventHandler {}
impl ::core::clone::Clone for WindowClosedEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WindowCreatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WindowCreatedEventArgs {}
impl ::core::clone::Clone for WindowCreatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WindowSizeChangedEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WindowSizeChangedEventHandler {}
impl ::core::clone::Clone for WindowSizeChangedEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WindowVisibilityChangedEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WindowVisibilityChangedEventHandler {}
impl ::core::clone::Clone for WindowVisibilityChangedEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XamlRoot(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for XamlRoot {}
impl ::core::clone::Clone for XamlRoot {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XamlRootChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for XamlRootChangedEventArgs {}
impl ::core::clone::Clone for XamlRootChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
