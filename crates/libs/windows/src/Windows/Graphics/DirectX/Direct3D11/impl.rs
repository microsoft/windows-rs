#[cfg(feature = "Foundation")]
pub trait IDirect3DDeviceImpl: Sized + IClosableImpl {
    fn Trim(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation")]
pub trait IDirect3DSurfaceImpl: Sized + IClosableImpl {
    fn Description(&self) -> ::windows::core::Result<Direct3DSurfaceDescription>;
}
