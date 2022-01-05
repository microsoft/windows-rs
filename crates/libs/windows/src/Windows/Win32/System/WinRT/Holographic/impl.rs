pub trait IHolographicCameraInteropImpl: Sized {
    fn CreateDirect3D12BackBufferResource();
    fn CreateDirect3D12HardwareProtectedBackBufferResource();
    fn AcquireDirect3D12BufferResource();
    fn AcquireDirect3D12BufferResourceWithTimeout();
    fn UnacquireDirect3D12BufferResource();
}
pub trait IHolographicCameraRenderingParametersInteropImpl: Sized {
    fn CommitDirect3D12Resource();
    fn CommitDirect3D12ResourceWithDepthData();
}
pub trait IHolographicQuadLayerInteropImpl: Sized {
    fn CreateDirect3D12ContentBufferResource();
    fn CreateDirect3D12HardwareProtectedContentBufferResource();
    fn AcquireDirect3D12BufferResource();
    fn AcquireDirect3D12BufferResourceWithTimeout();
    fn UnacquireDirect3D12BufferResource();
}
pub trait IHolographicQuadLayerUpdateParametersInteropImpl: Sized {
    fn CommitDirect3D12Resource();
}
