pub trait ID3DBlob_Impl: Sized {
    fn GetBufferPointer(&self) -> *mut ::core::ffi::c_void;
    fn GetBufferSize(&self) -> usize;
}
impl ::windows::core::RuntimeName for ID3DBlob {}
impl ID3DBlob_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3DBlob_Impl, const OFFSET: isize>() -> ID3DBlob_Vtbl {
        unsafe extern "system" fn GetBufferPointer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3DBlob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBufferPointer()
        }
        unsafe extern "system" fn GetBufferSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3DBlob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> usize {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBufferSize()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetBufferPointer: GetBufferPointer::<Identity, Impl, OFFSET>,
            GetBufferSize: GetBufferSize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3DBlob as ::windows::core::Interface>::IID
    }
}
pub trait ID3DDestructionNotifier_Impl: Sized {
    fn RegisterDestructionCallback(&self, callbackfn: &PFN_DESTRUCTION_CALLBACK, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<u32>;
    fn UnregisterDestructionCallback(&self, callbackid: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ID3DDestructionNotifier {}
impl ID3DDestructionNotifier_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3DDestructionNotifier_Impl, const OFFSET: isize>() -> ID3DDestructionNotifier_Vtbl {
        unsafe extern "system" fn RegisterDestructionCallback<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3DDestructionNotifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callbackfn: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void, pcallbackid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RegisterDestructionCallback(::core::mem::transmute(&callbackfn), ::core::mem::transmute_copy(&pdata)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcallbackid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterDestructionCallback<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3DDestructionNotifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callbackid: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnregisterDestructionCallback(::core::mem::transmute_copy(&callbackid)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            RegisterDestructionCallback: RegisterDestructionCallback::<Identity, Impl, OFFSET>,
            UnregisterDestructionCallback: UnregisterDestructionCallback::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3DDestructionNotifier as ::windows::core::Interface>::IID
    }
}
