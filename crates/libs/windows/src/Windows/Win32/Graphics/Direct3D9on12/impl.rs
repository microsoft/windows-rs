#[doc = "*Required features: `\"Win32_Graphics_Direct3D9on12\"`, `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Direct3D9\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Direct3D9"))]
pub trait IDirect3DDevice9On12_Impl: Sized {
    fn GetD3D12Device(&self, riid: *const ::windows_core::GUID, ppvdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn UnwrapUnderlyingResource(&self, presource: ::core::option::Option<&super::Direct3D9::IDirect3DResource9>, pcommandqueue: ::core::option::Option<&super::Direct3D12::ID3D12CommandQueue>, riid: *const ::windows_core::GUID, ppvresource12: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn ReturnUnderlyingResource(&self, presource: ::core::option::Option<&super::Direct3D9::IDirect3DResource9>, numsync: u32, psignalvalues: *mut u64, ppfences: *mut ::core::option::Option<super::Direct3D12::ID3D12Fence>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Direct3D9"))]
impl ::windows_core::RuntimeName for IDirect3DDevice9On12 {}
#[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Direct3D9"))]
impl IDirect3DDevice9On12_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9On12_Impl, const OFFSET: isize>() -> IDirect3DDevice9On12_Vtbl {
        unsafe extern "system" fn GetD3D12Device<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9On12_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppvdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetD3D12Device(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvdevice)).into()
        }
        unsafe extern "system" fn UnwrapUnderlyingResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9On12_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void, pcommandqueue: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppvresource12: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnwrapUnderlyingResource(::windows_core::from_raw_borrowed(&presource), ::windows_core::from_raw_borrowed(&pcommandqueue), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvresource12)).into()
        }
        unsafe extern "system" fn ReturnUnderlyingResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice9On12_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void, numsync: u32, psignalvalues: *mut u64, ppfences: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReturnUnderlyingResource(::windows_core::from_raw_borrowed(&presource), ::core::mem::transmute_copy(&numsync), ::core::mem::transmute_copy(&psignalvalues), ::core::mem::transmute_copy(&ppfences)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetD3D12Device: GetD3D12Device::<Identity, Impl, OFFSET>,
            UnwrapUnderlyingResource: UnwrapUnderlyingResource::<Identity, Impl, OFFSET>,
            ReturnUnderlyingResource: ReturnUnderlyingResource::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDirect3DDevice9On12 as ::windows_core::ComInterface>::IID
    }
}
