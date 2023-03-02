#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"implement\"`*"]
pub trait IMarshal_Impl: Sized {
    fn GetUnmarshalClass(&self, riid: *const ::windows::core::GUID, pv: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void, mshlflags: u32) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetMarshalSizeMax(&self, riid: *const ::windows::core::GUID, pv: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void, mshlflags: u32) -> ::windows::core::Result<u32>;
    fn MarshalInterface(&self, pstm: ::core::option::Option<&super::IStream>, riid: *const ::windows::core::GUID, pv: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void, mshlflags: u32) -> ::windows::core::Result<()>;
    fn UnmarshalInterface(&self, pstm: ::core::option::Option<&super::IStream>, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn ReleaseMarshalData(&self, pstm: ::core::option::Option<&super::IStream>) -> ::windows::core::Result<()>;
    fn DisconnectObject(&self, dwreserved: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IMarshal {}
impl IMarshal_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMarshal_Impl, const OFFSET: isize>() -> IMarshal_Vtbl {
        unsafe extern "system" fn GetUnmarshalClass<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMarshal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pv: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void, mshlflags: u32, pcid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetUnmarshalClass(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pv), ::core::mem::transmute_copy(&dwdestcontext), ::core::mem::transmute_copy(&pvdestcontext), ::core::mem::transmute_copy(&mshlflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMarshalSizeMax<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMarshal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pv: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void, mshlflags: u32, psize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMarshalSizeMax(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pv), ::core::mem::transmute_copy(&dwdestcontext), ::core::mem::transmute_copy(&pvdestcontext), ::core::mem::transmute_copy(&mshlflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psize, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MarshalInterface<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMarshal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstm: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pv: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void, mshlflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MarshalInterface(::windows::core::from_raw_borrowed(&pstm), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pv), ::core::mem::transmute_copy(&dwdestcontext), ::core::mem::transmute_copy(&pvdestcontext), ::core::mem::transmute_copy(&mshlflags)).into()
        }
        unsafe extern "system" fn UnmarshalInterface<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMarshal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstm: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnmarshalInterface(::windows::core::from_raw_borrowed(&pstm), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn ReleaseMarshalData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMarshal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstm: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseMarshalData(::windows::core::from_raw_borrowed(&pstm)).into()
        }
        unsafe extern "system" fn DisconnectObject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMarshal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DisconnectObject(::core::mem::transmute_copy(&dwreserved)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetUnmarshalClass: GetUnmarshalClass::<Identity, Impl, OFFSET>,
            GetMarshalSizeMax: GetMarshalSizeMax::<Identity, Impl, OFFSET>,
            MarshalInterface: MarshalInterface::<Identity, Impl, OFFSET>,
            UnmarshalInterface: UnmarshalInterface::<Identity, Impl, OFFSET>,
            ReleaseMarshalData: ReleaseMarshalData::<Identity, Impl, OFFSET>,
            DisconnectObject: DisconnectObject::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMarshal as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"implement\"`*"]
pub trait IMarshal2_Impl: Sized + IMarshal_Impl {}
impl ::windows::core::RuntimeName for IMarshal2 {}
impl IMarshal2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMarshal2_Impl, const OFFSET: isize>() -> IMarshal2_Vtbl {
        Self { base__: IMarshal_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMarshal2 as ::windows::core::ComInterface>::IID || iid == &<IMarshal as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IMarshalingStream_Impl: Sized + super::IStream_Impl {
    fn GetMarshalingContextAttribute(&self, attribute: super::CO_MARSHALING_CONTEXT_ATTRIBUTES) -> ::windows::core::Result<usize>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IMarshalingStream {}
#[cfg(feature = "Win32_Foundation")]
impl IMarshalingStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMarshalingStream_Impl, const OFFSET: isize>() -> IMarshalingStream_Vtbl {
        unsafe extern "system" fn GetMarshalingContextAttribute<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMarshalingStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attribute: super::CO_MARSHALING_CONTEXT_ATTRIBUTES, pattributevalue: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMarshalingContextAttribute(::core::mem::transmute_copy(&attribute)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pattributevalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::IStream_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetMarshalingContextAttribute: GetMarshalingContextAttribute::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMarshalingStream as ::windows::core::ComInterface>::IID || iid == &<super::ISequentialStream as ::windows::core::ComInterface>::IID || iid == &<super::IStream as ::windows::core::ComInterface>::IID
    }
}
