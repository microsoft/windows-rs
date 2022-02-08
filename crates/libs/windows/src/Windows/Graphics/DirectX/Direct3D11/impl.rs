#[cfg(feature = "Foundation")]
pub trait IDirect3DDevice_Impl: Sized + super::super::super::Foundation::IClosable_Impl {
    fn Trim(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IDirect3DDevice {
    const NAME: &'static str = "Windows.Graphics.DirectX.Direct3D11.IDirect3DDevice";
}
#[cfg(feature = "Foundation")]
impl IDirect3DDevice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice_Impl, const OFFSET: isize>() -> IDirect3DDevice_Vtbl {
        unsafe extern "system" fn Trim<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Trim().into()
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IDirect3DDevice, OFFSET>(), Trim: Trim::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3DDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IDirect3DSurface_Impl: Sized + super::super::super::Foundation::IClosable_Impl {
    fn Description(&self) -> ::windows::core::Result<Direct3DSurfaceDescription>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IDirect3DSurface {
    const NAME: &'static str = "Windows.Graphics.DirectX.Direct3D11.IDirect3DSurface";
}
#[cfg(feature = "Foundation")]
impl IDirect3DSurface_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DSurface_Impl, const OFFSET: isize>() -> IDirect3DSurface_Vtbl {
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Direct3DSurfaceDescription) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IDirect3DSurface, OFFSET>(), Description: Description::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3DSurface as ::windows::core::Interface>::IID
    }
}
