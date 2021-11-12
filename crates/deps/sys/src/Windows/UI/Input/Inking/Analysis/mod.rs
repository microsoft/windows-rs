#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IInkAnalysisInkBullet(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkAnalysisInkDrawing(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkAnalysisInkWord(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkAnalysisLine(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkAnalysisListItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkAnalysisNode(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkAnalysisParagraph(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkAnalysisResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkAnalysisRoot(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkAnalysisWritingRegion(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkAnalyzer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkAnalyzerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkAnalysisDrawingKind(pub i32);
impl InkAnalysisDrawingKind {
    pub const Drawing: InkAnalysisDrawingKind = InkAnalysisDrawingKind(0i32);
    pub const Circle: InkAnalysisDrawingKind = InkAnalysisDrawingKind(1i32);
    pub const Ellipse: InkAnalysisDrawingKind = InkAnalysisDrawingKind(2i32);
    pub const Triangle: InkAnalysisDrawingKind = InkAnalysisDrawingKind(3i32);
    pub const IsoscelesTriangle: InkAnalysisDrawingKind = InkAnalysisDrawingKind(4i32);
    pub const EquilateralTriangle: InkAnalysisDrawingKind = InkAnalysisDrawingKind(5i32);
    pub const RightTriangle: InkAnalysisDrawingKind = InkAnalysisDrawingKind(6i32);
    pub const Quadrilateral: InkAnalysisDrawingKind = InkAnalysisDrawingKind(7i32);
    pub const Rectangle: InkAnalysisDrawingKind = InkAnalysisDrawingKind(8i32);
    pub const Square: InkAnalysisDrawingKind = InkAnalysisDrawingKind(9i32);
    pub const Diamond: InkAnalysisDrawingKind = InkAnalysisDrawingKind(10i32);
    pub const Trapezoid: InkAnalysisDrawingKind = InkAnalysisDrawingKind(11i32);
    pub const Parallelogram: InkAnalysisDrawingKind = InkAnalysisDrawingKind(12i32);
    pub const Pentagon: InkAnalysisDrawingKind = InkAnalysisDrawingKind(13i32);
    pub const Hexagon: InkAnalysisDrawingKind = InkAnalysisDrawingKind(14i32);
}
#[repr(transparent)]
pub struct InkAnalysisInkBullet(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkAnalysisInkDrawing(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkAnalysisInkWord(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkAnalysisLine(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkAnalysisListItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkAnalysisNode(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkAnalysisNodeKind(pub i32);
impl InkAnalysisNodeKind {
    pub const UnclassifiedInk: InkAnalysisNodeKind = InkAnalysisNodeKind(0i32);
    pub const Root: InkAnalysisNodeKind = InkAnalysisNodeKind(1i32);
    pub const WritingRegion: InkAnalysisNodeKind = InkAnalysisNodeKind(2i32);
    pub const Paragraph: InkAnalysisNodeKind = InkAnalysisNodeKind(3i32);
    pub const Line: InkAnalysisNodeKind = InkAnalysisNodeKind(4i32);
    pub const InkWord: InkAnalysisNodeKind = InkAnalysisNodeKind(5i32);
    pub const InkBullet: InkAnalysisNodeKind = InkAnalysisNodeKind(6i32);
    pub const InkDrawing: InkAnalysisNodeKind = InkAnalysisNodeKind(7i32);
    pub const ListItem: InkAnalysisNodeKind = InkAnalysisNodeKind(8i32);
}
#[repr(transparent)]
pub struct InkAnalysisParagraph(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkAnalysisResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkAnalysisRoot(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkAnalysisStatus(pub i32);
impl InkAnalysisStatus {
    pub const Updated: InkAnalysisStatus = InkAnalysisStatus(0i32);
    pub const Unchanged: InkAnalysisStatus = InkAnalysisStatus(1i32);
}
#[repr(transparent)]
pub struct InkAnalysisStrokeKind(pub i32);
impl InkAnalysisStrokeKind {
    pub const Auto: InkAnalysisStrokeKind = InkAnalysisStrokeKind(0i32);
    pub const Writing: InkAnalysisStrokeKind = InkAnalysisStrokeKind(1i32);
    pub const Drawing: InkAnalysisStrokeKind = InkAnalysisStrokeKind(2i32);
}
#[repr(transparent)]
pub struct InkAnalysisWritingRegion(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkAnalyzer(pub *mut ::core::ffi::c_void);
