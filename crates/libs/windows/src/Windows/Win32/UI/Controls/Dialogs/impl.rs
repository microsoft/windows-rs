pub trait IPrintDialogCallbackImpl: Sized {
    fn InitDone();
    fn SelectionChange();
    fn HandleMessage();
}
impl ::windows::core::RuntimeName for IPrintDialogCallback {
    const NAME: &'static str = "Windows.Win32.UI.Controls.Dialogs.IPrintDialogCallback";
}
impl IPrintDialogCallbackVtbl {
    pub const fn new<Impl: IPrintDialogCallbackImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrintDialogCallbackVtbl {
        unsafe extern "system" fn InitDone<Impl: IPrintDialogCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InitDone() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectionChange<Impl: IPrintDialogCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectionChange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HandleMessage<Impl: IPrintDialogCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hdlg: super::super::super::Foundation::HWND, umsg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, presult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HandleMessage(
                &*(&hdlg as *const <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                umsg,
                &*(&wparam as *const <super::super::super::Foundation::WPARAM as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::WPARAM as ::windows::core::DefaultType>::DefaultType),
                &*(&lparam as *const <super::super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType),
                &*(&presult as *const <super::super::super::Foundation::LRESULT as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::LRESULT as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrintDialogCallback>, base.5, InitDone::<Impl, OFFSET>, SelectionChange::<Impl, OFFSET>, HandleMessage::<Impl, OFFSET>)
    }
}
pub trait IPrintDialogServicesImpl: Sized {
    fn GetCurrentDevMode();
    fn GetCurrentPrinterName();
    fn GetCurrentPortName();
}
impl ::windows::core::RuntimeName for IPrintDialogServices {
    const NAME: &'static str = "Windows.Win32.UI.Controls.Dialogs.IPrintDialogServices";
}
impl IPrintDialogServicesVtbl {
    pub const fn new<Impl: IPrintDialogServicesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrintDialogServicesVtbl {
        unsafe extern "system" fn GetCurrentDevMode<Impl: IPrintDialogServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdevmode: *mut super::super::super::Graphics::Gdi::DEVMODEA, pcbsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCurrentDevMode(&*(&pdevmode as *const <super::super::super::Graphics::Gdi::DEVMODEA as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Gdi::DEVMODEA as ::windows::core::DefaultType>::DefaultType), pcbsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentPrinterName<Impl: IPrintDialogServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprintername: super::super::super::Foundation::PWSTR, pcchsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCurrentPrinterName(::core::mem::transmute_copy(&pprintername), pcchsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentPortName<Impl: IPrintDialogServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pportname: super::super::super::Foundation::PWSTR, pcchsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCurrentPortName(::core::mem::transmute_copy(&pportname), pcchsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrintDialogServices>, base.5, GetCurrentDevMode::<Impl, OFFSET>, GetCurrentPrinterName::<Impl, OFFSET>, GetCurrentPortName::<Impl, OFFSET>)
    }
}
