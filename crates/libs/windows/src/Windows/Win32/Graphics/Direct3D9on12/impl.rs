pub trait IDirect3DDevice9On12Impl: Sized {
    fn GetD3D12Device();
    fn UnwrapUnderlyingResource();
    fn ReturnUnderlyingResource();
}
