#[cfg(feature = "Win32_Foundation")]
pub trait IRadialControllerConfigurationInteropImpl: Sized {
    fn GetForWindow();
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IRadialControllerConfigurationInterop {
    const NAME: &'static str = "";
}
#[cfg(feature = "Win32_Foundation")]
impl IRadialControllerConfigurationInteropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRadialControllerConfigurationInteropImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRadialControllerConfigurationInteropVtbl {
        unsafe extern "system" fn GetForWindow<Impl: IRadialControllerConfigurationInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRadialControllerConfigurationInterop>, ::windows::core::GetTrustLevel, GetForWindow::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRadialControllerConfigurationInterop as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRadialControllerIndependentInputSourceInteropImpl: Sized {
    fn CreateForWindow();
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IRadialControllerIndependentInputSourceInterop {
    const NAME: &'static str = "";
}
#[cfg(feature = "Win32_Foundation")]
impl IRadialControllerIndependentInputSourceInteropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRadialControllerIndependentInputSourceInteropImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRadialControllerIndependentInputSourceInteropVtbl {
        unsafe extern "system" fn CreateForWindow<Impl: IRadialControllerIndependentInputSourceInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRadialControllerIndependentInputSourceInterop>, ::windows::core::GetTrustLevel, CreateForWindow::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRadialControllerIndependentInputSourceInterop as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRadialControllerInteropImpl: Sized {
    fn CreateForWindow();
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IRadialControllerInterop {
    const NAME: &'static str = "";
}
#[cfg(feature = "Win32_Foundation")]
impl IRadialControllerInteropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRadialControllerInteropImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRadialControllerInteropVtbl {
        unsafe extern "system" fn CreateForWindow<Impl: IRadialControllerInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRadialControllerInterop>, ::windows::core::GetTrustLevel, CreateForWindow::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRadialControllerInterop as ::windows::core::Interface>::IID
    }
}
