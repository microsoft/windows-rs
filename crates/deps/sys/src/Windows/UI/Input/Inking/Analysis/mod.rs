#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IInkAnalysisInkBullet(pub *mut ::core::ffi::c_void);
pub struct IInkAnalysisInkDrawing(pub *mut ::core::ffi::c_void);
pub struct IInkAnalysisInkWord(pub *mut ::core::ffi::c_void);
pub struct IInkAnalysisLine(pub *mut ::core::ffi::c_void);
pub struct IInkAnalysisListItem(pub *mut ::core::ffi::c_void);
pub struct IInkAnalysisNode(pub *mut ::core::ffi::c_void);
pub struct IInkAnalysisParagraph(pub *mut ::core::ffi::c_void);
pub struct IInkAnalysisResult(pub *mut ::core::ffi::c_void);
pub struct IInkAnalysisRoot(pub *mut ::core::ffi::c_void);
pub struct IInkAnalysisWritingRegion(pub *mut ::core::ffi::c_void);
pub struct IInkAnalyzer(pub *mut ::core::ffi::c_void);
pub struct IInkAnalyzerFactory(pub *mut ::core::ffi::c_void);
pub struct InkAnalysisDrawingKind(i32);
pub struct InkAnalysisInkBullet(i32);
pub struct InkAnalysisInkDrawing(i32);
pub struct InkAnalysisInkWord(i32);
pub struct InkAnalysisLine(i32);
pub struct InkAnalysisListItem(i32);
pub struct InkAnalysisNode(i32);
pub struct InkAnalysisNodeKind(i32);
pub struct InkAnalysisParagraph(i32);
pub struct InkAnalysisResult(i32);
pub struct InkAnalysisRoot(i32);
pub struct InkAnalysisStatus(i32);
pub struct InkAnalysisStrokeKind(i32);
pub struct InkAnalysisWritingRegion(i32);
pub struct InkAnalyzer(i32);
