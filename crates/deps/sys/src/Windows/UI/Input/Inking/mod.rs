#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "UI_Input_Inking_Analysis")]
pub mod Analysis;
#[cfg(feature = "UI_Input_Inking_Core")]
pub mod Core;
#[cfg(feature = "UI_Input_Inking_Preview")]
pub mod Preview;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct HandwritingLineHeight(pub i32);
impl HandwritingLineHeight {
    pub const Small: HandwritingLineHeight = HandwritingLineHeight(0i32);
    pub const Medium: HandwritingLineHeight = HandwritingLineHeight(1i32);
    pub const Large: HandwritingLineHeight = HandwritingLineHeight(2i32);
}
#[repr(transparent)]
pub struct IInkDrawingAttributes(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkDrawingAttributes2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkDrawingAttributes3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkDrawingAttributes4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkDrawingAttributes5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkDrawingAttributesPencilProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkDrawingAttributesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkInputConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkInputConfiguration2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkInputProcessingConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkModelerAttributes(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkModelerAttributes2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkPoint(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkPoint2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkPointFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkPointFactory2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkPresenter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkPresenter2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkPresenter3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkPresenterProtractor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkPresenterProtractorFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkPresenterRuler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkPresenterRuler2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkPresenterRulerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkPresenterStencil(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkRecognitionResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkRecognizer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkRecognizerContainer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkStroke(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkStroke2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkStroke3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkStroke4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkStrokeBuilder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkStrokeBuilder2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkStrokeBuilder3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkStrokeContainer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkStrokeContainer2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkStrokeContainer3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkStrokeInput(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkStrokeRenderingSegment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkStrokesCollectedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkStrokesErasedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkSynchronizer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkUnprocessedInput(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPenAndInkSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPenAndInkSettings2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPenAndInkSettingsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkDrawingAttributes(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkDrawingAttributesKind(pub i32);
impl InkDrawingAttributesKind {
    pub const Default: InkDrawingAttributesKind = InkDrawingAttributesKind(0i32);
    pub const Pencil: InkDrawingAttributesKind = InkDrawingAttributesKind(1i32);
}
#[repr(transparent)]
pub struct InkDrawingAttributesPencilProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkHighContrastAdjustment(pub i32);
impl InkHighContrastAdjustment {
    pub const UseSystemColorsWhenNecessary: InkHighContrastAdjustment = InkHighContrastAdjustment(0i32);
    pub const UseSystemColors: InkHighContrastAdjustment = InkHighContrastAdjustment(1i32);
    pub const UseOriginalColors: InkHighContrastAdjustment = InkHighContrastAdjustment(2i32);
}
#[repr(transparent)]
pub struct InkInputConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkInputProcessingConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkInputProcessingMode(pub i32);
impl InkInputProcessingMode {
    pub const None: InkInputProcessingMode = InkInputProcessingMode(0i32);
    pub const Inking: InkInputProcessingMode = InkInputProcessingMode(1i32);
    pub const Erasing: InkInputProcessingMode = InkInputProcessingMode(2i32);
}
#[repr(transparent)]
pub struct InkInputRightDragAction(pub i32);
impl InkInputRightDragAction {
    pub const LeaveUnprocessed: InkInputRightDragAction = InkInputRightDragAction(0i32);
    pub const AllowProcessing: InkInputRightDragAction = InkInputRightDragAction(1i32);
}
#[repr(transparent)]
pub struct InkManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkManipulationMode(pub i32);
impl InkManipulationMode {
    pub const Inking: InkManipulationMode = InkManipulationMode(0i32);
    pub const Erasing: InkManipulationMode = InkManipulationMode(1i32);
    pub const Selecting: InkManipulationMode = InkManipulationMode(2i32);
}
#[repr(transparent)]
pub struct InkModelerAttributes(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkPersistenceFormat(pub i32);
impl InkPersistenceFormat {
    pub const GifWithEmbeddedIsf: InkPersistenceFormat = InkPersistenceFormat(0i32);
    pub const Isf: InkPersistenceFormat = InkPersistenceFormat(1i32);
}
#[repr(transparent)]
pub struct InkPoint(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkPresenter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkPresenterPredefinedConfiguration(pub i32);
impl InkPresenterPredefinedConfiguration {
    pub const SimpleSinglePointer: InkPresenterPredefinedConfiguration = InkPresenterPredefinedConfiguration(0i32);
    pub const SimpleMultiplePointer: InkPresenterPredefinedConfiguration = InkPresenterPredefinedConfiguration(1i32);
}
#[repr(transparent)]
pub struct InkPresenterProtractor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkPresenterRuler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkPresenterStencilKind(pub i32);
impl InkPresenterStencilKind {
    pub const Other: InkPresenterStencilKind = InkPresenterStencilKind(0i32);
    pub const Ruler: InkPresenterStencilKind = InkPresenterStencilKind(1i32);
    pub const Protractor: InkPresenterStencilKind = InkPresenterStencilKind(2i32);
}
#[repr(transparent)]
pub struct InkRecognitionResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkRecognitionTarget(pub i32);
impl InkRecognitionTarget {
    pub const All: InkRecognitionTarget = InkRecognitionTarget(0i32);
    pub const Selected: InkRecognitionTarget = InkRecognitionTarget(1i32);
    pub const Recent: InkRecognitionTarget = InkRecognitionTarget(2i32);
}
#[repr(transparent)]
pub struct InkRecognizer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkRecognizerContainer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkStroke(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkStrokeBuilder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkStrokeContainer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkStrokeInput(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkStrokeRenderingSegment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkStrokesCollectedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkStrokesErasedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkSynchronizer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkUnprocessedInput(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PenAndInkSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PenHandedness(pub i32);
impl PenHandedness {
    pub const Right: PenHandedness = PenHandedness(0i32);
    pub const Left: PenHandedness = PenHandedness(1i32);
}
#[repr(transparent)]
pub struct PenTipShape(pub i32);
impl PenTipShape {
    pub const Circle: PenTipShape = PenTipShape(0i32);
    pub const Rectangle: PenTipShape = PenTipShape(1i32);
}
