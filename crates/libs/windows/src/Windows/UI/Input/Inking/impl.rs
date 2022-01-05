#[cfg(feature = "implement_exclusive")]
pub trait IInkDrawingAttributesImpl: Sized {
    fn Color(&self) -> ::windows::core::Result<super::super::Color>;
    fn SetColor(&self, value: &super::super::Color) -> ::windows::core::Result<()>;
    fn PenTip(&self) -> ::windows::core::Result<PenTipShape>;
    fn SetPenTip(&self, value: PenTipShape) -> ::windows::core::Result<()>;
    fn Size(&self) -> ::windows::core::Result<super::super::super::Foundation::Size>;
    fn SetSize(&self, value: &super::super::super::Foundation::Size) -> ::windows::core::Result<()>;
    fn IgnorePressure(&self) -> ::windows::core::Result<bool>;
    fn SetIgnorePressure(&self, value: bool) -> ::windows::core::Result<()>;
    fn FitToCurve(&self) -> ::windows::core::Result<bool>;
    fn SetFitToCurve(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkDrawingAttributes2Impl: Sized {
    fn PenTipTransform(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Matrix3x2>;
    fn SetPenTipTransform(&self, value: &super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()>;
    fn DrawAsHighlighter(&self) -> ::windows::core::Result<bool>;
    fn SetDrawAsHighlighter(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkDrawingAttributes3Impl: Sized {
    fn Kind(&self) -> ::windows::core::Result<InkDrawingAttributesKind>;
    fn PencilProperties(&self) -> ::windows::core::Result<InkDrawingAttributesPencilProperties>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkDrawingAttributes4Impl: Sized {
    fn IgnoreTilt(&self) -> ::windows::core::Result<bool>;
    fn SetIgnoreTilt(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkDrawingAttributes5Impl: Sized {
    fn ModelerAttributes(&self) -> ::windows::core::Result<InkModelerAttributes>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkDrawingAttributesPencilPropertiesImpl: Sized {
    fn Opacity(&self) -> ::windows::core::Result<f64>;
    fn SetOpacity(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkDrawingAttributesStaticsImpl: Sized {
    fn CreateForPencil(&self) -> ::windows::core::Result<InkDrawingAttributes>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkInputConfigurationImpl: Sized {
    fn IsPrimaryBarrelButtonInputEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsPrimaryBarrelButtonInputEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsEraserInputEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsEraserInputEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkInputConfiguration2Impl: Sized {
    fn IsPenHapticFeedbackEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsPenHapticFeedbackEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkInputProcessingConfigurationImpl: Sized {
    fn Mode(&self) -> ::windows::core::Result<InkInputProcessingMode>;
    fn SetMode(&self, value: InkInputProcessingMode) -> ::windows::core::Result<()>;
    fn RightDragAction(&self) -> ::windows::core::Result<InkInputRightDragAction>;
    fn SetRightDragAction(&self, value: InkInputRightDragAction) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkManagerImpl: Sized + IInkRecognizerContainerImpl + IInkStrokeContainerImpl {
    fn Mode(&self) -> ::windows::core::Result<InkManipulationMode>;
    fn SetMode(&self, value: InkManipulationMode) -> ::windows::core::Result<()>;
    fn ProcessPointerDown(&self, pointerpoint: &::core::option::Option<super::PointerPoint>) -> ::windows::core::Result<()>;
    fn ProcessPointerUpdate(&self, pointerpoint: &::core::option::Option<super::PointerPoint>) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn ProcessPointerUp(&self, pointerpoint: &::core::option::Option<super::PointerPoint>) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
    fn SetDefaultDrawingAttributes(&self, drawingattributes: &::core::option::Option<InkDrawingAttributes>) -> ::windows::core::Result<()>;
    fn RecognizeAsync2(&self, recognitiontarget: InkRecognitionTarget) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkModelerAttributesImpl: Sized {
    fn PredictionTime(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetPredictionTime(&self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn ScalingFactor(&self) -> ::windows::core::Result<f32>;
    fn SetScalingFactor(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkModelerAttributes2Impl: Sized {
    fn UseVelocityBasedPressure(&self) -> ::windows::core::Result<bool>;
    fn SetUseVelocityBasedPressure(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkPointImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn Pressure(&self) -> ::windows::core::Result<f32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkPoint2Impl: Sized {
    fn TiltX(&self) -> ::windows::core::Result<f32>;
    fn TiltY(&self) -> ::windows::core::Result<f32>;
    fn Timestamp(&self) -> ::windows::core::Result<u64>;
}
pub trait IInkPointFactoryImpl: Sized {
    fn CreateInkPoint(&self, position: &super::super::super::Foundation::Point, pressure: f32) -> ::windows::core::Result<InkPoint>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkPointFactory2Impl: Sized {
    fn CreateInkPointWithTiltAndTimestamp(&self, position: &super::super::super::Foundation::Point, pressure: f32, tiltx: f32, tilty: f32, timestamp: u64) -> ::windows::core::Result<InkPoint>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkPresenterImpl: Sized {
    fn IsInputEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsInputEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn InputDeviceTypes(&self) -> ::windows::core::Result<super::super::Core::CoreInputDeviceTypes>;
    fn SetInputDeviceTypes(&self, value: super::super::Core::CoreInputDeviceTypes) -> ::windows::core::Result<()>;
    fn UnprocessedInput(&self) -> ::windows::core::Result<InkUnprocessedInput>;
    fn StrokeInput(&self) -> ::windows::core::Result<InkStrokeInput>;
    fn InputProcessingConfiguration(&self) -> ::windows::core::Result<InkInputProcessingConfiguration>;
    fn StrokeContainer(&self) -> ::windows::core::Result<InkStrokeContainer>;
    fn SetStrokeContainer(&self, value: &::core::option::Option<InkStrokeContainer>) -> ::windows::core::Result<()>;
    fn CopyDefaultDrawingAttributes(&self) -> ::windows::core::Result<InkDrawingAttributes>;
    fn UpdateDefaultDrawingAttributes(&self, value: &::core::option::Option<InkDrawingAttributes>) -> ::windows::core::Result<()>;
    fn ActivateCustomDrying(&self) -> ::windows::core::Result<InkSynchronizer>;
    fn SetPredefinedConfiguration(&self, value: InkPresenterPredefinedConfiguration) -> ::windows::core::Result<()>;
    fn StrokesCollected(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<InkPresenter, InkStrokesCollectedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveStrokesCollected(&self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StrokesErased(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<InkPresenter, InkStrokesErasedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveStrokesErased(&self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkPresenter2Impl: Sized + IInkPresenterImpl {
    fn HighContrastAdjustment(&self) -> ::windows::core::Result<InkHighContrastAdjustment>;
    fn SetHighContrastAdjustment(&self, value: InkHighContrastAdjustment) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkPresenter3Impl: Sized {
    fn InputConfiguration(&self) -> ::windows::core::Result<InkInputConfiguration>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkPresenterProtractorImpl: Sized + IInkPresenterStencilImpl {
    fn AreTickMarksVisible(&self) -> ::windows::core::Result<bool>;
    fn SetAreTickMarksVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn AreRaysVisible(&self) -> ::windows::core::Result<bool>;
    fn SetAreRaysVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsCenterMarkerVisible(&self) -> ::windows::core::Result<bool>;
    fn SetIsCenterMarkerVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsAngleReadoutVisible(&self) -> ::windows::core::Result<bool>;
    fn SetIsAngleReadoutVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsResizable(&self) -> ::windows::core::Result<bool>;
    fn SetIsResizable(&self, value: bool) -> ::windows::core::Result<()>;
    fn Radius(&self) -> ::windows::core::Result<f64>;
    fn SetRadius(&self, value: f64) -> ::windows::core::Result<()>;
    fn AccentColor(&self) -> ::windows::core::Result<super::super::Color>;
    fn SetAccentColor(&self, value: &super::super::Color) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkPresenterProtractorFactoryImpl: Sized {
    fn Create(&self, inkpresenter: &::core::option::Option<InkPresenter>) -> ::windows::core::Result<InkPresenterProtractor>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkPresenterRulerImpl: Sized + IInkPresenterStencilImpl {
    fn Length(&self) -> ::windows::core::Result<f64>;
    fn SetLength(&self, value: f64) -> ::windows::core::Result<()>;
    fn Width(&self) -> ::windows::core::Result<f64>;
    fn SetWidth(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkPresenterRuler2Impl: Sized {
    fn AreTickMarksVisible(&self) -> ::windows::core::Result<bool>;
    fn SetAreTickMarksVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsCompassVisible(&self) -> ::windows::core::Result<bool>;
    fn SetIsCompassVisible(&self, value: bool) -> ::windows::core::Result<()>;
}
pub trait IInkPresenterRulerFactoryImpl: Sized {
    fn Create(&self, inkpresenter: &::core::option::Option<InkPresenter>) -> ::windows::core::Result<InkPresenterRuler>;
}
pub trait IInkPresenterStencilImpl: Sized {
    fn Kind(&self) -> ::windows::core::Result<InkPresenterStencilKind>;
    fn IsVisible(&self) -> ::windows::core::Result<bool>;
    fn SetIsVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn BackgroundColor(&self) -> ::windows::core::Result<super::super::Color>;
    fn SetBackgroundColor(&self, value: &super::super::Color) -> ::windows::core::Result<()>;
    fn ForegroundColor(&self) -> ::windows::core::Result<super::super::Color>;
    fn SetForegroundColor(&self, value: &super::super::Color) -> ::windows::core::Result<()>;
    fn Transform(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Matrix3x2>;
    fn SetTransform(&self, value: &super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkRecognitionResultImpl: Sized {
    fn BoundingRect(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
    fn GetTextCandidates(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn GetStrokes(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkStroke>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkRecognizerImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
pub trait IInkRecognizerContainerImpl: Sized {
    fn SetDefaultRecognizer(&self, recognizer: &::core::option::Option<InkRecognizer>) -> ::windows::core::Result<()>;
    fn RecognizeAsync(&self, strokecollection: &::core::option::Option<InkStrokeContainer>, recognitiontarget: InkRecognitionTarget) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>>;
    fn GetRecognizers(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkRecognizer>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkStrokeImpl: Sized {
    fn DrawingAttributes(&self) -> ::windows::core::Result<InkDrawingAttributes>;
    fn SetDrawingAttributes(&self, value: &::core::option::Option<InkDrawingAttributes>) -> ::windows::core::Result<()>;
    fn BoundingRect(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
    fn Selected(&self) -> ::windows::core::Result<bool>;
    fn SetSelected(&self, value: bool) -> ::windows::core::Result<()>;
    fn Recognized(&self) -> ::windows::core::Result<bool>;
    fn GetRenderingSegments(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkStrokeRenderingSegment>>;
    fn Clone(&self) -> ::windows::core::Result<InkStroke>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkStroke2Impl: Sized {
    fn PointTransform(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Matrix3x2>;
    fn SetPointTransform(&self, value: &super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()>;
    fn GetInkPoints(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkPoint>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkStroke3Impl: Sized {
    fn Id(&self) -> ::windows::core::Result<u32>;
    fn StrokeStartedTime(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn SetStrokeStartedTime(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn StrokeDuration(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn SetStrokeDuration(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkStroke4Impl: Sized {
    fn PointerId(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkStrokeBuilderImpl: Sized {
    fn BeginStroke(&self, pointerpoint: &::core::option::Option<super::PointerPoint>) -> ::windows::core::Result<()>;
    fn AppendToStroke(&self, pointerpoint: &::core::option::Option<super::PointerPoint>) -> ::windows::core::Result<super::PointerPoint>;
    fn EndStroke(&self, pointerpoint: &::core::option::Option<super::PointerPoint>) -> ::windows::core::Result<InkStroke>;
    fn CreateStroke(&self, points: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Point>>) -> ::windows::core::Result<InkStroke>;
    fn SetDefaultDrawingAttributes(&self, drawingattributes: &::core::option::Option<InkDrawingAttributes>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkStrokeBuilder2Impl: Sized {
    fn CreateStrokeFromInkPoints(&self, inkpoints: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<InkPoint>>, transform: &super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<InkStroke>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkStrokeBuilder3Impl: Sized {
    fn CreateStrokeFromInkPoints(&self, inkpoints: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<InkPoint>>, transform: &super::super::super::Foundation::Numerics::Matrix3x2, strokestartedtime: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>, strokeduration: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<InkStroke>;
}
pub trait IInkStrokeContainerImpl: Sized {
    fn BoundingRect(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
    fn AddStroke(&self, stroke: &::core::option::Option<InkStroke>) -> ::windows::core::Result<()>;
    fn DeleteSelected(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
    fn MoveSelected(&self, translation: &super::super::super::Foundation::Point) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
    fn SelectWithPolyLine(&self, polyline: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Point>>) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
    fn SelectWithLine(&self, from: &super::super::super::Foundation::Point, to: &super::super::super::Foundation::Point) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
    fn CopySelectedToClipboard(&self) -> ::windows::core::Result<()>;
    fn PasteFromClipboard(&self, position: &super::super::super::Foundation::Point) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
    fn CanPasteFromClipboard(&self) -> ::windows::core::Result<bool>;
    fn LoadAsync(&self, inputstream: &::core::option::Option<super::super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncActionWithProgress<u64>>;
    fn SaveAsync(&self, outputstream: &::core::option::Option<super::super::super::Storage::Streams::IOutputStream>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>;
    fn UpdateRecognitionResults(&self, recognitionresults: &::core::option::Option<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>) -> ::windows::core::Result<()>;
    fn GetStrokes(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkStroke>>;
    fn GetRecognitionResults(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkStrokeContainer2Impl: Sized {
    fn AddStrokes(&self, strokes: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<InkStroke>>) -> ::windows::core::Result<()>;
    fn Clear(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkStrokeContainer3Impl: Sized {
    fn SaveWithFormatAsync(&self, outputstream: &::core::option::Option<super::super::super::Storage::Streams::IOutputStream>, inkpersistenceformat: InkPersistenceFormat) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>;
    fn GetStrokeById(&self, id: u32) -> ::windows::core::Result<InkStroke>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkStrokeInputImpl: Sized {
    fn StrokeStarted(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<InkStrokeInput, super::super::Core::PointerEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveStrokeStarted(&self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StrokeContinued(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<InkStrokeInput, super::super::Core::PointerEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveStrokeContinued(&self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StrokeEnded(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<InkStrokeInput, super::super::Core::PointerEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveStrokeEnded(&self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StrokeCanceled(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<InkStrokeInput, super::super::Core::PointerEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveStrokeCanceled(&self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn InkPresenter(&self) -> ::windows::core::Result<InkPresenter>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkStrokeRenderingSegmentImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn BezierControlPoint1(&self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn BezierControlPoint2(&self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn Pressure(&self) -> ::windows::core::Result<f32>;
    fn TiltX(&self) -> ::windows::core::Result<f32>;
    fn TiltY(&self) -> ::windows::core::Result<f32>;
    fn Twist(&self) -> ::windows::core::Result<f32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkStrokesCollectedEventArgsImpl: Sized {
    fn Strokes(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkStroke>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkStrokesErasedEventArgsImpl: Sized {
    fn Strokes(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkStroke>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkSynchronizerImpl: Sized {
    fn BeginDry(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkStroke>>;
    fn EndDry(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkUnprocessedInputImpl: Sized {
    fn PointerEntered(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerEntered(&self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerHovered(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerHovered(&self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerExited(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerExited(&self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerPressed(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerPressed(&self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerMoved(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerMoved(&self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerReleased(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerReleased(&self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerLost(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerLost(&self, cookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn InkPresenter(&self) -> ::windows::core::Result<InkPresenter>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPenAndInkSettingsImpl: Sized {
    fn IsHandwritingDirectlyIntoTextFieldEnabled(&self) -> ::windows::core::Result<bool>;
    fn PenHandedness(&self) -> ::windows::core::Result<PenHandedness>;
    fn HandwritingLineHeight(&self) -> ::windows::core::Result<HandwritingLineHeight>;
    fn FontFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn UserConsentsToHandwritingTelemetryCollection(&self) -> ::windows::core::Result<bool>;
    fn IsTouchHandwritingEnabled(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPenAndInkSettings2Impl: Sized {
    fn SetPenHandedness(&self, value: PenHandedness) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPenAndInkSettingsStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<PenAndInkSettings>;
}
