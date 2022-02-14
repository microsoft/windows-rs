#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub trait IDisplayDeviceInterop_Impl: Sized {
    fn CreateSharedHandle(&self, pobject: &::core::option::Option<::windows::core::IInspectable>, psecurityattributes: *const super::super::super::Security::SECURITY_ATTRIBUTES, access: u32, name: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::HANDLE>;
    fn OpenSharedHandle(&self, nthandle: super::super::super::Foundation::HANDLE, riid: &::windows::core::GUID) -> ::windows::core::Result<*mut ::core::ffi::c_void>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl IDisplayDeviceInterop_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayDeviceInterop_Impl, const OFFSET: isize>() -> IDisplayDeviceInterop_Vtbl {
        unsafe extern "system" fn CreateSharedHandle<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayDeviceInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobject: *mut ::core::ffi::c_void, psecurityattributes: *const super::super::super::Security::SECURITY_ATTRIBUTES, access: u32, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, phandle: *mut super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateSharedHandle(::core::mem::transmute(&pobject), ::core::mem::transmute_copy(&psecurityattributes), ::core::mem::transmute_copy(&access), ::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *phandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenSharedHandle<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayDeviceInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nthandle: super::super::super::Foundation::HANDLE, riid: ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OpenSharedHandle(::core::mem::transmute_copy(&nthandle), ::core::mem::transmute(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvobj = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateSharedHandle: CreateSharedHandle::<Identity, Impl, OFFSET>,
            OpenSharedHandle: OpenSharedHandle::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayDeviceInterop as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDisplayPathInterop_Impl: Sized {
    fn CreateSourcePresentationHandle(&self) -> ::windows::core::Result<super::super::super::Foundation::HANDLE>;
    fn GetSourceId(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDisplayPathInterop_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayPathInterop_Impl, const OFFSET: isize>() -> IDisplayPathInterop_Vtbl {
        unsafe extern "system" fn CreateSourcePresentationHandle<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayPathInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateSourcePresentationHandle() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceId<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayPathInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psourceid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSourceId() {
                ::core::result::Result::Ok(ok__) => {
                    *psourceid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateSourcePresentationHandle: CreateSourcePresentationHandle::<Identity, Impl, OFFSET>,
            GetSourceId: GetSourceId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayPathInterop as ::windows::core::Interface>::IID
    }
}
