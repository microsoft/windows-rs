#[cfg(feature = "Win32_Foundation")]
pub trait IPrintDialogCallbackImpl: Sized {
    fn InitDone();
    fn SelectionChange();
    fn HandleMessage();
}
#[cfg(feature = "Win32_Foundation")]
impl IPrintDialogCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintDialogCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintDialogCallbackVtbl {
        unsafe extern "system" fn InitDone<Impl: IPrintDialogCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SelectionChange<Impl: IPrintDialogCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HandleMessage<Impl: IPrintDialogCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdlg: super::super::super::Foundation::HWND, umsg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, presult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, InitDone::<Impl, IMPL_OFFSET>, SelectionChange::<Impl, IMPL_OFFSET>, HandleMessage::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintDialogCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IPrintDialogServicesImpl: Sized {
    fn GetCurrentDevMode();
    fn GetCurrentPrinterName();
    fn GetCurrentPortName();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IPrintDialogServicesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintDialogServicesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintDialogServicesVtbl {
        unsafe extern "system" fn GetCurrentDevMode<Impl: IPrintDialogServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevmode: *mut super::super::super::Graphics::Gdi::DEVMODEA, pcbsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrentPrinterName<Impl: IPrintDialogServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprintername: super::super::super::Foundation::PWSTR, pcchsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrentPortName<Impl: IPrintDialogServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pportname: super::super::super::Foundation::PWSTR, pcchsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetCurrentDevMode::<Impl, IMPL_OFFSET>, GetCurrentPrinterName::<Impl, IMPL_OFFSET>, GetCurrentPortName::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintDialogServices as ::windows::core::Interface>::IID
    }
}
