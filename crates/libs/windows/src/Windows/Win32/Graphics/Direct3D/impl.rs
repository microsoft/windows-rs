pub trait ID3DBlob_Impl: Sized {
    fn GetBufferPointer(&self) -> *mut ::core::ffi::c_void;
    fn GetBufferSize(&self) -> usize;
}
impl ID3DBlob_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3DBlob_Impl, const OFFSET: isize>() -> ID3DBlob_Vtbl {
        unsafe extern "system" fn GetBufferPointer<Identity: ::windows::core::IUnknownImpl, Impl: ID3DBlob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetBufferPointer()
        }
        unsafe extern "system" fn GetBufferSize<Identity: ::windows::core::IUnknownImpl, Impl: ID3DBlob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> usize {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetBufferSize()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
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
impl ID3DDestructionNotifier_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3DDestructionNotifier_Impl, const OFFSET: isize>() -> ID3DDestructionNotifier_Vtbl {
        unsafe extern "system" fn RegisterDestructionCallback<Identity: ::windows::core::IUnknownImpl, Impl: ID3DDestructionNotifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callbackfn: ::windows::core::RawPtr, pdata: *const ::core::ffi::c_void, pcallbackid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RegisterDestructionCallback(::core::mem::transmute(&callbackfn), ::core::mem::transmute_copy(&pdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcallbackid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterDestructionCallback<Identity: ::windows::core::IUnknownImpl, Impl: ID3DDestructionNotifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callbackid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnregisterDestructionCallback(::core::mem::transmute_copy(&callbackid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            RegisterDestructionCallback: RegisterDestructionCallback::<Identity, Impl, OFFSET>,
            UnregisterDestructionCallback: UnregisterDestructionCallback::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3DDestructionNotifier as ::windows::core::Interface>::IID
    }
}
pub trait ID3DInclude_Impl: Sized {
    fn Open(&self, includetype: D3D_INCLUDE_TYPE, pfilename: &::windows::core::PCSTR, pparentdata: *const ::core::ffi::c_void, ppdata: *mut *mut ::core::ffi::c_void, pbytes: *mut u32) -> ::windows::core::Result<()>;
    fn Close(&self, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ID3DInclude_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3DInclude_Impl, const OFFSET: isize>() -> ID3DInclude_Vtbl {
        unsafe extern "system" fn Open<Identity: ::windows::core::IUnknownImpl, Impl: ID3DInclude_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, includetype: D3D_INCLUDE_TYPE, pfilename: ::windows::core::PCSTR, pparentdata: *const ::core::ffi::c_void, ppdata: *mut *mut ::core::ffi::c_void, pbytes: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Open(::core::mem::transmute_copy(&includetype), ::core::mem::transmute(&pfilename), ::core::mem::transmute_copy(&pparentdata), ::core::mem::transmute_copy(&ppdata), ::core::mem::transmute_copy(&pbytes)).into()
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl, Impl: ID3DInclude_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Close(::core::mem::transmute_copy(&pdata)).into()
        }
        Self { Open: Open::<Identity, Impl, OFFSET>, Close: Close::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3DInclude as ::windows::core::Interface>::IID
    }
}
