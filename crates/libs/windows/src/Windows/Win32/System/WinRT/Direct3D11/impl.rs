pub trait IDirect3DDxgiInterfaceAccessImpl: Sized {
    fn GetInterface(&mut self, iid: *const ::windows::core::GUID, p: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl IDirect3DDxgiInterfaceAccessVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirect3DDxgiInterfaceAccessImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirect3DDxgiInterfaceAccessVtbl {
        unsafe extern "system" fn GetInterface<Impl: IDirect3DDxgiInterfaceAccessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iid: *const ::windows::core::GUID, p: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetInterface(::core::mem::transmute_copy(&iid), ::core::mem::transmute_copy(&p)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetInterface: GetInterface::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirect3DDxgiInterfaceAccess as ::windows::core::Interface>::IID
    }
}
