#[cfg(feature = "Win32_Foundation")]
pub trait IPrintManagerInteropImpl: Sized {
    fn GetForWindow(&mut self, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, printmanager: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn ShowPrintUIForWindowAsync(&mut self, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IPrintManagerInterop {
    const NAME: &'static str = "";
}
#[cfg(feature = "Win32_Foundation")]
impl IPrintManagerInteropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintManagerInteropImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintManagerInteropVtbl {
        unsafe extern "system" fn GetForWindow<Impl: IPrintManagerInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, printmanager: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetForWindow(::core::mem::transmute_copy(&appwindow), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&printmanager)).into()
        }
        unsafe extern "system" fn ShowPrintUIForWindowAsync<Impl: IPrintManagerInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowPrintUIForWindowAsync(::core::mem::transmute_copy(&appwindow), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&asyncoperation)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintManagerInterop, BASE_OFFSET>(),
            GetForWindow: GetForWindow::<Impl, IMPL_OFFSET>,
            ShowPrintUIForWindowAsync: ShowPrintUIForWindowAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintManagerInterop as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com"))]
pub trait IPrintWorkflowConfigurationNativeImpl: Sized {
    fn PrinterQueue(&mut self) -> ::windows::core::Result<super::super::super::Graphics::Printing::IPrinterQueue>;
    fn DriverProperties(&mut self) -> ::windows::core::Result<super::super::super::Graphics::Printing::IPrinterPropertyBag>;
    fn UserProperties(&mut self) -> ::windows::core::Result<super::super::super::Graphics::Printing::IPrinterPropertyBag>;
}
#[cfg(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com"))]
impl IPrintWorkflowConfigurationNativeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowConfigurationNativeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintWorkflowConfigurationNativeVtbl {
        unsafe extern "system" fn PrinterQueue<Impl: IPrintWorkflowConfigurationNativeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrinterQueue() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverProperties<Impl: IPrintWorkflowConfigurationNativeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DriverProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserProperties<Impl: IPrintWorkflowConfigurationNativeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            PrinterQueue: PrinterQueue::<Impl, IMPL_OFFSET>,
            DriverProperties: DriverProperties::<Impl, IMPL_OFFSET>,
            UserProperties: UserProperties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintWorkflowConfigurationNative as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_Xps")]
pub trait IPrintWorkflowObjectModelSourceFileContentNativeImpl: Sized {
    fn StartXpsOMGeneration(&mut self, receiver: ::core::option::Option<IPrintWorkflowXpsReceiver>) -> ::windows::core::Result<()>;
    fn ObjectFactory(&mut self) -> ::windows::core::Result<super::super::super::Storage::Xps::IXpsOMObjectFactory1>;
}
#[cfg(feature = "Win32_Storage_Xps")]
impl IPrintWorkflowObjectModelSourceFileContentNativeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowObjectModelSourceFileContentNativeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintWorkflowObjectModelSourceFileContentNativeVtbl {
        unsafe extern "system" fn StartXpsOMGeneration<Impl: IPrintWorkflowObjectModelSourceFileContentNativeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, receiver: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartXpsOMGeneration(::core::mem::transmute(&receiver)).into()
        }
        unsafe extern "system" fn ObjectFactory<Impl: IPrintWorkflowObjectModelSourceFileContentNativeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ObjectFactory() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            StartXpsOMGeneration: StartXpsOMGeneration::<Impl, IMPL_OFFSET>,
            ObjectFactory: ObjectFactory::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintWorkflowObjectModelSourceFileContentNative as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_Xps")]
pub trait IPrintWorkflowXpsObjectModelTargetPackageNativeImpl: Sized {
    fn DocumentPackageTarget(&mut self) -> ::windows::core::Result<super::super::super::Storage::Xps::IXpsDocumentPackageTarget>;
}
#[cfg(feature = "Win32_Storage_Xps")]
impl IPrintWorkflowXpsObjectModelTargetPackageNativeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowXpsObjectModelTargetPackageNativeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintWorkflowXpsObjectModelTargetPackageNativeVtbl {
        unsafe extern "system" fn DocumentPackageTarget<Impl: IPrintWorkflowXpsObjectModelTargetPackageNativeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DocumentPackageTarget() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), DocumentPackageTarget: DocumentPackageTarget::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintWorkflowXpsObjectModelTargetPackageNative as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
pub trait IPrintWorkflowXpsReceiverImpl: Sized {
    fn SetDocumentSequencePrintTicket(&mut self, documentsequenceprintticket: ::core::option::Option<super::super::Com::IStream>) -> ::windows::core::Result<()>;
    fn SetDocumentSequenceUri(&mut self, documentsequenceuri: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn AddDocumentData(&mut self, documentid: u32, documentprintticket: ::core::option::Option<super::super::Com::IStream>, documenturi: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn AddPage(&mut self, documentid: u32, pageid: u32, pagereference: ::core::option::Option<super::super::super::Storage::Xps::IXpsOMPageReference>, pageuri: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
impl IPrintWorkflowXpsReceiverVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowXpsReceiverImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintWorkflowXpsReceiverVtbl {
        unsafe extern "system" fn SetDocumentSequencePrintTicket<Impl: IPrintWorkflowXpsReceiverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documentsequenceprintticket: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDocumentSequencePrintTicket(::core::mem::transmute(&documentsequenceprintticket)).into()
        }
        unsafe extern "system" fn SetDocumentSequenceUri<Impl: IPrintWorkflowXpsReceiverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documentsequenceuri: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDocumentSequenceUri(::core::mem::transmute_copy(&documentsequenceuri)).into()
        }
        unsafe extern "system" fn AddDocumentData<Impl: IPrintWorkflowXpsReceiverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documentid: u32, documentprintticket: ::windows::core::RawPtr, documenturi: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddDocumentData(::core::mem::transmute_copy(&documentid), ::core::mem::transmute(&documentprintticket), ::core::mem::transmute_copy(&documenturi)).into()
        }
        unsafe extern "system" fn AddPage<Impl: IPrintWorkflowXpsReceiverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documentid: u32, pageid: u32, pagereference: ::windows::core::RawPtr, pageuri: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPage(::core::mem::transmute_copy(&documentid), ::core::mem::transmute_copy(&pageid), ::core::mem::transmute(&pagereference), ::core::mem::transmute_copy(&pageuri)).into()
        }
        unsafe extern "system" fn Close<Impl: IPrintWorkflowXpsReceiverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetDocumentSequencePrintTicket: SetDocumentSequencePrintTicket::<Impl, IMPL_OFFSET>,
            SetDocumentSequenceUri: SetDocumentSequenceUri::<Impl, IMPL_OFFSET>,
            AddDocumentData: AddDocumentData::<Impl, IMPL_OFFSET>,
            AddPage: AddPage::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintWorkflowXpsReceiver as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
pub trait IPrintWorkflowXpsReceiver2Impl: Sized + IPrintWorkflowXpsReceiverImpl {
    fn Failed(&mut self, xpserror: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
impl IPrintWorkflowXpsReceiver2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowXpsReceiver2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintWorkflowXpsReceiver2Vtbl {
        unsafe extern "system" fn Failed<Impl: IPrintWorkflowXpsReceiver2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xpserror: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Failed(::core::mem::transmute_copy(&xpserror)).into()
        }
        Self { base: IPrintWorkflowXpsReceiverVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Failed: Failed::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintWorkflowXpsReceiver2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPrinting3DManagerInteropImpl: Sized {
    fn GetForWindow(&mut self, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, printmanager: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn ShowPrintUIForWindowAsync(&mut self, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IPrinting3DManagerInterop {
    const NAME: &'static str = "";
}
#[cfg(feature = "Win32_Foundation")]
impl IPrinting3DManagerInteropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DManagerInteropImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrinting3DManagerInteropVtbl {
        unsafe extern "system" fn GetForWindow<Impl: IPrinting3DManagerInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, printmanager: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetForWindow(::core::mem::transmute_copy(&appwindow), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&printmanager)).into()
        }
        unsafe extern "system" fn ShowPrintUIForWindowAsync<Impl: IPrinting3DManagerInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowPrintUIForWindowAsync(::core::mem::transmute_copy(&appwindow), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&asyncoperation)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrinting3DManagerInterop, BASE_OFFSET>(),
            GetForWindow: GetForWindow::<Impl, IMPL_OFFSET>,
            ShowPrintUIForWindowAsync: ShowPrintUIForWindowAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinting3DManagerInterop as ::windows::core::Interface>::IID
    }
}
