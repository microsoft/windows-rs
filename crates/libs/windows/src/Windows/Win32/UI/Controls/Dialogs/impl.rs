#[cfg(feature = "Win32_Foundation")]
pub trait IPrintDialogCallback_Impl: Sized {
    fn InitDone(&self) -> ::windows::core::Result<()>;
    fn SelectionChange(&self) -> ::windows::core::Result<()>;
    fn HandleMessage(&self, hdlg: super::super::super::Foundation::HWND, umsg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, presult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IPrintDialogCallback {}
#[cfg(feature = "Win32_Foundation")]
impl IPrintDialogCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrintDialogCallback_Impl, const OFFSET: isize>() -> IPrintDialogCallback_Vtbl {
        unsafe extern "system" fn InitDone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrintDialogCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitDone().into()
        }
        unsafe extern "system" fn SelectionChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrintDialogCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SelectionChange().into()
        }
        unsafe extern "system" fn HandleMessage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrintDialogCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdlg: super::super::super::Foundation::HWND, umsg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, presult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.HandleMessage(::core::mem::transmute_copy(&hdlg), ::core::mem::transmute_copy(&umsg), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam), ::core::mem::transmute_copy(&presult)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            InitDone: InitDone::<Identity, Impl, OFFSET>,
            SelectionChange: SelectionChange::<Identity, Impl, OFFSET>,
            HandleMessage: HandleMessage::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintDialogCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IPrintDialogServices_Impl: Sized {
    fn GetCurrentDevMode(&self, pdevmode: *mut super::super::super::Graphics::Gdi::DEVMODEA, pcbsize: *mut u32) -> ::windows::core::Result<()>;
    fn GetCurrentPrinterName(&self, pprintername: ::windows::core::PWSTR, pcchsize: *mut u32) -> ::windows::core::Result<()>;
    fn GetCurrentPortName(&self, pportname: ::windows::core::PWSTR, pcchsize: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::RuntimeName for IPrintDialogServices {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IPrintDialogServices_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrintDialogServices_Impl, const OFFSET: isize>() -> IPrintDialogServices_Vtbl {
        unsafe extern "system" fn GetCurrentDevMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrintDialogServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevmode: *mut super::super::super::Graphics::Gdi::DEVMODEA, pcbsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCurrentDevMode(::core::mem::transmute_copy(&pdevmode), ::core::mem::transmute_copy(&pcbsize)).into()
        }
        unsafe extern "system" fn GetCurrentPrinterName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrintDialogServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprintername: ::windows::core::PWSTR, pcchsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCurrentPrinterName(::core::mem::transmute_copy(&pprintername), ::core::mem::transmute_copy(&pcchsize)).into()
        }
        unsafe extern "system" fn GetCurrentPortName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrintDialogServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pportname: ::windows::core::PWSTR, pcchsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCurrentPortName(::core::mem::transmute_copy(&pportname), ::core::mem::transmute_copy(&pcchsize)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetCurrentDevMode: GetCurrentDevMode::<Identity, Impl, OFFSET>,
            GetCurrentPrinterName: GetCurrentPrinterName::<Identity, Impl, OFFSET>,
            GetCurrentPortName: GetCurrentPortName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintDialogServices as ::windows::core::Interface>::IID
    }
}
