#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub trait IDisplayDeviceInterop_Impl: Sized {
    fn CreateSharedHandle(&mut self, pobject: ::core::option::Option<::windows::core::IInspectable>, psecurityattributes: *const super::super::super::Security::SECURITY_ATTRIBUTES, access: u32, name: ::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::HANDLE>;
    fn OpenSharedHandle(&mut self, nthandle: super::super::super::Foundation::HANDLE, riid: ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl IDisplayDeviceInterop_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayDeviceInterop_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayDeviceInterop_Vtbl {
        unsafe extern "system" fn CreateSharedHandle<Impl: IDisplayDeviceInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobject: *mut ::core::ffi::c_void, psecurityattributes: *const super::super::super::Security::SECURITY_ATTRIBUTES, access: u32, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, phandle: *mut super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSharedHandle(::core::mem::transmute(&pobject), ::core::mem::transmute_copy(&psecurityattributes), ::core::mem::transmute_copy(&access), ::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *phandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenSharedHandle<Impl: IDisplayDeviceInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nthandle: super::super::super::Foundation::HANDLE, riid: ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OpenSharedHandle(::core::mem::transmute_copy(&nthandle), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvobj)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateSharedHandle: CreateSharedHandle::<Impl, IMPL_OFFSET>,
            OpenSharedHandle: OpenSharedHandle::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayDeviceInterop as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDisplayPathInterop_Impl: Sized {
    fn CreateSourcePresentationHandle(&mut self) -> ::windows::core::Result<super::super::super::Foundation::HANDLE>;
    fn GetSourceId(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDisplayPathInterop_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayPathInterop_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayPathInterop_Vtbl {
        unsafe extern "system" fn CreateSourcePresentationHandle<Impl: IDisplayPathInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSourcePresentationHandle() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceId<Impl: IDisplayPathInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psourceid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSourceId() {
                ::core::result::Result::Ok(ok__) => {
                    *psourceid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateSourcePresentationHandle: CreateSourcePresentationHandle::<Impl, IMPL_OFFSET>,
            GetSourceId: GetSourceId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayPathInterop as ::windows::core::Interface>::IID
    }
}
