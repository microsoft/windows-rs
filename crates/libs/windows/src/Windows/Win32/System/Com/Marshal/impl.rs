pub trait IMarshalImpl: Sized {
    fn GetUnmarshalClass();
    fn GetMarshalSizeMax();
    fn MarshalInterface();
    fn UnmarshalInterface();
    fn ReleaseMarshalData();
    fn DisconnectObject();
}
impl ::windows::core::RuntimeName for IMarshal {
    const NAME: &'static str = "Windows.Win32.System.Com.Marshal.IMarshal";
}
impl IMarshalVtbl {
    pub const fn new<Impl: IMarshalImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMarshalVtbl {
        unsafe extern "system" fn GetUnmarshalClass<Impl: IMarshalImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pv: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::core::ffi::c_void, mshlflags: u32, pcid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetUnmarshalClass(
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&pv as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                dwdestcontext,
                &*(&pvdestcontext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                mshlflags,
                ::core::mem::transmute_copy(&pcid),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMarshalSizeMax<Impl: IMarshalImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pv: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::core::ffi::c_void, mshlflags: u32, psize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetMarshalSizeMax(
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&pv as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                dwdestcontext,
                &*(&pvdestcontext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                mshlflags,
                ::core::mem::transmute_copy(&psize),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MarshalInterface<Impl: IMarshalImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstm: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, pv: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::core::ffi::c_void, mshlflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MarshalInterface(
                &*(&pstm as *const <super::IStream as ::windows::core::Abi>::Abi as *const <super::IStream as ::windows::core::DefaultType>::DefaultType),
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&pv as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                dwdestcontext,
                &*(&pvdestcontext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                mshlflags,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnmarshalInterface<Impl: IMarshalImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstm: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UnmarshalInterface(&*(&pstm as *const <super::IStream as ::windows::core::Abi>::Abi as *const <super::IStream as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseMarshalData<Impl: IMarshalImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstm: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReleaseMarshalData(&*(&pstm as *const <super::IStream as ::windows::core::Abi>::Abi as *const <super::IStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisconnectObject<Impl: IMarshalImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisconnectObject(dwreserved) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMarshal>, base.5, GetUnmarshalClass::<Impl, OFFSET>, GetMarshalSizeMax::<Impl, OFFSET>, MarshalInterface::<Impl, OFFSET>, UnmarshalInterface::<Impl, OFFSET>, ReleaseMarshalData::<Impl, OFFSET>, DisconnectObject::<Impl, OFFSET>)
    }
}
pub trait IMarshal2Impl: Sized + IMarshalImpl {}
impl ::windows::core::RuntimeName for IMarshal2 {
    const NAME: &'static str = "Windows.Win32.System.Com.Marshal.IMarshal2";
}
impl IMarshal2Vtbl {
    pub const fn new<Impl: IMarshal2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMarshal2Vtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMarshal2>, base.5)
    }
}
pub trait IMarshalingStreamImpl: Sized + IStreamImpl + ISequentialStreamImpl {
    fn GetMarshalingContextAttribute();
}
impl ::windows::core::RuntimeName for IMarshalingStream {
    const NAME: &'static str = "Windows.Win32.System.Com.Marshal.IMarshalingStream";
}
impl IMarshalingStreamVtbl {
    pub const fn new<Impl: IMarshalingStreamImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMarshalingStreamVtbl {
        unsafe extern "system" fn GetMarshalingContextAttribute<Impl: IMarshalingStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, attribute: super::CO_MARSHALING_CONTEXT_ATTRIBUTES, pattributevalue: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetMarshalingContextAttribute(attribute, ::core::mem::transmute_copy(&pattributevalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMarshalingStream>, base.5, GetMarshalingContextAttribute::<Impl, OFFSET>)
    }
}
