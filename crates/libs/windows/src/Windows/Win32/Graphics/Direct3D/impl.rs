pub trait ID3DBlobImpl: Sized {
    fn GetBufferPointer();
    fn GetBufferSize();
}
impl ::windows::core::RuntimeName for ID3DBlob {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D.ID3DBlob";
}
impl ID3DBlobVtbl {
    pub const fn new<Impl: ID3DBlobImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID3DBlobVtbl {
        unsafe extern "system" fn GetBufferPointer<Impl: ID3DBlobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetBufferPointer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBufferSize<Impl: ID3DBlobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> usize {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetBufferSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID3DBlob>, base.5, GetBufferPointer::<Impl, OFFSET>, GetBufferSize::<Impl, OFFSET>)
    }
}
pub trait ID3DDestructionNotifierImpl: Sized {
    fn RegisterDestructionCallback();
    fn UnregisterDestructionCallback();
}
impl ::windows::core::RuntimeName for ID3DDestructionNotifier {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D.ID3DDestructionNotifier";
}
impl ID3DDestructionNotifierVtbl {
    pub const fn new<Impl: ID3DDestructionNotifierImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID3DDestructionNotifierVtbl {
        unsafe extern "system" fn RegisterDestructionCallback<Impl: ID3DDestructionNotifierImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, callbackfn: ::windows::core::RawPtr, pdata: *const ::core::ffi::c_void, pcallbackid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RegisterDestructionCallback(&*(&callbackfn as *const <PFN_DESTRUCTION_CALLBACK as ::windows::core::Abi>::Abi as *const <PFN_DESTRUCTION_CALLBACK as ::windows::core::DefaultType>::DefaultType), &*(&pdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pcallbackid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterDestructionCallback<Impl: ID3DDestructionNotifierImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, callbackid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UnregisterDestructionCallback(callbackid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID3DDestructionNotifier>, base.5, RegisterDestructionCallback::<Impl, OFFSET>, UnregisterDestructionCallback::<Impl, OFFSET>)
    }
}
pub trait ID3DIncludeImpl: Sized {
    fn Open();
    fn Close();
}
impl ::windows::core::RuntimeName for ID3DInclude {
    const NAME: &'static str = "Windows.Win32.Graphics.Direct3D.ID3DInclude";
}
impl ID3DIncludeVtbl {
    pub const fn new<Impl: ID3DIncludeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ID3DIncludeVtbl {
        unsafe extern "system" fn Open<Impl: ID3DIncludeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, includetype: D3D_INCLUDE_TYPE, pfilename: super::super::Foundation::PSTR, pparentdata: *const ::core::ffi::c_void, ppdata: *mut *mut ::core::ffi::c_void, pbytes: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Open(
                includetype,
                &*(&pfilename as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pparentdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                &*(&ppdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                pbytes,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: ID3DIncludeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Close(&*(&pdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ID3DInclude>, base.5, Open::<Impl, OFFSET>, Close::<Impl, OFFSET>)
    }
}
