#[cfg(feature = "Foundation")]
pub trait IDirect3DDeviceImpl: Sized + IClosableImpl {
    fn Trim(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IDirect3DDevice {
    const NAME: &'static str = "Windows.Graphics.DirectX.Direct3D11.IDirect3DDevice";
}
#[cfg(feature = "Foundation")]
impl IDirect3DDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDeviceImpl, const OFFSET: isize>() -> IDirect3DDeviceVtbl {
        unsafe extern "system" fn Trim<Impl: IDirect3DDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Trim().into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDirect3DDevice>, ::windows::core::GetTrustLevel, Trim::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Foundation")]
pub trait IDirect3DSurfaceImpl: Sized + IClosableImpl {
    fn Description(&self) -> ::windows::core::Result<Direct3DSurfaceDescription>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IDirect3DSurface {
    const NAME: &'static str = "Windows.Graphics.DirectX.Direct3D11.IDirect3DSurface";
}
#[cfg(feature = "Foundation")]
impl IDirect3DSurfaceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DSurfaceImpl, const OFFSET: isize>() -> IDirect3DSurfaceVtbl {
        unsafe extern "system" fn Description<Impl: IDirect3DSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Direct3DSurfaceDescription) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDirect3DSurface>, ::windows::core::GetTrustLevel, Description::<Impl, OFFSET>)
    }
}
