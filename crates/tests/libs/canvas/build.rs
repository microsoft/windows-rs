fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    windows_bindgen::builder()
        .output(std::path::Path::new(&std::env::var_os("OUT_DIR").unwrap()).join("bindings.rs"))
        .filters([
            "ID2D1DeviceContext::{}",
            "ID2D1PathGeometry1::{}",
            "ID2D1PathGeometry::GetFigureCount",
            "ID2D1RenderTarget::GetPixelSize",
            "IDXGISwapChain1::GetDesc1",
        ])
        .flat()
        .minimal()
        .write();
}
