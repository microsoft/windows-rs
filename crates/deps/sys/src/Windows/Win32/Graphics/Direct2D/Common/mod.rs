#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn D2D1_ALPHA_MODE();
    fn D2D1_BEZIER_SEGMENT();
    fn D2D1_BLEND_MODE();
    fn D2D1_BORDER_MODE();
    fn D2D1_COLORMATRIX_ALPHA_MODE();
    fn D2D1_COLOR_F();
    fn D2D1_COMPOSITE_MODE();
    fn D2D1_FIGURE_BEGIN();
    fn D2D1_FIGURE_END();
    fn D2D1_FILL_MODE();
    fn D2D1_PATH_SEGMENT();
    fn D2D1_PIXEL_FORMAT();
    fn D2D1_TURBULENCE_NOISE();
    fn D2D_COLOR_F();
    fn D2D_MATRIX_3X2_F();
    fn D2D_MATRIX_4X3_F();
    fn D2D_MATRIX_4X4_F();
    fn D2D_MATRIX_5X4_F();
    fn D2D_POINT_2F();
    fn D2D_POINT_2U();
    fn D2D_RECT_F();
    fn D2D_RECT_U();
    fn D2D_SIZE_F();
    fn D2D_SIZE_U();
    fn D2D_VECTOR_2F();
    fn D2D_VECTOR_3F();
    fn D2D_VECTOR_4F();
    fn ID2D1SimplifiedGeometrySink();
}
