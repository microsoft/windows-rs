#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn IInkAnalysisInkBullet();
    fn IInkAnalysisInkDrawing();
    fn IInkAnalysisInkWord();
    fn IInkAnalysisLine();
    fn IInkAnalysisListItem();
    fn IInkAnalysisNode();
    fn IInkAnalysisParagraph();
    fn IInkAnalysisResult();
    fn IInkAnalysisRoot();
    fn IInkAnalysisWritingRegion();
    fn IInkAnalyzer();
    fn IInkAnalyzerFactory();
    fn InkAnalysisDrawingKind();
    fn InkAnalysisInkBullet();
    fn InkAnalysisInkDrawing();
    fn InkAnalysisInkWord();
    fn InkAnalysisLine();
    fn InkAnalysisListItem();
    fn InkAnalysisNode();
    fn InkAnalysisNodeKind();
    fn InkAnalysisParagraph();
    fn InkAnalysisResult();
    fn InkAnalysisRoot();
    fn InkAnalysisStatus();
    fn InkAnalysisStrokeKind();
    fn InkAnalysisWritingRegion();
    fn InkAnalyzer();
}
