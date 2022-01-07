pub trait IDisplayDeviceInteropImpl: Sized {
    fn CreateSharedHandle();
    fn OpenSharedHandle();
}
impl ::windows::core::RuntimeName for IDisplayDeviceInterop {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Display.IDisplayDeviceInterop";
}
impl IDisplayDeviceInteropVtbl {
    pub const fn new<Impl: IDisplayDeviceInteropImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDisplayDeviceInteropVtbl {
        unsafe extern "system" fn CreateSharedHandle<Impl: IDisplayDeviceInteropImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pobject: *mut ::core::ffi::c_void, psecurityattributes: *const super::super::super::Security::SECURITY_ATTRIBUTES, access: u32, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, phandle: *mut super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateSharedHandle(
                &*(&pobject as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                &*(&psecurityattributes as *const <super::super::super::Security::SECURITY_ATTRIBUTES as ::windows::core::Abi>::Abi as *const <super::super::super::Security::SECURITY_ATTRIBUTES as ::windows::core::DefaultType>::DefaultType),
                access,
                &*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&phandle),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenSharedHandle<Impl: IDisplayDeviceInteropImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nthandle: super::super::super::Foundation::HANDLE, riid: ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenSharedHandle(&*(&nthandle as *const <super::super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppvobj)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDisplayDeviceInterop>, base.5, CreateSharedHandle::<Impl, OFFSET>, OpenSharedHandle::<Impl, OFFSET>)
    }
}
pub trait IDisplayPathInteropImpl: Sized {
    fn CreateSourcePresentationHandle();
    fn GetSourceId();
}
impl ::windows::core::RuntimeName for IDisplayPathInterop {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Display.IDisplayPathInterop";
}
impl IDisplayPathInteropVtbl {
    pub const fn new<Impl: IDisplayPathInteropImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDisplayPathInteropVtbl {
        unsafe extern "system" fn CreateSourcePresentationHandle<Impl: IDisplayPathInteropImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateSourcePresentationHandle(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceId<Impl: IDisplayPathInteropImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psourceid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSourceId(::core::mem::transmute_copy(&psourceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDisplayPathInterop>, base.5, CreateSourcePresentationHandle::<Impl, OFFSET>, GetSourceId::<Impl, OFFSET>)
    }
}
