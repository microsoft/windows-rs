fn main() {
    windows::runtime::build! {
        Windows::Win32::Graphics::Direct3D12::ID3D12FunctionReflection,
    };
}
