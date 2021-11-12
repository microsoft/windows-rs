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
    pub const Small: Self = Self(0i32);
    pub const Medium: Self = Self(1i32);
    pub const Large: Self = Self(2i32);
}
impl ::core::marker::Copy for HandwritingLineHeight {}
impl ::core::clone::Clone for HandwritingLineHeight {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkDrawingAttributes(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkDrawingAttributes {}
impl ::core::clone::Clone for IInkDrawingAttributes {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkDrawingAttributes2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkDrawingAttributes2 {}
impl ::core::clone::Clone for IInkDrawingAttributes2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkDrawingAttributes3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkDrawingAttributes3 {}
impl ::core::clone::Clone for IInkDrawingAttributes3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkDrawingAttributes4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkDrawingAttributes4 {}
impl ::core::clone::Clone for IInkDrawingAttributes4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkDrawingAttributes5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkDrawingAttributes5 {}
impl ::core::clone::Clone for IInkDrawingAttributes5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkDrawingAttributesPencilProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkDrawingAttributesPencilProperties {}
impl ::core::clone::Clone for IInkDrawingAttributesPencilProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkDrawingAttributesStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkDrawingAttributesStatics {}
impl ::core::clone::Clone for IInkDrawingAttributesStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkInputConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkInputConfiguration {}
impl ::core::clone::Clone for IInkInputConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkInputConfiguration2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkInputConfiguration2 {}
impl ::core::clone::Clone for IInkInputConfiguration2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkInputProcessingConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkInputProcessingConfiguration {}
impl ::core::clone::Clone for IInkInputProcessingConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkManager {}
impl ::core::clone::Clone for IInkManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkModelerAttributes(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkModelerAttributes {}
impl ::core::clone::Clone for IInkModelerAttributes {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkModelerAttributes2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkModelerAttributes2 {}
impl ::core::clone::Clone for IInkModelerAttributes2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkPoint(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkPoint {}
impl ::core::clone::Clone for IInkPoint {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkPoint2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkPoint2 {}
impl ::core::clone::Clone for IInkPoint2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkPointFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkPointFactory {}
impl ::core::clone::Clone for IInkPointFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkPointFactory2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkPointFactory2 {}
impl ::core::clone::Clone for IInkPointFactory2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkPresenter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkPresenter {}
impl ::core::clone::Clone for IInkPresenter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkPresenter2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkPresenter2 {}
impl ::core::clone::Clone for IInkPresenter2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkPresenter3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkPresenter3 {}
impl ::core::clone::Clone for IInkPresenter3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkPresenterProtractor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkPresenterProtractor {}
impl ::core::clone::Clone for IInkPresenterProtractor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkPresenterProtractorFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkPresenterProtractorFactory {}
impl ::core::clone::Clone for IInkPresenterProtractorFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkPresenterRuler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkPresenterRuler {}
impl ::core::clone::Clone for IInkPresenterRuler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkPresenterRuler2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkPresenterRuler2 {}
impl ::core::clone::Clone for IInkPresenterRuler2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkPresenterRulerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkPresenterRulerFactory {}
impl ::core::clone::Clone for IInkPresenterRulerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkPresenterStencil(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkPresenterStencil {}
impl ::core::clone::Clone for IInkPresenterStencil {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkRecognitionResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkRecognitionResult {}
impl ::core::clone::Clone for IInkRecognitionResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkRecognizer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkRecognizer {}
impl ::core::clone::Clone for IInkRecognizer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkRecognizerContainer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkRecognizerContainer {}
impl ::core::clone::Clone for IInkRecognizerContainer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkStroke(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkStroke {}
impl ::core::clone::Clone for IInkStroke {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkStroke2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkStroke2 {}
impl ::core::clone::Clone for IInkStroke2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkStroke3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkStroke3 {}
impl ::core::clone::Clone for IInkStroke3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkStroke4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkStroke4 {}
impl ::core::clone::Clone for IInkStroke4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkStrokeBuilder(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkStrokeBuilder {}
impl ::core::clone::Clone for IInkStrokeBuilder {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkStrokeBuilder2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkStrokeBuilder2 {}
impl ::core::clone::Clone for IInkStrokeBuilder2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkStrokeBuilder3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkStrokeBuilder3 {}
impl ::core::clone::Clone for IInkStrokeBuilder3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkStrokeContainer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkStrokeContainer {}
impl ::core::clone::Clone for IInkStrokeContainer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkStrokeContainer2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkStrokeContainer2 {}
impl ::core::clone::Clone for IInkStrokeContainer2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkStrokeContainer3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkStrokeContainer3 {}
impl ::core::clone::Clone for IInkStrokeContainer3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkStrokeInput(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkStrokeInput {}
impl ::core::clone::Clone for IInkStrokeInput {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkStrokeRenderingSegment(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkStrokeRenderingSegment {}
impl ::core::clone::Clone for IInkStrokeRenderingSegment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkStrokesCollectedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkStrokesCollectedEventArgs {}
impl ::core::clone::Clone for IInkStrokesCollectedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkStrokesErasedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkStrokesErasedEventArgs {}
impl ::core::clone::Clone for IInkStrokesErasedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkSynchronizer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkSynchronizer {}
impl ::core::clone::Clone for IInkSynchronizer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkUnprocessedInput(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkUnprocessedInput {}
impl ::core::clone::Clone for IInkUnprocessedInput {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPenAndInkSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPenAndInkSettings {}
impl ::core::clone::Clone for IPenAndInkSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPenAndInkSettings2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPenAndInkSettings2 {}
impl ::core::clone::Clone for IPenAndInkSettings2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPenAndInkSettingsStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPenAndInkSettingsStatics {}
impl ::core::clone::Clone for IPenAndInkSettingsStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkDrawingAttributes(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for InkDrawingAttributes {}
impl ::core::clone::Clone for InkDrawingAttributes {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkDrawingAttributesKind(pub i32);
impl InkDrawingAttributesKind {
    pub const Default: Self = Self(0i32);
    pub const Pencil: Self = Self(1i32);
}
impl ::core::marker::Copy for InkDrawingAttributesKind {}
impl ::core::clone::Clone for InkDrawingAttributesKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkDrawingAttributesPencilProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for InkDrawingAttributesPencilProperties {}
impl ::core::clone::Clone for InkDrawingAttributesPencilProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkHighContrastAdjustment(pub i32);
impl InkHighContrastAdjustment {
    pub const UseSystemColorsWhenNecessary: Self = Self(0i32);
    pub const UseSystemColors: Self = Self(1i32);
    pub const UseOriginalColors: Self = Self(2i32);
}
impl ::core::marker::Copy for InkHighContrastAdjustment {}
impl ::core::clone::Clone for InkHighContrastAdjustment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkInputConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for InkInputConfiguration {}
impl ::core::clone::Clone for InkInputConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkInputProcessingConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for InkInputProcessingConfiguration {}
impl ::core::clone::Clone for InkInputProcessingConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkInputProcessingMode(pub i32);
impl InkInputProcessingMode {
    pub const None: Self = Self(0i32);
    pub const Inking: Self = Self(1i32);
    pub const Erasing: Self = Self(2i32);
}
impl ::core::marker::Copy for InkInputProcessingMode {}
impl ::core::clone::Clone for InkInputProcessingMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkInputRightDragAction(pub i32);
impl InkInputRightDragAction {
    pub const LeaveUnprocessed: Self = Self(0i32);
    pub const AllowProcessing: Self = Self(1i32);
}
impl ::core::marker::Copy for InkInputRightDragAction {}
impl ::core::clone::Clone for InkInputRightDragAction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for InkManager {}
impl ::core::clone::Clone for InkManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkManipulationMode(pub i32);
impl InkManipulationMode {
    pub const Inking: Self = Self(0i32);
    pub const Erasing: Self = Self(1i32);
    pub const Selecting: Self = Self(2i32);
}
impl ::core::marker::Copy for InkManipulationMode {}
impl ::core::clone::Clone for InkManipulationMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkModelerAttributes(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for InkModelerAttributes {}
impl ::core::clone::Clone for InkModelerAttributes {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkPersistenceFormat(pub i32);
impl InkPersistenceFormat {
    pub const GifWithEmbeddedIsf: Self = Self(0i32);
    pub const Isf: Self = Self(1i32);
}
impl ::core::marker::Copy for InkPersistenceFormat {}
impl ::core::clone::Clone for InkPersistenceFormat {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkPoint(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for InkPoint {}
impl ::core::clone::Clone for InkPoint {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkPresenter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for InkPresenter {}
impl ::core::clone::Clone for InkPresenter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkPresenterPredefinedConfiguration(pub i32);
impl InkPresenterPredefinedConfiguration {
    pub const SimpleSinglePointer: Self = Self(0i32);
    pub const SimpleMultiplePointer: Self = Self(1i32);
}
impl ::core::marker::Copy for InkPresenterPredefinedConfiguration {}
impl ::core::clone::Clone for InkPresenterPredefinedConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkPresenterProtractor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for InkPresenterProtractor {}
impl ::core::clone::Clone for InkPresenterProtractor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkPresenterRuler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for InkPresenterRuler {}
impl ::core::clone::Clone for InkPresenterRuler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkPresenterStencilKind(pub i32);
impl InkPresenterStencilKind {
    pub const Other: Self = Self(0i32);
    pub const Ruler: Self = Self(1i32);
    pub const Protractor: Self = Self(2i32);
}
impl ::core::marker::Copy for InkPresenterStencilKind {}
impl ::core::clone::Clone for InkPresenterStencilKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkRecognitionResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for InkRecognitionResult {}
impl ::core::clone::Clone for InkRecognitionResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkRecognitionTarget(pub i32);
impl InkRecognitionTarget {
    pub const All: Self = Self(0i32);
    pub const Selected: Self = Self(1i32);
    pub const Recent: Self = Self(2i32);
}
impl ::core::marker::Copy for InkRecognitionTarget {}
impl ::core::clone::Clone for InkRecognitionTarget {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkRecognizer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for InkRecognizer {}
impl ::core::clone::Clone for InkRecognizer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkRecognizerContainer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for InkRecognizerContainer {}
impl ::core::clone::Clone for InkRecognizerContainer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkStroke(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for InkStroke {}
impl ::core::clone::Clone for InkStroke {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkStrokeBuilder(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for InkStrokeBuilder {}
impl ::core::clone::Clone for InkStrokeBuilder {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkStrokeContainer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for InkStrokeContainer {}
impl ::core::clone::Clone for InkStrokeContainer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkStrokeInput(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for InkStrokeInput {}
impl ::core::clone::Clone for InkStrokeInput {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkStrokeRenderingSegment(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for InkStrokeRenderingSegment {}
impl ::core::clone::Clone for InkStrokeRenderingSegment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkStrokesCollectedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for InkStrokesCollectedEventArgs {}
impl ::core::clone::Clone for InkStrokesCollectedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkStrokesErasedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for InkStrokesErasedEventArgs {}
impl ::core::clone::Clone for InkStrokesErasedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkSynchronizer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for InkSynchronizer {}
impl ::core::clone::Clone for InkSynchronizer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkUnprocessedInput(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for InkUnprocessedInput {}
impl ::core::clone::Clone for InkUnprocessedInput {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PenAndInkSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PenAndInkSettings {}
impl ::core::clone::Clone for PenAndInkSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PenHandedness(pub i32);
impl PenHandedness {
    pub const Right: Self = Self(0i32);
    pub const Left: Self = Self(1i32);
}
impl ::core::marker::Copy for PenHandedness {}
impl ::core::clone::Clone for PenHandedness {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PenTipShape(pub i32);
impl PenTipShape {
    pub const Circle: Self = Self(0i32);
    pub const Rectangle: Self = Self(1i32);
}
impl ::core::marker::Copy for PenTipShape {}
impl ::core::clone::Clone for PenTipShape {
    fn clone(&self) -> Self {
        *self
    }
}
