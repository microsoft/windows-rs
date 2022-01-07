pub trait IPrintManagerInteropImpl: Sized {
    fn GetForWindow();
    fn ShowPrintUIForWindowAsync();
}
impl ::windows::core::RuntimeName for IPrintManagerInterop {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Printing.IPrintManagerInterop";
}
impl IPrintManagerInteropVtbl {
    pub const fn new<Impl: IPrintManagerInteropImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrintManagerInteropVtbl {
        unsafe extern "system" fn GetForWindow<Impl: IPrintManagerInteropImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, printmanager: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForWindow(&*(&appwindow as *const <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&printmanager)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowPrintUIForWindowAsync<Impl: IPrintManagerInteropImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ShowPrintUIForWindowAsync(&*(&appwindow as *const <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&asyncoperation)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrintManagerInterop>, base.5, GetForWindow::<Impl, OFFSET>, ShowPrintUIForWindowAsync::<Impl, OFFSET>)
    }
}
pub trait IPrintWorkflowConfigurationNativeImpl: Sized {
    fn PrinterQueue();
    fn DriverProperties();
    fn UserProperties();
}
impl ::windows::core::RuntimeName for IPrintWorkflowConfigurationNative {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Printing.IPrintWorkflowConfigurationNative";
}
impl IPrintWorkflowConfigurationNativeVtbl {
    pub const fn new<Impl: IPrintWorkflowConfigurationNativeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrintWorkflowConfigurationNativeVtbl {
        unsafe extern "system" fn PrinterQueue<Impl: IPrintWorkflowConfigurationNativeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrinterQueue(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverProperties<Impl: IPrintWorkflowConfigurationNativeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DriverProperties(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserProperties<Impl: IPrintWorkflowConfigurationNativeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UserProperties(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrintWorkflowConfigurationNative>, base.5, PrinterQueue::<Impl, OFFSET>, DriverProperties::<Impl, OFFSET>, UserProperties::<Impl, OFFSET>)
    }
}
pub trait IPrintWorkflowObjectModelSourceFileContentNativeImpl: Sized {
    fn StartXpsOMGeneration();
    fn ObjectFactory();
}
impl ::windows::core::RuntimeName for IPrintWorkflowObjectModelSourceFileContentNative {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Printing.IPrintWorkflowObjectModelSourceFileContentNative";
}
impl IPrintWorkflowObjectModelSourceFileContentNativeVtbl {
    pub const fn new<Impl: IPrintWorkflowObjectModelSourceFileContentNativeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrintWorkflowObjectModelSourceFileContentNativeVtbl {
        unsafe extern "system" fn StartXpsOMGeneration<Impl: IPrintWorkflowObjectModelSourceFileContentNativeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, receiver: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartXpsOMGeneration(&*(&receiver as *const <IPrintWorkflowXpsReceiver as ::windows::core::Abi>::Abi as *const <IPrintWorkflowXpsReceiver as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ObjectFactory<Impl: IPrintWorkflowObjectModelSourceFileContentNativeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ObjectFactory(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrintWorkflowObjectModelSourceFileContentNative>, base.5, StartXpsOMGeneration::<Impl, OFFSET>, ObjectFactory::<Impl, OFFSET>)
    }
}
pub trait IPrintWorkflowXpsObjectModelTargetPackageNativeImpl: Sized {
    fn DocumentPackageTarget();
}
impl ::windows::core::RuntimeName for IPrintWorkflowXpsObjectModelTargetPackageNative {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Printing.IPrintWorkflowXpsObjectModelTargetPackageNative";
}
impl IPrintWorkflowXpsObjectModelTargetPackageNativeVtbl {
    pub const fn new<Impl: IPrintWorkflowXpsObjectModelTargetPackageNativeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrintWorkflowXpsObjectModelTargetPackageNativeVtbl {
        unsafe extern "system" fn DocumentPackageTarget<Impl: IPrintWorkflowXpsObjectModelTargetPackageNativeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DocumentPackageTarget(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrintWorkflowXpsObjectModelTargetPackageNative>, base.5, DocumentPackageTarget::<Impl, OFFSET>)
    }
}
pub trait IPrintWorkflowXpsReceiverImpl: Sized {
    fn SetDocumentSequencePrintTicket();
    fn SetDocumentSequenceUri();
    fn AddDocumentData();
    fn AddPage();
    fn Close();
}
impl ::windows::core::RuntimeName for IPrintWorkflowXpsReceiver {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Printing.IPrintWorkflowXpsReceiver";
}
impl IPrintWorkflowXpsReceiverVtbl {
    pub const fn new<Impl: IPrintWorkflowXpsReceiverImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrintWorkflowXpsReceiverVtbl {
        unsafe extern "system" fn SetDocumentSequencePrintTicket<Impl: IPrintWorkflowXpsReceiverImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, documentsequenceprintticket: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDocumentSequencePrintTicket(&*(&documentsequenceprintticket as *const <super::super::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::Com::IStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDocumentSequenceUri<Impl: IPrintWorkflowXpsReceiverImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, documentsequenceuri: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDocumentSequenceUri(&*(&documentsequenceuri as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddDocumentData<Impl: IPrintWorkflowXpsReceiverImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, documentid: u32, documentprintticket: ::windows::core::RawPtr, documenturi: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddDocumentData(documentid, &*(&documentprintticket as *const <super::super::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::Com::IStream as ::windows::core::DefaultType>::DefaultType), &*(&documenturi as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPage<Impl: IPrintWorkflowXpsReceiverImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, documentid: u32, pageid: u32, pagereference: ::windows::core::RawPtr, pageuri: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddPage(documentid, pageid, &*(&pagereference as *const <super::super::super::Storage::Xps::IXpsOMPageReference as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Xps::IXpsOMPageReference as ::windows::core::DefaultType>::DefaultType), &*(&pageuri as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: IPrintWorkflowXpsReceiverImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Close() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrintWorkflowXpsReceiver>, base.5, SetDocumentSequencePrintTicket::<Impl, OFFSET>, SetDocumentSequenceUri::<Impl, OFFSET>, AddDocumentData::<Impl, OFFSET>, AddPage::<Impl, OFFSET>, Close::<Impl, OFFSET>)
    }
}
pub trait IPrintWorkflowXpsReceiver2Impl: Sized + IPrintWorkflowXpsReceiverImpl {
    fn Failed();
}
impl ::windows::core::RuntimeName for IPrintWorkflowXpsReceiver2 {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Printing.IPrintWorkflowXpsReceiver2";
}
impl IPrintWorkflowXpsReceiver2Vtbl {
    pub const fn new<Impl: IPrintWorkflowXpsReceiver2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrintWorkflowXpsReceiver2Vtbl {
        unsafe extern "system" fn Failed<Impl: IPrintWorkflowXpsReceiver2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xpserror: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Failed(xpserror) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrintWorkflowXpsReceiver2>, base.5, Failed::<Impl, OFFSET>)
    }
}
pub trait IPrinting3DManagerInteropImpl: Sized {
    fn GetForWindow();
    fn ShowPrintUIForWindowAsync();
}
impl ::windows::core::RuntimeName for IPrinting3DManagerInterop {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Printing.IPrinting3DManagerInterop";
}
impl IPrinting3DManagerInteropVtbl {
    pub const fn new<Impl: IPrinting3DManagerInteropImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrinting3DManagerInteropVtbl {
        unsafe extern "system" fn GetForWindow<Impl: IPrinting3DManagerInteropImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, printmanager: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForWindow(&*(&appwindow as *const <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&printmanager)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowPrintUIForWindowAsync<Impl: IPrinting3DManagerInteropImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ShowPrintUIForWindowAsync(&*(&appwindow as *const <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&asyncoperation)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrinting3DManagerInterop>, base.5, GetForWindow::<Impl, OFFSET>, ShowPrintUIForWindowAsync::<Impl, OFFSET>)
    }
}
