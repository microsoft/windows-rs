#[cfg(feature = "implement_exclusive")]
pub trait IAttachableInputObjectImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IAttachableInputObjectFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ICrossSlidingEventArgsImpl: Sized {
    fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::Devices::Input::PointerDeviceType>;
    fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn CrossSlidingState(&self) -> ::windows::core::Result<CrossSlidingState>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICrossSlidingEventArgs2Impl: Sized {
    fn ContactCount(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDraggingEventArgsImpl: Sized {
    fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::Devices::Input::PointerDeviceType>;
    fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn DraggingState(&self) -> ::windows::core::Result<DraggingState>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDraggingEventArgs2Impl: Sized {
    fn ContactCount(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEdgeGestureImpl: Sized {
    fn Starting(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<EdgeGesture, EdgeGestureEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStarting(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Completed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<EdgeGesture, EdgeGestureEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCompleted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Canceled(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<EdgeGesture, EdgeGestureEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCanceled(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEdgeGestureEventArgsImpl: Sized {
    fn Kind(&self) -> ::windows::core::Result<EdgeGestureKind>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEdgeGestureStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<EdgeGesture>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGestureRecognizerImpl: Sized {
    fn GestureSettings(&self) -> ::windows::core::Result<GestureSettings>;
    fn SetGestureSettings(&self, value: GestureSettings) -> ::windows::core::Result<()>;
    fn IsInertial(&self) -> ::windows::core::Result<bool>;
    fn IsActive(&self) -> ::windows::core::Result<bool>;
    fn ShowGestureFeedback(&self) -> ::windows::core::Result<bool>;
    fn SetShowGestureFeedback(&self, value: bool) -> ::windows::core::Result<()>;
    fn PivotCenter(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn SetPivotCenter(&self, value: &super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn PivotRadius(&self) -> ::windows::core::Result<f32>;
    fn SetPivotRadius(&self, value: f32) -> ::windows::core::Result<()>;
    fn InertiaTranslationDeceleration(&self) -> ::windows::core::Result<f32>;
    fn SetInertiaTranslationDeceleration(&self, value: f32) -> ::windows::core::Result<()>;
    fn InertiaRotationDeceleration(&self) -> ::windows::core::Result<f32>;
    fn SetInertiaRotationDeceleration(&self, value: f32) -> ::windows::core::Result<()>;
    fn InertiaExpansionDeceleration(&self) -> ::windows::core::Result<f32>;
    fn SetInertiaExpansionDeceleration(&self, value: f32) -> ::windows::core::Result<()>;
    fn InertiaTranslationDisplacement(&self) -> ::windows::core::Result<f32>;
    fn SetInertiaTranslationDisplacement(&self, value: f32) -> ::windows::core::Result<()>;
    fn InertiaRotationAngle(&self) -> ::windows::core::Result<f32>;
    fn SetInertiaRotationAngle(&self, value: f32) -> ::windows::core::Result<()>;
    fn InertiaExpansion(&self) -> ::windows::core::Result<f32>;
    fn SetInertiaExpansion(&self, value: f32) -> ::windows::core::Result<()>;
    fn ManipulationExact(&self) -> ::windows::core::Result<bool>;
    fn SetManipulationExact(&self, value: bool) -> ::windows::core::Result<()>;
    fn CrossSlideThresholds(&self) -> ::windows::core::Result<CrossSlideThresholds>;
    fn SetCrossSlideThresholds(&self, value: &CrossSlideThresholds) -> ::windows::core::Result<()>;
    fn CrossSlideHorizontally(&self) -> ::windows::core::Result<bool>;
    fn SetCrossSlideHorizontally(&self, value: bool) -> ::windows::core::Result<()>;
    fn CrossSlideExact(&self) -> ::windows::core::Result<bool>;
    fn SetCrossSlideExact(&self, value: bool) -> ::windows::core::Result<()>;
    fn AutoProcessInertia(&self) -> ::windows::core::Result<bool>;
    fn SetAutoProcessInertia(&self, value: bool) -> ::windows::core::Result<()>;
    fn MouseWheelParameters(&self) -> ::windows::core::Result<MouseWheelParameters>;
    fn CanBeDoubleTap(&self, value: &::core::option::Option<PointerPoint>) -> ::windows::core::Result<bool>;
    fn ProcessDownEvent(&self, value: &::core::option::Option<PointerPoint>) -> ::windows::core::Result<()>;
    fn ProcessMoveEvents(&self, value: &::core::option::Option<super::super::Foundation::Collections::IVector<PointerPoint>>) -> ::windows::core::Result<()>;
    fn ProcessUpEvent(&self, value: &::core::option::Option<PointerPoint>) -> ::windows::core::Result<()>;
    fn ProcessMouseWheelEvent(&self, value: &::core::option::Option<PointerPoint>, isshiftkeydown: bool, iscontrolkeydown: bool) -> ::windows::core::Result<()>;
    fn ProcessInertia(&self) -> ::windows::core::Result<()>;
    fn CompleteGesture(&self) -> ::windows::core::Result<()>;
    fn Tapped(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<GestureRecognizer, TappedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveTapped(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RightTapped(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<GestureRecognizer, RightTappedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRightTapped(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Holding(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<GestureRecognizer, HoldingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveHolding(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Dragging(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<GestureRecognizer, DraggingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDragging(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ManipulationStarted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<GestureRecognizer, ManipulationStartedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveManipulationStarted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ManipulationUpdated(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<GestureRecognizer, ManipulationUpdatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveManipulationUpdated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ManipulationInertiaStarting(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<GestureRecognizer, ManipulationInertiaStartingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveManipulationInertiaStarting(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ManipulationCompleted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<GestureRecognizer, ManipulationCompletedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveManipulationCompleted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CrossSliding(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<GestureRecognizer, CrossSlidingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCrossSliding(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGestureRecognizer2Impl: Sized {
    fn TapMinContactCount(&self) -> ::windows::core::Result<u32>;
    fn SetTapMinContactCount(&self, value: u32) -> ::windows::core::Result<()>;
    fn TapMaxContactCount(&self) -> ::windows::core::Result<u32>;
    fn SetTapMaxContactCount(&self, value: u32) -> ::windows::core::Result<()>;
    fn HoldMinContactCount(&self) -> ::windows::core::Result<u32>;
    fn SetHoldMinContactCount(&self, value: u32) -> ::windows::core::Result<()>;
    fn HoldMaxContactCount(&self) -> ::windows::core::Result<u32>;
    fn SetHoldMaxContactCount(&self, value: u32) -> ::windows::core::Result<()>;
    fn HoldRadius(&self) -> ::windows::core::Result<f32>;
    fn SetHoldRadius(&self, value: f32) -> ::windows::core::Result<()>;
    fn HoldStartDelay(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetHoldStartDelay(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn TranslationMinContactCount(&self) -> ::windows::core::Result<u32>;
    fn SetTranslationMinContactCount(&self, value: u32) -> ::windows::core::Result<()>;
    fn TranslationMaxContactCount(&self) -> ::windows::core::Result<u32>;
    fn SetTranslationMaxContactCount(&self, value: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHoldingEventArgsImpl: Sized {
    fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::Devices::Input::PointerDeviceType>;
    fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn HoldingState(&self) -> ::windows::core::Result<HoldingState>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHoldingEventArgs2Impl: Sized {
    fn ContactCount(&self) -> ::windows::core::Result<u32>;
    fn CurrentContactCount(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInputActivationListenerImpl: Sized {
    fn State(&self) -> ::windows::core::Result<InputActivationState>;
    fn InputActivationChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<InputActivationListener, InputActivationListenerActivationChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveInputActivationChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInputActivationListenerActivationChangedEventArgsImpl: Sized {
    fn State(&self) -> ::windows::core::Result<InputActivationState>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyboardDeliveryInterceptorImpl: Sized {
    fn IsInterceptionEnabledWhenInForeground(&self) -> ::windows::core::Result<bool>;
    fn SetIsInterceptionEnabledWhenInForeground(&self, value: bool) -> ::windows::core::Result<()>;
    fn KeyDown(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<KeyboardDeliveryInterceptor, super::Core::KeyEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveKeyDown(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn KeyUp(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<KeyboardDeliveryInterceptor, super::Core::KeyEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveKeyUp(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyboardDeliveryInterceptorStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<KeyboardDeliveryInterceptor>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IManipulationCompletedEventArgsImpl: Sized {
    fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::Devices::Input::PointerDeviceType>;
    fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn Cumulative(&self) -> ::windows::core::Result<ManipulationDelta>;
    fn Velocities(&self) -> ::windows::core::Result<ManipulationVelocities>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IManipulationCompletedEventArgs2Impl: Sized {
    fn ContactCount(&self) -> ::windows::core::Result<u32>;
    fn CurrentContactCount(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IManipulationInertiaStartingEventArgsImpl: Sized {
    fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::Devices::Input::PointerDeviceType>;
    fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn Delta(&self) -> ::windows::core::Result<ManipulationDelta>;
    fn Cumulative(&self) -> ::windows::core::Result<ManipulationDelta>;
    fn Velocities(&self) -> ::windows::core::Result<ManipulationVelocities>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IManipulationInertiaStartingEventArgs2Impl: Sized {
    fn ContactCount(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IManipulationStartedEventArgsImpl: Sized {
    fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::Devices::Input::PointerDeviceType>;
    fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn Cumulative(&self) -> ::windows::core::Result<ManipulationDelta>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IManipulationStartedEventArgs2Impl: Sized {
    fn ContactCount(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IManipulationUpdatedEventArgsImpl: Sized {
    fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::Devices::Input::PointerDeviceType>;
    fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn Delta(&self) -> ::windows::core::Result<ManipulationDelta>;
    fn Cumulative(&self) -> ::windows::core::Result<ManipulationDelta>;
    fn Velocities(&self) -> ::windows::core::Result<ManipulationVelocities>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IManipulationUpdatedEventArgs2Impl: Sized {
    fn ContactCount(&self) -> ::windows::core::Result<u32>;
    fn CurrentContactCount(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMouseWheelParametersImpl: Sized {
    fn CharTranslation(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn SetCharTranslation(&self, value: &super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn DeltaScale(&self) -> ::windows::core::Result<f32>;
    fn SetDeltaScale(&self, value: f32) -> ::windows::core::Result<()>;
    fn DeltaRotationAngle(&self) -> ::windows::core::Result<f32>;
    fn SetDeltaRotationAngle(&self, value: f32) -> ::windows::core::Result<()>;
    fn PageTranslation(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn SetPageTranslation(&self, value: &super::super::Foundation::Point) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointerPointImpl: Sized {
    fn PointerDevice(&self) -> ::windows::core::Result<super::super::Devices::Input::PointerDevice>;
    fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn RawPosition(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn PointerId(&self) -> ::windows::core::Result<u32>;
    fn FrameId(&self) -> ::windows::core::Result<u32>;
    fn Timestamp(&self) -> ::windows::core::Result<u64>;
    fn IsInContact(&self) -> ::windows::core::Result<bool>;
    fn Properties(&self) -> ::windows::core::Result<PointerPointProperties>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointerPointPropertiesImpl: Sized {
    fn Pressure(&self) -> ::windows::core::Result<f32>;
    fn IsInverted(&self) -> ::windows::core::Result<bool>;
    fn IsEraser(&self) -> ::windows::core::Result<bool>;
    fn Orientation(&self) -> ::windows::core::Result<f32>;
    fn XTilt(&self) -> ::windows::core::Result<f32>;
    fn YTilt(&self) -> ::windows::core::Result<f32>;
    fn Twist(&self) -> ::windows::core::Result<f32>;
    fn ContactRect(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn ContactRectRaw(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn TouchConfidence(&self) -> ::windows::core::Result<bool>;
    fn IsLeftButtonPressed(&self) -> ::windows::core::Result<bool>;
    fn IsRightButtonPressed(&self) -> ::windows::core::Result<bool>;
    fn IsMiddleButtonPressed(&self) -> ::windows::core::Result<bool>;
    fn MouseWheelDelta(&self) -> ::windows::core::Result<i32>;
    fn IsHorizontalMouseWheel(&self) -> ::windows::core::Result<bool>;
    fn IsPrimary(&self) -> ::windows::core::Result<bool>;
    fn IsInRange(&self) -> ::windows::core::Result<bool>;
    fn IsCanceled(&self) -> ::windows::core::Result<bool>;
    fn IsBarrelButtonPressed(&self) -> ::windows::core::Result<bool>;
    fn IsXButton1Pressed(&self) -> ::windows::core::Result<bool>;
    fn IsXButton2Pressed(&self) -> ::windows::core::Result<bool>;
    fn PointerUpdateKind(&self) -> ::windows::core::Result<PointerUpdateKind>;
    fn HasUsage(&self, usagepage: u32, usageid: u32) -> ::windows::core::Result<bool>;
    fn GetUsageValue(&self, usagepage: u32, usageid: u32) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointerPointProperties2Impl: Sized {
    fn ZDistance(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f32>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointerPointStaticsImpl: Sized {
    fn GetCurrentPoint(&self, pointerid: u32) -> ::windows::core::Result<PointerPoint>;
    fn GetIntermediatePoints(&self, pointerid: u32) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<PointerPoint>>;
    fn GetCurrentPointTransformed(&self, pointerid: u32, transform: &::core::option::Option<IPointerPointTransform>) -> ::windows::core::Result<PointerPoint>;
    fn GetIntermediatePointsTransformed(&self, pointerid: u32, transform: &::core::option::Option<IPointerPointTransform>) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<PointerPoint>>;
}
pub trait IPointerPointTransformImpl: Sized {
    fn Inverse(&self) -> ::windows::core::Result<IPointerPointTransform>;
    fn TryTransform(&self, inpoint: &super::super::Foundation::Point, outpoint: &mut super::super::Foundation::Point) -> ::windows::core::Result<bool>;
    fn TransformBounds(&self, rect: &super::super::Foundation::Rect) -> ::windows::core::Result<super::super::Foundation::Rect>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointerVisualizationSettingsImpl: Sized {
    fn SetIsContactFeedbackEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsContactFeedbackEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsBarrelButtonFeedbackEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsBarrelButtonFeedbackEnabled(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointerVisualizationSettingsStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<PointerVisualizationSettings>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerImpl: Sized {
    fn Menu(&self) -> ::windows::core::Result<RadialControllerMenu>;
    fn RotationResolutionInDegrees(&self) -> ::windows::core::Result<f64>;
    fn SetRotationResolutionInDegrees(&self, value: f64) -> ::windows::core::Result<()>;
    fn UseAutomaticHapticFeedback(&self) -> ::windows::core::Result<bool>;
    fn SetUseAutomaticHapticFeedback(&self, value: bool) -> ::windows::core::Result<()>;
    fn ScreenContactStarted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RadialController, RadialControllerScreenContactStartedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveScreenContactStarted(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ScreenContactEnded(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RadialController, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveScreenContactEnded(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ScreenContactContinued(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RadialController, RadialControllerScreenContactContinuedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveScreenContactContinued(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ControlLost(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RadialController, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveControlLost(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RotationChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RadialController, RadialControllerRotationChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRotationChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ButtonClicked(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RadialController, RadialControllerButtonClickedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveButtonClicked(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ControlAcquired(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RadialController, RadialControllerControlAcquiredEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveControlAcquired(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialController2Impl: Sized {
    fn ButtonPressed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RadialController, RadialControllerButtonPressedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveButtonPressed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ButtonHolding(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RadialController, RadialControllerButtonHoldingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveButtonHolding(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ButtonReleased(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RadialController, RadialControllerButtonReleasedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveButtonReleased(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerButtonClickedEventArgsImpl: Sized {
    fn Contact(&self) -> ::windows::core::Result<RadialControllerScreenContact>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerButtonClickedEventArgs2Impl: Sized {
    fn SimpleHapticsController(&self) -> ::windows::core::Result<super::super::Devices::Haptics::SimpleHapticsController>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerButtonHoldingEventArgsImpl: Sized {
    fn Contact(&self) -> ::windows::core::Result<RadialControllerScreenContact>;
    fn SimpleHapticsController(&self) -> ::windows::core::Result<super::super::Devices::Haptics::SimpleHapticsController>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerButtonPressedEventArgsImpl: Sized {
    fn Contact(&self) -> ::windows::core::Result<RadialControllerScreenContact>;
    fn SimpleHapticsController(&self) -> ::windows::core::Result<super::super::Devices::Haptics::SimpleHapticsController>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerButtonReleasedEventArgsImpl: Sized {
    fn Contact(&self) -> ::windows::core::Result<RadialControllerScreenContact>;
    fn SimpleHapticsController(&self) -> ::windows::core::Result<super::super::Devices::Haptics::SimpleHapticsController>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerConfigurationImpl: Sized {
    fn SetDefaultMenuItems(&self, buttons: &::core::option::Option<super::super::Foundation::Collections::IIterable<RadialControllerSystemMenuItemKind>>) -> ::windows::core::Result<()>;
    fn ResetToDefaultMenuItems(&self) -> ::windows::core::Result<()>;
    fn TrySelectDefaultMenuItem(&self, r#type: RadialControllerSystemMenuItemKind) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerConfiguration2Impl: Sized {
    fn SetActiveControllerWhenMenuIsSuppressed(&self, value: &::core::option::Option<RadialController>) -> ::windows::core::Result<()>;
    fn ActiveControllerWhenMenuIsSuppressed(&self) -> ::windows::core::Result<RadialController>;
    fn SetIsMenuSuppressed(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsMenuSuppressed(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerConfigurationStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<RadialControllerConfiguration>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerConfigurationStatics2Impl: Sized {
    fn SetAppController(&self, value: &::core::option::Option<RadialController>) -> ::windows::core::Result<()>;
    fn AppController(&self) -> ::windows::core::Result<RadialController>;
    fn SetIsAppControllerEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsAppControllerEnabled(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerControlAcquiredEventArgsImpl: Sized {
    fn Contact(&self) -> ::windows::core::Result<RadialControllerScreenContact>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerControlAcquiredEventArgs2Impl: Sized {
    fn IsButtonPressed(&self) -> ::windows::core::Result<bool>;
    fn SimpleHapticsController(&self) -> ::windows::core::Result<super::super::Devices::Haptics::SimpleHapticsController>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerMenuImpl: Sized {
    fn Items(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<RadialControllerMenuItem>>;
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn GetSelectedMenuItem(&self) -> ::windows::core::Result<RadialControllerMenuItem>;
    fn SelectMenuItem(&self, menuitem: &::core::option::Option<RadialControllerMenuItem>) -> ::windows::core::Result<()>;
    fn TrySelectPreviouslySelectedMenuItem(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerMenuItemImpl: Sized {
    fn DisplayText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Tag(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetTag(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn Invoked(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RadialControllerMenuItem, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveInvoked(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerMenuItemStaticsImpl: Sized {
    fn CreateFromIcon(&self, displaytext: &::windows::core::HSTRING, icon: &::core::option::Option<super::super::Storage::Streams::RandomAccessStreamReference>) -> ::windows::core::Result<RadialControllerMenuItem>;
    fn CreateFromKnownIcon(&self, displaytext: &::windows::core::HSTRING, value: RadialControllerMenuKnownIcon) -> ::windows::core::Result<RadialControllerMenuItem>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerMenuItemStatics2Impl: Sized {
    fn CreateFromFontGlyph(&self, displaytext: &::windows::core::HSTRING, glyph: &::windows::core::HSTRING, fontfamily: &::windows::core::HSTRING) -> ::windows::core::Result<RadialControllerMenuItem>;
    fn CreateFromFontGlyphWithUri(&self, displaytext: &::windows::core::HSTRING, glyph: &::windows::core::HSTRING, fontfamily: &::windows::core::HSTRING, fonturi: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<RadialControllerMenuItem>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerRotationChangedEventArgsImpl: Sized {
    fn RotationDeltaInDegrees(&self) -> ::windows::core::Result<f64>;
    fn Contact(&self) -> ::windows::core::Result<RadialControllerScreenContact>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerRotationChangedEventArgs2Impl: Sized {
    fn IsButtonPressed(&self) -> ::windows::core::Result<bool>;
    fn SimpleHapticsController(&self) -> ::windows::core::Result<super::super::Devices::Haptics::SimpleHapticsController>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerScreenContactImpl: Sized {
    fn Bounds(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerScreenContactContinuedEventArgsImpl: Sized {
    fn Contact(&self) -> ::windows::core::Result<RadialControllerScreenContact>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerScreenContactContinuedEventArgs2Impl: Sized {
    fn IsButtonPressed(&self) -> ::windows::core::Result<bool>;
    fn SimpleHapticsController(&self) -> ::windows::core::Result<super::super::Devices::Haptics::SimpleHapticsController>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerScreenContactEndedEventArgsImpl: Sized {
    fn IsButtonPressed(&self) -> ::windows::core::Result<bool>;
    fn SimpleHapticsController(&self) -> ::windows::core::Result<super::super::Devices::Haptics::SimpleHapticsController>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerScreenContactStartedEventArgsImpl: Sized {
    fn Contact(&self) -> ::windows::core::Result<RadialControllerScreenContact>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerScreenContactStartedEventArgs2Impl: Sized {
    fn IsButtonPressed(&self) -> ::windows::core::Result<bool>;
    fn SimpleHapticsController(&self) -> ::windows::core::Result<super::super::Devices::Haptics::SimpleHapticsController>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerStaticsImpl: Sized {
    fn IsSupported(&self) -> ::windows::core::Result<bool>;
    fn CreateForCurrentView(&self) -> ::windows::core::Result<RadialController>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRightTappedEventArgsImpl: Sized {
    fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::Devices::Input::PointerDeviceType>;
    fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRightTappedEventArgs2Impl: Sized {
    fn ContactCount(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemButtonEventControllerImpl: Sized {
    fn SystemFunctionButtonPressed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SystemButtonEventController, SystemFunctionButtonEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSystemFunctionButtonPressed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SystemFunctionButtonReleased(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SystemButtonEventController, SystemFunctionButtonEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSystemFunctionButtonReleased(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SystemFunctionLockChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SystemButtonEventController, SystemFunctionLockChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSystemFunctionLockChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SystemFunctionLockIndicatorChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SystemButtonEventController, SystemFunctionLockIndicatorChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSystemFunctionLockIndicatorChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemButtonEventControllerStaticsImpl: Sized {
    fn CreateForDispatcherQueue(&self, queue: &::core::option::Option<super::super::System::DispatcherQueue>) -> ::windows::core::Result<SystemButtonEventController>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemFunctionButtonEventArgsImpl: Sized {
    fn Timestamp(&self) -> ::windows::core::Result<u64>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemFunctionLockChangedEventArgsImpl: Sized {
    fn Timestamp(&self) -> ::windows::core::Result<u64>;
    fn IsLocked(&self) -> ::windows::core::Result<bool>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemFunctionLockIndicatorChangedEventArgsImpl: Sized {
    fn Timestamp(&self) -> ::windows::core::Result<u64>;
    fn IsIndicatorOn(&self) -> ::windows::core::Result<bool>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITappedEventArgsImpl: Sized {
    fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::Devices::Input::PointerDeviceType>;
    fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn TapCount(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITappedEventArgs2Impl: Sized {
    fn ContactCount(&self) -> ::windows::core::Result<u32>;
}
