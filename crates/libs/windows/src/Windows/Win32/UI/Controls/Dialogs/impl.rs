#[cfg(feature = "Win32_Foundation")]
pub trait IPrintDialogCallbackImpl: Sized {
    fn InitDone(&mut self) -> ::windows::core::Result<()>;
    fn SelectionChange(&mut self) -> ::windows::core::Result<()>;
    fn HandleMessage(&mut self, hdlg: super::super::super::Foundation::HWND, umsg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, presult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPrintDialogCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintDialogCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintDialogCallbackVtbl {
        unsafe extern "system" fn InitDone<Impl: IPrintDialogCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitDone().into()
        }
        unsafe extern "system" fn SelectionChange<Impl: IPrintDialogCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SelectionChange().into()
        }
        unsafe extern "system" fn HandleMessage<Impl: IPrintDialogCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdlg: super::super::super::Foundation::HWND, umsg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, presult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HandleMessage(::core::mem::transmute_copy(&hdlg), ::core::mem::transmute_copy(&umsg), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam), ::core::mem::transmute_copy(&presult)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            InitDone: InitDone::<Impl, IMPL_OFFSET>,
            SelectionChange: SelectionChange::<Impl, IMPL_OFFSET>,
            HandleMessage: HandleMessage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintDialogCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IPrintDialogServicesImpl: Sized {
    fn GetCurrentDevMode(&mut self, pdevmode: *mut super::super::super::Graphics::Gdi::DEVMODEA, pcbsize: *mut u32) -> ::windows::core::Result<()>;
    fn GetCurrentPrinterName(&mut self, pprintername: super::super::super::Foundation::PWSTR, pcchsize: *mut u32) -> ::windows::core::Result<()>;
    fn GetCurrentPortName(&mut self, pportname: super::super::super::Foundation::PWSTR, pcchsize: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IPrintDialogServicesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintDialogServicesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintDialogServicesVtbl {
        unsafe extern "system" fn GetCurrentDevMode<Impl: IPrintDialogServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevmode: *mut super::super::super::Graphics::Gdi::DEVMODEA, pcbsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCurrentDevMode(::core::mem::transmute_copy(&pdevmode), ::core::mem::transmute_copy(&pcbsize)).into()
        }
        unsafe extern "system" fn GetCurrentPrinterName<Impl: IPrintDialogServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprintername: super::super::super::Foundation::PWSTR, pcchsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCurrentPrinterName(::core::mem::transmute_copy(&pprintername), ::core::mem::transmute_copy(&pcchsize)).into()
        }
        unsafe extern "system" fn GetCurrentPortName<Impl: IPrintDialogServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pportname: super::super::super::Foundation::PWSTR, pcchsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCurrentPortName(::core::mem::transmute_copy(&pportname), ::core::mem::transmute_copy(&pcchsize)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCurrentDevMode: GetCurrentDevMode::<Impl, IMPL_OFFSET>,
            GetCurrentPrinterName: GetCurrentPrinterName::<Impl, IMPL_OFFSET>,
            GetCurrentPortName: GetCurrentPortName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintDialogServices as ::windows::core::Interface>::IID
    }
}
