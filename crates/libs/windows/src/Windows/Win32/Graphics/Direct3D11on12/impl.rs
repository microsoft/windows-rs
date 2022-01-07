pub trait ID3D11On12DeviceImpl: Sized {
    fn CreateWrappedResource();
    fn ReleaseWrappedResources();
    fn AcquireWrappedResources();
}
impl ::windows::core::RuntimeName for ID3D11On12Device {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11on12.ID3D11On12Device";
}
impl ID3D11On12DeviceVtbl {
    pub const fn new<Impl: ID3D11On12DeviceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID3D11On12DeviceVtbl {
        unsafe extern "system" fn CreateWrappedResource<Impl: ID3D11On12DeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presource12: *mut ::core::ffi::c_void, pflags11: *const D3D11_RESOURCE_FLAGS, instate: super::Direct3D12::D3D12_RESOURCE_STATES, outstate: super::Direct3D12::D3D12_RESOURCE_STATES, riid: *const ::windows::core::GUID, ppresource11: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateWrappedResource(
                &*(&presource12 as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&pflags11 as *const <D3D11_RESOURCE_FLAGS as ::windows::core::Abi>::Abi as *const <D3D11_RESOURCE_FLAGS as ::windows::core::DefaultType>::DefaultType),
                instate,
                outstate,
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppresource11),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseWrappedResources<Impl: ID3D11On12DeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppresources: *const ::windows::core::RawPtr, numresources: u32) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ReleaseWrappedResources(&*(&ppresources as *const <super::Direct3D11::ID3D11Resource as ::windows::core::Abi>::Abi as *const <super::Direct3D11::ID3D11Resource as ::windows::core::DefaultType>::DefaultType), numresources).into()
        }
        unsafe extern "system" fn AcquireWrappedResources<Impl: ID3D11On12DeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppresources: *const ::windows::core::RawPtr, numresources: u32) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).AcquireWrappedResources(&*(&ppresources as *const <super::Direct3D11::ID3D11Resource as ::windows::core::Abi>::Abi as *const <super::Direct3D11::ID3D11Resource as ::windows::core::DefaultType>::DefaultType), numresources).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID3D11On12Device>, base.5, CreateWrappedResource::<Impl, OFFSET>, ReleaseWrappedResources::<Impl, OFFSET>, AcquireWrappedResources::<Impl, OFFSET>)
    }
}
pub trait ID3D11On12Device1Impl: Sized + ID3D11On12DeviceImpl {
    fn GetD3D12Device();
}
impl ::windows::core::RuntimeName for ID3D11On12Device1 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11on12.ID3D11On12Device1";
}
impl ID3D11On12Device1Vtbl {
    pub const fn new<Impl: ID3D11On12Device1Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID3D11On12Device1Vtbl {
        unsafe extern "system" fn GetD3D12Device<Impl: ID3D11On12Device1Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetD3D12Device(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppvdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID3D11On12Device1>, base.5, GetD3D12Device::<Impl, OFFSET>)
    }
}
pub trait ID3D11On12Device2Impl: Sized + ID3D11On12Device1Impl + ID3D11On12DeviceImpl {
    fn UnwrapUnderlyingResource();
    fn ReturnUnderlyingResource();
}
impl ::windows::core::RuntimeName for ID3D11On12Device2 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D11on12.ID3D11On12Device2";
}
impl ID3D11On12Device2Vtbl {
    pub const fn new<Impl: ID3D11On12Device2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID3D11On12Device2Vtbl {
        unsafe extern "system" fn UnwrapUnderlyingResource<Impl: ID3D11On12Device2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presource11: ::windows::core::RawPtr, pcommandqueue: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppvresource12: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UnwrapUnderlyingResource(
                &*(&presource11 as *const <super::Direct3D11::ID3D11Resource as ::windows::core::Abi>::Abi as *const <super::Direct3D11::ID3D11Resource as ::windows::core::DefaultType>::DefaultType),
                &*(&pcommandqueue as *const <super::Direct3D12::ID3D12CommandQueue as ::windows::core::Abi>::Abi as *const <super::Direct3D12::ID3D12CommandQueue as ::windows::core::DefaultType>::DefaultType),
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppvresource12),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReturnUnderlyingResource<Impl: ID3D11On12Device2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presource11: ::windows::core::RawPtr, numsync: u32, psignalvalues: *const u64, ppfences: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReturnUnderlyingResource(&*(&presource11 as *const <super::Direct3D11::ID3D11Resource as ::windows::core::Abi>::Abi as *const <super::Direct3D11::ID3D11Resource as ::windows::core::DefaultType>::DefaultType), numsync, psignalvalues, &*(&ppfences as *const <super::Direct3D12::ID3D12Fence as ::windows::core::Abi>::Abi as *const <super::Direct3D12::ID3D12Fence as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID3D11On12Device2>, base.5, UnwrapUnderlyingResource::<Impl, OFFSET>, ReturnUnderlyingResource::<Impl, OFFSET>)
    }
}
