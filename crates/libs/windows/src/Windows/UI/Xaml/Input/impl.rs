#[cfg(feature = "implement_exclusive")]
pub trait IAccessKeyDisplayDismissedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IAccessKeyDisplayRequestedEventArgsImpl: Sized {
    fn PressedKeys(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccessKeyInvokedEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccessKeyManagerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IAccessKeyManagerStaticsImpl: Sized {
    fn IsDisplayModeEnabled(&self) -> ::windows::core::Result<bool>;
    fn IsDisplayModeEnabledChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveIsDisplayModeEnabledChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ExitDisplayMode(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccessKeyManagerStatics2Impl: Sized {
    fn AreKeyTipsEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetAreKeyTipsEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICanExecuteRequestedEventArgsImpl: Sized {
    fn Parameter(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CanExecute(&self) -> ::windows::core::Result<bool>;
    fn SetCanExecute(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICharacterReceivedRoutedEventArgsImpl: Sized {
    fn Character(&self) -> ::windows::core::Result<u16>;
    fn KeyStatus(&self) -> ::windows::core::Result<super::super::Core::CorePhysicalKeyStatus>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
}
pub trait ICommandImpl: Sized {
    fn CanExecuteChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCanExecuteChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CanExecute(&self, parameter: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<bool>;
    fn Execute(&self, parameter: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContextRequestedEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn TryGetPosition(&self, relativeto: &::core::option::Option<super::UIElement>, point: &mut super::super::super::Foundation::Point) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDoubleTappedRoutedEventArgsImpl: Sized {
    fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::super::Devices::Input::PointerDeviceType>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn GetPosition(&self, relativeto: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<super::super::super::Foundation::Point>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IExecuteRequestedEventArgsImpl: Sized {
    fn Parameter(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFindNextElementOptionsImpl: Sized {
    fn SearchRoot(&self) -> ::windows::core::Result<super::DependencyObject>;
    fn SetSearchRoot(&self, value: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<()>;
    fn ExclusionRect(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
    fn SetExclusionRect(&self, value: &super::super::super::Foundation::Rect) -> ::windows::core::Result<()>;
    fn HintRect(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
    fn SetHintRect(&self, value: &super::super::super::Foundation::Rect) -> ::windows::core::Result<()>;
    fn XYFocusNavigationStrategyOverride(&self) -> ::windows::core::Result<XYFocusNavigationStrategyOverride>;
    fn SetXYFocusNavigationStrategyOverride(&self, value: XYFocusNavigationStrategyOverride) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFocusManagerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IFocusManagerGotFocusEventArgsImpl: Sized {
    fn NewFocusedElement(&self) -> ::windows::core::Result<super::DependencyObject>;
    fn CorrelationId(&self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFocusManagerLostFocusEventArgsImpl: Sized {
    fn OldFocusedElement(&self) -> ::windows::core::Result<super::DependencyObject>;
    fn CorrelationId(&self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFocusManagerStaticsImpl: Sized {
    fn GetFocusedElement(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFocusManagerStatics2Impl: Sized {
    fn TryMoveFocus(&self, focusnavigationdirection: FocusNavigationDirection) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFocusManagerStatics3Impl: Sized {
    fn FindNextFocusableElement(&self, focusnavigationdirection: FocusNavigationDirection) -> ::windows::core::Result<super::UIElement>;
    fn FindNextFocusableElementWithHint(&self, focusnavigationdirection: FocusNavigationDirection, hintrect: &super::super::super::Foundation::Rect) -> ::windows::core::Result<super::UIElement>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFocusManagerStatics4Impl: Sized {
    fn TryMoveFocusWithOptions(&self, focusnavigationdirection: FocusNavigationDirection, focusnavigationoptions: &::core::option::Option<FindNextElementOptions>) -> ::windows::core::Result<bool>;
    fn FindNextElement(&self, focusnavigationdirection: FocusNavigationDirection) -> ::windows::core::Result<super::DependencyObject>;
    fn FindFirstFocusableElement(&self, searchscope: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<super::DependencyObject>;
    fn FindLastFocusableElement(&self, searchscope: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<super::DependencyObject>;
    fn FindNextElementWithOptions(&self, focusnavigationdirection: FocusNavigationDirection, focusnavigationoptions: &::core::option::Option<FindNextElementOptions>) -> ::windows::core::Result<super::DependencyObject>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFocusManagerStatics5Impl: Sized {
    fn TryFocusAsync(&self, element: &::core::option::Option<super::DependencyObject>, value: super::FocusState) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<FocusMovementResult>>;
    fn TryMoveFocusAsync(&self, focusnavigationdirection: FocusNavigationDirection) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<FocusMovementResult>>;
    fn TryMoveFocusWithOptionsAsync(&self, focusnavigationdirection: FocusNavigationDirection, focusnavigationoptions: &::core::option::Option<FindNextElementOptions>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<FocusMovementResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFocusManagerStatics6Impl: Sized {
    fn GotFocus(&self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<FocusManagerGotFocusEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveGotFocus(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn LostFocus(&self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<FocusManagerLostFocusEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveLostFocus(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GettingFocus(&self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<GettingFocusEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveGettingFocus(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn LosingFocus(&self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<LosingFocusEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveLosingFocus(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFocusManagerStatics7Impl: Sized {
    fn GetFocusedElement(&self, xamlroot: &::core::option::Option<super::XamlRoot>) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFocusMovementResultImpl: Sized {
    fn Succeeded(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGettingFocusEventArgsImpl: Sized {
    fn OldFocusedElement(&self) -> ::windows::core::Result<super::DependencyObject>;
    fn NewFocusedElement(&self) -> ::windows::core::Result<super::DependencyObject>;
    fn SetNewFocusedElement(&self, value: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<()>;
    fn FocusState(&self) -> ::windows::core::Result<super::FocusState>;
    fn Direction(&self) -> ::windows::core::Result<FocusNavigationDirection>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn InputDevice(&self) -> ::windows::core::Result<FocusInputDeviceKind>;
    fn Cancel(&self) -> ::windows::core::Result<bool>;
    fn SetCancel(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGettingFocusEventArgs2Impl: Sized {
    fn TryCancel(&self) -> ::windows::core::Result<bool>;
    fn TrySetNewFocusedElement(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGettingFocusEventArgs3Impl: Sized {
    fn CorrelationId(&self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHoldingRoutedEventArgsImpl: Sized {
    fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::super::Devices::Input::PointerDeviceType>;
    fn HoldingState(&self) -> ::windows::core::Result<super::super::Input::HoldingState>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn GetPosition(&self, relativeto: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<super::super::super::Foundation::Point>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInertiaExpansionBehaviorImpl: Sized {
    fn DesiredDeceleration(&self) -> ::windows::core::Result<f64>;
    fn SetDesiredDeceleration(&self, value: f64) -> ::windows::core::Result<()>;
    fn DesiredExpansion(&self) -> ::windows::core::Result<f64>;
    fn SetDesiredExpansion(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInertiaRotationBehaviorImpl: Sized {
    fn DesiredDeceleration(&self) -> ::windows::core::Result<f64>;
    fn SetDesiredDeceleration(&self, value: f64) -> ::windows::core::Result<()>;
    fn DesiredRotation(&self) -> ::windows::core::Result<f64>;
    fn SetDesiredRotation(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInertiaTranslationBehaviorImpl: Sized {
    fn DesiredDeceleration(&self) -> ::windows::core::Result<f64>;
    fn SetDesiredDeceleration(&self, value: f64) -> ::windows::core::Result<()>;
    fn DesiredDisplacement(&self) -> ::windows::core::Result<f64>;
    fn SetDesiredDisplacement(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInputScopeImpl: Sized {
    fn Names(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<InputScopeName>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInputScopeNameImpl: Sized {
    fn NameValue(&self) -> ::windows::core::Result<InputScopeNameValue>;
    fn SetNameValue(&self, value: InputScopeNameValue) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInputScopeNameFactoryImpl: Sized {
    fn CreateInstance(&self, namevalue: InputScopeNameValue) -> ::windows::core::Result<InputScopeName>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyRoutedEventArgsImpl: Sized {
    fn Key(&self) -> ::windows::core::Result<super::super::super::System::VirtualKey>;
    fn KeyStatus(&self) -> ::windows::core::Result<super::super::Core::CorePhysicalKeyStatus>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyRoutedEventArgs2Impl: Sized {
    fn OriginalKey(&self) -> ::windows::core::Result<super::super::super::System::VirtualKey>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyRoutedEventArgs3Impl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyboardAcceleratorImpl: Sized {
    fn Key(&self) -> ::windows::core::Result<super::super::super::System::VirtualKey>;
    fn SetKey(&self, value: super::super::super::System::VirtualKey) -> ::windows::core::Result<()>;
    fn Modifiers(&self) -> ::windows::core::Result<super::super::super::System::VirtualKeyModifiers>;
    fn SetModifiers(&self, value: super::super::super::System::VirtualKeyModifiers) -> ::windows::core::Result<()>;
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn ScopeOwner(&self) -> ::windows::core::Result<super::DependencyObject>;
    fn SetScopeOwner(&self, value: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<()>;
    fn Invoked(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<KeyboardAccelerator, KeyboardAcceleratorInvokedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveInvoked(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyboardAcceleratorFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<KeyboardAccelerator>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyboardAcceleratorInvokedEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn Element(&self) -> ::windows::core::Result<super::DependencyObject>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyboardAcceleratorInvokedEventArgs2Impl: Sized {
    fn KeyboardAccelerator(&self) -> ::windows::core::Result<KeyboardAccelerator>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyboardAcceleratorStaticsImpl: Sized {
    fn KeyProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ModifiersProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ScopeOwnerProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILosingFocusEventArgsImpl: Sized {
    fn OldFocusedElement(&self) -> ::windows::core::Result<super::DependencyObject>;
    fn NewFocusedElement(&self) -> ::windows::core::Result<super::DependencyObject>;
    fn SetNewFocusedElement(&self, value: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<()>;
    fn FocusState(&self) -> ::windows::core::Result<super::FocusState>;
    fn Direction(&self) -> ::windows::core::Result<FocusNavigationDirection>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn InputDevice(&self) -> ::windows::core::Result<FocusInputDeviceKind>;
    fn Cancel(&self) -> ::windows::core::Result<bool>;
    fn SetCancel(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILosingFocusEventArgs2Impl: Sized {
    fn TryCancel(&self) -> ::windows::core::Result<bool>;
    fn TrySetNewFocusedElement(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILosingFocusEventArgs3Impl: Sized {
    fn CorrelationId(&self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IManipulationCompletedRoutedEventArgsImpl: Sized {
    fn Container(&self) -> ::windows::core::Result<super::UIElement>;
    fn Position(&self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn IsInertial(&self) -> ::windows::core::Result<bool>;
    fn Cumulative(&self) -> ::windows::core::Result<super::super::Input::ManipulationDelta>;
    fn Velocities(&self) -> ::windows::core::Result<super::super::Input::ManipulationVelocities>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::super::Devices::Input::PointerDeviceType>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IManipulationDeltaRoutedEventArgsImpl: Sized {
    fn Container(&self) -> ::windows::core::Result<super::UIElement>;
    fn Position(&self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn IsInertial(&self) -> ::windows::core::Result<bool>;
    fn Delta(&self) -> ::windows::core::Result<super::super::Input::ManipulationDelta>;
    fn Cumulative(&self) -> ::windows::core::Result<super::super::Input::ManipulationDelta>;
    fn Velocities(&self) -> ::windows::core::Result<super::super::Input::ManipulationVelocities>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::super::Devices::Input::PointerDeviceType>;
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IManipulationInertiaStartingRoutedEventArgsImpl: Sized {
    fn Container(&self) -> ::windows::core::Result<super::UIElement>;
    fn ExpansionBehavior(&self) -> ::windows::core::Result<InertiaExpansionBehavior>;
    fn SetExpansionBehavior(&self, value: &::core::option::Option<InertiaExpansionBehavior>) -> ::windows::core::Result<()>;
    fn RotationBehavior(&self) -> ::windows::core::Result<InertiaRotationBehavior>;
    fn SetRotationBehavior(&self, value: &::core::option::Option<InertiaRotationBehavior>) -> ::windows::core::Result<()>;
    fn TranslationBehavior(&self) -> ::windows::core::Result<InertiaTranslationBehavior>;
    fn SetTranslationBehavior(&self, value: &::core::option::Option<InertiaTranslationBehavior>) -> ::windows::core::Result<()>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::super::Devices::Input::PointerDeviceType>;
    fn Delta(&self) -> ::windows::core::Result<super::super::Input::ManipulationDelta>;
    fn Cumulative(&self) -> ::windows::core::Result<super::super::Input::ManipulationDelta>;
    fn Velocities(&self) -> ::windows::core::Result<super::super::Input::ManipulationVelocities>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IManipulationPivotImpl: Sized {
    fn Center(&self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn SetCenter(&self, value: &super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn Radius(&self) -> ::windows::core::Result<f64>;
    fn SetRadius(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IManipulationPivotFactoryImpl: Sized {
    fn CreateInstanceWithCenterAndRadius(&self, center: &super::super::super::Foundation::Point, radius: f64) -> ::windows::core::Result<ManipulationPivot>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IManipulationStartedRoutedEventArgsImpl: Sized {
    fn Container(&self) -> ::windows::core::Result<super::UIElement>;
    fn Position(&self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::super::Devices::Input::PointerDeviceType>;
    fn Cumulative(&self) -> ::windows::core::Result<super::super::Input::ManipulationDelta>;
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IManipulationStartedRoutedEventArgsFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ManipulationStartedRoutedEventArgs>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IManipulationStartingRoutedEventArgsImpl: Sized {
    fn Mode(&self) -> ::windows::core::Result<ManipulationModes>;
    fn SetMode(&self, value: ManipulationModes) -> ::windows::core::Result<()>;
    fn Container(&self) -> ::windows::core::Result<super::UIElement>;
    fn SetContainer(&self, value: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn Pivot(&self) -> ::windows::core::Result<ManipulationPivot>;
    fn SetPivot(&self, value: &::core::option::Option<ManipulationPivot>) -> ::windows::core::Result<()>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INoFocusCandidateFoundEventArgsImpl: Sized {
    fn Direction(&self) -> ::windows::core::Result<FocusNavigationDirection>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn InputDevice(&self) -> ::windows::core::Result<FocusInputDeviceKind>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointerImpl: Sized {
    fn PointerId(&self) -> ::windows::core::Result<u32>;
    fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::super::Devices::Input::PointerDeviceType>;
    fn IsInContact(&self) -> ::windows::core::Result<bool>;
    fn IsInRange(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointerRoutedEventArgsImpl: Sized {
    fn Pointer(&self) -> ::windows::core::Result<Pointer>;
    fn KeyModifiers(&self) -> ::windows::core::Result<super::super::super::System::VirtualKeyModifiers>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn GetCurrentPoint(&self, relativeto: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<super::super::Input::PointerPoint>;
    fn GetIntermediatePoints(&self, relativeto: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<super::super::Input::PointerPoint>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointerRoutedEventArgs2Impl: Sized {
    fn IsGenerated(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProcessKeyboardAcceleratorEventArgsImpl: Sized {
    fn Key(&self) -> ::windows::core::Result<super::super::super::System::VirtualKey>;
    fn Modifiers(&self) -> ::windows::core::Result<super::super::super::System::VirtualKeyModifiers>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRightTappedRoutedEventArgsImpl: Sized {
    fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::super::Devices::Input::PointerDeviceType>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn GetPosition(&self, relativeto: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<super::super::super::Foundation::Point>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStandardUICommandImpl: Sized {
    fn Kind(&self) -> ::windows::core::Result<StandardUICommandKind>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStandardUICommand2Impl: Sized {
    fn SetKind(&self, value: StandardUICommandKind) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStandardUICommandFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<StandardUICommand>;
    fn CreateInstanceWithKind(&self, kind: StandardUICommandKind, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<StandardUICommand>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStandardUICommandStaticsImpl: Sized {
    fn KindProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITappedRoutedEventArgsImpl: Sized {
    fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::super::Devices::Input::PointerDeviceType>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn GetPosition(&self, relativeto: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<super::super::super::Foundation::Point>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlUICommandImpl: Sized {
    fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLabel(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IconSource(&self) -> ::windows::core::Result<super::Controls::IconSource>;
    fn SetIconSource(&self, value: &::core::option::Option<super::Controls::IconSource>) -> ::windows::core::Result<()>;
    fn KeyboardAccelerators(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<KeyboardAccelerator>>;
    fn AccessKey(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAccessKey(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Command(&self) -> ::windows::core::Result<ICommand>;
    fn SetCommand(&self, value: &::core::option::Option<ICommand>) -> ::windows::core::Result<()>;
    fn ExecuteRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<XamlUICommand, ExecuteRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveExecuteRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CanExecuteRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<XamlUICommand, CanExecuteRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCanExecuteRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NotifyCanExecuteChanged(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlUICommandFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<XamlUICommand>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlUICommandStaticsImpl: Sized {
    fn LabelProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IconSourceProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn KeyboardAcceleratorsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn AccessKeyProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn DescriptionProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CommandProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
