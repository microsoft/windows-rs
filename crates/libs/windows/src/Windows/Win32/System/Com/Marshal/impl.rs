pub trait IMarshalImpl: Sized {
    fn GetUnmarshalClass();
    fn GetMarshalSizeMax();
    fn MarshalInterface();
    fn UnmarshalInterface();
    fn ReleaseMarshalData();
    fn DisconnectObject();
}
impl IMarshalVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMarshalImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMarshalVtbl {
        unsafe extern "system" fn GetUnmarshalClass<Impl: IMarshalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pv: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::core::ffi::c_void, mshlflags: u32, pcid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMarshalSizeMax<Impl: IMarshalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pv: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::core::ffi::c_void, mshlflags: u32, psize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MarshalInterface<Impl: IMarshalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstm: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, pv: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::core::ffi::c_void, mshlflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnmarshalInterface<Impl: IMarshalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstm: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseMarshalData<Impl: IMarshalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstm: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisconnectObject<Impl: IMarshalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetUnmarshalClass::<Impl, IMPL_OFFSET>, GetMarshalSizeMax::<Impl, IMPL_OFFSET>, MarshalInterface::<Impl, IMPL_OFFSET>, UnmarshalInterface::<Impl, IMPL_OFFSET>, ReleaseMarshalData::<Impl, IMPL_OFFSET>, DisconnectObject::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMarshal as ::windows::core::Interface>::IID
    }
}
pub trait IMarshal2Impl: Sized + IMarshalImpl {}
impl IMarshal2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMarshal2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMarshal2Vtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetUnmarshalClass::<Impl, IMPL_OFFSET>, GetMarshalSizeMax::<Impl, IMPL_OFFSET>, MarshalInterface::<Impl, IMPL_OFFSET>, UnmarshalInterface::<Impl, IMPL_OFFSET>, ReleaseMarshalData::<Impl, IMPL_OFFSET>, DisconnectObject::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMarshal2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IMarshalingStreamImpl: Sized + IStreamImpl + ISequentialStreamImpl {
    fn GetMarshalingContextAttribute();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl IMarshalingStreamVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMarshalingStreamImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMarshalingStreamVtbl {
        unsafe extern "system" fn GetMarshalingContextAttribute<Impl: IMarshalingStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attribute: super::CO_MARSHALING_CONTEXT_ATTRIBUTES, pattributevalue: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            Read::<Impl, IMPL_OFFSET>,
            Write::<Impl, IMPL_OFFSET>,
            Seek::<Impl, IMPL_OFFSET>,
            SetSize::<Impl, IMPL_OFFSET>,
            CopyTo::<Impl, IMPL_OFFSET>,
            Commit::<Impl, IMPL_OFFSET>,
            Revert::<Impl, IMPL_OFFSET>,
            LockRegion::<Impl, IMPL_OFFSET>,
            UnlockRegion::<Impl, IMPL_OFFSET>,
            Stat::<Impl, IMPL_OFFSET>,
            Clone::<Impl, IMPL_OFFSET>,
            GetMarshalingContextAttribute::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMarshalingStream as ::windows::core::Interface>::IID
    }
}
