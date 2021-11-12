#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
#[repr(transparent)]
pub struct Application(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ApplicationHighContrastAdjustment(pub u32);
impl ApplicationHighContrastAdjustment {
    pub const None: ApplicationHighContrastAdjustment = ApplicationHighContrastAdjustment(0u32);
    pub const Auto: ApplicationHighContrastAdjustment = ApplicationHighContrastAdjustment(4294967295u32);
}
#[repr(transparent)]
pub struct ApplicationInitializationCallback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ApplicationInitializationCallbackParams(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ApplicationRequiresPointerMode(pub i32);
impl ApplicationRequiresPointerMode {
    pub const Auto: ApplicationRequiresPointerMode = ApplicationRequiresPointerMode(0i32);
    pub const WhenRequested: ApplicationRequiresPointerMode = ApplicationRequiresPointerMode(1i32);
}
#[repr(transparent)]
pub struct ApplicationTheme(pub i32);
impl ApplicationTheme {
    pub const Light: ApplicationTheme = ApplicationTheme(0i32);
    pub const Dark: ApplicationTheme = ApplicationTheme(1i32);
}
#[repr(transparent)]
pub struct AutomationTextAttributesEnum(pub i32);
impl AutomationTextAttributesEnum {
    pub const AnimationStyleAttribute: AutomationTextAttributesEnum = AutomationTextAttributesEnum(40000i32);
    pub const BackgroundColorAttribute: AutomationTextAttributesEnum = AutomationTextAttributesEnum(40001i32);
    pub const BulletStyleAttribute: AutomationTextAttributesEnum = AutomationTextAttributesEnum(40002i32);
    pub const CapStyleAttribute: AutomationTextAttributesEnum = AutomationTextAttributesEnum(40003i32);
    pub const CultureAttribute: AutomationTextAttributesEnum = AutomationTextAttributesEnum(40004i32);
    pub const FontNameAttribute: AutomationTextAttributesEnum = AutomationTextAttributesEnum(40005i32);
    pub const FontSizeAttribute: AutomationTextAttributesEnum = AutomationTextAttributesEnum(40006i32);
    pub const FontWeightAttribute: AutomationTextAttributesEnum = AutomationTextAttributesEnum(40007i32);
    pub const ForegroundColorAttribute: AutomationTextAttributesEnum = AutomationTextAttributesEnum(40008i32);
    pub const HorizontalTextAlignmentAttribute: AutomationTextAttributesEnum = AutomationTextAttributesEnum(40009i32);
    pub const IndentationFirstLineAttribute: AutomationTextAttributesEnum = AutomationTextAttributesEnum(40010i32);
    pub const IndentationLeadingAttribute: AutomationTextAttributesEnum = AutomationTextAttributesEnum(40011i32);
    pub const IndentationTrailingAttribute: AutomationTextAttributesEnum = AutomationTextAttributesEnum(40012i32);
    pub const IsHiddenAttribute: AutomationTextAttributesEnum = AutomationTextAttributesEnum(40013i32);
    pub const IsItalicAttribute: AutomationTextAttributesEnum = AutomationTextAttributesEnum(40014i32);
    pub const IsReadOnlyAttribute: AutomationTextAttributesEnum = AutomationTextAttributesEnum(40015i32);
    pub const IsSubscriptAttribute: AutomationTextAttributesEnum = AutomationTextAttributesEnum(40016i32);
    pub const IsSuperscriptAttribute: AutomationTextAttributesEnum = AutomationTextAttributesEnum(40017i32);
    pub const MarginBottomAttribute: AutomationTextAttributesEnum = AutomationTextAttributesEnum(40018i32);
    pub const MarginLeadingAttribute: AutomationTextAttributesEnum = AutomationTextAttributesEnum(40019i32);
    pub const MarginTopAttribute: AutomationTextAttributesEnum = AutomationTextAttributesEnum(40020i32);
    pub const MarginTrailingAttribute: AutomationTextAttributesEnum = AutomationTextAttributesEnum(40021i32);
    pub const OutlineStylesAttribute: AutomationTextAttributesEnum = AutomationTextAttributesEnum(40022i32);
    pub const OverlineColorAttribute: AutomationTextAttributesEnum = AutomationTextAttributesEnum(40023i32);
    pub const OverlineStyleAttribute: AutomationTextAttributesEnum = AutomationTextAttributesEnum(40024i32);
    pub const StrikethroughColorAttribute: AutomationTextAttributesEnum = AutomationTextAttributesEnum(40025i32);
    pub const StrikethroughStyleAttribute: AutomationTextAttributesEnum = AutomationTextAttributesEnum(40026i32);
    pub const TabsAttribute: AutomationTextAttributesEnum = AutomationTextAttributesEnum(40027i32);
    pub const TextFlowDirectionsAttribute: AutomationTextAttributesEnum = AutomationTextAttributesEnum(40028i32);
    pub const UnderlineColorAttribute: AutomationTextAttributesEnum = AutomationTextAttributesEnum(40029i32);
    pub const UnderlineStyleAttribute: AutomationTextAttributesEnum = AutomationTextAttributesEnum(40030i32);
    pub const AnnotationTypesAttribute: AutomationTextAttributesEnum = AutomationTextAttributesEnum(40031i32);
    pub const AnnotationObjectsAttribute: AutomationTextAttributesEnum = AutomationTextAttributesEnum(40032i32);
    pub const StyleNameAttribute: AutomationTextAttributesEnum = AutomationTextAttributesEnum(40033i32);
    pub const StyleIdAttribute: AutomationTextAttributesEnum = AutomationTextAttributesEnum(40034i32);
    pub const LinkAttribute: AutomationTextAttributesEnum = AutomationTextAttributesEnum(40035i32);
    pub const IsActiveAttribute: AutomationTextAttributesEnum = AutomationTextAttributesEnum(40036i32);
    pub const SelectionActiveEndAttribute: AutomationTextAttributesEnum = AutomationTextAttributesEnum(40037i32);
    pub const CaretPositionAttribute: AutomationTextAttributesEnum = AutomationTextAttributesEnum(40038i32);
    pub const CaretBidiModeAttribute: AutomationTextAttributesEnum = AutomationTextAttributesEnum(40039i32);
}
#[repr(transparent)]
pub struct BindingFailedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BindingFailedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BringIntoViewOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BringIntoViewRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BrushTransition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ColorPaletteResources(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct CornerRadius(i32);
#[repr(transparent)]
pub struct CornerRadiusHelper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CreateDefaultValueCallback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DataContextChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DataTemplate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DataTemplateKey(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DebugSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DependencyObject(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DependencyObjectCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DependencyProperty(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DependencyPropertyChangedCallback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DependencyPropertyChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DependencyPropertyChangedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispatcherTimer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DragEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DragEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DragOperationDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DragStartingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DragUI(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DragUIOverride(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DropCompletedEventArgs(pub *mut ::core::ffi::c_void);
#[cfg(feature = "Foundation")]
#[repr(C)]
pub struct Duration(i32);
#[repr(transparent)]
pub struct DurationHelper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DurationType(pub i32);
impl DurationType {
    pub const Automatic: DurationType = DurationType(0i32);
    pub const TimeSpan: DurationType = DurationType(1i32);
    pub const Forever: DurationType = DurationType(2i32);
}
#[repr(transparent)]
pub struct EffectiveViewportChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ElementFactoryGetArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ElementFactoryRecycleArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ElementHighContrastAdjustment(pub u32);
impl ElementHighContrastAdjustment {
    pub const None: ElementHighContrastAdjustment = ElementHighContrastAdjustment(0u32);
    pub const Application: ElementHighContrastAdjustment = ElementHighContrastAdjustment(2147483648u32);
    pub const Auto: ElementHighContrastAdjustment = ElementHighContrastAdjustment(4294967295u32);
}
#[repr(transparent)]
pub struct ElementSoundKind(pub i32);
impl ElementSoundKind {
    pub const Focus: ElementSoundKind = ElementSoundKind(0i32);
    pub const Invoke: ElementSoundKind = ElementSoundKind(1i32);
    pub const Show: ElementSoundKind = ElementSoundKind(2i32);
    pub const Hide: ElementSoundKind = ElementSoundKind(3i32);
    pub const MovePrevious: ElementSoundKind = ElementSoundKind(4i32);
    pub const MoveNext: ElementSoundKind = ElementSoundKind(5i32);
    pub const GoBack: ElementSoundKind = ElementSoundKind(6i32);
}
#[repr(transparent)]
pub struct ElementSoundMode(pub i32);
impl ElementSoundMode {
    pub const Default: ElementSoundMode = ElementSoundMode(0i32);
    pub const FocusOnly: ElementSoundMode = ElementSoundMode(1i32);
    pub const Off: ElementSoundMode = ElementSoundMode(2i32);
}
#[repr(transparent)]
pub struct ElementSoundPlayer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ElementSoundPlayerState(pub i32);
impl ElementSoundPlayerState {
    pub const Auto: ElementSoundPlayerState = ElementSoundPlayerState(0i32);
    pub const Off: ElementSoundPlayerState = ElementSoundPlayerState(1i32);
    pub const On: ElementSoundPlayerState = ElementSoundPlayerState(2i32);
}
#[repr(transparent)]
pub struct ElementSpatialAudioMode(pub i32);
impl ElementSpatialAudioMode {
    pub const Auto: ElementSpatialAudioMode = ElementSpatialAudioMode(0i32);
    pub const Off: ElementSpatialAudioMode = ElementSpatialAudioMode(1i32);
    pub const On: ElementSpatialAudioMode = ElementSpatialAudioMode(2i32);
}
#[repr(transparent)]
pub struct ElementTheme(pub i32);
impl ElementTheme {
    pub const Default: ElementTheme = ElementTheme(0i32);
    pub const Light: ElementTheme = ElementTheme(1i32);
    pub const Dark: ElementTheme = ElementTheme(2i32);
}
#[repr(transparent)]
pub struct EnteredBackgroundEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EventTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ExceptionRoutedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ExceptionRoutedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FlowDirection(pub i32);
impl FlowDirection {
    pub const LeftToRight: FlowDirection = FlowDirection(0i32);
    pub const RightToLeft: FlowDirection = FlowDirection(1i32);
}
#[repr(transparent)]
pub struct FocusState(pub i32);
impl FocusState {
    pub const Unfocused: FocusState = FocusState(0i32);
    pub const Pointer: FocusState = FocusState(1i32);
    pub const Keyboard: FocusState = FocusState(2i32);
    pub const Programmatic: FocusState = FocusState(3i32);
}
#[repr(transparent)]
pub struct FocusVisualKind(pub i32);
impl FocusVisualKind {
    pub const DottedLine: FocusVisualKind = FocusVisualKind(0i32);
    pub const HighVisibility: FocusVisualKind = FocusVisualKind(1i32);
    pub const Reveal: FocusVisualKind = FocusVisualKind(2i32);
}
#[repr(transparent)]
pub struct FontCapitals(pub i32);
impl FontCapitals {
    pub const Normal: FontCapitals = FontCapitals(0i32);
    pub const AllSmallCaps: FontCapitals = FontCapitals(1i32);
    pub const SmallCaps: FontCapitals = FontCapitals(2i32);
    pub const AllPetiteCaps: FontCapitals = FontCapitals(3i32);
    pub const PetiteCaps: FontCapitals = FontCapitals(4i32);
    pub const Unicase: FontCapitals = FontCapitals(5i32);
    pub const Titling: FontCapitals = FontCapitals(6i32);
}
#[repr(transparent)]
pub struct FontEastAsianLanguage(pub i32);
impl FontEastAsianLanguage {
    pub const Normal: FontEastAsianLanguage = FontEastAsianLanguage(0i32);
    pub const HojoKanji: FontEastAsianLanguage = FontEastAsianLanguage(1i32);
    pub const Jis04: FontEastAsianLanguage = FontEastAsianLanguage(2i32);
    pub const Jis78: FontEastAsianLanguage = FontEastAsianLanguage(3i32);
    pub const Jis83: FontEastAsianLanguage = FontEastAsianLanguage(4i32);
    pub const Jis90: FontEastAsianLanguage = FontEastAsianLanguage(5i32);
    pub const NlcKanji: FontEastAsianLanguage = FontEastAsianLanguage(6i32);
    pub const Simplified: FontEastAsianLanguage = FontEastAsianLanguage(7i32);
    pub const Traditional: FontEastAsianLanguage = FontEastAsianLanguage(8i32);
    pub const TraditionalNames: FontEastAsianLanguage = FontEastAsianLanguage(9i32);
}
#[repr(transparent)]
pub struct FontEastAsianWidths(pub i32);
impl FontEastAsianWidths {
    pub const Normal: FontEastAsianWidths = FontEastAsianWidths(0i32);
    pub const Full: FontEastAsianWidths = FontEastAsianWidths(1i32);
    pub const Half: FontEastAsianWidths = FontEastAsianWidths(2i32);
    pub const Proportional: FontEastAsianWidths = FontEastAsianWidths(3i32);
    pub const Quarter: FontEastAsianWidths = FontEastAsianWidths(4i32);
    pub const Third: FontEastAsianWidths = FontEastAsianWidths(5i32);
}
#[repr(transparent)]
pub struct FontFraction(pub i32);
impl FontFraction {
    pub const Normal: FontFraction = FontFraction(0i32);
    pub const Stacked: FontFraction = FontFraction(1i32);
    pub const Slashed: FontFraction = FontFraction(2i32);
}
#[repr(transparent)]
pub struct FontNumeralAlignment(pub i32);
impl FontNumeralAlignment {
    pub const Normal: FontNumeralAlignment = FontNumeralAlignment(0i32);
    pub const Proportional: FontNumeralAlignment = FontNumeralAlignment(1i32);
    pub const Tabular: FontNumeralAlignment = FontNumeralAlignment(2i32);
}
#[repr(transparent)]
pub struct FontNumeralStyle(pub i32);
impl FontNumeralStyle {
    pub const Normal: FontNumeralStyle = FontNumeralStyle(0i32);
    pub const Lining: FontNumeralStyle = FontNumeralStyle(1i32);
    pub const OldStyle: FontNumeralStyle = FontNumeralStyle(2i32);
}
#[repr(transparent)]
pub struct FontVariants(pub i32);
impl FontVariants {
    pub const Normal: FontVariants = FontVariants(0i32);
    pub const Superscript: FontVariants = FontVariants(1i32);
    pub const Subscript: FontVariants = FontVariants(2i32);
    pub const Ordinal: FontVariants = FontVariants(3i32);
    pub const Inferior: FontVariants = FontVariants(4i32);
    pub const Ruby: FontVariants = FontVariants(5i32);
}
#[repr(transparent)]
pub struct FrameworkElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FrameworkTemplate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FrameworkView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FrameworkViewSource(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct GridLength(i32);
#[repr(transparent)]
pub struct GridLengthHelper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GridUnitType(pub i32);
impl GridUnitType {
    pub const Auto: GridUnitType = GridUnitType(0i32);
    pub const Pixel: GridUnitType = GridUnitType(1i32);
    pub const Star: GridUnitType = GridUnitType(2i32);
}
#[repr(transparent)]
pub struct HorizontalAlignment(pub i32);
impl HorizontalAlignment {
    pub const Left: HorizontalAlignment = HorizontalAlignment(0i32);
    pub const Center: HorizontalAlignment = HorizontalAlignment(1i32);
    pub const Right: HorizontalAlignment = HorizontalAlignment(2i32);
    pub const Stretch: HorizontalAlignment = HorizontalAlignment(3i32);
}
#[repr(transparent)]
pub struct IAdaptiveTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAdaptiveTriggerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAdaptiveTriggerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IApplication(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IApplication2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IApplication3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IApplicationFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IApplicationInitializationCallbackParams(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IApplicationOverrides(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IApplicationOverrides2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IApplicationStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBindingFailedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBringIntoViewOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBringIntoViewOptions2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBringIntoViewRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBrushTransition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBrushTransitionFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IColorPaletteResources(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IColorPaletteResourcesFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICornerRadiusHelper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICornerRadiusHelperStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataContextChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataTemplate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataTemplateExtension(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataTemplateFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataTemplateKey(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataTemplateKeyFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataTemplateStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugSettings2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugSettings3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugSettings4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDependencyObject(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDependencyObject2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDependencyObjectCollectionFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDependencyObjectFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDependencyProperty(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDependencyPropertyChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDependencyPropertyStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDispatcherTimer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDispatcherTimerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDragEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDragEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDragEventArgs3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDragOperationDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDragStartingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDragStartingEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDragUI(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDragUIOverride(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDropCompletedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDurationHelper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDurationHelperStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEffectiveViewportChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IElementFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IElementFactoryGetArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IElementFactoryGetArgsFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IElementFactoryRecycleArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IElementFactoryRecycleArgsFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IElementSoundPlayer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IElementSoundPlayerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IElementSoundPlayerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEventTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IExceptionRoutedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IExceptionRoutedEventArgsFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFrameworkElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFrameworkElement2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFrameworkElement3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFrameworkElement4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFrameworkElement6(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFrameworkElement7(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFrameworkElementFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFrameworkElementOverrides(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFrameworkElementOverrides2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFrameworkElementProtected7(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFrameworkElementStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFrameworkElementStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFrameworkElementStatics4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFrameworkElementStatics5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFrameworkElementStatics6(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFrameworkTemplate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFrameworkTemplateFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFrameworkView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFrameworkViewSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGridLengthHelper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGridLengthHelperStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaFailedRoutedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPointHelper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPointHelperStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPropertyMetadata(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPropertyMetadataFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPropertyMetadataStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPropertyPath(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPropertyPathFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRectHelper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRectHelperStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IResourceDictionary(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IResourceDictionaryFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRoutedEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRoutedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRoutedEventArgsFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScalarTransition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScalarTransitionFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISetter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISetter2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISetterBase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISetterBaseCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISetterBaseFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISetterFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISizeChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISizeHelper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISizeHelperStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStateTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStateTriggerBase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStateTriggerBaseFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStateTriggerBaseProtected(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStateTriggerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStyle(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStyleFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITargetPropertyPath(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITargetPropertyPathFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IThicknessHelper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IThicknessHelperStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITriggerAction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITriggerActionFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITriggerBase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITriggerBaseFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIElement10(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIElement2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIElement3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIElement4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIElement5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIElement7(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIElement8(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIElement9(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIElementFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIElementOverrides(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIElementOverrides7(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIElementOverrides8(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIElementOverrides9(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIElementStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIElementStatics10(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIElementStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIElementStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIElementStatics4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIElementStatics5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIElementStatics6(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIElementStatics7(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIElementStatics8(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIElementStatics9(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIElementWeakCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIElementWeakCollectionFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUnhandledExceptionEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVector3Transition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVector3TransitionFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVisualState(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVisualState2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVisualStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVisualStateGroup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVisualStateManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVisualStateManagerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVisualStateManagerOverrides(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVisualStateManagerProtected(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVisualStateManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVisualTransition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVisualTransitionFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWindow(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWindow2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWindow3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWindow4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWindowCreatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWindowStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXamlRoot(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXamlRootChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LeavingBackgroundEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LineStackingStrategy(pub i32);
impl LineStackingStrategy {
    pub const MaxHeight: LineStackingStrategy = LineStackingStrategy(0i32);
    pub const BlockLineHeight: LineStackingStrategy = LineStackingStrategy(1i32);
    pub const BaselineToBaseline: LineStackingStrategy = LineStackingStrategy(2i32);
}
#[repr(transparent)]
pub struct MediaFailedRoutedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct OpticalMarginAlignment(pub i32);
impl OpticalMarginAlignment {
    pub const None: OpticalMarginAlignment = OpticalMarginAlignment(0i32);
    pub const TrimSideBearings: OpticalMarginAlignment = OpticalMarginAlignment(1i32);
}
#[repr(transparent)]
pub struct PointHelper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PropertyChangedCallback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PropertyMetadata(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PropertyPath(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RectHelper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ResourceDictionary(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RoutedEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RoutedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RoutedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ScalarTransition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Setter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SetterBase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SetterBaseCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SizeChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SizeChangedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SizeHelper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StateTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StateTriggerBase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Style(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SuspendingEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TargetPropertyPath(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TextAlignment(pub i32);
impl TextAlignment {
    pub const Center: TextAlignment = TextAlignment(0i32);
    pub const Left: TextAlignment = TextAlignment(1i32);
    pub const Start: TextAlignment = TextAlignment(1i32);
    pub const Right: TextAlignment = TextAlignment(2i32);
    pub const End: TextAlignment = TextAlignment(2i32);
    pub const Justify: TextAlignment = TextAlignment(3i32);
    pub const DetectFromContent: TextAlignment = TextAlignment(4i32);
}
#[repr(transparent)]
pub struct TextLineBounds(pub i32);
impl TextLineBounds {
    pub const Full: TextLineBounds = TextLineBounds(0i32);
    pub const TrimToCapHeight: TextLineBounds = TextLineBounds(1i32);
    pub const TrimToBaseline: TextLineBounds = TextLineBounds(2i32);
    pub const Tight: TextLineBounds = TextLineBounds(3i32);
}
#[repr(transparent)]
pub struct TextReadingOrder(pub i32);
impl TextReadingOrder {
    pub const Default: TextReadingOrder = TextReadingOrder(0i32);
    pub const UseFlowDirection: TextReadingOrder = TextReadingOrder(0i32);
    pub const DetectFromContent: TextReadingOrder = TextReadingOrder(1i32);
}
#[repr(transparent)]
pub struct TextTrimming(pub i32);
impl TextTrimming {
    pub const None: TextTrimming = TextTrimming(0i32);
    pub const CharacterEllipsis: TextTrimming = TextTrimming(1i32);
    pub const WordEllipsis: TextTrimming = TextTrimming(2i32);
    pub const Clip: TextTrimming = TextTrimming(3i32);
}
#[repr(transparent)]
pub struct TextWrapping(pub i32);
impl TextWrapping {
    pub const NoWrap: TextWrapping = TextWrapping(1i32);
    pub const Wrap: TextWrapping = TextWrapping(2i32);
    pub const WrapWholeWords: TextWrapping = TextWrapping(3i32);
}
#[repr(C)]
pub struct Thickness(i32);
#[repr(transparent)]
pub struct ThicknessHelper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TriggerAction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TriggerActionCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TriggerBase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TriggerCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UIElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UIElementWeakCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UnhandledExceptionEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UnhandledExceptionEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Vector3Transition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Vector3TransitionComponents(pub u32);
impl Vector3TransitionComponents {
    pub const X: Vector3TransitionComponents = Vector3TransitionComponents(1u32);
    pub const Y: Vector3TransitionComponents = Vector3TransitionComponents(2u32);
    pub const Z: Vector3TransitionComponents = Vector3TransitionComponents(4u32);
}
#[repr(transparent)]
pub struct VerticalAlignment(pub i32);
impl VerticalAlignment {
    pub const Top: VerticalAlignment = VerticalAlignment(0i32);
    pub const Center: VerticalAlignment = VerticalAlignment(1i32);
    pub const Bottom: VerticalAlignment = VerticalAlignment(2i32);
    pub const Stretch: VerticalAlignment = VerticalAlignment(3i32);
}
#[repr(transparent)]
pub struct Visibility(pub i32);
impl Visibility {
    pub const Visible: Visibility = Visibility(0i32);
    pub const Collapsed: Visibility = Visibility(1i32);
}
#[repr(transparent)]
pub struct VisualState(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VisualStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VisualStateChangedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VisualStateGroup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VisualStateManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VisualTransition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Window(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WindowActivatedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WindowClosedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WindowCreatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WindowSizeChangedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WindowVisibilityChangedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct XamlRoot(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct XamlRootChangedEventArgs(pub *mut ::core::ffi::c_void);
