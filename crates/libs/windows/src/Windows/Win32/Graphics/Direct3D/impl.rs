pub trait ID3DBlobImpl: Sized {
    fn GetBufferPointer();
    fn GetBufferSize();
}
impl ID3DBlobVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3DBlobImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3DBlobVtbl {
        unsafe extern "system" fn GetBufferPointer<Impl: ID3DBlobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBufferSize<Impl: ID3DBlobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> usize {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetBufferPointer::<Impl, IMPL_OFFSET>, GetBufferSize::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3DBlob as ::windows::core::Interface>::IID
    }
}
pub trait ID3DDestructionNotifierImpl: Sized {
    fn RegisterDestructionCallback();
    fn UnregisterDestructionCallback();
}
impl ID3DDestructionNotifierVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3DDestructionNotifierImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3DDestructionNotifierVtbl {
        unsafe extern "system" fn RegisterDestructionCallback<Impl: ID3DDestructionNotifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callbackfn: ::windows::core::RawPtr, pdata: *const ::core::ffi::c_void, pcallbackid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnregisterDestructionCallback<Impl: ID3DDestructionNotifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callbackid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, RegisterDestructionCallback::<Impl, IMPL_OFFSET>, UnregisterDestructionCallback::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3DDestructionNotifier as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ID3DIncludeImpl: Sized {
    fn Open();
    fn Close();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3DIncludeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ID3DIncludeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ID3DIncludeVtbl {
        unsafe extern "system" fn Open<Impl: ID3DIncludeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, includetype: D3D_INCLUDE_TYPE, pfilename: super::super::Foundation::PSTR, pparentdata: *const ::core::ffi::c_void, ppdata: *mut *mut ::core::ffi::c_void, pbytes: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Close<Impl: ID3DIncludeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(Open::<Impl, IMPL_OFFSET>, Close::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3DInclude as ::windows::core::Interface>::IID
    }
}
