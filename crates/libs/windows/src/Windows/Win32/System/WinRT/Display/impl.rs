#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub trait IDisplayDeviceInteropImpl: Sized {
    fn CreateSharedHandle();
    fn OpenSharedHandle();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl IDisplayDeviceInteropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayDeviceInteropImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayDeviceInteropVtbl {
        unsafe extern "system" fn CreateSharedHandle<Impl: IDisplayDeviceInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobject: *mut ::core::ffi::c_void, psecurityattributes: *const super::super::super::Security::SECURITY_ATTRIBUTES, access: u32, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, phandle: *mut super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OpenSharedHandle<Impl: IDisplayDeviceInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nthandle: super::super::super::Foundation::HANDLE, riid: ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateSharedHandle::<Impl, IMPL_OFFSET>, OpenSharedHandle::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayDeviceInterop as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDisplayPathInteropImpl: Sized {
    fn CreateSourcePresentationHandle();
    fn GetSourceId();
}
#[cfg(feature = "Win32_Foundation")]
impl IDisplayPathInteropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayPathInteropImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayPathInteropVtbl {
        unsafe extern "system" fn CreateSourcePresentationHandle<Impl: IDisplayPathInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSourceId<Impl: IDisplayPathInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psourceid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateSourcePresentationHandle::<Impl, IMPL_OFFSET>, GetSourceId::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayPathInterop as ::windows::core::Interface>::IID
    }
}
