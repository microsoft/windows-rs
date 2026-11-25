fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    windows_bindgen::builder()
        .sys()
        .filters([
            "D3DMATRIX",
            "D2D_MATRIX_3X2_F",
            "D2D_MATRIX_4X4_F",
            "D2D_POINT_2F",
            "D2D_RECT_F",
            "D2D_VECTOR_2F",
            "D2D_VECTOR_3F",
            "D2D_VECTOR_4F",
        ])
        .write();
}
