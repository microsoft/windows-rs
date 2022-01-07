pub trait IDirect3DDevice9On12Impl: Sized {
    fn GetD3D12Device();
    fn UnwrapUnderlyingResource();
    fn ReturnUnderlyingResource();
}
impl ::windows::core::RuntimeName for IDirect3DDevice9On12 {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D9on12.IDirect3DDevice9On12";
}
impl IDirect3DDevice9On12Vtbl {
    pub const fn new<Impl: IDirect3DDevice9On12Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirect3DDevice9On12Vtbl {
        unsafe extern "system" fn GetD3D12Device<Impl: IDirect3DDevice9On12Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetD3D12Device(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&ppvdevice as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnwrapUnderlyingResource<Impl: IDirect3DDevice9On12Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, pcommandqueue: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppvresource12: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UnwrapUnderlyingResource(
                &*(&presource as *const <super::Direct3D9::IDirect3DResource9 as ::windows::core::Abi>::Abi as *const <super::Direct3D9::IDirect3DResource9 as ::windows::core::DefaultType>::DefaultType),
                &*(&pcommandqueue as *const <super::Direct3D12::ID3D12CommandQueue as ::windows::core::Abi>::Abi as *const <super::Direct3D12::ID3D12CommandQueue as ::windows::core::DefaultType>::DefaultType),
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&ppvresource12 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReturnUnderlyingResource<Impl: IDirect3DDevice9On12Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presource: ::windows::core::RawPtr, numsync: u32, psignalvalues: *mut u64, ppfences: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReturnUnderlyingResource(&*(&presource as *const <super::Direct3D9::IDirect3DResource9 as ::windows::core::Abi>::Abi as *const <super::Direct3D9::IDirect3DResource9 as ::windows::core::DefaultType>::DefaultType), numsync, psignalvalues, ::core::mem::transmute_copy(&ppfences)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirect3DDevice9On12>, base.5, GetD3D12Device::<Impl, OFFSET>, UnwrapUnderlyingResource::<Impl, OFFSET>, ReturnUnderlyingResource::<Impl, OFFSET>)
    }
}
