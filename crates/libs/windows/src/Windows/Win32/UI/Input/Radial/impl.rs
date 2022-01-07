pub trait IRadialControllerConfigurationInteropImpl: Sized {
    fn GetForWindow();
}
impl ::windows::core::RuntimeName for IRadialControllerConfigurationInterop {
    const NAME: &'static str = "Windows.Win32.UI.Input.Radial.IRadialControllerConfigurationInterop";
}
impl IRadialControllerConfigurationInteropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRadialControllerConfigurationInteropImpl, const OFFSET: isize>() -> IRadialControllerConfigurationInteropVtbl {
        unsafe extern "system" fn GetForWindow<Impl: IRadialControllerConfigurationInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForWindow(&*(&hwnd as *const <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRadialControllerConfigurationInterop>, ::windows::core::GetTrustLevel, GetForWindow::<Impl, OFFSET>)
    }
}
pub trait IRadialControllerIndependentInputSourceInteropImpl: Sized {
    fn CreateForWindow();
}
impl ::windows::core::RuntimeName for IRadialControllerIndependentInputSourceInterop {
    const NAME: &'static str = "Windows.Win32.UI.Input.Radial.IRadialControllerIndependentInputSourceInterop";
}
impl IRadialControllerIndependentInputSourceInteropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRadialControllerIndependentInputSourceInteropImpl, const OFFSET: isize>() -> IRadialControllerIndependentInputSourceInteropVtbl {
        unsafe extern "system" fn CreateForWindow<Impl: IRadialControllerIndependentInputSourceInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateForWindow(&*(&hwnd as *const <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRadialControllerIndependentInputSourceInterop>, ::windows::core::GetTrustLevel, CreateForWindow::<Impl, OFFSET>)
    }
}
pub trait IRadialControllerInteropImpl: Sized {
    fn CreateForWindow();
}
impl ::windows::core::RuntimeName for IRadialControllerInterop {
    const NAME: &'static str = "Windows.Win32.UI.Input.Radial.IRadialControllerInterop";
}
impl IRadialControllerInteropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRadialControllerInteropImpl, const OFFSET: isize>() -> IRadialControllerInteropVtbl {
        unsafe extern "system" fn CreateForWindow<Impl: IRadialControllerInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateForWindow(&*(&hwnd as *const <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRadialControllerInterop>, ::windows::core::GetTrustLevel, CreateForWindow::<Impl, OFFSET>)
    }
}
