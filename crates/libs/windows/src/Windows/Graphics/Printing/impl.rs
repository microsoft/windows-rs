pub trait IPrintDocumentSource_Impl: Sized {}
impl ::windows::core::RuntimeName for IPrintDocumentSource {
    const NAME: &'static str = "Windows.Graphics.Printing.IPrintDocumentSource";
}
impl IPrintDocumentSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintDocumentSource_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintDocumentSource_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintDocumentSource, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintDocumentSource as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPrintManager_Impl: Sized {
    fn PrintTaskRequested(&mut self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PrintManager, PrintTaskRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePrintTaskRequested(&mut self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrintManager {
    const NAME: &'static str = "Windows.Graphics.Printing.IPrintManager";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPrintManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintManager_Vtbl {
        unsafe extern "system" fn PrintTaskRequested<Impl: IPrintManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrintTaskRequested(&*(&eventhandler as *const <super::super::Foundation::TypedEventHandler<PrintManager, PrintTaskRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PrintManager, PrintTaskRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePrintTaskRequested<Impl: IPrintManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePrintTaskRequested(&*(&eventcookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintManager, BASE_OFFSET>(),
            PrintTaskRequested: PrintTaskRequested::<Impl, IMPL_OFFSET>,
            RemovePrintTaskRequested: RemovePrintTaskRequested::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPrintManagerStatic_Impl: Sized {
    fn GetForCurrentView(&mut self) -> ::windows::core::Result<PrintManager>;
    fn ShowPrintUIAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrintManagerStatic {
    const NAME: &'static str = "Windows.Graphics.Printing.IPrintManagerStatic";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPrintManagerStatic_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintManagerStatic_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintManagerStatic_Vtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: IPrintManagerStatic_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForCurrentView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowPrintUIAsync<Impl: IPrintManagerStatic_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowPrintUIAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintManagerStatic, BASE_OFFSET>(),
            GetForCurrentView: GetForCurrentView::<Impl, IMPL_OFFSET>,
            ShowPrintUIAsync: ShowPrintUIAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintManagerStatic as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintManagerStatic2_Impl: Sized {
    fn IsSupported(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintManagerStatic2 {
    const NAME: &'static str = "Windows.Graphics.Printing.IPrintManagerStatic2";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintManagerStatic2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintManagerStatic2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintManagerStatic2_Vtbl {
        unsafe extern "system" fn IsSupported<Impl: IPrintManagerStatic2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintManagerStatic2, BASE_OFFSET>(), IsSupported: IsSupported::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintManagerStatic2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPrintPageInfo_Impl: Sized {
    fn SetMediaSize(&mut self, value: PrintMediaSize) -> ::windows::core::Result<()>;
    fn MediaSize(&mut self) -> ::windows::core::Result<PrintMediaSize>;
    fn SetPageSize(&mut self, value: &super::super::Foundation::Size) -> ::windows::core::Result<()>;
    fn PageSize(&mut self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn SetDpiX(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn DpiX(&mut self) -> ::windows::core::Result<u32>;
    fn SetDpiY(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn DpiY(&mut self) -> ::windows::core::Result<u32>;
    fn SetOrientation(&mut self, value: PrintOrientation) -> ::windows::core::Result<()>;
    fn Orientation(&mut self) -> ::windows::core::Result<PrintOrientation>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrintPageInfo {
    const NAME: &'static str = "Windows.Graphics.Printing.IPrintPageInfo";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPrintPageInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintPageInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintPageInfo_Vtbl {
        unsafe extern "system" fn SetMediaSize<Impl: IPrintPageInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PrintMediaSize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMediaSize(value).into()
        }
        unsafe extern "system" fn MediaSize<Impl: IPrintPageInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintMediaSize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPageSize<Impl: IPrintPageInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPageSize(&*(&value as *const <super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PageSize<Impl: IPrintPageInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PageSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDpiX<Impl: IPrintPageInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDpiX(value).into()
        }
        unsafe extern "system" fn DpiX<Impl: IPrintPageInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DpiX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDpiY<Impl: IPrintPageInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDpiY(value).into()
        }
        unsafe extern "system" fn DpiY<Impl: IPrintPageInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DpiY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOrientation<Impl: IPrintPageInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PrintOrientation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOrientation(value).into()
        }
        unsafe extern "system" fn Orientation<Impl: IPrintPageInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintOrientation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Orientation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintPageInfo, BASE_OFFSET>(),
            SetMediaSize: SetMediaSize::<Impl, IMPL_OFFSET>,
            MediaSize: MediaSize::<Impl, IMPL_OFFSET>,
            SetPageSize: SetPageSize::<Impl, IMPL_OFFSET>,
            PageSize: PageSize::<Impl, IMPL_OFFSET>,
            SetDpiX: SetDpiX::<Impl, IMPL_OFFSET>,
            DpiX: DpiX::<Impl, IMPL_OFFSET>,
            SetDpiY: SetDpiY::<Impl, IMPL_OFFSET>,
            DpiY: DpiY::<Impl, IMPL_OFFSET>,
            SetOrientation: SetOrientation::<Impl, IMPL_OFFSET>,
            Orientation: Orientation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintPageInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintPageRange_Impl: Sized {
    fn FirstPageNumber(&mut self) -> ::windows::core::Result<i32>;
    fn LastPageNumber(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintPageRange {
    const NAME: &'static str = "Windows.Graphics.Printing.IPrintPageRange";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintPageRange_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintPageRange_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintPageRange_Vtbl {
        unsafe extern "system" fn FirstPageNumber<Impl: IPrintPageRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FirstPageNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastPageNumber<Impl: IPrintPageRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastPageNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintPageRange, BASE_OFFSET>(),
            FirstPageNumber: FirstPageNumber::<Impl, IMPL_OFFSET>,
            LastPageNumber: LastPageNumber::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintPageRange as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintPageRangeFactory_Impl: Sized {
    fn Create(&mut self, firstpage: i32, lastpage: i32) -> ::windows::core::Result<PrintPageRange>;
    fn CreateWithSinglePage(&mut self, page: i32) -> ::windows::core::Result<PrintPageRange>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintPageRangeFactory {
    const NAME: &'static str = "Windows.Graphics.Printing.IPrintPageRangeFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintPageRangeFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintPageRangeFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintPageRangeFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: IPrintPageRangeFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, firstpage: i32, lastpage: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(firstpage, lastpage) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithSinglePage<Impl: IPrintPageRangeFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, page: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithSinglePage(page) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintPageRangeFactory, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
            CreateWithSinglePage: CreateWithSinglePage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintPageRangeFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintPageRangeOptions_Impl: Sized {
    fn SetAllowAllPages(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn AllowAllPages(&mut self) -> ::windows::core::Result<bool>;
    fn SetAllowCurrentPage(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn AllowCurrentPage(&mut self) -> ::windows::core::Result<bool>;
    fn SetAllowCustomSetOfPages(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn AllowCustomSetOfPages(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintPageRangeOptions {
    const NAME: &'static str = "Windows.Graphics.Printing.IPrintPageRangeOptions";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintPageRangeOptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintPageRangeOptions_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintPageRangeOptions_Vtbl {
        unsafe extern "system" fn SetAllowAllPages<Impl: IPrintPageRangeOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowAllPages(value).into()
        }
        unsafe extern "system" fn AllowAllPages<Impl: IPrintPageRangeOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowAllPages() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowCurrentPage<Impl: IPrintPageRangeOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowCurrentPage(value).into()
        }
        unsafe extern "system" fn AllowCurrentPage<Impl: IPrintPageRangeOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowCurrentPage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowCustomSetOfPages<Impl: IPrintPageRangeOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowCustomSetOfPages(value).into()
        }
        unsafe extern "system" fn AllowCustomSetOfPages<Impl: IPrintPageRangeOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowCustomSetOfPages() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintPageRangeOptions, BASE_OFFSET>(),
            SetAllowAllPages: SetAllowAllPages::<Impl, IMPL_OFFSET>,
            AllowAllPages: AllowAllPages::<Impl, IMPL_OFFSET>,
            SetAllowCurrentPage: SetAllowCurrentPage::<Impl, IMPL_OFFSET>,
            AllowCurrentPage: AllowCurrentPage::<Impl, IMPL_OFFSET>,
            SetAllowCustomSetOfPages: SetAllowCustomSetOfPages::<Impl, IMPL_OFFSET>,
            AllowCustomSetOfPages: AllowCustomSetOfPages::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintPageRangeOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPrintTask_Impl: Sized {
    fn Properties(&mut self) -> ::windows::core::Result<super::super::ApplicationModel::DataTransfer::DataPackagePropertySet>;
    fn Source(&mut self) -> ::windows::core::Result<IPrintDocumentSource>;
    fn Options(&mut self) -> ::windows::core::Result<PrintTaskOptions>;
    fn Previewing(&mut self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PrintTask, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePreviewing(&mut self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Submitting(&mut self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PrintTask, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSubmitting(&mut self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Progressing(&mut self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PrintTask, PrintTaskProgressingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveProgressing(&mut self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Completed(&mut self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PrintTask, PrintTaskCompletedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCompleted(&mut self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrintTask {
    const NAME: &'static str = "Windows.Graphics.Printing.IPrintTask";
}
#[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation", feature = "implement_exclusive"))]
impl IPrintTask_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTask_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintTask_Vtbl {
        unsafe extern "system" fn Properties<Impl: IPrintTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Source<Impl: IPrintTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Source() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Options<Impl: IPrintTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Options() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Previewing<Impl: IPrintTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Previewing(&*(&eventhandler as *const <super::super::Foundation::TypedEventHandler<PrintTask, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PrintTask, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePreviewing<Impl: IPrintTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePreviewing(&*(&eventcookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Submitting<Impl: IPrintTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Submitting(&*(&eventhandler as *const <super::super::Foundation::TypedEventHandler<PrintTask, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PrintTask, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSubmitting<Impl: IPrintTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSubmitting(&*(&eventcookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Progressing<Impl: IPrintTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Progressing(&*(&eventhandler as *const <super::super::Foundation::TypedEventHandler<PrintTask, PrintTaskProgressingEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PrintTask, PrintTaskProgressingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveProgressing<Impl: IPrintTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveProgressing(&*(&eventcookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Completed<Impl: IPrintTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Completed(&*(&eventhandler as *const <super::super::Foundation::TypedEventHandler<PrintTask, PrintTaskCompletedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PrintTask, PrintTaskCompletedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCompleted<Impl: IPrintTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCompleted(&*(&eventcookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintTask, BASE_OFFSET>(),
            Properties: Properties::<Impl, IMPL_OFFSET>,
            Source: Source::<Impl, IMPL_OFFSET>,
            Options: Options::<Impl, IMPL_OFFSET>,
            Previewing: Previewing::<Impl, IMPL_OFFSET>,
            RemovePreviewing: RemovePreviewing::<Impl, IMPL_OFFSET>,
            Submitting: Submitting::<Impl, IMPL_OFFSET>,
            RemoveSubmitting: RemoveSubmitting::<Impl, IMPL_OFFSET>,
            Progressing: Progressing::<Impl, IMPL_OFFSET>,
            RemoveProgressing: RemoveProgressing::<Impl, IMPL_OFFSET>,
            Completed: Completed::<Impl, IMPL_OFFSET>,
            RemoveCompleted: RemoveCompleted::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintTask as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintTask2_Impl: Sized {
    fn SetIsPreviewEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsPreviewEnabled(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintTask2 {
    const NAME: &'static str = "Windows.Graphics.Printing.IPrintTask2";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintTask2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTask2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintTask2_Vtbl {
        unsafe extern "system" fn SetIsPreviewEnabled<Impl: IPrintTask2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsPreviewEnabled(value).into()
        }
        unsafe extern "system" fn IsPreviewEnabled<Impl: IPrintTask2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPreviewEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintTask2, BASE_OFFSET>(),
            SetIsPreviewEnabled: SetIsPreviewEnabled::<Impl, IMPL_OFFSET>,
            IsPreviewEnabled: IsPreviewEnabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintTask2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintTaskCompletedEventArgs_Impl: Sized {
    fn Completion(&mut self) -> ::windows::core::Result<PrintTaskCompletion>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintTaskCompletedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.IPrintTaskCompletedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintTaskCompletedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTaskCompletedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintTaskCompletedEventArgs_Vtbl {
        unsafe extern "system" fn Completion<Impl: IPrintTaskCompletedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintTaskCompletion) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Completion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintTaskCompletedEventArgs, BASE_OFFSET>(),
            Completion: Completion::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintTaskCompletedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IPrintTaskOptions_Impl: Sized {
    fn SetBordering(&mut self, value: PrintBordering) -> ::windows::core::Result<()>;
    fn Bordering(&mut self) -> ::windows::core::Result<PrintBordering>;
    fn GetPagePrintTicket(&mut self, printpageinfo: &::core::option::Option<PrintPageInfo>) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStream>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrintTaskOptions {
    const NAME: &'static str = "Windows.Graphics.Printing.IPrintTaskOptions";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IPrintTaskOptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTaskOptions_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintTaskOptions_Vtbl {
        unsafe extern "system" fn SetBordering<Impl: IPrintTaskOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PrintBordering) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBordering(value).into()
        }
        unsafe extern "system" fn Bordering<Impl: IPrintTaskOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintBordering) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Bordering() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPagePrintTicket<Impl: IPrintTaskOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, printpageinfo: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPagePrintTicket(&*(&printpageinfo as *const <PrintPageInfo as ::windows::core::Abi>::Abi as *const <PrintPageInfo as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintTaskOptions, BASE_OFFSET>(),
            SetBordering: SetBordering::<Impl, IMPL_OFFSET>,
            Bordering: Bordering::<Impl, IMPL_OFFSET>,
            GetPagePrintTicket: GetPagePrintTicket::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintTaskOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPrintTaskOptions2_Impl: Sized {
    fn PageRangeOptions(&mut self) -> ::windows::core::Result<PrintPageRangeOptions>;
    fn CustomPageRanges(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<PrintPageRange>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrintTaskOptions2 {
    const NAME: &'static str = "Windows.Graphics.Printing.IPrintTaskOptions2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPrintTaskOptions2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTaskOptions2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintTaskOptions2_Vtbl {
        unsafe extern "system" fn PageRangeOptions<Impl: IPrintTaskOptions2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PageRangeOptions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CustomPageRanges<Impl: IPrintTaskOptions2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CustomPageRanges() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintTaskOptions2, BASE_OFFSET>(),
            PageRangeOptions: PageRangeOptions::<Impl, IMPL_OFFSET>,
            CustomPageRanges: CustomPageRanges::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintTaskOptions2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IPrintTaskOptionsCore_Impl: Sized {
    fn GetPageDescription(&mut self, jobpagenumber: u32) -> ::windows::core::Result<PrintPageDescription>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IPrintTaskOptionsCore {
    const NAME: &'static str = "Windows.Graphics.Printing.IPrintTaskOptionsCore";
}
#[cfg(feature = "Foundation")]
impl IPrintTaskOptionsCore_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTaskOptionsCore_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintTaskOptionsCore_Vtbl {
        unsafe extern "system" fn GetPageDescription<Impl: IPrintTaskOptionsCore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, jobpagenumber: u32, result__: *mut PrintPageDescription) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPageDescription(jobpagenumber) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintTaskOptionsCore, BASE_OFFSET>(),
            GetPageDescription: GetPageDescription::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintTaskOptionsCore as ::windows::core::Interface>::IID
    }
}
pub trait IPrintTaskOptionsCoreProperties_Impl: Sized {
    fn SetMediaSize(&mut self, value: PrintMediaSize) -> ::windows::core::Result<()>;
    fn MediaSize(&mut self) -> ::windows::core::Result<PrintMediaSize>;
    fn SetMediaType(&mut self, value: PrintMediaType) -> ::windows::core::Result<()>;
    fn MediaType(&mut self) -> ::windows::core::Result<PrintMediaType>;
    fn SetOrientation(&mut self, value: PrintOrientation) -> ::windows::core::Result<()>;
    fn Orientation(&mut self) -> ::windows::core::Result<PrintOrientation>;
    fn SetPrintQuality(&mut self, value: PrintQuality) -> ::windows::core::Result<()>;
    fn PrintQuality(&mut self) -> ::windows::core::Result<PrintQuality>;
    fn SetColorMode(&mut self, value: PrintColorMode) -> ::windows::core::Result<()>;
    fn ColorMode(&mut self) -> ::windows::core::Result<PrintColorMode>;
    fn SetDuplex(&mut self, value: PrintDuplex) -> ::windows::core::Result<()>;
    fn Duplex(&mut self) -> ::windows::core::Result<PrintDuplex>;
    fn SetCollation(&mut self, value: PrintCollation) -> ::windows::core::Result<()>;
    fn Collation(&mut self) -> ::windows::core::Result<PrintCollation>;
    fn SetStaple(&mut self, value: PrintStaple) -> ::windows::core::Result<()>;
    fn Staple(&mut self) -> ::windows::core::Result<PrintStaple>;
    fn SetHolePunch(&mut self, value: PrintHolePunch) -> ::windows::core::Result<()>;
    fn HolePunch(&mut self) -> ::windows::core::Result<PrintHolePunch>;
    fn SetBinding(&mut self, value: PrintBinding) -> ::windows::core::Result<()>;
    fn Binding(&mut self) -> ::windows::core::Result<PrintBinding>;
    fn MinCopies(&mut self) -> ::windows::core::Result<u32>;
    fn MaxCopies(&mut self) -> ::windows::core::Result<u32>;
    fn SetNumberOfCopies(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn NumberOfCopies(&mut self) -> ::windows::core::Result<u32>;
}
impl ::windows::core::RuntimeName for IPrintTaskOptionsCoreProperties {
    const NAME: &'static str = "Windows.Graphics.Printing.IPrintTaskOptionsCoreProperties";
}
impl IPrintTaskOptionsCoreProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTaskOptionsCoreProperties_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintTaskOptionsCoreProperties_Vtbl {
        unsafe extern "system" fn SetMediaSize<Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PrintMediaSize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMediaSize(value).into()
        }
        unsafe extern "system" fn MediaSize<Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintMediaSize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMediaType<Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PrintMediaType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMediaType(value).into()
        }
        unsafe extern "system" fn MediaType<Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintMediaType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOrientation<Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PrintOrientation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOrientation(value).into()
        }
        unsafe extern "system" fn Orientation<Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintOrientation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Orientation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrintQuality<Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PrintQuality) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrintQuality(value).into()
        }
        unsafe extern "system" fn PrintQuality<Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintQuality) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrintQuality() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorMode<Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PrintColorMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColorMode(value).into()
        }
        unsafe extern "system" fn ColorMode<Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintColorMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ColorMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDuplex<Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PrintDuplex) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDuplex(value).into()
        }
        unsafe extern "system" fn Duplex<Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintDuplex) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Duplex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCollation<Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PrintCollation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCollation(value).into()
        }
        unsafe extern "system" fn Collation<Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintCollation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Collation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStaple<Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PrintStaple) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStaple(value).into()
        }
        unsafe extern "system" fn Staple<Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintStaple) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Staple() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHolePunch<Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PrintHolePunch) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHolePunch(value).into()
        }
        unsafe extern "system" fn HolePunch<Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintHolePunch) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HolePunch() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBinding<Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PrintBinding) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBinding(value).into()
        }
        unsafe extern "system" fn Binding<Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintBinding) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Binding() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinCopies<Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinCopies() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxCopies<Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxCopies() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNumberOfCopies<Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNumberOfCopies(value).into()
        }
        unsafe extern "system" fn NumberOfCopies<Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NumberOfCopies() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintTaskOptionsCoreProperties, BASE_OFFSET>(),
            SetMediaSize: SetMediaSize::<Impl, IMPL_OFFSET>,
            MediaSize: MediaSize::<Impl, IMPL_OFFSET>,
            SetMediaType: SetMediaType::<Impl, IMPL_OFFSET>,
            MediaType: MediaType::<Impl, IMPL_OFFSET>,
            SetOrientation: SetOrientation::<Impl, IMPL_OFFSET>,
            Orientation: Orientation::<Impl, IMPL_OFFSET>,
            SetPrintQuality: SetPrintQuality::<Impl, IMPL_OFFSET>,
            PrintQuality: PrintQuality::<Impl, IMPL_OFFSET>,
            SetColorMode: SetColorMode::<Impl, IMPL_OFFSET>,
            ColorMode: ColorMode::<Impl, IMPL_OFFSET>,
            SetDuplex: SetDuplex::<Impl, IMPL_OFFSET>,
            Duplex: Duplex::<Impl, IMPL_OFFSET>,
            SetCollation: SetCollation::<Impl, IMPL_OFFSET>,
            Collation: Collation::<Impl, IMPL_OFFSET>,
            SetStaple: SetStaple::<Impl, IMPL_OFFSET>,
            Staple: Staple::<Impl, IMPL_OFFSET>,
            SetHolePunch: SetHolePunch::<Impl, IMPL_OFFSET>,
            HolePunch: HolePunch::<Impl, IMPL_OFFSET>,
            SetBinding: SetBinding::<Impl, IMPL_OFFSET>,
            Binding: Binding::<Impl, IMPL_OFFSET>,
            MinCopies: MinCopies::<Impl, IMPL_OFFSET>,
            MaxCopies: MaxCopies::<Impl, IMPL_OFFSET>,
            SetNumberOfCopies: SetNumberOfCopies::<Impl, IMPL_OFFSET>,
            NumberOfCopies: NumberOfCopies::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintTaskOptionsCoreProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IPrintTaskOptionsCoreUIConfiguration_Impl: Sized {
    fn DisplayedOptions(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IPrintTaskOptionsCoreUIConfiguration {
    const NAME: &'static str = "Windows.Graphics.Printing.IPrintTaskOptionsCoreUIConfiguration";
}
#[cfg(feature = "Foundation_Collections")]
impl IPrintTaskOptionsCoreUIConfiguration_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTaskOptionsCoreUIConfiguration_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintTaskOptionsCoreUIConfiguration_Vtbl {
        unsafe extern "system" fn DisplayedOptions<Impl: IPrintTaskOptionsCoreUIConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayedOptions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintTaskOptionsCoreUIConfiguration, BASE_OFFSET>(),
            DisplayedOptions: DisplayedOptions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintTaskOptionsCoreUIConfiguration as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintTaskProgressingEventArgs_Impl: Sized {
    fn DocumentPageCount(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintTaskProgressingEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.IPrintTaskProgressingEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintTaskProgressingEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTaskProgressingEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintTaskProgressingEventArgs_Vtbl {
        unsafe extern "system" fn DocumentPageCount<Impl: IPrintTaskProgressingEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DocumentPageCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintTaskProgressingEventArgs, BASE_OFFSET>(),
            DocumentPageCount: DocumentPageCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintTaskProgressingEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPrintTaskRequest_Impl: Sized {
    fn Deadline(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn CreatePrintTask(&mut self, title: &::windows::core::HSTRING, handler: &::core::option::Option<PrintTaskSourceRequestedHandler>) -> ::windows::core::Result<PrintTask>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<PrintTaskRequestedDeferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrintTaskRequest {
    const NAME: &'static str = "Windows.Graphics.Printing.IPrintTaskRequest";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPrintTaskRequest_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTaskRequest_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintTaskRequest_Vtbl {
        unsafe extern "system" fn Deadline<Impl: IPrintTaskRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Deadline() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePrintTask<Impl: IPrintTaskRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, title: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, handler: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePrintTask(&*(&title as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&handler as *const <PrintTaskSourceRequestedHandler as ::windows::core::Abi>::Abi as *const <PrintTaskSourceRequestedHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IPrintTaskRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintTaskRequest, BASE_OFFSET>(),
            Deadline: Deadline::<Impl, IMPL_OFFSET>,
            CreatePrintTask: CreatePrintTask::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintTaskRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintTaskRequestedDeferral_Impl: Sized {
    fn Complete(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintTaskRequestedDeferral {
    const NAME: &'static str = "Windows.Graphics.Printing.IPrintTaskRequestedDeferral";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintTaskRequestedDeferral_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTaskRequestedDeferral_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintTaskRequestedDeferral_Vtbl {
        unsafe extern "system" fn Complete<Impl: IPrintTaskRequestedDeferral_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintTaskRequestedDeferral, BASE_OFFSET>(), Complete: Complete::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintTaskRequestedDeferral as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintTaskRequestedEventArgs_Impl: Sized {
    fn Request(&mut self) -> ::windows::core::Result<PrintTaskRequest>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintTaskRequestedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.IPrintTaskRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintTaskRequestedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTaskRequestedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintTaskRequestedEventArgs_Vtbl {
        unsafe extern "system" fn Request<Impl: IPrintTaskRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintTaskRequestedEventArgs, BASE_OFFSET>(), Request: Request::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintTaskRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPrintTaskSourceRequestedArgs_Impl: Sized {
    fn Deadline(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn SetSource(&mut self, source: &::core::option::Option<IPrintDocumentSource>) -> ::windows::core::Result<()>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<PrintTaskSourceRequestedDeferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrintTaskSourceRequestedArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.IPrintTaskSourceRequestedArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPrintTaskSourceRequestedArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTaskSourceRequestedArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintTaskSourceRequestedArgs_Vtbl {
        unsafe extern "system" fn Deadline<Impl: IPrintTaskSourceRequestedArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Deadline() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSource<Impl: IPrintTaskSourceRequestedArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSource(&*(&source as *const <IPrintDocumentSource as ::windows::core::Abi>::Abi as *const <IPrintDocumentSource as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetDeferral<Impl: IPrintTaskSourceRequestedArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintTaskSourceRequestedArgs, BASE_OFFSET>(),
            Deadline: Deadline::<Impl, IMPL_OFFSET>,
            SetSource: SetSource::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintTaskSourceRequestedArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintTaskSourceRequestedDeferral_Impl: Sized {
    fn Complete(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintTaskSourceRequestedDeferral {
    const NAME: &'static str = "Windows.Graphics.Printing.IPrintTaskSourceRequestedDeferral";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintTaskSourceRequestedDeferral_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTaskSourceRequestedDeferral_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintTaskSourceRequestedDeferral_Vtbl {
        unsafe extern "system" fn Complete<Impl: IPrintTaskSourceRequestedDeferral_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintTaskSourceRequestedDeferral, BASE_OFFSET>(),
            Complete: Complete::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintTaskSourceRequestedDeferral as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintTaskTargetDeviceSupport_Impl: Sized {
    fn SetIsPrinterTargetEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsPrinterTargetEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIs3DManufacturingTargetEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Is3DManufacturingTargetEnabled(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintTaskTargetDeviceSupport {
    const NAME: &'static str = "Windows.Graphics.Printing.IPrintTaskTargetDeviceSupport";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintTaskTargetDeviceSupport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTaskTargetDeviceSupport_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintTaskTargetDeviceSupport_Vtbl {
        unsafe extern "system" fn SetIsPrinterTargetEnabled<Impl: IPrintTaskTargetDeviceSupport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsPrinterTargetEnabled(value).into()
        }
        unsafe extern "system" fn IsPrinterTargetEnabled<Impl: IPrintTaskTargetDeviceSupport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPrinterTargetEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIs3DManufacturingTargetEnabled<Impl: IPrintTaskTargetDeviceSupport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIs3DManufacturingTargetEnabled(value).into()
        }
        unsafe extern "system" fn Is3DManufacturingTargetEnabled<Impl: IPrintTaskTargetDeviceSupport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Is3DManufacturingTargetEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintTaskTargetDeviceSupport, BASE_OFFSET>(),
            SetIsPrinterTargetEnabled: SetIsPrinterTargetEnabled::<Impl, IMPL_OFFSET>,
            IsPrinterTargetEnabled: IsPrinterTargetEnabled::<Impl, IMPL_OFFSET>,
            SetIs3DManufacturingTargetEnabled: SetIs3DManufacturingTargetEnabled::<Impl, IMPL_OFFSET>,
            Is3DManufacturingTargetEnabled: Is3DManufacturingTargetEnabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintTaskTargetDeviceSupport as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStandardPrintTaskOptionsStatic_Impl: Sized {
    fn MediaSize(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MediaType(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Orientation(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PrintQuality(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ColorMode(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Duplex(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Collation(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Staple(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HolePunch(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Binding(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Copies(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NUp(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn InputBin(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStandardPrintTaskOptionsStatic {
    const NAME: &'static str = "Windows.Graphics.Printing.IStandardPrintTaskOptionsStatic";
}
#[cfg(feature = "implement_exclusive")]
impl IStandardPrintTaskOptionsStatic_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStandardPrintTaskOptionsStatic_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStandardPrintTaskOptionsStatic_Vtbl {
        unsafe extern "system" fn MediaSize<Impl: IStandardPrintTaskOptionsStatic_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaType<Impl: IStandardPrintTaskOptionsStatic_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Orientation<Impl: IStandardPrintTaskOptionsStatic_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Orientation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrintQuality<Impl: IStandardPrintTaskOptionsStatic_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrintQuality() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ColorMode<Impl: IStandardPrintTaskOptionsStatic_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ColorMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Duplex<Impl: IStandardPrintTaskOptionsStatic_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Duplex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Collation<Impl: IStandardPrintTaskOptionsStatic_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Collation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Staple<Impl: IStandardPrintTaskOptionsStatic_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Staple() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HolePunch<Impl: IStandardPrintTaskOptionsStatic_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HolePunch() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Binding<Impl: IStandardPrintTaskOptionsStatic_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Binding() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Copies<Impl: IStandardPrintTaskOptionsStatic_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Copies() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NUp<Impl: IStandardPrintTaskOptionsStatic_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NUp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InputBin<Impl: IStandardPrintTaskOptionsStatic_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InputBin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStandardPrintTaskOptionsStatic, BASE_OFFSET>(),
            MediaSize: MediaSize::<Impl, IMPL_OFFSET>,
            MediaType: MediaType::<Impl, IMPL_OFFSET>,
            Orientation: Orientation::<Impl, IMPL_OFFSET>,
            PrintQuality: PrintQuality::<Impl, IMPL_OFFSET>,
            ColorMode: ColorMode::<Impl, IMPL_OFFSET>,
            Duplex: Duplex::<Impl, IMPL_OFFSET>,
            Collation: Collation::<Impl, IMPL_OFFSET>,
            Staple: Staple::<Impl, IMPL_OFFSET>,
            HolePunch: HolePunch::<Impl, IMPL_OFFSET>,
            Binding: Binding::<Impl, IMPL_OFFSET>,
            Copies: Copies::<Impl, IMPL_OFFSET>,
            NUp: NUp::<Impl, IMPL_OFFSET>,
            InputBin: InputBin::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStandardPrintTaskOptionsStatic as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStandardPrintTaskOptionsStatic2_Impl: Sized {
    fn Bordering(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStandardPrintTaskOptionsStatic2 {
    const NAME: &'static str = "Windows.Graphics.Printing.IStandardPrintTaskOptionsStatic2";
}
#[cfg(feature = "implement_exclusive")]
impl IStandardPrintTaskOptionsStatic2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStandardPrintTaskOptionsStatic2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStandardPrintTaskOptionsStatic2_Vtbl {
        unsafe extern "system" fn Bordering<Impl: IStandardPrintTaskOptionsStatic2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Bordering() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStandardPrintTaskOptionsStatic2, BASE_OFFSET>(),
            Bordering: Bordering::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStandardPrintTaskOptionsStatic2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStandardPrintTaskOptionsStatic3_Impl: Sized {
    fn CustomPageRanges(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStandardPrintTaskOptionsStatic3 {
    const NAME: &'static str = "Windows.Graphics.Printing.IStandardPrintTaskOptionsStatic3";
}
#[cfg(feature = "implement_exclusive")]
impl IStandardPrintTaskOptionsStatic3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStandardPrintTaskOptionsStatic3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStandardPrintTaskOptionsStatic3_Vtbl {
        unsafe extern "system" fn CustomPageRanges<Impl: IStandardPrintTaskOptionsStatic3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CustomPageRanges() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStandardPrintTaskOptionsStatic3, BASE_OFFSET>(),
            CustomPageRanges: CustomPageRanges::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStandardPrintTaskOptionsStatic3 as ::windows::core::Interface>::IID
    }
}
