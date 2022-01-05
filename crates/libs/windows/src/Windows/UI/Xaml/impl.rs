#[cfg(feature = "implement_exclusive")]
pub trait IAdaptiveTriggerImpl: Sized {
    fn MinWindowWidth(&self) -> ::windows::core::Result<f64>;
    fn SetMinWindowWidth(&self, value: f64) -> ::windows::core::Result<()>;
    fn MinWindowHeight(&self) -> ::windows::core::Result<f64>;
    fn SetMinWindowHeight(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdaptiveTriggerFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<AdaptiveTrigger>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdaptiveTriggerStaticsImpl: Sized {
    fn MinWindowWidthProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn MinWindowHeightProperty(&self) -> ::windows::core::Result<DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationImpl: Sized {
    fn Resources(&self) -> ::windows::core::Result<ResourceDictionary>;
    fn SetResources(&self, value: &::core::option::Option<ResourceDictionary>) -> ::windows::core::Result<()>;
    fn DebugSettings(&self) -> ::windows::core::Result<DebugSettings>;
    fn RequestedTheme(&self) -> ::windows::core::Result<ApplicationTheme>;
    fn SetRequestedTheme(&self, value: ApplicationTheme) -> ::windows::core::Result<()>;
    fn UnhandledException(&self, handler: &::core::option::Option<UnhandledExceptionEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUnhandledException(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Suspending(&self, handler: &::core::option::Option<SuspendingEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSuspending(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Resuming(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveResuming(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Exit(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplication2Impl: Sized {
    fn FocusVisualKind(&self) -> ::windows::core::Result<FocusVisualKind>;
    fn SetFocusVisualKind(&self, value: FocusVisualKind) -> ::windows::core::Result<()>;
    fn RequiresPointerMode(&self) -> ::windows::core::Result<ApplicationRequiresPointerMode>;
    fn SetRequiresPointerMode(&self, value: ApplicationRequiresPointerMode) -> ::windows::core::Result<()>;
    fn LeavingBackground(&self, handler: &::core::option::Option<LeavingBackgroundEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLeavingBackground(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EnteredBackground(&self, handler: &::core::option::Option<EnteredBackgroundEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveEnteredBackground(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplication3Impl: Sized {
    fn HighContrastAdjustment(&self) -> ::windows::core::Result<ApplicationHighContrastAdjustment>;
    fn SetHighContrastAdjustment(&self, value: ApplicationHighContrastAdjustment) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<Application>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationInitializationCallbackParamsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationOverridesImpl: Sized {
    fn OnActivated(&self, args: &::core::option::Option<super::super::ApplicationModel::Activation::IActivatedEventArgs>) -> ::windows::core::Result<()>;
    fn OnLaunched(&self, args: &::core::option::Option<super::super::ApplicationModel::Activation::LaunchActivatedEventArgs>) -> ::windows::core::Result<()>;
    fn OnFileActivated(&self, args: &::core::option::Option<super::super::ApplicationModel::Activation::FileActivatedEventArgs>) -> ::windows::core::Result<()>;
    fn OnSearchActivated(&self, args: &::core::option::Option<super::super::ApplicationModel::Activation::SearchActivatedEventArgs>) -> ::windows::core::Result<()>;
    fn OnShareTargetActivated(&self, args: &::core::option::Option<super::super::ApplicationModel::Activation::ShareTargetActivatedEventArgs>) -> ::windows::core::Result<()>;
    fn OnFileOpenPickerActivated(&self, args: &::core::option::Option<super::super::ApplicationModel::Activation::FileOpenPickerActivatedEventArgs>) -> ::windows::core::Result<()>;
    fn OnFileSavePickerActivated(&self, args: &::core::option::Option<super::super::ApplicationModel::Activation::FileSavePickerActivatedEventArgs>) -> ::windows::core::Result<()>;
    fn OnCachedFileUpdaterActivated(&self, args: &::core::option::Option<super::super::ApplicationModel::Activation::CachedFileUpdaterActivatedEventArgs>) -> ::windows::core::Result<()>;
    fn OnWindowCreated(&self, args: &::core::option::Option<WindowCreatedEventArgs>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationOverrides2Impl: Sized {
    fn OnBackgroundActivated(&self, args: &::core::option::Option<super::super::ApplicationModel::Activation::BackgroundActivatedEventArgs>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationStaticsImpl: Sized {
    fn Current(&self) -> ::windows::core::Result<Application>;
    fn Start(&self, callback: &::core::option::Option<ApplicationInitializationCallback>) -> ::windows::core::Result<()>;
    fn LoadComponent(&self, component: &::core::option::Option<::windows::core::IInspectable>, resourcelocator: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn LoadComponentWithResourceLocation(&self, component: &::core::option::Option<::windows::core::IInspectable>, resourcelocator: &::core::option::Option<super::super::Foundation::Uri>, componentresourcelocation: Controls::Primitives::ComponentResourceLocation) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBindingFailedEventArgsImpl: Sized {
    fn Message(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBringIntoViewOptionsImpl: Sized {
    fn AnimationDesired(&self) -> ::windows::core::Result<bool>;
    fn SetAnimationDesired(&self, value: bool) -> ::windows::core::Result<()>;
    fn TargetRect(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::Rect>>;
    fn SetTargetRect(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::Rect>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBringIntoViewOptions2Impl: Sized {
    fn HorizontalAlignmentRatio(&self) -> ::windows::core::Result<f64>;
    fn SetHorizontalAlignmentRatio(&self, value: f64) -> ::windows::core::Result<()>;
    fn VerticalAlignmentRatio(&self) -> ::windows::core::Result<f64>;
    fn SetVerticalAlignmentRatio(&self, value: f64) -> ::windows::core::Result<()>;
    fn HorizontalOffset(&self) -> ::windows::core::Result<f64>;
    fn SetHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()>;
    fn VerticalOffset(&self) -> ::windows::core::Result<f64>;
    fn SetVerticalOffset(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBringIntoViewRequestedEventArgsImpl: Sized {
    fn TargetElement(&self) -> ::windows::core::Result<UIElement>;
    fn SetTargetElement(&self, value: &::core::option::Option<UIElement>) -> ::windows::core::Result<()>;
    fn AnimationDesired(&self) -> ::windows::core::Result<bool>;
    fn SetAnimationDesired(&self, value: bool) -> ::windows::core::Result<()>;
    fn TargetRect(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn SetTargetRect(&self, value: &super::super::Foundation::Rect) -> ::windows::core::Result<()>;
    fn HorizontalAlignmentRatio(&self) -> ::windows::core::Result<f64>;
    fn VerticalAlignmentRatio(&self) -> ::windows::core::Result<f64>;
    fn HorizontalOffset(&self) -> ::windows::core::Result<f64>;
    fn SetHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()>;
    fn VerticalOffset(&self) -> ::windows::core::Result<f64>;
    fn SetVerticalOffset(&self, value: f64) -> ::windows::core::Result<()>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBrushTransitionImpl: Sized {
    fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetDuration(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBrushTransitionFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<BrushTransition>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorPaletteResourcesImpl: Sized {
    fn AltHigh(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetAltHigh(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn AltLow(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetAltLow(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn AltMedium(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetAltMedium(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn AltMediumHigh(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetAltMediumHigh(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn AltMediumLow(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetAltMediumLow(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn BaseHigh(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetBaseHigh(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn BaseLow(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetBaseLow(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn BaseMedium(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetBaseMedium(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn BaseMediumHigh(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetBaseMediumHigh(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn BaseMediumLow(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetBaseMediumLow(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ChromeAltLow(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetChromeAltLow(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ChromeBlackHigh(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetChromeBlackHigh(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ChromeBlackLow(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetChromeBlackLow(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ChromeBlackMediumLow(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetChromeBlackMediumLow(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ChromeBlackMedium(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetChromeBlackMedium(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ChromeDisabledHigh(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetChromeDisabledHigh(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ChromeDisabledLow(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetChromeDisabledLow(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ChromeHigh(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetChromeHigh(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ChromeLow(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetChromeLow(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ChromeMedium(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetChromeMedium(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ChromeMediumLow(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetChromeMediumLow(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ChromeWhite(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetChromeWhite(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ChromeGray(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetChromeGray(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ListLow(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetListLow(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ListMedium(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetListMedium(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ErrorText(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetErrorText(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn Accent(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetAccent(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorPaletteResourcesFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ColorPaletteResources>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICornerRadiusHelperImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ICornerRadiusHelperStaticsImpl: Sized {
    fn FromRadii(&self, topleft: f64, topright: f64, bottomright: f64, bottomleft: f64) -> ::windows::core::Result<CornerRadius>;
    fn FromUniformRadius(&self, uniformradius: f64) -> ::windows::core::Result<CornerRadius>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataContextChangedEventArgsImpl: Sized {
    fn NewValue(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataTemplateImpl: Sized {
    fn LoadContent(&self) -> ::windows::core::Result<DependencyObject>;
}
pub trait IDataTemplateExtensionImpl: Sized {
    fn ResetTemplate(&self) -> ::windows::core::Result<()>;
    fn ProcessBinding(&self, phase: u32) -> ::windows::core::Result<bool>;
    fn ProcessBindings(&self, arg: &::core::option::Option<Controls::ContainerContentChangingEventArgs>) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataTemplateFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<DataTemplate>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataTemplateKeyImpl: Sized {
    fn DataType(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetDataType(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataTemplateKeyFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<DataTemplateKey>;
    fn CreateInstanceWithType(&self, datatype: &::core::option::Option<::windows::core::IInspectable>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<DataTemplateKey>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataTemplateStatics2Impl: Sized {
    fn ExtensionInstanceProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn GetExtensionInstance(&self, element: &::core::option::Option<FrameworkElement>) -> ::windows::core::Result<IDataTemplateExtension>;
    fn SetExtensionInstance(&self, element: &::core::option::Option<FrameworkElement>, value: &::core::option::Option<IDataTemplateExtension>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDebugSettingsImpl: Sized {
    fn EnableFrameRateCounter(&self) -> ::windows::core::Result<bool>;
    fn SetEnableFrameRateCounter(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsBindingTracingEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsBindingTracingEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsOverdrawHeatMapEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsOverdrawHeatMapEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn BindingFailed(&self, handler: &::core::option::Option<BindingFailedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBindingFailed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDebugSettings2Impl: Sized {
    fn EnableRedrawRegions(&self) -> ::windows::core::Result<bool>;
    fn SetEnableRedrawRegions(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDebugSettings3Impl: Sized {
    fn IsTextPerformanceVisualizationEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsTextPerformanceVisualizationEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDebugSettings4Impl: Sized {
    fn FailFastOnErrors(&self) -> ::windows::core::Result<bool>;
    fn SetFailFastOnErrors(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDependencyObjectImpl: Sized {
    fn GetValue(&self, dp: &::core::option::Option<DependencyProperty>) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetValue(&self, dp: &::core::option::Option<DependencyProperty>, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn ClearValue(&self, dp: &::core::option::Option<DependencyProperty>) -> ::windows::core::Result<()>;
    fn ReadLocalValue(&self, dp: &::core::option::Option<DependencyProperty>) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn GetAnimationBaseValue(&self, dp: &::core::option::Option<DependencyProperty>) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn Dispatcher(&self) -> ::windows::core::Result<super::Core::CoreDispatcher>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDependencyObject2Impl: Sized {
    fn RegisterPropertyChangedCallback(&self, dp: &::core::option::Option<DependencyProperty>, callback: &::core::option::Option<DependencyPropertyChangedCallback>) -> ::windows::core::Result<i64>;
    fn UnregisterPropertyChangedCallback(&self, dp: &::core::option::Option<DependencyProperty>, token: i64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDependencyObjectCollectionFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<DependencyObjectCollection>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDependencyObjectFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<DependencyObject>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDependencyPropertyImpl: Sized {
    fn GetMetadata(&self, fortype: &Interop::TypeName) -> ::windows::core::Result<PropertyMetadata>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDependencyPropertyChangedEventArgsImpl: Sized {
    fn Property(&self) -> ::windows::core::Result<DependencyProperty>;
    fn OldValue(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn NewValue(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDependencyPropertyStaticsImpl: Sized {
    fn UnsetValue(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn Register(&self, name: &::windows::core::HSTRING, propertytype: &Interop::TypeName, ownertype: &Interop::TypeName, typemetadata: &::core::option::Option<PropertyMetadata>) -> ::windows::core::Result<DependencyProperty>;
    fn RegisterAttached(&self, name: &::windows::core::HSTRING, propertytype: &Interop::TypeName, ownertype: &Interop::TypeName, defaultmetadata: &::core::option::Option<PropertyMetadata>) -> ::windows::core::Result<DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDispatcherTimerImpl: Sized {
    fn Interval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetInterval(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn Tick(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveTick(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDispatcherTimerFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<DispatcherTimer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDragEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn Data(&self) -> ::windows::core::Result<super::super::ApplicationModel::DataTransfer::DataPackage>;
    fn SetData(&self, value: &::core::option::Option<super::super::ApplicationModel::DataTransfer::DataPackage>) -> ::windows::core::Result<()>;
    fn GetPosition(&self, relativeto: &::core::option::Option<UIElement>) -> ::windows::core::Result<super::super::Foundation::Point>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDragEventArgs2Impl: Sized {
    fn DataView(&self) -> ::windows::core::Result<super::super::ApplicationModel::DataTransfer::DataPackageView>;
    fn DragUIOverride(&self) -> ::windows::core::Result<DragUIOverride>;
    fn Modifiers(&self) -> ::windows::core::Result<super::super::ApplicationModel::DataTransfer::DragDrop::DragDropModifiers>;
    fn AcceptedOperation(&self) -> ::windows::core::Result<super::super::ApplicationModel::DataTransfer::DataPackageOperation>;
    fn SetAcceptedOperation(&self, value: super::super::ApplicationModel::DataTransfer::DataPackageOperation) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<DragOperationDeferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDragEventArgs3Impl: Sized {
    fn AllowedOperations(&self) -> ::windows::core::Result<super::super::ApplicationModel::DataTransfer::DataPackageOperation>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDragOperationDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDragStartingEventArgsImpl: Sized {
    fn Cancel(&self) -> ::windows::core::Result<bool>;
    fn SetCancel(&self, value: bool) -> ::windows::core::Result<()>;
    fn Data(&self) -> ::windows::core::Result<super::super::ApplicationModel::DataTransfer::DataPackage>;
    fn DragUI(&self) -> ::windows::core::Result<DragUI>;
    fn GetDeferral(&self) -> ::windows::core::Result<DragOperationDeferral>;
    fn GetPosition(&self, relativeto: &::core::option::Option<UIElement>) -> ::windows::core::Result<super::super::Foundation::Point>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDragStartingEventArgs2Impl: Sized {
    fn AllowedOperations(&self) -> ::windows::core::Result<super::super::ApplicationModel::DataTransfer::DataPackageOperation>;
    fn SetAllowedOperations(&self, value: super::super::ApplicationModel::DataTransfer::DataPackageOperation) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDragUIImpl: Sized {
    fn SetContentFromBitmapImage(&self, bitmapimage: &::core::option::Option<Media::Imaging::BitmapImage>) -> ::windows::core::Result<()>;
    fn SetContentFromBitmapImageWithAnchorPoint(&self, bitmapimage: &::core::option::Option<Media::Imaging::BitmapImage>, anchorpoint: &super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn SetContentFromSoftwareBitmap(&self, softwarebitmap: &::core::option::Option<super::super::Graphics::Imaging::SoftwareBitmap>) -> ::windows::core::Result<()>;
    fn SetContentFromSoftwareBitmapWithAnchorPoint(&self, softwarebitmap: &::core::option::Option<super::super::Graphics::Imaging::SoftwareBitmap>, anchorpoint: &super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn SetContentFromDataPackage(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDragUIOverrideImpl: Sized {
    fn Caption(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCaption(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsContentVisible(&self) -> ::windows::core::Result<bool>;
    fn SetIsContentVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsCaptionVisible(&self) -> ::windows::core::Result<bool>;
    fn SetIsCaptionVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsGlyphVisible(&self) -> ::windows::core::Result<bool>;
    fn SetIsGlyphVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn Clear(&self) -> ::windows::core::Result<()>;
    fn SetContentFromBitmapImage(&self, bitmapimage: &::core::option::Option<Media::Imaging::BitmapImage>) -> ::windows::core::Result<()>;
    fn SetContentFromBitmapImageWithAnchorPoint(&self, bitmapimage: &::core::option::Option<Media::Imaging::BitmapImage>, anchorpoint: &super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn SetContentFromSoftwareBitmap(&self, softwarebitmap: &::core::option::Option<super::super::Graphics::Imaging::SoftwareBitmap>) -> ::windows::core::Result<()>;
    fn SetContentFromSoftwareBitmapWithAnchorPoint(&self, softwarebitmap: &::core::option::Option<super::super::Graphics::Imaging::SoftwareBitmap>, anchorpoint: &super::super::Foundation::Point) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDropCompletedEventArgsImpl: Sized {
    fn DropResult(&self) -> ::windows::core::Result<super::super::ApplicationModel::DataTransfer::DataPackageOperation>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDurationHelperImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IDurationHelperStaticsImpl: Sized {
    fn Automatic(&self) -> ::windows::core::Result<Duration>;
    fn Forever(&self) -> ::windows::core::Result<Duration>;
    fn Compare(&self, duration1: &Duration, duration2: &Duration) -> ::windows::core::Result<i32>;
    fn FromTimeSpan(&self, timespan: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<Duration>;
    fn GetHasTimeSpan(&self, target: &Duration) -> ::windows::core::Result<bool>;
    fn Add(&self, target: &Duration, duration: &Duration) -> ::windows::core::Result<Duration>;
    fn Equals(&self, target: &Duration, value: &Duration) -> ::windows::core::Result<bool>;
    fn Subtract(&self, target: &Duration, duration: &Duration) -> ::windows::core::Result<Duration>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEffectiveViewportChangedEventArgsImpl: Sized {
    fn EffectiveViewport(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn MaxViewport(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn BringIntoViewDistanceX(&self) -> ::windows::core::Result<f64>;
    fn BringIntoViewDistanceY(&self) -> ::windows::core::Result<f64>;
}
pub trait IElementFactoryImpl: Sized {
    fn GetElement(&self, args: &::core::option::Option<ElementFactoryGetArgs>) -> ::windows::core::Result<UIElement>;
    fn RecycleElement(&self, args: &::core::option::Option<ElementFactoryRecycleArgs>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IElementFactoryGetArgsImpl: Sized {
    fn Data(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetData(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn Parent(&self) -> ::windows::core::Result<UIElement>;
    fn SetParent(&self, value: &::core::option::Option<UIElement>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IElementFactoryGetArgsFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ElementFactoryGetArgs>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IElementFactoryRecycleArgsImpl: Sized {
    fn Element(&self) -> ::windows::core::Result<UIElement>;
    fn SetElement(&self, value: &::core::option::Option<UIElement>) -> ::windows::core::Result<()>;
    fn Parent(&self) -> ::windows::core::Result<UIElement>;
    fn SetParent(&self, value: &::core::option::Option<UIElement>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IElementFactoryRecycleArgsFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ElementFactoryRecycleArgs>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IElementSoundPlayerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IElementSoundPlayerStaticsImpl: Sized {
    fn Volume(&self) -> ::windows::core::Result<f64>;
    fn SetVolume(&self, value: f64) -> ::windows::core::Result<()>;
    fn State(&self) -> ::windows::core::Result<ElementSoundPlayerState>;
    fn SetState(&self, value: ElementSoundPlayerState) -> ::windows::core::Result<()>;
    fn Play(&self, sound: ElementSoundKind) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IElementSoundPlayerStatics2Impl: Sized {
    fn SpatialAudioMode(&self) -> ::windows::core::Result<ElementSpatialAudioMode>;
    fn SetSpatialAudioMode(&self, value: ElementSpatialAudioMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEventTriggerImpl: Sized {
    fn RoutedEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn SetRoutedEvent(&self, value: &::core::option::Option<RoutedEvent>) -> ::windows::core::Result<()>;
    fn Actions(&self) -> ::windows::core::Result<TriggerActionCollection>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IExceptionRoutedEventArgsImpl: Sized {
    fn ErrorMessage(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IExceptionRoutedEventArgsFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkElementImpl: Sized {
    fn Triggers(&self) -> ::windows::core::Result<TriggerCollection>;
    fn Resources(&self) -> ::windows::core::Result<ResourceDictionary>;
    fn SetResources(&self, value: &::core::option::Option<ResourceDictionary>) -> ::windows::core::Result<()>;
    fn Tag(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetTag(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLanguage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ActualWidth(&self) -> ::windows::core::Result<f64>;
    fn ActualHeight(&self) -> ::windows::core::Result<f64>;
    fn Width(&self) -> ::windows::core::Result<f64>;
    fn SetWidth(&self, value: f64) -> ::windows::core::Result<()>;
    fn Height(&self) -> ::windows::core::Result<f64>;
    fn SetHeight(&self, value: f64) -> ::windows::core::Result<()>;
    fn MinWidth(&self) -> ::windows::core::Result<f64>;
    fn SetMinWidth(&self, value: f64) -> ::windows::core::Result<()>;
    fn MaxWidth(&self) -> ::windows::core::Result<f64>;
    fn SetMaxWidth(&self, value: f64) -> ::windows::core::Result<()>;
    fn MinHeight(&self) -> ::windows::core::Result<f64>;
    fn SetMinHeight(&self, value: f64) -> ::windows::core::Result<()>;
    fn MaxHeight(&self) -> ::windows::core::Result<f64>;
    fn SetMaxHeight(&self, value: f64) -> ::windows::core::Result<()>;
    fn HorizontalAlignment(&self) -> ::windows::core::Result<HorizontalAlignment>;
    fn SetHorizontalAlignment(&self, value: HorizontalAlignment) -> ::windows::core::Result<()>;
    fn VerticalAlignment(&self) -> ::windows::core::Result<VerticalAlignment>;
    fn SetVerticalAlignment(&self, value: VerticalAlignment) -> ::windows::core::Result<()>;
    fn Margin(&self) -> ::windows::core::Result<Thickness>;
    fn SetMargin(&self, value: &Thickness) -> ::windows::core::Result<()>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn BaseUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn DataContext(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetDataContext(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn Style(&self) -> ::windows::core::Result<Style>;
    fn SetStyle(&self, value: &::core::option::Option<Style>) -> ::windows::core::Result<()>;
    fn Parent(&self) -> ::windows::core::Result<DependencyObject>;
    fn FlowDirection(&self) -> ::windows::core::Result<FlowDirection>;
    fn SetFlowDirection(&self, value: FlowDirection) -> ::windows::core::Result<()>;
    fn Loaded(&self, handler: &::core::option::Option<RoutedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLoaded(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Unloaded(&self, handler: &::core::option::Option<RoutedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUnloaded(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SizeChanged(&self, handler: &::core::option::Option<SizeChangedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSizeChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn LayoutUpdated(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLayoutUpdated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FindName(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetBinding(&self, dp: &::core::option::Option<DependencyProperty>, binding: &::core::option::Option<Data::BindingBase>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkElement2Impl: Sized {
    fn RequestedTheme(&self) -> ::windows::core::Result<ElementTheme>;
    fn SetRequestedTheme(&self, value: ElementTheme) -> ::windows::core::Result<()>;
    fn DataContextChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<FrameworkElement, DataContextChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDataContextChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetBindingExpression(&self, dp: &::core::option::Option<DependencyProperty>) -> ::windows::core::Result<Data::BindingExpression>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkElement3Impl: Sized {
    fn Loading(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<FrameworkElement, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLoading(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkElement4Impl: Sized {
    fn AllowFocusOnInteraction(&self) -> ::windows::core::Result<bool>;
    fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::core::Result<()>;
    fn FocusVisualMargin(&self) -> ::windows::core::Result<Thickness>;
    fn SetFocusVisualMargin(&self, value: &Thickness) -> ::windows::core::Result<()>;
    fn FocusVisualSecondaryThickness(&self) -> ::windows::core::Result<Thickness>;
    fn SetFocusVisualSecondaryThickness(&self, value: &Thickness) -> ::windows::core::Result<()>;
    fn FocusVisualPrimaryThickness(&self) -> ::windows::core::Result<Thickness>;
    fn SetFocusVisualPrimaryThickness(&self, value: &Thickness) -> ::windows::core::Result<()>;
    fn FocusVisualSecondaryBrush(&self) -> ::windows::core::Result<Media::Brush>;
    fn SetFocusVisualSecondaryBrush(&self, value: &::core::option::Option<Media::Brush>) -> ::windows::core::Result<()>;
    fn FocusVisualPrimaryBrush(&self) -> ::windows::core::Result<Media::Brush>;
    fn SetFocusVisualPrimaryBrush(&self, value: &::core::option::Option<Media::Brush>) -> ::windows::core::Result<()>;
    fn AllowFocusWhenDisabled(&self) -> ::windows::core::Result<bool>;
    fn SetAllowFocusWhenDisabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkElement6Impl: Sized {
    fn ActualTheme(&self) -> ::windows::core::Result<ElementTheme>;
    fn ActualThemeChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<FrameworkElement, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveActualThemeChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkElement7Impl: Sized {
    fn IsLoaded(&self) -> ::windows::core::Result<bool>;
    fn EffectiveViewportChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<FrameworkElement, EffectiveViewportChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveEffectiveViewportChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkElementFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<FrameworkElement>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkElementOverridesImpl: Sized {
    fn MeasureOverride(&self, availablesize: &super::super::Foundation::Size) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn ArrangeOverride(&self, finalsize: &super::super::Foundation::Size) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn OnApplyTemplate(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkElementOverrides2Impl: Sized {
    fn GoToElementStateCore(&self, statename: &::windows::core::HSTRING, usetransitions: bool) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkElementProtected7Impl: Sized {
    fn InvalidateViewport(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkElementStaticsImpl: Sized {
    fn TagProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn LanguageProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn ActualWidthProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn ActualHeightProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn WidthProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn HeightProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn MinWidthProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn MaxWidthProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn MinHeightProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn MaxHeightProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn HorizontalAlignmentProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn VerticalAlignmentProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn MarginProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn NameProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn DataContextProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn StyleProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn FlowDirectionProperty(&self) -> ::windows::core::Result<DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkElementStatics2Impl: Sized {
    fn RequestedThemeProperty(&self) -> ::windows::core::Result<DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkElementStatics4Impl: Sized {
    fn AllowFocusOnInteractionProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn FocusVisualMarginProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn FocusVisualSecondaryThicknessProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn FocusVisualPrimaryThicknessProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn FocusVisualSecondaryBrushProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn FocusVisualPrimaryBrushProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn AllowFocusWhenDisabledProperty(&self) -> ::windows::core::Result<DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkElementStatics5Impl: Sized {
    fn DeferTree(&self, element: &::core::option::Option<DependencyObject>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkElementStatics6Impl: Sized {
    fn ActualThemeProperty(&self) -> ::windows::core::Result<DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkTemplateImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkTemplateFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<FrameworkTemplate>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkViewImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkViewSourceImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IGridLengthHelperImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IGridLengthHelperStaticsImpl: Sized {
    fn Auto(&self) -> ::windows::core::Result<GridLength>;
    fn FromPixels(&self, pixels: f64) -> ::windows::core::Result<GridLength>;
    fn FromValueAndType(&self, value: f64, r#type: GridUnitType) -> ::windows::core::Result<GridLength>;
    fn GetIsAbsolute(&self, target: &GridLength) -> ::windows::core::Result<bool>;
    fn GetIsAuto(&self, target: &GridLength) -> ::windows::core::Result<bool>;
    fn GetIsStar(&self, target: &GridLength) -> ::windows::core::Result<bool>;
    fn Equals(&self, target: &GridLength, value: &GridLength) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaFailedRoutedEventArgsImpl: Sized {
    fn ErrorTrace(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointHelperImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IPointHelperStaticsImpl: Sized {
    fn FromCoordinates(&self, x: f32, y: f32) -> ::windows::core::Result<super::super::Foundation::Point>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPropertyMetadataImpl: Sized {
    fn DefaultValue(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateDefaultValueCallback(&self) -> ::windows::core::Result<CreateDefaultValueCallback>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPropertyMetadataFactoryImpl: Sized {
    fn CreateInstanceWithDefaultValue(&self, defaultvalue: &::core::option::Option<::windows::core::IInspectable>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<PropertyMetadata>;
    fn CreateInstanceWithDefaultValueAndCallback(&self, defaultvalue: &::core::option::Option<::windows::core::IInspectable>, propertychangedcallback: &::core::option::Option<PropertyChangedCallback>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<PropertyMetadata>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPropertyMetadataStaticsImpl: Sized {
    fn CreateWithDefaultValue(&self, defaultvalue: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<PropertyMetadata>;
    fn CreateWithDefaultValueAndCallback(&self, defaultvalue: &::core::option::Option<::windows::core::IInspectable>, propertychangedcallback: &::core::option::Option<PropertyChangedCallback>) -> ::windows::core::Result<PropertyMetadata>;
    fn CreateWithFactory(&self, createdefaultvaluecallback: &::core::option::Option<CreateDefaultValueCallback>) -> ::windows::core::Result<PropertyMetadata>;
    fn CreateWithFactoryAndCallback(&self, createdefaultvaluecallback: &::core::option::Option<CreateDefaultValueCallback>, propertychangedcallback: &::core::option::Option<PropertyChangedCallback>) -> ::windows::core::Result<PropertyMetadata>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPropertyPathImpl: Sized {
    fn Path(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPropertyPathFactoryImpl: Sized {
    fn CreateInstance(&self, path: &::windows::core::HSTRING) -> ::windows::core::Result<PropertyPath>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRectHelperImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IRectHelperStaticsImpl: Sized {
    fn Empty(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn FromCoordinatesAndDimensions(&self, x: f32, y: f32, width: f32, height: f32) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn FromPoints(&self, point1: &super::super::Foundation::Point, point2: &super::super::Foundation::Point) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn FromLocationAndSize(&self, location: &super::super::Foundation::Point, size: &super::super::Foundation::Size) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn GetIsEmpty(&self, target: &super::super::Foundation::Rect) -> ::windows::core::Result<bool>;
    fn GetBottom(&self, target: &super::super::Foundation::Rect) -> ::windows::core::Result<f32>;
    fn GetLeft(&self, target: &super::super::Foundation::Rect) -> ::windows::core::Result<f32>;
    fn GetRight(&self, target: &super::super::Foundation::Rect) -> ::windows::core::Result<f32>;
    fn GetTop(&self, target: &super::super::Foundation::Rect) -> ::windows::core::Result<f32>;
    fn Contains(&self, target: &super::super::Foundation::Rect, point: &super::super::Foundation::Point) -> ::windows::core::Result<bool>;
    fn Equals(&self, target: &super::super::Foundation::Rect, value: &super::super::Foundation::Rect) -> ::windows::core::Result<bool>;
    fn Intersect(&self, target: &super::super::Foundation::Rect, rect: &super::super::Foundation::Rect) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn UnionWithPoint(&self, target: &super::super::Foundation::Rect, point: &super::super::Foundation::Point) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn UnionWithRect(&self, target: &super::super::Foundation::Rect, rect: &super::super::Foundation::Rect) -> ::windows::core::Result<super::super::Foundation::Rect>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IResourceDictionaryImpl: Sized {
    fn Source(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetSource(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn MergedDictionaries(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ResourceDictionary>>;
    fn ThemeDictionaries(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::IInspectable, ::windows::core::IInspectable>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IResourceDictionaryFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ResourceDictionary>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRoutedEventImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IRoutedEventArgsImpl: Sized {
    fn OriginalSource(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRoutedEventArgsFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<RoutedEventArgs>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IScalarTransitionImpl: Sized {
    fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetDuration(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IScalarTransitionFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ScalarTransition>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISetterImpl: Sized {
    fn Property(&self) -> ::windows::core::Result<DependencyProperty>;
    fn SetProperty(&self, value: &::core::option::Option<DependencyProperty>) -> ::windows::core::Result<()>;
    fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetValue(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISetter2Impl: Sized {
    fn Target(&self) -> ::windows::core::Result<TargetPropertyPath>;
    fn SetTarget(&self, value: &::core::option::Option<TargetPropertyPath>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISetterBaseImpl: Sized {
    fn IsSealed(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISetterBaseCollectionImpl: Sized {
    fn IsSealed(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISetterBaseFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ISetterFactoryImpl: Sized {
    fn CreateInstance(&self, targetproperty: &::core::option::Option<DependencyProperty>, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<Setter>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISizeChangedEventArgsImpl: Sized {
    fn PreviousSize(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn NewSize(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISizeHelperImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ISizeHelperStaticsImpl: Sized {
    fn Empty(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn FromDimensions(&self, width: f32, height: f32) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn GetIsEmpty(&self, target: &super::super::Foundation::Size) -> ::windows::core::Result<bool>;
    fn Equals(&self, target: &super::super::Foundation::Size, value: &super::super::Foundation::Size) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStateTriggerImpl: Sized {
    fn IsActive(&self) -> ::windows::core::Result<bool>;
    fn SetIsActive(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStateTriggerBaseImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IStateTriggerBaseFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<StateTriggerBase>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStateTriggerBaseProtectedImpl: Sized {
    fn SetActive(&self, isactive: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStateTriggerStaticsImpl: Sized {
    fn IsActiveProperty(&self) -> ::windows::core::Result<DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStyleImpl: Sized {
    fn IsSealed(&self) -> ::windows::core::Result<bool>;
    fn Setters(&self) -> ::windows::core::Result<SetterBaseCollection>;
    fn TargetType(&self) -> ::windows::core::Result<Interop::TypeName>;
    fn SetTargetType(&self, value: &Interop::TypeName) -> ::windows::core::Result<()>;
    fn BasedOn(&self) -> ::windows::core::Result<Style>;
    fn SetBasedOn(&self, value: &::core::option::Option<Style>) -> ::windows::core::Result<()>;
    fn Seal(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStyleFactoryImpl: Sized {
    fn CreateInstance(&self, targettype: &Interop::TypeName) -> ::windows::core::Result<Style>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITargetPropertyPathImpl: Sized {
    fn Path(&self) -> ::windows::core::Result<PropertyPath>;
    fn SetPath(&self, value: &::core::option::Option<PropertyPath>) -> ::windows::core::Result<()>;
    fn Target(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetTarget(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITargetPropertyPathFactoryImpl: Sized {
    fn CreateInstance(&self, targetproperty: &::core::option::Option<DependencyProperty>) -> ::windows::core::Result<TargetPropertyPath>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IThicknessHelperImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IThicknessHelperStaticsImpl: Sized {
    fn FromLengths(&self, left: f64, top: f64, right: f64, bottom: f64) -> ::windows::core::Result<Thickness>;
    fn FromUniformLength(&self, uniformlength: f64) -> ::windows::core::Result<Thickness>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITriggerActionImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ITriggerActionFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ITriggerBaseImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ITriggerBaseFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElementImpl: Sized {
    fn DesiredSize(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn AllowDrop(&self) -> ::windows::core::Result<bool>;
    fn SetAllowDrop(&self, value: bool) -> ::windows::core::Result<()>;
    fn Opacity(&self) -> ::windows::core::Result<f64>;
    fn SetOpacity(&self, value: f64) -> ::windows::core::Result<()>;
    fn Clip(&self) -> ::windows::core::Result<Media::RectangleGeometry>;
    fn SetClip(&self, value: &::core::option::Option<Media::RectangleGeometry>) -> ::windows::core::Result<()>;
    fn RenderTransform(&self) -> ::windows::core::Result<Media::Transform>;
    fn SetRenderTransform(&self, value: &::core::option::Option<Media::Transform>) -> ::windows::core::Result<()>;
    fn Projection(&self) -> ::windows::core::Result<Media::Projection>;
    fn SetProjection(&self, value: &::core::option::Option<Media::Projection>) -> ::windows::core::Result<()>;
    fn RenderTransformOrigin(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn SetRenderTransformOrigin(&self, value: &super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn IsHitTestVisible(&self) -> ::windows::core::Result<bool>;
    fn SetIsHitTestVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn Visibility(&self) -> ::windows::core::Result<Visibility>;
    fn SetVisibility(&self, value: Visibility) -> ::windows::core::Result<()>;
    fn RenderSize(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn UseLayoutRounding(&self) -> ::windows::core::Result<bool>;
    fn SetUseLayoutRounding(&self, value: bool) -> ::windows::core::Result<()>;
    fn Transitions(&self) -> ::windows::core::Result<Media::Animation::TransitionCollection>;
    fn SetTransitions(&self, value: &::core::option::Option<Media::Animation::TransitionCollection>) -> ::windows::core::Result<()>;
    fn CacheMode(&self) -> ::windows::core::Result<Media::CacheMode>;
    fn SetCacheMode(&self, value: &::core::option::Option<Media::CacheMode>) -> ::windows::core::Result<()>;
    fn IsTapEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsTapEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsDoubleTapEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsDoubleTapEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsRightTapEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsRightTapEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsHoldingEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsHoldingEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn ManipulationMode(&self) -> ::windows::core::Result<Input::ManipulationModes>;
    fn SetManipulationMode(&self, value: Input::ManipulationModes) -> ::windows::core::Result<()>;
    fn PointerCaptures(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<Input::Pointer>>;
    fn KeyUp(&self, handler: &::core::option::Option<Input::KeyEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveKeyUp(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn KeyDown(&self, handler: &::core::option::Option<Input::KeyEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveKeyDown(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GotFocus(&self, handler: &::core::option::Option<RoutedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveGotFocus(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn LostFocus(&self, handler: &::core::option::Option<RoutedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLostFocus(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DragEnter(&self, handler: &::core::option::Option<DragEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDragEnter(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DragLeave(&self, handler: &::core::option::Option<DragEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDragLeave(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DragOver(&self, handler: &::core::option::Option<DragEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDragOver(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Drop(&self, handler: &::core::option::Option<DragEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDrop(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerPressed(&self, handler: &::core::option::Option<Input::PointerEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerPressed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerMoved(&self, handler: &::core::option::Option<Input::PointerEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerMoved(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerReleased(&self, handler: &::core::option::Option<Input::PointerEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerReleased(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerEntered(&self, handler: &::core::option::Option<Input::PointerEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerEntered(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerExited(&self, handler: &::core::option::Option<Input::PointerEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerExited(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerCaptureLost(&self, handler: &::core::option::Option<Input::PointerEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerCaptureLost(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerCanceled(&self, handler: &::core::option::Option<Input::PointerEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerCanceled(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerWheelChanged(&self, handler: &::core::option::Option<Input::PointerEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerWheelChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Tapped(&self, handler: &::core::option::Option<Input::TappedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveTapped(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DoubleTapped(&self, handler: &::core::option::Option<Input::DoubleTappedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDoubleTapped(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Holding(&self, handler: &::core::option::Option<Input::HoldingEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveHolding(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RightTapped(&self, handler: &::core::option::Option<Input::RightTappedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRightTapped(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ManipulationStarting(&self, handler: &::core::option::Option<Input::ManipulationStartingEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveManipulationStarting(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ManipulationInertiaStarting(&self, handler: &::core::option::Option<Input::ManipulationInertiaStartingEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveManipulationInertiaStarting(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ManipulationStarted(&self, handler: &::core::option::Option<Input::ManipulationStartedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveManipulationStarted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ManipulationDelta(&self, handler: &::core::option::Option<Input::ManipulationDeltaEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveManipulationDelta(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ManipulationCompleted(&self, handler: &::core::option::Option<Input::ManipulationCompletedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveManipulationCompleted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Measure(&self, availablesize: &super::super::Foundation::Size) -> ::windows::core::Result<()>;
    fn Arrange(&self, finalrect: &super::super::Foundation::Rect) -> ::windows::core::Result<()>;
    fn CapturePointer(&self, value: &::core::option::Option<Input::Pointer>) -> ::windows::core::Result<bool>;
    fn ReleasePointerCapture(&self, value: &::core::option::Option<Input::Pointer>) -> ::windows::core::Result<()>;
    fn ReleasePointerCaptures(&self) -> ::windows::core::Result<()>;
    fn AddHandler(&self, routedevent: &::core::option::Option<RoutedEvent>, handler: &::core::option::Option<::windows::core::IInspectable>, handledeventstoo: bool) -> ::windows::core::Result<()>;
    fn RemoveHandler(&self, routedevent: &::core::option::Option<RoutedEvent>, handler: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn TransformToVisual(&self, visual: &::core::option::Option<UIElement>) -> ::windows::core::Result<Media::GeneralTransform>;
    fn InvalidateMeasure(&self) -> ::windows::core::Result<()>;
    fn InvalidateArrange(&self) -> ::windows::core::Result<()>;
    fn UpdateLayout(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElement10Impl: Sized {
    fn ActualOffset(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn ActualSize(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn XamlRoot(&self) -> ::windows::core::Result<XamlRoot>;
    fn SetXamlRoot(&self, value: &::core::option::Option<XamlRoot>) -> ::windows::core::Result<()>;
    fn UIContext(&self) -> ::windows::core::Result<super::UIContext>;
    fn Shadow(&self) -> ::windows::core::Result<Media::Shadow>;
    fn SetShadow(&self, value: &::core::option::Option<Media::Shadow>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElement2Impl: Sized {
    fn CompositeMode(&self) -> ::windows::core::Result<Media::ElementCompositeMode>;
    fn SetCompositeMode(&self, value: Media::ElementCompositeMode) -> ::windows::core::Result<()>;
    fn CancelDirectManipulations(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElement3Impl: Sized {
    fn Transform3D(&self) -> ::windows::core::Result<Media::Media3D::Transform3D>;
    fn SetTransform3D(&self, value: &::core::option::Option<Media::Media3D::Transform3D>) -> ::windows::core::Result<()>;
    fn CanDrag(&self) -> ::windows::core::Result<bool>;
    fn SetCanDrag(&self, value: bool) -> ::windows::core::Result<()>;
    fn DragStarting(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UIElement, DragStartingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDragStarting(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DropCompleted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UIElement, DropCompletedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDropCompleted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StartDragAsync(&self, pointerpoint: &::core::option::Option<super::Input::PointerPoint>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::ApplicationModel::DataTransfer::DataPackageOperation>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElement4Impl: Sized {
    fn ContextFlyout(&self) -> ::windows::core::Result<Controls::Primitives::FlyoutBase>;
    fn SetContextFlyout(&self, value: &::core::option::Option<Controls::Primitives::FlyoutBase>) -> ::windows::core::Result<()>;
    fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::core::Result<bool>;
    fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsAccessKeyScope(&self) -> ::windows::core::Result<bool>;
    fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::core::Result<()>;
    fn AccessKeyScopeOwner(&self) -> ::windows::core::Result<DependencyObject>;
    fn SetAccessKeyScopeOwner(&self, value: &::core::option::Option<DependencyObject>) -> ::windows::core::Result<()>;
    fn AccessKey(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAccessKey(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ContextRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UIElement, Input::ContextRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveContextRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ContextCanceled(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UIElement, RoutedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveContextCanceled(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AccessKeyDisplayRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UIElement, Input::AccessKeyDisplayRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAccessKeyDisplayRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AccessKeyDisplayDismissed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UIElement, Input::AccessKeyDisplayDismissedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAccessKeyDisplayDismissed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AccessKeyInvoked(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UIElement, Input::AccessKeyInvokedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAccessKeyInvoked(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElement5Impl: Sized {
    fn Lights(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Media::XamlLight>>;
    fn KeyTipPlacementMode(&self) -> ::windows::core::Result<Input::KeyTipPlacementMode>;
    fn SetKeyTipPlacementMode(&self, value: Input::KeyTipPlacementMode) -> ::windows::core::Result<()>;
    fn KeyTipHorizontalOffset(&self) -> ::windows::core::Result<f64>;
    fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()>;
    fn KeyTipVerticalOffset(&self) -> ::windows::core::Result<f64>;
    fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::core::Result<()>;
    fn XYFocusKeyboardNavigation(&self) -> ::windows::core::Result<Input::XYFocusKeyboardNavigationMode>;
    fn SetXYFocusKeyboardNavigation(&self, value: Input::XYFocusKeyboardNavigationMode) -> ::windows::core::Result<()>;
    fn XYFocusUpNavigationStrategy(&self) -> ::windows::core::Result<Input::XYFocusNavigationStrategy>;
    fn SetXYFocusUpNavigationStrategy(&self, value: Input::XYFocusNavigationStrategy) -> ::windows::core::Result<()>;
    fn XYFocusDownNavigationStrategy(&self) -> ::windows::core::Result<Input::XYFocusNavigationStrategy>;
    fn SetXYFocusDownNavigationStrategy(&self, value: Input::XYFocusNavigationStrategy) -> ::windows::core::Result<()>;
    fn XYFocusLeftNavigationStrategy(&self) -> ::windows::core::Result<Input::XYFocusNavigationStrategy>;
    fn SetXYFocusLeftNavigationStrategy(&self, value: Input::XYFocusNavigationStrategy) -> ::windows::core::Result<()>;
    fn XYFocusRightNavigationStrategy(&self) -> ::windows::core::Result<Input::XYFocusNavigationStrategy>;
    fn SetXYFocusRightNavigationStrategy(&self, value: Input::XYFocusNavigationStrategy) -> ::windows::core::Result<()>;
    fn HighContrastAdjustment(&self) -> ::windows::core::Result<ElementHighContrastAdjustment>;
    fn SetHighContrastAdjustment(&self, value: ElementHighContrastAdjustment) -> ::windows::core::Result<()>;
    fn TabFocusNavigation(&self) -> ::windows::core::Result<Input::KeyboardNavigationMode>;
    fn SetTabFocusNavigation(&self, value: Input::KeyboardNavigationMode) -> ::windows::core::Result<()>;
    fn GettingFocus(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UIElement, Input::GettingFocusEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveGettingFocus(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn LosingFocus(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UIElement, Input::LosingFocusEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLosingFocus(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NoFocusCandidateFound(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UIElement, Input::NoFocusCandidateFoundEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveNoFocusCandidateFound(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StartBringIntoView(&self) -> ::windows::core::Result<()>;
    fn StartBringIntoViewWithOptions(&self, options: &::core::option::Option<BringIntoViewOptions>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElement7Impl: Sized {
    fn KeyboardAccelerators(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Input::KeyboardAccelerator>>;
    fn CharacterReceived(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UIElement, Input::CharacterReceivedRoutedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCharacterReceived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ProcessKeyboardAccelerators(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UIElement, Input::ProcessKeyboardAcceleratorEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveProcessKeyboardAccelerators(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PreviewKeyDown(&self, handler: &::core::option::Option<Input::KeyEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePreviewKeyDown(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PreviewKeyUp(&self, handler: &::core::option::Option<Input::KeyEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePreviewKeyUp(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TryInvokeKeyboardAccelerator(&self, args: &::core::option::Option<Input::ProcessKeyboardAcceleratorEventArgs>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElement8Impl: Sized {
    fn KeyTipTarget(&self) -> ::windows::core::Result<DependencyObject>;
    fn SetKeyTipTarget(&self, value: &::core::option::Option<DependencyObject>) -> ::windows::core::Result<()>;
    fn KeyboardAcceleratorPlacementTarget(&self) -> ::windows::core::Result<DependencyObject>;
    fn SetKeyboardAcceleratorPlacementTarget(&self, value: &::core::option::Option<DependencyObject>) -> ::windows::core::Result<()>;
    fn KeyboardAcceleratorPlacementMode(&self) -> ::windows::core::Result<Input::KeyboardAcceleratorPlacementMode>;
    fn SetKeyboardAcceleratorPlacementMode(&self, value: Input::KeyboardAcceleratorPlacementMode) -> ::windows::core::Result<()>;
    fn BringIntoViewRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UIElement, BringIntoViewRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBringIntoViewRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElement9Impl: Sized {
    fn CanBeScrollAnchor(&self) -> ::windows::core::Result<bool>;
    fn SetCanBeScrollAnchor(&self, value: bool) -> ::windows::core::Result<()>;
    fn OpacityTransition(&self) -> ::windows::core::Result<ScalarTransition>;
    fn SetOpacityTransition(&self, value: &::core::option::Option<ScalarTransition>) -> ::windows::core::Result<()>;
    fn Translation(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn SetTranslation(&self, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn TranslationTransition(&self) -> ::windows::core::Result<Vector3Transition>;
    fn SetTranslationTransition(&self, value: &::core::option::Option<Vector3Transition>) -> ::windows::core::Result<()>;
    fn Rotation(&self) -> ::windows::core::Result<f32>;
    fn SetRotation(&self, value: f32) -> ::windows::core::Result<()>;
    fn RotationTransition(&self) -> ::windows::core::Result<ScalarTransition>;
    fn SetRotationTransition(&self, value: &::core::option::Option<ScalarTransition>) -> ::windows::core::Result<()>;
    fn Scale(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn SetScale(&self, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn ScaleTransition(&self) -> ::windows::core::Result<Vector3Transition>;
    fn SetScaleTransition(&self, value: &::core::option::Option<Vector3Transition>) -> ::windows::core::Result<()>;
    fn TransformMatrix(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Matrix4x4>;
    fn SetTransformMatrix(&self, value: &super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::Result<()>;
    fn CenterPoint(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn SetCenterPoint(&self, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn RotationAxis(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn SetRotationAxis(&self, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn StartAnimation(&self, animation: &::core::option::Option<super::Composition::ICompositionAnimationBase>) -> ::windows::core::Result<()>;
    fn StopAnimation(&self, animation: &::core::option::Option<super::Composition::ICompositionAnimationBase>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElementFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElementOverridesImpl: Sized {
    fn OnCreateAutomationPeer(&self) -> ::windows::core::Result<Automation::Peers::AutomationPeer>;
    fn OnDisconnectVisualChildren(&self) -> ::windows::core::Result<()>;
    fn FindSubElementsForTouchTargeting(&self, point: &super::super::Foundation::Point, boundingrect: &super::super::Foundation::Rect) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IIterable<super::super::Foundation::Point>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElementOverrides7Impl: Sized {
    fn GetChildrenInTabFocusOrder(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<DependencyObject>>;
    fn OnProcessKeyboardAccelerators(&self, args: &::core::option::Option<Input::ProcessKeyboardAcceleratorEventArgs>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElementOverrides8Impl: Sized {
    fn OnKeyboardAcceleratorInvoked(&self, args: &::core::option::Option<Input::KeyboardAcceleratorInvokedEventArgs>) -> ::windows::core::Result<()>;
    fn OnBringIntoViewRequested(&self, e: &::core::option::Option<BringIntoViewRequestedEventArgs>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElementOverrides9Impl: Sized {
    fn PopulatePropertyInfoOverride(&self, propertyname: &::windows::core::HSTRING, animationpropertyinfo: &::core::option::Option<super::Composition::AnimationPropertyInfo>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElementStaticsImpl: Sized {
    fn KeyDownEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn KeyUpEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn PointerEnteredEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn PointerPressedEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn PointerMovedEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn PointerReleasedEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn PointerExitedEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn PointerCaptureLostEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn PointerCanceledEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn PointerWheelChangedEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn TappedEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn DoubleTappedEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn HoldingEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn RightTappedEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn ManipulationStartingEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn ManipulationInertiaStartingEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn ManipulationStartedEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn ManipulationDeltaEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn ManipulationCompletedEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn DragEnterEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn DragLeaveEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn DragOverEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn DropEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn AllowDropProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn OpacityProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn ClipProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn RenderTransformProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn ProjectionProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn RenderTransformOriginProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn IsHitTestVisibleProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn VisibilityProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn UseLayoutRoundingProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn TransitionsProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn CacheModeProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn IsTapEnabledProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn IsDoubleTapEnabledProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn IsRightTapEnabledProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn IsHoldingEnabledProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn ManipulationModeProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn PointerCapturesProperty(&self) -> ::windows::core::Result<DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElementStatics10Impl: Sized {
    fn ShadowProperty(&self) -> ::windows::core::Result<DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElementStatics2Impl: Sized {
    fn CompositeModeProperty(&self) -> ::windows::core::Result<DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElementStatics3Impl: Sized {
    fn Transform3DProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn CanDragProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn TryStartDirectManipulation(&self, value: &::core::option::Option<Input::Pointer>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElementStatics4Impl: Sized {
    fn ContextFlyoutProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn ExitDisplayModeOnAccessKeyInvokedProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn IsAccessKeyScopeProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn AccessKeyScopeOwnerProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn AccessKeyProperty(&self) -> ::windows::core::Result<DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElementStatics5Impl: Sized {
    fn LightsProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn KeyTipPlacementModeProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn KeyTipHorizontalOffsetProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn KeyTipVerticalOffsetProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn XYFocusKeyboardNavigationProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn XYFocusUpNavigationStrategyProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn XYFocusDownNavigationStrategyProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn XYFocusLeftNavigationStrategyProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn XYFocusRightNavigationStrategyProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn HighContrastAdjustmentProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn TabFocusNavigationProperty(&self) -> ::windows::core::Result<DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElementStatics6Impl: Sized {
    fn GettingFocusEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn LosingFocusEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn NoFocusCandidateFoundEvent(&self) -> ::windows::core::Result<RoutedEvent>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElementStatics7Impl: Sized {
    fn PreviewKeyDownEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn CharacterReceivedEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn PreviewKeyUpEvent(&self) -> ::windows::core::Result<RoutedEvent>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElementStatics8Impl: Sized {
    fn BringIntoViewRequestedEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn ContextRequestedEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn KeyTipTargetProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn KeyboardAcceleratorPlacementTargetProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn KeyboardAcceleratorPlacementModeProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn RegisterAsScrollPort(&self, element: &::core::option::Option<UIElement>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElementStatics9Impl: Sized {
    fn CanBeScrollAnchorProperty(&self) -> ::windows::core::Result<DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElementWeakCollectionImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElementWeakCollectionFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<UIElementWeakCollection>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUnhandledExceptionEventArgsImpl: Sized {
    fn Exception(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn Message(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVector3TransitionImpl: Sized {
    fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetDuration(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn Components(&self) -> ::windows::core::Result<Vector3TransitionComponents>;
    fn SetComponents(&self, value: Vector3TransitionComponents) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVector3TransitionFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<Vector3Transition>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisualStateImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Storyboard(&self) -> ::windows::core::Result<Media::Animation::Storyboard>;
    fn SetStoryboard(&self, value: &::core::option::Option<Media::Animation::Storyboard>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisualState2Impl: Sized {
    fn Setters(&self) -> ::windows::core::Result<SetterBaseCollection>;
    fn StateTriggers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<StateTriggerBase>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisualStateChangedEventArgsImpl: Sized {
    fn OldState(&self) -> ::windows::core::Result<VisualState>;
    fn SetOldState(&self, value: &::core::option::Option<VisualState>) -> ::windows::core::Result<()>;
    fn NewState(&self) -> ::windows::core::Result<VisualState>;
    fn SetNewState(&self, value: &::core::option::Option<VisualState>) -> ::windows::core::Result<()>;
    fn Control(&self) -> ::windows::core::Result<Controls::Control>;
    fn SetControl(&self, value: &::core::option::Option<Controls::Control>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisualStateGroupImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Transitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VisualTransition>>;
    fn States(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VisualState>>;
    fn CurrentState(&self) -> ::windows::core::Result<VisualState>;
    fn CurrentStateChanged(&self, handler: &::core::option::Option<VisualStateChangedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCurrentStateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CurrentStateChanging(&self, handler: &::core::option::Option<VisualStateChangedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCurrentStateChanging(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisualStateManagerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IVisualStateManagerFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<VisualStateManager>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisualStateManagerOverridesImpl: Sized {
    fn GoToStateCore(&self, control: &::core::option::Option<Controls::Control>, templateroot: &::core::option::Option<FrameworkElement>, statename: &::windows::core::HSTRING, group: &::core::option::Option<VisualStateGroup>, state: &::core::option::Option<VisualState>, usetransitions: bool) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisualStateManagerProtectedImpl: Sized {
    fn RaiseCurrentStateChanging(&self, stategroup: &::core::option::Option<VisualStateGroup>, oldstate: &::core::option::Option<VisualState>, newstate: &::core::option::Option<VisualState>, control: &::core::option::Option<Controls::Control>) -> ::windows::core::Result<()>;
    fn RaiseCurrentStateChanged(&self, stategroup: &::core::option::Option<VisualStateGroup>, oldstate: &::core::option::Option<VisualState>, newstate: &::core::option::Option<VisualState>, control: &::core::option::Option<Controls::Control>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisualStateManagerStaticsImpl: Sized {
    fn GetVisualStateGroups(&self, obj: &::core::option::Option<FrameworkElement>) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VisualStateGroup>>;
    fn CustomVisualStateManagerProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn GetCustomVisualStateManager(&self, obj: &::core::option::Option<FrameworkElement>) -> ::windows::core::Result<VisualStateManager>;
    fn SetCustomVisualStateManager(&self, obj: &::core::option::Option<FrameworkElement>, value: &::core::option::Option<VisualStateManager>) -> ::windows::core::Result<()>;
    fn GoToState(&self, control: &::core::option::Option<Controls::Control>, statename: &::windows::core::HSTRING, usetransitions: bool) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisualTransitionImpl: Sized {
    fn GeneratedDuration(&self) -> ::windows::core::Result<Duration>;
    fn SetGeneratedDuration(&self, value: &Duration) -> ::windows::core::Result<()>;
    fn GeneratedEasingFunction(&self) -> ::windows::core::Result<Media::Animation::EasingFunctionBase>;
    fn SetGeneratedEasingFunction(&self, value: &::core::option::Option<Media::Animation::EasingFunctionBase>) -> ::windows::core::Result<()>;
    fn To(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTo(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn From(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetFrom(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Storyboard(&self) -> ::windows::core::Result<Media::Animation::Storyboard>;
    fn SetStoryboard(&self, value: &::core::option::Option<Media::Animation::Storyboard>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisualTransitionFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<VisualTransition>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWindowImpl: Sized {
    fn Bounds(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn Visible(&self) -> ::windows::core::Result<bool>;
    fn Content(&self) -> ::windows::core::Result<UIElement>;
    fn SetContent(&self, value: &::core::option::Option<UIElement>) -> ::windows::core::Result<()>;
    fn CoreWindow(&self) -> ::windows::core::Result<super::Core::CoreWindow>;
    fn Dispatcher(&self) -> ::windows::core::Result<super::Core::CoreDispatcher>;
    fn Activated(&self, handler: &::core::option::Option<WindowActivatedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveActivated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Closed(&self, handler: &::core::option::Option<WindowClosedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SizeChanged(&self, handler: &::core::option::Option<WindowSizeChangedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSizeChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn VisibilityChanged(&self, handler: &::core::option::Option<WindowVisibilityChangedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveVisibilityChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Activate(&self) -> ::windows::core::Result<()>;
    fn Close(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWindow2Impl: Sized {
    fn SetTitleBar(&self, value: &::core::option::Option<UIElement>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWindow3Impl: Sized {
    fn Compositor(&self) -> ::windows::core::Result<super::Composition::Compositor>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWindow4Impl: Sized {
    fn UIContext(&self) -> ::windows::core::Result<super::UIContext>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWindowCreatedEventArgsImpl: Sized {
    fn Window(&self) -> ::windows::core::Result<Window>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWindowStaticsImpl: Sized {
    fn Current(&self) -> ::windows::core::Result<Window>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlRootImpl: Sized {
    fn Content(&self) -> ::windows::core::Result<UIElement>;
    fn Size(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn RasterizationScale(&self) -> ::windows::core::Result<f64>;
    fn IsHostVisible(&self) -> ::windows::core::Result<bool>;
    fn UIContext(&self) -> ::windows::core::Result<super::UIContext>;
    fn Changed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<XamlRoot, XamlRootChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlRootChangedEventArgsImpl: Sized {}
