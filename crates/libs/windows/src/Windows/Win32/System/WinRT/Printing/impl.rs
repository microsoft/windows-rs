#[cfg(feature = "Win32_Foundation")]
pub trait IPrintManagerInterop_Impl: Sized {
    fn GetForWindow(&self, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, printmanager: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn ShowPrintUIForWindowAsync(&self, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IPrintManagerInterop {
    const NAME: &'static str = "";
}
#[cfg(feature = "Win32_Foundation")]
impl IPrintManagerInterop_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintManagerInterop_Impl, const OFFSET: isize>() -> IPrintManagerInterop_Vtbl {
        unsafe extern "system" fn GetForWindow<Identity: ::windows::core::IUnknownImpl, Impl: IPrintManagerInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, printmanager: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetForWindow(::core::mem::transmute_copy(&appwindow), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&printmanager)).into()
        }
        unsafe extern "system" fn ShowPrintUIForWindowAsync<Identity: ::windows::core::IUnknownImpl, Impl: IPrintManagerInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ShowPrintUIForWindowAsync(::core::mem::transmute_copy(&appwindow), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&asyncoperation)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintManagerInterop, OFFSET>(),
            GetForWindow: GetForWindow::<Identity, Impl, OFFSET>,
            ShowPrintUIForWindowAsync: ShowPrintUIForWindowAsync::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintManagerInterop as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com"))]
pub trait IPrintWorkflowConfigurationNative_Impl: Sized {
    fn PrinterQueue(&self) -> ::windows::core::Result<super::super::super::Graphics::Printing::IPrinterQueue>;
    fn DriverProperties(&self) -> ::windows::core::Result<super::super::super::Graphics::Printing::IPrinterPropertyBag>;
    fn UserProperties(&self) -> ::windows::core::Result<super::super::super::Graphics::Printing::IPrinterPropertyBag>;
}
#[cfg(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com"))]
impl IPrintWorkflowConfigurationNative_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowConfigurationNative_Impl, const OFFSET: isize>() -> IPrintWorkflowConfigurationNative_Vtbl {
        unsafe extern "system" fn PrinterQueue<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowConfigurationNative_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PrinterQueue() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverProperties<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowConfigurationNative_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DriverProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserProperties<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowConfigurationNative_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UserProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            PrinterQueue: PrinterQueue::<Identity, Impl, OFFSET>,
            DriverProperties: DriverProperties::<Identity, Impl, OFFSET>,
            UserProperties: UserProperties::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintWorkflowConfigurationNative as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_Xps")]
pub trait IPrintWorkflowObjectModelSourceFileContentNative_Impl: Sized {
    fn StartXpsOMGeneration(&self, receiver: &::core::option::Option<IPrintWorkflowXpsReceiver>) -> ::windows::core::Result<()>;
    fn ObjectFactory(&self) -> ::windows::core::Result<super::super::super::Storage::Xps::IXpsOMObjectFactory1>;
}
#[cfg(feature = "Win32_Storage_Xps")]
impl IPrintWorkflowObjectModelSourceFileContentNative_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowObjectModelSourceFileContentNative_Impl, const OFFSET: isize>() -> IPrintWorkflowObjectModelSourceFileContentNative_Vtbl {
        unsafe extern "system" fn StartXpsOMGeneration<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowObjectModelSourceFileContentNative_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, receiver: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StartXpsOMGeneration(::core::mem::transmute(&receiver)).into()
        }
        unsafe extern "system" fn ObjectFactory<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowObjectModelSourceFileContentNative_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ObjectFactory() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            StartXpsOMGeneration: StartXpsOMGeneration::<Identity, Impl, OFFSET>,
            ObjectFactory: ObjectFactory::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintWorkflowObjectModelSourceFileContentNative as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_Xps")]
pub trait IPrintWorkflowXpsObjectModelTargetPackageNative_Impl: Sized {
    fn DocumentPackageTarget(&self) -> ::windows::core::Result<super::super::super::Storage::Xps::IXpsDocumentPackageTarget>;
}
#[cfg(feature = "Win32_Storage_Xps")]
impl IPrintWorkflowXpsObjectModelTargetPackageNative_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowXpsObjectModelTargetPackageNative_Impl, const OFFSET: isize>() -> IPrintWorkflowXpsObjectModelTargetPackageNative_Vtbl {
        unsafe extern "system" fn DocumentPackageTarget<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowXpsObjectModelTargetPackageNative_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DocumentPackageTarget() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), DocumentPackageTarget: DocumentPackageTarget::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintWorkflowXpsObjectModelTargetPackageNative as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
pub trait IPrintWorkflowXpsReceiver_Impl: Sized {
    fn SetDocumentSequencePrintTicket(&self, documentsequenceprintticket: &::core::option::Option<super::super::Com::IStream>) -> ::windows::core::Result<()>;
    fn SetDocumentSequenceUri(&self, documentsequenceuri: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn AddDocumentData(&self, documentid: u32, documentprintticket: &::core::option::Option<super::super::Com::IStream>, documenturi: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn AddPage(&self, documentid: u32, pageid: u32, pagereference: &::core::option::Option<super::super::super::Storage::Xps::IXpsOMPageReference>, pageuri: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn Close(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
impl IPrintWorkflowXpsReceiver_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowXpsReceiver_Impl, const OFFSET: isize>() -> IPrintWorkflowXpsReceiver_Vtbl {
        unsafe extern "system" fn SetDocumentSequencePrintTicket<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowXpsReceiver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documentsequenceprintticket: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDocumentSequencePrintTicket(::core::mem::transmute(&documentsequenceprintticket)).into()
        }
        unsafe extern "system" fn SetDocumentSequenceUri<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowXpsReceiver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documentsequenceuri: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDocumentSequenceUri(::core::mem::transmute(&documentsequenceuri)).into()
        }
        unsafe extern "system" fn AddDocumentData<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowXpsReceiver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documentid: u32, documentprintticket: ::windows::core::RawPtr, documenturi: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddDocumentData(::core::mem::transmute_copy(&documentid), ::core::mem::transmute(&documentprintticket), ::core::mem::transmute(&documenturi)).into()
        }
        unsafe extern "system" fn AddPage<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowXpsReceiver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documentid: u32, pageid: u32, pagereference: ::windows::core::RawPtr, pageuri: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddPage(::core::mem::transmute_copy(&documentid), ::core::mem::transmute_copy(&pageid), ::core::mem::transmute(&pagereference), ::core::mem::transmute(&pageuri)).into()
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowXpsReceiver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Close().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetDocumentSequencePrintTicket: SetDocumentSequencePrintTicket::<Identity, Impl, OFFSET>,
            SetDocumentSequenceUri: SetDocumentSequenceUri::<Identity, Impl, OFFSET>,
            AddDocumentData: AddDocumentData::<Identity, Impl, OFFSET>,
            AddPage: AddPage::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintWorkflowXpsReceiver as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
pub trait IPrintWorkflowXpsReceiver2_Impl: Sized + IPrintWorkflowXpsReceiver_Impl {
    fn Failed(&self, xpserror: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
impl IPrintWorkflowXpsReceiver2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowXpsReceiver2_Impl, const OFFSET: isize>() -> IPrintWorkflowXpsReceiver2_Vtbl {
        unsafe extern "system" fn Failed<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowXpsReceiver2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xpserror: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Failed(::core::mem::transmute_copy(&xpserror)).into()
        }
        Self { base: IPrintWorkflowXpsReceiver_Vtbl::new::<Identity, Impl, OFFSET>(), Failed: Failed::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintWorkflowXpsReceiver2 as ::windows::core::Interface>::IID || iid == &<IPrintWorkflowXpsReceiver as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPrinting3DManagerInterop_Impl: Sized {
    fn GetForWindow(&self, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, printmanager: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn ShowPrintUIForWindowAsync(&self, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IPrinting3DManagerInterop {
    const NAME: &'static str = "";
}
#[cfg(feature = "Win32_Foundation")]
impl IPrinting3DManagerInterop_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DManagerInterop_Impl, const OFFSET: isize>() -> IPrinting3DManagerInterop_Vtbl {
        unsafe extern "system" fn GetForWindow<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DManagerInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, printmanager: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetForWindow(::core::mem::transmute_copy(&appwindow), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&printmanager)).into()
        }
        unsafe extern "system" fn ShowPrintUIForWindowAsync<Identity: ::windows::core::IUnknownImpl, Impl: IPrinting3DManagerInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ShowPrintUIForWindowAsync(::core::mem::transmute_copy(&appwindow), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&asyncoperation)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrinting3DManagerInterop, OFFSET>(),
            GetForWindow: GetForWindow::<Identity, Impl, OFFSET>,
            ShowPrintUIForWindowAsync: ShowPrintUIForWindowAsync::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrinting3DManagerInterop as ::windows::core::Interface>::IID
    }
}
