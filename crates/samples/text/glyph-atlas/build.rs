fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    windows_bindgen::builder()
        .output(std::path::Path::new(&std::env::var_os("OUT_DIR").unwrap()).join("bindings.rs"))
        .filters([
            "D3D11CreateDevice",
            "D3D11_BIND_CONSTANT_BUFFER",
            "D3D11_BIND_SHADER_RESOURCE",
            "D3D11_BLEND_INV_SRC1_ALPHA",
            "D3D11_BLEND_INV_SRC1_COLOR",
            "D3D11_BLEND_ONE",
            "D3D11_BLEND_OP_ADD",
            "D3D11_COLOR_WRITE_ENABLE_ALL",
            "D3D11_CPU_ACCESS_WRITE",
            "D3D11_MAP_WRITE_DISCARD",
            "D3D11_RESOURCE_MISC_BUFFER_STRUCTURED",
            "D3D11_SDK_VERSION",
            "D3D11_USAGE_DEFAULT",
            "D3D11_USAGE_DYNAMIC",
            "D3DCOMPILE_OPTIMIZATION_LEVEL3",
            "D3DCompile",
            "D3D_DRIVER_TYPE_HARDWARE",
            "D3D_DRIVER_TYPE_WARP",
            "D3D_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP",
            "DXGI_ERROR_DEVICE_REMOVED",
            "DXGI_ERROR_DEVICE_RESET",
            "DXGI_FORMAT_B8G8R8A8_UNORM",
            "DXGI_FORMAT_R8G8B8A8_UNORM",
            "DXGI_SWAP_EFFECT_FLIP_DISCARD",
            "DXGI_USAGE_RENDER_TARGET_OUTPUT",
            "GetClientRect",
            "GetDpiForWindow",
            "ID3D10Blob::{GetBufferPointer, GetBufferSize}",
            "ID3D11Device::{CreateBlendState, CreateBuffer, CreatePixelShader, CreateRenderTargetView, CreateShaderResourceView, CreateTexture2D, CreateVertexShader}",
            "ID3D11DeviceContext::{ClearRenderTargetView, DrawInstanced, IASetPrimitiveTopology, Map, OMSetBlendState, OMSetRenderTargets, PSSetConstantBuffers, PSSetShader, PSSetShaderResources, RSSetViewports, Unmap, UpdateSubresource, VSSetConstantBuffers, VSSetShader, VSSetShaderResources}",
            "ID3D11Texture2D::{}",
            "IDXGIDevice::GetAdapter",
            "IDXGIFactory2::CreateSwapChainForHwnd",
            "IDXGIObject::GetParent",
            "IDXGISwapChain::{GetBuffer, Present, ResizeBuffers}",
            "SWP_NOACTIVATE",
            "SWP_NOZORDER",
            "SetWindowPos",
            "SetWindowTextW",
            "WM_CHAR",
            "WM_DPICHANGED",
            "WM_MOUSEWHEEL",
        ])
        .flat()
        .minimal()
        .write();
}
