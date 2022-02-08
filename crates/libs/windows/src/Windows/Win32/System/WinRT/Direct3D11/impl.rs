pub trait IDirect3DDxgiInterfaceAccess_Impl: Sized {
    fn GetInterface(&self, iid: *const ::windows::core::GUID, p: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl IDirect3DDxgiInterfaceAccess_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDxgiInterfaceAccess_Impl, const OFFSET: isize>() -> IDirect3DDxgiInterfaceAccess_Vtbl {
        unsafe extern "system" fn GetInterface<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDxgiInterfaceAccess_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iid: *const ::windows::core::GUID, p: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetInterface(::core::mem::transmute_copy(&iid), ::core::mem::transmute_copy(&p)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetInterface: GetInterface::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3DDxgiInterfaceAccess as ::windows::core::Interface>::IID
    }
}
