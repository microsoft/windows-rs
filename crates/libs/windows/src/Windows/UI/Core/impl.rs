#[cfg(feature = "implement_exclusive")]
pub trait IAcceleratorKeyEventArgsImpl: Sized + ICoreWindowEventArgsImpl {
    fn EventType(&self) -> ::windows::core::Result<CoreAcceleratorKeyEventType>;
    fn VirtualKey(&self) -> ::windows::core::Result<super::super::System::VirtualKey>;
    fn KeyStatus(&self) -> ::windows::core::Result<CorePhysicalKeyStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAcceleratorKeyEventArgs2Impl: Sized + ICoreWindowEventArgsImpl {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationProviderRequestedEventArgsImpl: Sized + ICoreWindowEventArgsImpl {
    fn AutomationProvider(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetAutomationProvider(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackRequestedEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICharacterReceivedEventArgsImpl: Sized + ICoreWindowEventArgsImpl {
    fn KeyCode(&self) -> ::windows::core::Result<u32>;
    fn KeyStatus(&self) -> ::windows::core::Result<CorePhysicalKeyStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IClosestInteractiveBoundsRequestedEventArgsImpl: Sized {
    fn PointerPosition(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn SearchBounds(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn ClosestInteractiveBounds(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn SetClosestInteractiveBounds(&self, value: &super::super::Foundation::Rect) -> ::windows::core::Result<()>;
}
pub trait ICoreAcceleratorKeysImpl: Sized {
    fn AcceleratorKeyActivated(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreDispatcher, AcceleratorKeyEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAcceleratorKeyActivated(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreClosestInteractiveBoundsRequestedImpl: Sized {
    fn ClosestInteractiveBoundsRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreComponentInputSource, ClosestInteractiveBoundsRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosestInteractiveBoundsRequested(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreComponentFocusableImpl: Sized {
    fn HasFocus(&self) -> ::windows::core::Result<bool>;
    fn GotFocus(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, CoreWindowEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveGotFocus(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn LostFocus(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, CoreWindowEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLostFocus(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreCursorImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<u32>;
    fn Type(&self) -> ::windows::core::Result<CoreCursorType>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreCursorFactoryImpl: Sized {
    fn CreateCursor(&self, r#type: CoreCursorType, id: u32) -> ::windows::core::Result<CoreCursor>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreDispatcherImpl: Sized + ICoreAcceleratorKeysImpl {
    fn HasThreadAccess(&self) -> ::windows::core::Result<bool>;
    fn ProcessEvents(&self, options: CoreProcessEventsOption) -> ::windows::core::Result<()>;
    fn RunAsync(&self, priority: CoreDispatcherPriority, agilecallback: &::core::option::Option<DispatchedHandler>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn RunIdleAsync(&self, agilecallback: &::core::option::Option<IdleDispatchedHandler>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreDispatcher2Impl: Sized {
    fn TryRunAsync(&self, priority: CoreDispatcherPriority, agilecallback: &::core::option::Option<DispatchedHandler>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryRunIdleAsync(&self, agilecallback: &::core::option::Option<IdleDispatchedHandler>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreDispatcherWithTaskPriorityImpl: Sized {
    fn CurrentPriority(&self) -> ::windows::core::Result<CoreDispatcherPriority>;
    fn SetCurrentPriority(&self, value: CoreDispatcherPriority) -> ::windows::core::Result<()>;
    fn ShouldYield(&self) -> ::windows::core::Result<bool>;
    fn ShouldYieldToPriority(&self, priority: CoreDispatcherPriority) -> ::windows::core::Result<bool>;
    fn StopProcessEvents(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreIndependentInputSourceControllerImpl: Sized {
    fn IsTransparentForUncontrolledInput(&self) -> ::windows::core::Result<bool>;
    fn SetIsTransparentForUncontrolledInput(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsPalmRejectionEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsPalmRejectionEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn Source(&self) -> ::windows::core::Result<CoreIndependentInputSource>;
    fn SetControlledInput(&self, inputtypes: CoreInputDeviceTypes) -> ::windows::core::Result<()>;
    fn SetControlledInputWithFilters(&self, inputtypes: CoreInputDeviceTypes, required: CoreIndependentInputFilters, excluded: CoreIndependentInputFilters) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreIndependentInputSourceControllerStaticsImpl: Sized {
    fn CreateForVisual(&self, visual: &::core::option::Option<super::Composition::Visual>) -> ::windows::core::Result<CoreIndependentInputSourceController>;
    fn CreateForIVisualElement(&self, visualelement: &::core::option::Option<super::Composition::IVisualElement>) -> ::windows::core::Result<CoreIndependentInputSourceController>;
}
pub trait ICoreInputSourceBaseImpl: Sized {
    fn Dispatcher(&self) -> ::windows::core::Result<CoreDispatcher>;
    fn IsInputEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsInputEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn InputEnabled(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, InputEnabledEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveInputEnabled(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreKeyboardInputSourceImpl: Sized {
    fn GetCurrentKeyState(&self, virtualkey: super::super::System::VirtualKey) -> ::windows::core::Result<CoreVirtualKeyStates>;
    fn CharacterReceived(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, CharacterReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCharacterReceived(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn KeyDown(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, KeyEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveKeyDown(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn KeyUp(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, KeyEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveKeyUp(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreKeyboardInputSource2Impl: Sized {
    fn GetCurrentKeyEventDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
pub trait ICorePointerInputSourceImpl: Sized {
    fn ReleasePointerCapture(&self) -> ::windows::core::Result<()>;
    fn SetPointerCapture(&self) -> ::windows::core::Result<()>;
    fn HasCapture(&self) -> ::windows::core::Result<bool>;
    fn PointerPosition(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn PointerCursor(&self) -> ::windows::core::Result<CoreCursor>;
    fn SetPointerCursor(&self, value: &::core::option::Option<CoreCursor>) -> ::windows::core::Result<()>;
    fn PointerCaptureLost(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerCaptureLost(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerEntered(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerEntered(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerExited(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerExited(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerMoved(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerMoved(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerPressed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerPressed(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerReleased(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerReleased(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerWheelChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerWheelChanged(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
pub trait ICorePointerInputSource2Impl: Sized + ICorePointerInputSourceImpl {
    fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::System::DispatcherQueue>;
}
pub trait ICorePointerRedirectorImpl: Sized {
    fn PointerRoutedAway(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ICorePointerRedirector, PointerEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerRoutedAway(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerRoutedTo(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ICorePointerRedirector, PointerEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerRoutedTo(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerRoutedReleased(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ICorePointerRedirector, PointerEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerRoutedReleased(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreTouchHitTestingImpl: Sized {
    fn TouchHitTesting(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, TouchHitTestingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveTouchHitTesting(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
pub trait ICoreWindowImpl: Sized {
    fn AutomationHostProvider(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn Bounds(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn CustomProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet>;
    fn Dispatcher(&self) -> ::windows::core::Result<CoreDispatcher>;
    fn FlowDirection(&self) -> ::windows::core::Result<CoreWindowFlowDirection>;
    fn SetFlowDirection(&self, value: CoreWindowFlowDirection) -> ::windows::core::Result<()>;
    fn IsInputEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsInputEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn PointerCursor(&self) -> ::windows::core::Result<CoreCursor>;
    fn SetPointerCursor(&self, value: &::core::option::Option<CoreCursor>) -> ::windows::core::Result<()>;
    fn PointerPosition(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn Visible(&self) -> ::windows::core::Result<bool>;
    fn Activate(&self) -> ::windows::core::Result<()>;
    fn Close(&self) -> ::windows::core::Result<()>;
    fn GetAsyncKeyState(&self, virtualkey: super::super::System::VirtualKey) -> ::windows::core::Result<CoreVirtualKeyStates>;
    fn GetKeyState(&self, virtualkey: super::super::System::VirtualKey) -> ::windows::core::Result<CoreVirtualKeyStates>;
    fn ReleasePointerCapture(&self) -> ::windows::core::Result<()>;
    fn SetPointerCapture(&self) -> ::windows::core::Result<()>;
    fn Activated(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreWindow, WindowActivatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveActivated(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AutomationProviderRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreWindow, AutomationProviderRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAutomationProviderRequested(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CharacterReceived(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreWindow, CharacterReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCharacterReceived(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Closed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreWindow, CoreWindowEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn InputEnabled(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreWindow, InputEnabledEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveInputEnabled(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn KeyDown(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreWindow, KeyEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveKeyDown(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn KeyUp(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreWindow, KeyEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveKeyUp(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerCaptureLost(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerCaptureLost(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerEntered(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerEntered(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerExited(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerExited(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerMoved(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerMoved(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerPressed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerPressed(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerReleased(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerReleased(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TouchHitTesting(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreWindow, TouchHitTestingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveTouchHitTesting(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerWheelChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerWheelChanged(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SizeChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreWindow, WindowSizeChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSizeChanged(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn VisibilityChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreWindow, VisibilityChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveVisibilityChanged(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreWindow2Impl: Sized {
    fn SetPointerPosition(&self, value: &super::super::Foundation::Point) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreWindow3Impl: Sized {
    fn ClosestInteractiveBoundsRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreWindow, ClosestInteractiveBoundsRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosestInteractiveBoundsRequested(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetCurrentKeyEventDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreWindow4Impl: Sized {
    fn ResizeStarted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreWindow, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveResizeStarted(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ResizeCompleted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreWindow, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveResizeCompleted(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreWindow5Impl: Sized {
    fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::System::DispatcherQueue>;
    fn ActivationMode(&self) -> ::windows::core::Result<CoreWindowActivationMode>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreWindowDialogImpl: Sized {
    fn Showing(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreWindow, CoreWindowPopupShowingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveShowing(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MaxSize(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn MinSize(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsInteractionDelayed(&self) -> ::windows::core::Result<i32>;
    fn SetIsInteractionDelayed(&self, value: i32) -> ::windows::core::Result<()>;
    fn Commands(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Popups::IUICommand>>;
    fn DefaultCommandIndex(&self) -> ::windows::core::Result<u32>;
    fn SetDefaultCommandIndex(&self, value: u32) -> ::windows::core::Result<()>;
    fn CancelCommandIndex(&self) -> ::windows::core::Result<u32>;
    fn SetCancelCommandIndex(&self, value: u32) -> ::windows::core::Result<()>;
    fn BackButtonCommand(&self) -> ::windows::core::Result<super::Popups::UICommandInvokedHandler>;
    fn SetBackButtonCommand(&self, value: &::core::option::Option<super::Popups::UICommandInvokedHandler>) -> ::windows::core::Result<()>;
    fn ShowAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::Popups::IUICommand>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreWindowDialogFactoryImpl: Sized {
    fn CreateWithTitle(&self, title: &::windows::core::HSTRING) -> ::windows::core::Result<CoreWindowDialog>;
}
pub trait ICoreWindowEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreWindowFlyoutImpl: Sized {
    fn Showing(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreWindow, CoreWindowPopupShowingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveShowing(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MaxSize(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn MinSize(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsInteractionDelayed(&self) -> ::windows::core::Result<i32>;
    fn SetIsInteractionDelayed(&self, value: i32) -> ::windows::core::Result<()>;
    fn Commands(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Popups::IUICommand>>;
    fn DefaultCommandIndex(&self) -> ::windows::core::Result<u32>;
    fn SetDefaultCommandIndex(&self, value: u32) -> ::windows::core::Result<()>;
    fn BackButtonCommand(&self) -> ::windows::core::Result<super::Popups::UICommandInvokedHandler>;
    fn SetBackButtonCommand(&self, value: &::core::option::Option<super::Popups::UICommandInvokedHandler>) -> ::windows::core::Result<()>;
    fn ShowAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::Popups::IUICommand>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreWindowFlyoutFactoryImpl: Sized {
    fn Create(&self, position: &super::super::Foundation::Point) -> ::windows::core::Result<CoreWindowFlyout>;
    fn CreateWithTitle(&self, position: &super::super::Foundation::Point, title: &::windows::core::HSTRING) -> ::windows::core::Result<CoreWindowFlyout>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreWindowPopupShowingEventArgsImpl: Sized {
    fn SetDesiredSize(&self, value: &super::super::Foundation::Size) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreWindowResizeManagerImpl: Sized {
    fn NotifyLayoutCompleted(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreWindowResizeManagerLayoutCapabilityImpl: Sized {
    fn SetShouldWaitForLayoutCompletion(&self, value: bool) -> ::windows::core::Result<()>;
    fn ShouldWaitForLayoutCompletion(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreWindowResizeManagerStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<CoreWindowResizeManager>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreWindowStaticImpl: Sized {
    fn GetForCurrentThread(&self) -> ::windows::core::Result<CoreWindow>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreWindowWithContextImpl: Sized {
    fn UIContext(&self) -> ::windows::core::Result<super::UIContext>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IIdleDispatchedHandlerArgsImpl: Sized {
    fn IsDispatcherIdle(&self) -> ::windows::core::Result<bool>;
}
pub trait IInitializeWithCoreWindowImpl: Sized {
    fn Initialize(&self, window: &::core::option::Option<CoreWindow>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInputEnabledEventArgsImpl: Sized + ICoreWindowEventArgsImpl {
    fn InputEnabled(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyEventArgsImpl: Sized + ICoreWindowEventArgsImpl {
    fn VirtualKey(&self) -> ::windows::core::Result<super::super::System::VirtualKey>;
    fn KeyStatus(&self) -> ::windows::core::Result<CorePhysicalKeyStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyEventArgs2Impl: Sized + ICoreWindowEventArgsImpl {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointerEventArgsImpl: Sized + ICoreWindowEventArgsImpl {
    fn CurrentPoint(&self) -> ::windows::core::Result<super::Input::PointerPoint>;
    fn KeyModifiers(&self) -> ::windows::core::Result<super::super::System::VirtualKeyModifiers>;
    fn GetIntermediatePoints(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Input::PointerPoint>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemNavigationManagerImpl: Sized {
    fn BackRequested(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<BackRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBackRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemNavigationManager2Impl: Sized {
    fn AppViewBackButtonVisibility(&self) -> ::windows::core::Result<AppViewBackButtonVisibility>;
    fn SetAppViewBackButtonVisibility(&self, value: AppViewBackButtonVisibility) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemNavigationManagerStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<SystemNavigationManager>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITouchHitTestingEventArgsImpl: Sized + ICoreWindowEventArgsImpl {
    fn ProximityEvaluation(&self) -> ::windows::core::Result<CoreProximityEvaluation>;
    fn SetProximityEvaluation(&self, value: &CoreProximityEvaluation) -> ::windows::core::Result<()>;
    fn Point(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn BoundingBox(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn EvaluateProximityToRect(&self, controlboundingbox: &super::super::Foundation::Rect) -> ::windows::core::Result<CoreProximityEvaluation>;
    fn EvaluateProximityToPolygon(&self, controlvertices: &[<super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<CoreProximityEvaluation>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisibilityChangedEventArgsImpl: Sized + ICoreWindowEventArgsImpl {
    fn Visible(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWindowActivatedEventArgsImpl: Sized + ICoreWindowEventArgsImpl {
    fn WindowActivationState(&self) -> ::windows::core::Result<CoreWindowActivationState>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWindowSizeChangedEventArgsImpl: Sized + ICoreWindowEventArgsImpl {
    fn Size(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
}
