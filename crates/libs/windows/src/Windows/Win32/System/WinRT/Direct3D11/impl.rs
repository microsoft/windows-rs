pub trait IDirect3DDxgiInterfaceAccessImpl: Sized {
    fn GetInterface();
}
impl ::windows::core::RuntimeName for IDirect3DDxgiInterfaceAccess {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Direct3D11.IDirect3DDxgiInterfaceAccess";
}
impl IDirect3DDxgiInterfaceAccessVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDxgiInterfaceAccessImpl, const OFFSET: isize>() -> IDirect3DDxgiInterfaceAccessVtbl {
        unsafe extern "system" fn GetInterface<Impl: IDirect3DDxgiInterfaceAccessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, p: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInterface(&*(&iid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&p)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDirect3DDxgiInterfaceAccess>, ::windows::core::GetTrustLevel, GetInterface::<Impl, OFFSET>)
    }
}
