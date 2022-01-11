#[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Direct3D9"))]
pub trait IDirect3DDevice9On12Impl: Sized {
    fn GetD3D12Device();
    fn UnwrapUnderlyingResource();
    fn ReturnUnderlyingResource();
}
#[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Direct3D9"))]
impl IDirect3DDevice9On12Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDevice9On12Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirect3DDevice9On12Vtbl {
        unsafe extern "system" fn GetD3D12Device<Impl: IDirect3DDevice9On12Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnwrapUnderlyingResource<Impl: IDirect3DDevice9On12Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pcommandqueue: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppvresource12: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReturnUnderlyingResource<Impl: IDirect3DDevice9On12Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, numsync: u32, psignalvalues: *mut u64, ppfences: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetD3D12Device::<Impl, IMPL_OFFSET>, UnwrapUnderlyingResource::<Impl, IMPL_OFFSET>, ReturnUnderlyingResource::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3DDevice9On12 as ::windows::core::Interface>::IID
    }
}
