#[cfg(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12"))]
pub trait ID3D11On12Device_Impl: Sized {
    fn CreateWrappedResource(&self, presource12: &::core::option::Option<::windows::core::IUnknown>, pflags11: *const D3D11_RESOURCE_FLAGS, instate: super::Direct3D12::D3D12_RESOURCE_STATES, outstate: super::Direct3D12::D3D12_RESOURCE_STATES, riid: *const ::windows::core::GUID, ppresource11: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn ReleaseWrappedResources(&self, ppresources: *const ::core::option::Option<super::Direct3D11::ID3D11Resource>, numresources: u32);
    fn AcquireWrappedResources(&self, ppresources: *const ::core::option::Option<super::Direct3D11::ID3D11Resource>, numresources: u32);
}
#[cfg(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12"))]
impl ::windows::core::RuntimeName for ID3D11On12Device {}
#[cfg(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12"))]
impl ID3D11On12Device_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D11On12Device_Impl, const OFFSET: isize>() -> ID3D11On12Device_Vtbl {
        unsafe extern "system" fn CreateWrappedResource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D11On12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource12: *mut ::core::ffi::c_void, pflags11: *const D3D11_RESOURCE_FLAGS, instate: super::Direct3D12::D3D12_RESOURCE_STATES, outstate: super::Direct3D12::D3D12_RESOURCE_STATES, riid: *const ::windows::core::GUID, ppresource11: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateWrappedResource(::core::mem::transmute(&presource12), ::core::mem::transmute_copy(&pflags11), ::core::mem::transmute_copy(&instate), ::core::mem::transmute_copy(&outstate), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppresource11)).into()
        }
        unsafe extern "system" fn ReleaseWrappedResources<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D11On12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresources: *const *mut ::core::ffi::c_void, numresources: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseWrappedResources(::core::mem::transmute_copy(&ppresources), ::core::mem::transmute_copy(&numresources))
        }
        unsafe extern "system" fn AcquireWrappedResources<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D11On12Device_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresources: *const *mut ::core::ffi::c_void, numresources: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AcquireWrappedResources(::core::mem::transmute_copy(&ppresources), ::core::mem::transmute_copy(&numresources))
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateWrappedResource: CreateWrappedResource::<Identity, Impl, OFFSET>,
            ReleaseWrappedResources: ReleaseWrappedResources::<Identity, Impl, OFFSET>,
            AcquireWrappedResources: AcquireWrappedResources::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11On12Device as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12"))]
pub trait ID3D11On12Device1_Impl: Sized + ID3D11On12Device_Impl {
    fn GetD3D12Device(&self, riid: *const ::windows::core::GUID, ppvdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12"))]
impl ::windows::core::RuntimeName for ID3D11On12Device1 {}
#[cfg(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12"))]
impl ID3D11On12Device1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D11On12Device1_Impl, const OFFSET: isize>() -> ID3D11On12Device1_Vtbl {
        unsafe extern "system" fn GetD3D12Device<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D11On12Device1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetD3D12Device(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvdevice)).into()
        }
        Self { base__: ID3D11On12Device_Vtbl::new::<Identity, Impl, OFFSET>(), GetD3D12Device: GetD3D12Device::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11On12Device1 as ::windows::core::Interface>::IID || iid == &<ID3D11On12Device as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12"))]
pub trait ID3D11On12Device2_Impl: Sized + ID3D11On12Device_Impl + ID3D11On12Device1_Impl {
    fn UnwrapUnderlyingResource(&self, presource11: &::core::option::Option<super::Direct3D11::ID3D11Resource>, pcommandqueue: &::core::option::Option<super::Direct3D12::ID3D12CommandQueue>, riid: *const ::windows::core::GUID, ppvresource12: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn ReturnUnderlyingResource(&self, presource11: &::core::option::Option<super::Direct3D11::ID3D11Resource>, numsync: u32, psignalvalues: *const u64, ppfences: *const ::core::option::Option<super::Direct3D12::ID3D12Fence>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12"))]
impl ::windows::core::RuntimeName for ID3D11On12Device2 {}
#[cfg(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12"))]
impl ID3D11On12Device2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D11On12Device2_Impl, const OFFSET: isize>() -> ID3D11On12Device2_Vtbl {
        unsafe extern "system" fn UnwrapUnderlyingResource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D11On12Device2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource11: *mut ::core::ffi::c_void, pcommandqueue: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvresource12: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnwrapUnderlyingResource(::core::mem::transmute(&presource11), ::core::mem::transmute(&pcommandqueue), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvresource12)).into()
        }
        unsafe extern "system" fn ReturnUnderlyingResource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3D11On12Device2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presource11: *mut ::core::ffi::c_void, numsync: u32, psignalvalues: *const u64, ppfences: *const *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReturnUnderlyingResource(::core::mem::transmute(&presource11), ::core::mem::transmute_copy(&numsync), ::core::mem::transmute_copy(&psignalvalues), ::core::mem::transmute_copy(&ppfences)).into()
        }
        Self {
            base__: ID3D11On12Device1_Vtbl::new::<Identity, Impl, OFFSET>(),
            UnwrapUnderlyingResource: UnwrapUnderlyingResource::<Identity, Impl, OFFSET>,
            ReturnUnderlyingResource: ReturnUnderlyingResource::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3D11On12Device2 as ::windows::core::Interface>::IID || iid == &<ID3D11On12Device as ::windows::core::Interface>::IID || iid == &<ID3D11On12Device1 as ::windows::core::Interface>::IID
    }
}
