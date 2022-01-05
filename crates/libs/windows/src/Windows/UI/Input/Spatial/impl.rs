#[cfg(feature = "implement_exclusive")]
pub trait ISpatialGestureRecognizerImpl: Sized {
    fn RecognitionStarted(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialRecognitionStartedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveRecognitionStarted(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RecognitionEnded(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialRecognitionEndedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveRecognitionEnded(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Tapped(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialTappedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveTapped(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn HoldStarted(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialHoldStartedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveHoldStarted(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn HoldCompleted(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialHoldCompletedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveHoldCompleted(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn HoldCanceled(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialHoldCanceledEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveHoldCanceled(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ManipulationStarted(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialManipulationStartedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveManipulationStarted(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ManipulationUpdated(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialManipulationUpdatedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveManipulationUpdated(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ManipulationCompleted(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialManipulationCompletedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveManipulationCompleted(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ManipulationCanceled(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialManipulationCanceledEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveManipulationCanceled(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NavigationStarted(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialNavigationStartedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveNavigationStarted(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NavigationUpdated(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialNavigationUpdatedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveNavigationUpdated(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NavigationCompleted(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialNavigationCompletedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveNavigationCompleted(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NavigationCanceled(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialGestureRecognizer, SpatialNavigationCanceledEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveNavigationCanceled(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CaptureInteraction(&self, interaction: &::core::option::Option<SpatialInteraction>) -> ::windows::core::Result<()>;
    fn CancelPendingGestures(&self) -> ::windows::core::Result<()>;
    fn TrySetGestureSettings(&self, settings: SpatialGestureSettings) -> ::windows::core::Result<bool>;
    fn GestureSettings(&self) -> ::windows::core::Result<SpatialGestureSettings>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialGestureRecognizerFactoryImpl: Sized {
    fn Create(&self, settings: SpatialGestureSettings) -> ::windows::core::Result<SpatialGestureRecognizer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialHoldCanceledEventArgsImpl: Sized {
    fn InteractionSourceKind(&self) -> ::windows::core::Result<SpatialInteractionSourceKind>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialHoldCompletedEventArgsImpl: Sized {
    fn InteractionSourceKind(&self) -> ::windows::core::Result<SpatialInteractionSourceKind>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialHoldStartedEventArgsImpl: Sized {
    fn InteractionSourceKind(&self) -> ::windows::core::Result<SpatialInteractionSourceKind>;
    fn TryGetPointerPose(&self, coordinatesystem: &::core::option::Option<super::super::super::Perception::Spatial::SpatialCoordinateSystem>) -> ::windows::core::Result<SpatialPointerPose>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialInteractionImpl: Sized {
    fn SourceState(&self) -> ::windows::core::Result<SpatialInteractionSourceState>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialInteractionControllerImpl: Sized {
    fn HasTouchpad(&self) -> ::windows::core::Result<bool>;
    fn HasThumbstick(&self) -> ::windows::core::Result<bool>;
    fn SimpleHapticsController(&self) -> ::windows::core::Result<super::super::super::Devices::Haptics::SimpleHapticsController>;
    fn VendorId(&self) -> ::windows::core::Result<u16>;
    fn ProductId(&self) -> ::windows::core::Result<u16>;
    fn Version(&self) -> ::windows::core::Result<u16>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialInteractionController2Impl: Sized + ISpatialInteractionControllerImpl {
    fn TryGetRenderableModelAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IRandomAccessStreamWithContentType>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialInteractionController3Impl: Sized + ISpatialInteractionControllerImpl + ISpatialInteractionController2Impl {
    fn TryGetBatteryReport(&self) -> ::windows::core::Result<super::super::super::Devices::Power::BatteryReport>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialInteractionControllerPropertiesImpl: Sized {
    fn IsTouchpadTouched(&self) -> ::windows::core::Result<bool>;
    fn IsTouchpadPressed(&self) -> ::windows::core::Result<bool>;
    fn IsThumbstickPressed(&self) -> ::windows::core::Result<bool>;
    fn ThumbstickX(&self) -> ::windows::core::Result<f64>;
    fn ThumbstickY(&self) -> ::windows::core::Result<f64>;
    fn TouchpadX(&self) -> ::windows::core::Result<f64>;
    fn TouchpadY(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialInteractionDetectedEventArgsImpl: Sized {
    fn InteractionSourceKind(&self) -> ::windows::core::Result<SpatialInteractionSourceKind>;
    fn TryGetPointerPose(&self, coordinatesystem: &::core::option::Option<super::super::super::Perception::Spatial::SpatialCoordinateSystem>) -> ::windows::core::Result<SpatialPointerPose>;
    fn Interaction(&self) -> ::windows::core::Result<SpatialInteraction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialInteractionDetectedEventArgs2Impl: Sized + ISpatialInteractionDetectedEventArgsImpl {
    fn InteractionSource(&self) -> ::windows::core::Result<SpatialInteractionSource>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialInteractionManagerImpl: Sized {
    fn SourceDetected(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionSourceEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSourceDetected(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SourceLost(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionSourceEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSourceLost(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SourceUpdated(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionSourceEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSourceUpdated(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SourcePressed(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionSourceEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSourcePressed(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SourceReleased(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionSourceEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSourceReleased(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn InteractionDetected(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<SpatialInteractionManager, SpatialInteractionDetectedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveInteractionDetected(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetDetectedSourcesAtTimestamp(&self, timestamp: &::core::option::Option<super::super::super::Perception::PerceptionTimestamp>) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<SpatialInteractionSourceState>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialInteractionManagerStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<SpatialInteractionManager>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialInteractionManagerStatics2Impl: Sized {
    fn IsSourceKindSupported(&self, kind: SpatialInteractionSourceKind) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialInteractionSourceImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<u32>;
    fn Kind(&self) -> ::windows::core::Result<SpatialInteractionSourceKind>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialInteractionSource2Impl: Sized + ISpatialInteractionSourceImpl {
    fn IsPointingSupported(&self) -> ::windows::core::Result<bool>;
    fn IsMenuSupported(&self) -> ::windows::core::Result<bool>;
    fn IsGraspSupported(&self) -> ::windows::core::Result<bool>;
    fn Controller(&self) -> ::windows::core::Result<SpatialInteractionController>;
    fn TryGetStateAtTimestamp(&self, timestamp: &::core::option::Option<super::super::super::Perception::PerceptionTimestamp>) -> ::windows::core::Result<SpatialInteractionSourceState>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialInteractionSource3Impl: Sized + ISpatialInteractionSourceImpl + ISpatialInteractionSource2Impl {
    fn Handedness(&self) -> ::windows::core::Result<SpatialInteractionSourceHandedness>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialInteractionSource4Impl: Sized {
    fn TryCreateHandMeshObserver(&self) -> ::windows::core::Result<super::super::super::Perception::People::HandMeshObserver>;
    fn TryCreateHandMeshObserverAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Perception::People::HandMeshObserver>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialInteractionSourceEventArgsImpl: Sized {
    fn State(&self) -> ::windows::core::Result<SpatialInteractionSourceState>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialInteractionSourceEventArgs2Impl: Sized + ISpatialInteractionSourceEventArgsImpl {
    fn PressKind(&self) -> ::windows::core::Result<SpatialInteractionPressKind>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialInteractionSourceLocationImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>>;
    fn Velocity(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialInteractionSourceLocation2Impl: Sized {
    fn Orientation(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Quaternion>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialInteractionSourceLocation3Impl: Sized + ISpatialInteractionSourceLocation2Impl {
    fn PositionAccuracy(&self) -> ::windows::core::Result<SpatialInteractionSourcePositionAccuracy>;
    fn AngularVelocity(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>>;
    fn SourcePointerPose(&self) -> ::windows::core::Result<SpatialPointerInteractionSourcePose>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialInteractionSourcePropertiesImpl: Sized {
    fn TryGetSourceLossMitigationDirection(&self, coordinatesystem: &::core::option::Option<super::super::super::Perception::Spatial::SpatialCoordinateSystem>) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>>;
    fn SourceLossRisk(&self) -> ::windows::core::Result<f64>;
    fn TryGetLocation(&self, coordinatesystem: &::core::option::Option<super::super::super::Perception::Spatial::SpatialCoordinateSystem>) -> ::windows::core::Result<SpatialInteractionSourceLocation>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialInteractionSourceStateImpl: Sized {
    fn Source(&self) -> ::windows::core::Result<SpatialInteractionSource>;
    fn Properties(&self) -> ::windows::core::Result<SpatialInteractionSourceProperties>;
    fn IsPressed(&self) -> ::windows::core::Result<bool>;
    fn Timestamp(&self) -> ::windows::core::Result<super::super::super::Perception::PerceptionTimestamp>;
    fn TryGetPointerPose(&self, coordinatesystem: &::core::option::Option<super::super::super::Perception::Spatial::SpatialCoordinateSystem>) -> ::windows::core::Result<SpatialPointerPose>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialInteractionSourceState2Impl: Sized + ISpatialInteractionSourceStateImpl {
    fn IsSelectPressed(&self) -> ::windows::core::Result<bool>;
    fn IsMenuPressed(&self) -> ::windows::core::Result<bool>;
    fn IsGrasped(&self) -> ::windows::core::Result<bool>;
    fn SelectPressedValue(&self) -> ::windows::core::Result<f64>;
    fn ControllerProperties(&self) -> ::windows::core::Result<SpatialInteractionControllerProperties>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialInteractionSourceState3Impl: Sized + ISpatialInteractionSourceStateImpl + ISpatialInteractionSourceState2Impl {
    fn TryGetHandPose(&self) -> ::windows::core::Result<super::super::super::Perception::People::HandPose>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialManipulationCanceledEventArgsImpl: Sized {
    fn InteractionSourceKind(&self) -> ::windows::core::Result<SpatialInteractionSourceKind>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialManipulationCompletedEventArgsImpl: Sized {
    fn InteractionSourceKind(&self) -> ::windows::core::Result<SpatialInteractionSourceKind>;
    fn TryGetCumulativeDelta(&self, coordinatesystem: &::core::option::Option<super::super::super::Perception::Spatial::SpatialCoordinateSystem>) -> ::windows::core::Result<SpatialManipulationDelta>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialManipulationDeltaImpl: Sized {
    fn Translation(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialManipulationStartedEventArgsImpl: Sized {
    fn InteractionSourceKind(&self) -> ::windows::core::Result<SpatialInteractionSourceKind>;
    fn TryGetPointerPose(&self, coordinatesystem: &::core::option::Option<super::super::super::Perception::Spatial::SpatialCoordinateSystem>) -> ::windows::core::Result<SpatialPointerPose>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialManipulationUpdatedEventArgsImpl: Sized {
    fn InteractionSourceKind(&self) -> ::windows::core::Result<SpatialInteractionSourceKind>;
    fn TryGetCumulativeDelta(&self, coordinatesystem: &::core::option::Option<super::super::super::Perception::Spatial::SpatialCoordinateSystem>) -> ::windows::core::Result<SpatialManipulationDelta>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialNavigationCanceledEventArgsImpl: Sized {
    fn InteractionSourceKind(&self) -> ::windows::core::Result<SpatialInteractionSourceKind>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialNavigationCompletedEventArgsImpl: Sized {
    fn InteractionSourceKind(&self) -> ::windows::core::Result<SpatialInteractionSourceKind>;
    fn NormalizedOffset(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialNavigationStartedEventArgsImpl: Sized {
    fn InteractionSourceKind(&self) -> ::windows::core::Result<SpatialInteractionSourceKind>;
    fn TryGetPointerPose(&self, coordinatesystem: &::core::option::Option<super::super::super::Perception::Spatial::SpatialCoordinateSystem>) -> ::windows::core::Result<SpatialPointerPose>;
    fn IsNavigatingX(&self) -> ::windows::core::Result<bool>;
    fn IsNavigatingY(&self) -> ::windows::core::Result<bool>;
    fn IsNavigatingZ(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialNavigationUpdatedEventArgsImpl: Sized {
    fn InteractionSourceKind(&self) -> ::windows::core::Result<SpatialInteractionSourceKind>;
    fn NormalizedOffset(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialPointerInteractionSourcePoseImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
    fn ForwardDirection(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
    fn UpDirection(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialPointerInteractionSourcePose2Impl: Sized + ISpatialPointerInteractionSourcePoseImpl {
    fn Orientation(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Quaternion>;
    fn PositionAccuracy(&self) -> ::windows::core::Result<SpatialInteractionSourcePositionAccuracy>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialPointerPoseImpl: Sized {
    fn Timestamp(&self) -> ::windows::core::Result<super::super::super::Perception::PerceptionTimestamp>;
    fn Head(&self) -> ::windows::core::Result<super::super::super::Perception::People::HeadPose>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialPointerPose2Impl: Sized + ISpatialPointerPoseImpl {
    fn TryGetInteractionSourcePose(&self, source: &::core::option::Option<SpatialInteractionSource>) -> ::windows::core::Result<SpatialPointerInteractionSourcePose>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialPointerPose3Impl: Sized {
    fn Eyes(&self) -> ::windows::core::Result<super::super::super::Perception::People::EyesPose>;
    fn IsHeadCapturedBySystem(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialPointerPoseStaticsImpl: Sized {
    fn TryGetAtTimestamp(&self, coordinatesystem: &::core::option::Option<super::super::super::Perception::Spatial::SpatialCoordinateSystem>, timestamp: &::core::option::Option<super::super::super::Perception::PerceptionTimestamp>) -> ::windows::core::Result<SpatialPointerPose>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialRecognitionEndedEventArgsImpl: Sized {
    fn InteractionSourceKind(&self) -> ::windows::core::Result<SpatialInteractionSourceKind>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialRecognitionStartedEventArgsImpl: Sized {
    fn InteractionSourceKind(&self) -> ::windows::core::Result<SpatialInteractionSourceKind>;
    fn TryGetPointerPose(&self, coordinatesystem: &::core::option::Option<super::super::super::Perception::Spatial::SpatialCoordinateSystem>) -> ::windows::core::Result<SpatialPointerPose>;
    fn IsGesturePossible(&self, gesture: SpatialGestureSettings) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialTappedEventArgsImpl: Sized {
    fn InteractionSourceKind(&self) -> ::windows::core::Result<SpatialInteractionSourceKind>;
    fn TryGetPointerPose(&self, coordinatesystem: &::core::option::Option<super::super::super::Perception::Spatial::SpatialCoordinateSystem>) -> ::windows::core::Result<SpatialPointerPose>;
    fn TapCount(&self) -> ::windows::core::Result<u32>;
}
