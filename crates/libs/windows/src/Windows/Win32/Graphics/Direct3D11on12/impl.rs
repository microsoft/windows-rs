pub trait ID3D11On12DeviceImpl: Sized {
    fn CreateWrappedResource();
    fn ReleaseWrappedResources();
    fn AcquireWrappedResources();
}
pub trait ID3D11On12Device1Impl: Sized + ID3D11On12DeviceImpl {
    fn GetD3D12Device();
}
pub trait ID3D11On12Device2Impl: Sized + ID3D11On12Device1Impl + ID3D11On12DeviceImpl {
    fn UnwrapUnderlyingResource();
    fn ReturnUnderlyingResource();
}
