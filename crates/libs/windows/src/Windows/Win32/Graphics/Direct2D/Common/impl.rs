pub trait ID2D1SimplifiedGeometrySinkImpl: Sized {
    fn SetFillMode();
    fn SetSegmentFlags();
    fn BeginFigure();
    fn AddLines();
    fn AddBeziers();
    fn EndFigure();
    fn Close();
}
