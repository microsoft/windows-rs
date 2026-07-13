fn main() {
    windows_reactor_setup::as_self_contained();

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let bindings = format!("{out_dir}/bindings.rs");

    windows_bindgen::builder()
        .output(&bindings)
        .flat()
        .filters([
            // Direct3D 11
            "D3D11CreateDevice",
            "D3D_DRIVER_TYPE_HARDWARE",
            "D3D_FEATURE_LEVEL_11_0",
            "D3D11_CREATE_DEVICE_BGRA_SUPPORT",
            "D3D11_SDK_VERSION",
            // Direct2D
            "D2D1CreateFactory",
            "D2D1_FACTORY_TYPE_MULTI_THREADED",
            "D2D1_DEVICE_CONTEXT_OPTIONS_NONE",
            "D2D1_BITMAP_PROPERTIES1",
            "D2D1_ALPHA_MODE_PREMULTIPLIED",
            "D2D1_BITMAP_OPTIONS_TARGET",
            "D2D1_BITMAP_OPTIONS_CANNOT_DRAW",
            "D2D_COLOR_F",
            "D2D1_ELLIPSE",
            "D2DERR_RECREATE_TARGET",
            "ID2D1Factory1",
            "ID2D1Device",
            "ID2D1DeviceContext",
            "ID2D1RenderTarget",
            "ID2D1SolidColorBrush",
            // Interfaces and structs referenced by the methods above; without these
            // the referencing methods are silently dropped from the projection.
            "ID3D11Device",
            "ID3D11DeviceContext",
            "ID2D1Image",
            "ID2D1Bitmap1",
            "ID2D1Brush",
            "D2D1_TAG",
            "D2D1_BRUSH_PROPERTIES",
            "D2D1_FACTORY_OPTIONS",
            // DXGI
            "IDXGIDevice",
            "IDXGIObject",
            "IDXGIFactory2",
            "IDXGISwapChain",
            "IDXGISwapChain1",
            "IDXGISurface",
            "IDXGIAdapter",
            "IDXGIOutput",
            "DXGI_SWAP_CHAIN_DESC1",
            "DXGI_FORMAT_B8G8R8A8_UNORM",
            "DXGI_FORMAT_UNKNOWN",
            "DXGI_USAGE_RENDER_TARGET_OUTPUT",
            "DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL",
            "DXGI_ALPHA_MODE_PREMULTIPLIED",
            "DXGI_STATUS_OCCLUDED",
            "DXGI_ERROR_DEVICE_HUNG",
            "DXGI_ERROR_DEVICE_REMOVED",
            "DXGI_ERROR_DEVICE_RESET",
            "DXGI_ERROR_DRIVER_INTERNAL_ERROR",
            "DXGI_ERROR_INVALID_CALL",
        ])
        .write();
}
