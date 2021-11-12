#![allow(non_snake_case, non_camel_case_types)]
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
pub struct ApplicationHighContrastAdjustment(i32);
#[repr(transparent)]
pub struct ApplicationInitializationCallback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ApplicationInitializationCallbackParams(pub *mut ::core::ffi::c_void);
pub struct ApplicationRequiresPointerMode(i32);
pub struct ApplicationTheme(i32);
pub struct AutomationTextAttributesEnum(i32);
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
pub struct Duration(i32);
#[repr(transparent)]
pub struct DurationHelper(pub *mut ::core::ffi::c_void);
pub struct DurationType(i32);
#[repr(transparent)]
pub struct EffectiveViewportChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ElementFactoryGetArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ElementFactoryRecycleArgs(pub *mut ::core::ffi::c_void);
pub struct ElementHighContrastAdjustment(i32);
pub struct ElementSoundKind(i32);
pub struct ElementSoundMode(i32);
#[repr(transparent)]
pub struct ElementSoundPlayer(pub *mut ::core::ffi::c_void);
pub struct ElementSoundPlayerState(i32);
pub struct ElementSpatialAudioMode(i32);
pub struct ElementTheme(i32);
#[repr(transparent)]
pub struct EnteredBackgroundEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EventTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ExceptionRoutedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ExceptionRoutedEventHandler(pub *mut ::core::ffi::c_void);
pub struct FlowDirection(i32);
pub struct FocusState(i32);
pub struct FocusVisualKind(i32);
pub struct FontCapitals(i32);
pub struct FontEastAsianLanguage(i32);
pub struct FontEastAsianWidths(i32);
pub struct FontFraction(i32);
pub struct FontNumeralAlignment(i32);
pub struct FontNumeralStyle(i32);
pub struct FontVariants(i32);
#[repr(transparent)]
pub struct FrameworkElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FrameworkTemplate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FrameworkView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FrameworkViewSource(pub *mut ::core::ffi::c_void);
pub struct GridLength(i32);
#[repr(transparent)]
pub struct GridLengthHelper(pub *mut ::core::ffi::c_void);
pub struct GridUnitType(i32);
pub struct HorizontalAlignment(i32);
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
pub struct LineStackingStrategy(i32);
#[repr(transparent)]
pub struct MediaFailedRoutedEventArgs(pub *mut ::core::ffi::c_void);
pub struct OpticalMarginAlignment(i32);
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
pub struct TextAlignment(i32);
pub struct TextLineBounds(i32);
pub struct TextReadingOrder(i32);
pub struct TextTrimming(i32);
pub struct TextWrapping(i32);
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
pub struct Vector3TransitionComponents(i32);
pub struct VerticalAlignment(i32);
pub struct Visibility(i32);
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
