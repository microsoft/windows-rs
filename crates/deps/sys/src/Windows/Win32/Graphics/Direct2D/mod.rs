#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub mod Common;
#[link(name = "windows")]
extern "system" {
    fn D2D1ComputeMaximumScaleFactor();
    fn D2D1ConvertColorSpace();
    fn D2D1CreateDevice();
    fn D2D1CreateDeviceContext();
    fn D2D1CreateFactory();
    fn D2D1GetGradientMeshInteriorPointsFromCoonsPatch();
    fn D2D1InvertMatrix();
    fn D2D1IsMatrixInvertible();
    fn D2D1MakeRotateMatrix();
    fn D2D1MakeSkewMatrix();
    fn D2D1SinCos();
    fn D2D1Tan();
    fn D2D1Vec3Length();
}
