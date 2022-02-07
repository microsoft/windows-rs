pub trait IMarshal_Impl: Sized {
    fn GetUnmarshalClass(&self, riid: *const ::windows::core::GUID, pv: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::core::ffi::c_void, mshlflags: u32, pcid: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetMarshalSizeMax(&self, riid: *const ::windows::core::GUID, pv: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::core::ffi::c_void, mshlflags: u32, psize: *mut u32) -> ::windows::core::Result<()>;
    fn MarshalInterface(&self, pstm: &::core::option::Option<super::IStream>, riid: *const ::windows::core::GUID, pv: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::core::ffi::c_void, mshlflags: u32) -> ::windows::core::Result<()>;
    fn UnmarshalInterface(&self, pstm: &::core::option::Option<super::IStream>, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn ReleaseMarshalData(&self, pstm: &::core::option::Option<super::IStream>) -> ::windows::core::Result<()>;
    fn DisconnectObject(&self, dwreserved: u32) -> ::windows::core::Result<()>;
}
impl IMarshal_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMarshal_Impl, const OFFSET: isize>() -> IMarshal_Vtbl {
        unsafe extern "system" fn GetUnmarshalClass<Identity: ::windows::core::IUnknownImpl, Impl: IMarshal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pv: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::core::ffi::c_void, mshlflags: u32, pcid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetUnmarshalClass(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pv), ::core::mem::transmute_copy(&dwdestcontext), ::core::mem::transmute_copy(&pvdestcontext), ::core::mem::transmute_copy(&mshlflags), ::core::mem::transmute_copy(&pcid)).into()
        }
        unsafe extern "system" fn GetMarshalSizeMax<Identity: ::windows::core::IUnknownImpl, Impl: IMarshal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pv: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::core::ffi::c_void, mshlflags: u32, psize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetMarshalSizeMax(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pv), ::core::mem::transmute_copy(&dwdestcontext), ::core::mem::transmute_copy(&pvdestcontext), ::core::mem::transmute_copy(&mshlflags), ::core::mem::transmute_copy(&psize)).into()
        }
        unsafe extern "system" fn MarshalInterface<Identity: ::windows::core::IUnknownImpl, Impl: IMarshal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstm: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, pv: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::core::ffi::c_void, mshlflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).MarshalInterface(::core::mem::transmute(&pstm), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pv), ::core::mem::transmute_copy(&dwdestcontext), ::core::mem::transmute_copy(&pvdestcontext), ::core::mem::transmute_copy(&mshlflags)).into()
        }
        unsafe extern "system" fn UnmarshalInterface<Identity: ::windows::core::IUnknownImpl, Impl: IMarshal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstm: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnmarshalInterface(::core::mem::transmute(&pstm), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn ReleaseMarshalData<Identity: ::windows::core::IUnknownImpl, Impl: IMarshal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstm: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReleaseMarshalData(::core::mem::transmute(&pstm)).into()
        }
        unsafe extern "system" fn DisconnectObject<Identity: ::windows::core::IUnknownImpl, Impl: IMarshal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DisconnectObject(::core::mem::transmute_copy(&dwreserved)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetUnmarshalClass: GetUnmarshalClass::<Identity, Impl, OFFSET>,
            GetMarshalSizeMax: GetMarshalSizeMax::<Identity, Impl, OFFSET>,
            MarshalInterface: MarshalInterface::<Identity, Impl, OFFSET>,
            UnmarshalInterface: UnmarshalInterface::<Identity, Impl, OFFSET>,
            ReleaseMarshalData: ReleaseMarshalData::<Identity, Impl, OFFSET>,
            DisconnectObject: DisconnectObject::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMarshal as ::windows::core::Interface>::IID
    }
}
pub trait IMarshal2_Impl: Sized + IMarshal_Impl {}
impl IMarshal2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMarshal2_Impl, const OFFSET: isize>() -> IMarshal2_Vtbl {
        Self { base: IMarshal_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMarshal2 as ::windows::core::Interface>::IID || iid == &<IMarshal as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IMarshalingStream_Impl: Sized + super::ISequentialStream_Impl + super::IStream_Impl {
    fn GetMarshalingContextAttribute(&self, attribute: super::CO_MARSHALING_CONTEXT_ATTRIBUTES) -> ::windows::core::Result<usize>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl IMarshalingStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMarshalingStream_Impl, const OFFSET: isize>() -> IMarshalingStream_Vtbl {
        unsafe extern "system" fn GetMarshalingContextAttribute<Identity: ::windows::core::IUnknownImpl, Impl: IMarshalingStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attribute: super::CO_MARSHALING_CONTEXT_ATTRIBUTES, pattributevalue: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMarshalingContextAttribute(::core::mem::transmute_copy(&attribute)) {
                ::core::result::Result::Ok(ok__) => {
                    *pattributevalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::IStream_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetMarshalingContextAttribute: GetMarshalingContextAttribute::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMarshalingStream as ::windows::core::Interface>::IID || iid == &<super::ISequentialStream as ::windows::core::Interface>::IID || iid == &<super::IStream as ::windows::core::Interface>::IID
    }
}
