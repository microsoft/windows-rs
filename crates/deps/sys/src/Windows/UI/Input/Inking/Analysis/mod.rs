#![allow(non_snake_case, non_camel_case_types)]
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
#[repr(C)]
pub struct InkAnalysisDrawingKind(i32);
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
#[repr(C)]
pub struct InkAnalysisNodeKind(i32);
#[repr(transparent)]
pub struct InkAnalysisParagraph(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkAnalysisResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkAnalysisRoot(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct InkAnalysisStatus(i32);
#[repr(C)]
pub struct InkAnalysisStrokeKind(i32);
#[repr(transparent)]
pub struct InkAnalysisWritingRegion(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkAnalyzer(pub *mut ::core::ffi::c_void);
