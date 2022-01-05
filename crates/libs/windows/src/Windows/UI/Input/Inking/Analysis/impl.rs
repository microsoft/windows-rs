#[cfg(feature = "implement_exclusive")]
pub trait IInkAnalysisInkBulletImpl: Sized + IInkAnalysisNodeImpl {
    fn RecognizedText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkAnalysisInkDrawingImpl: Sized + IInkAnalysisNodeImpl {
    fn DrawingKind(&self) -> ::windows::core::Result<InkAnalysisDrawingKind>;
    fn Center(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn Points(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkAnalysisInkWordImpl: Sized + IInkAnalysisNodeImpl {
    fn RecognizedText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TextAlternates(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkAnalysisLineImpl: Sized + IInkAnalysisNodeImpl {
    fn RecognizedText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IndentLevel(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkAnalysisListItemImpl: Sized + IInkAnalysisNodeImpl {
    fn RecognizedText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
pub trait IInkAnalysisNodeImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<u32>;
    fn Kind(&self) -> ::windows::core::Result<InkAnalysisNodeKind>;
    fn BoundingRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect>;
    fn RotatedBoundingRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>>;
    fn Children(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>>;
    fn Parent(&self) -> ::windows::core::Result<IInkAnalysisNode>;
    fn GetStrokeIds(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<u32>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkAnalysisParagraphImpl: Sized + IInkAnalysisNodeImpl {
    fn RecognizedText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkAnalysisResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<InkAnalysisStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkAnalysisRootImpl: Sized + IInkAnalysisNodeImpl {
    fn RecognizedText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FindNodes(&self, nodekind: InkAnalysisNodeKind) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkAnalysisWritingRegionImpl: Sized + IInkAnalysisNodeImpl {
    fn RecognizedText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkAnalyzerImpl: Sized {
    fn AnalysisRoot(&self) -> ::windows::core::Result<InkAnalysisRoot>;
    fn IsAnalyzing(&self) -> ::windows::core::Result<bool>;
    fn AddDataForStroke(&self, stroke: &::core::option::Option<super::InkStroke>) -> ::windows::core::Result<()>;
    fn AddDataForStrokes(&self, strokes: &::core::option::Option<super::super::super::super::Foundation::Collections::IIterable<super::InkStroke>>) -> ::windows::core::Result<()>;
    fn ClearDataForAllStrokes(&self) -> ::windows::core::Result<()>;
    fn RemoveDataForStroke(&self, strokeid: u32) -> ::windows::core::Result<()>;
    fn RemoveDataForStrokes(&self, strokeids: &::core::option::Option<super::super::super::super::Foundation::Collections::IIterable<u32>>) -> ::windows::core::Result<()>;
    fn ReplaceDataForStroke(&self, stroke: &::core::option::Option<super::InkStroke>) -> ::windows::core::Result<()>;
    fn SetStrokeDataKind(&self, strokeid: u32, strokekind: InkAnalysisStrokeKind) -> ::windows::core::Result<()>;
    fn AnalyzeAsync(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<InkAnalysisResult>>;
}
pub trait IInkAnalyzerFactoryImpl: Sized {
    fn CreateAnalyzer(&self) -> ::windows::core::Result<InkAnalyzer>;
}
