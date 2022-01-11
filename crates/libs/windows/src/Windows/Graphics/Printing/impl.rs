pub trait IPrintDocumentSourceImpl: Sized {}
impl ::windows::core::RuntimeName for IPrintDocumentSource {
    const NAME: &'static str = "Windows.Graphics.Printing.IPrintDocumentSource";
}
impl IPrintDocumentSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintDocumentSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintDocumentSourceVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintDocumentSource, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintDocumentSource as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPrintManagerImpl: Sized {
    fn PrintTaskRequested(&self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PrintManager, PrintTaskRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePrintTaskRequested(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrintManager {
    const NAME: &'static str = "Windows.Graphics.Printing.IPrintManager";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPrintManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintManagerVtbl {
        unsafe extern "system" fn PrintTaskRequested<Impl: IPrintManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemovePrintTaskRequested<Impl: IPrintManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
pub trait IPrintManagerStaticImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<PrintManager>;
    fn ShowPrintUIAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrintManagerStatic {
    const NAME: &'static str = "Windows.Graphics.Printing.IPrintManagerStatic";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPrintManagerStaticVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintManagerStaticImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintManagerStaticVtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: IPrintManagerStaticImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ShowPrintUIAsync<Impl: IPrintManagerStaticImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IPrintManagerStatic2Impl: Sized {
    fn IsSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintManagerStatic2 {
    const NAME: &'static str = "Windows.Graphics.Printing.IPrintManagerStatic2";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintManagerStatic2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintManagerStatic2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintManagerStatic2Vtbl {
        unsafe extern "system" fn IsSupported<Impl: IPrintManagerStatic2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
pub trait IPrintPageInfoImpl: Sized {
    fn SetMediaSize(&self, value: PrintMediaSize) -> ::windows::core::Result<()>;
    fn MediaSize(&self) -> ::windows::core::Result<PrintMediaSize>;
    fn SetPageSize(&self, value: &super::super::Foundation::Size) -> ::windows::core::Result<()>;
    fn PageSize(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn SetDpiX(&self, value: u32) -> ::windows::core::Result<()>;
    fn DpiX(&self) -> ::windows::core::Result<u32>;
    fn SetDpiY(&self, value: u32) -> ::windows::core::Result<()>;
    fn DpiY(&self) -> ::windows::core::Result<u32>;
    fn SetOrientation(&self, value: PrintOrientation) -> ::windows::core::Result<()>;
    fn Orientation(&self) -> ::windows::core::Result<PrintOrientation>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrintPageInfo {
    const NAME: &'static str = "Windows.Graphics.Printing.IPrintPageInfo";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPrintPageInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintPageInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintPageInfoVtbl {
        unsafe extern "system" fn SetMediaSize<Impl: IPrintPageInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PrintMediaSize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMediaSize(value).into()
        }
        unsafe extern "system" fn MediaSize<Impl: IPrintPageInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintMediaSize) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPageSize<Impl: IPrintPageInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPageSize(&*(&value as *const <super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PageSize<Impl: IPrintPageInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDpiX<Impl: IPrintPageInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDpiX(value).into()
        }
        unsafe extern "system" fn DpiX<Impl: IPrintPageInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDpiY<Impl: IPrintPageInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDpiY(value).into()
        }
        unsafe extern "system" fn DpiY<Impl: IPrintPageInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetOrientation<Impl: IPrintPageInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PrintOrientation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOrientation(value).into()
        }
        unsafe extern "system" fn Orientation<Impl: IPrintPageInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintOrientation) -> ::windows::core::HRESULT {
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
pub trait IPrintPageRangeImpl: Sized {
    fn FirstPageNumber(&self) -> ::windows::core::Result<i32>;
    fn LastPageNumber(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintPageRange {
    const NAME: &'static str = "Windows.Graphics.Printing.IPrintPageRange";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintPageRangeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintPageRangeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintPageRangeVtbl {
        unsafe extern "system" fn FirstPageNumber<Impl: IPrintPageRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LastPageNumber<Impl: IPrintPageRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
pub trait IPrintPageRangeFactoryImpl: Sized {
    fn Create(&self, firstpage: i32, lastpage: i32) -> ::windows::core::Result<PrintPageRange>;
    fn CreateWithSinglePage(&self, page: i32) -> ::windows::core::Result<PrintPageRange>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintPageRangeFactory {
    const NAME: &'static str = "Windows.Graphics.Printing.IPrintPageRangeFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintPageRangeFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintPageRangeFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintPageRangeFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IPrintPageRangeFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, firstpage: i32, lastpage: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateWithSinglePage<Impl: IPrintPageRangeFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, page: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IPrintPageRangeOptionsImpl: Sized {
    fn SetAllowAllPages(&self, value: bool) -> ::windows::core::Result<()>;
    fn AllowAllPages(&self) -> ::windows::core::Result<bool>;
    fn SetAllowCurrentPage(&self, value: bool) -> ::windows::core::Result<()>;
    fn AllowCurrentPage(&self) -> ::windows::core::Result<bool>;
    fn SetAllowCustomSetOfPages(&self, value: bool) -> ::windows::core::Result<()>;
    fn AllowCustomSetOfPages(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintPageRangeOptions {
    const NAME: &'static str = "Windows.Graphics.Printing.IPrintPageRangeOptions";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintPageRangeOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintPageRangeOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintPageRangeOptionsVtbl {
        unsafe extern "system" fn SetAllowAllPages<Impl: IPrintPageRangeOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowAllPages(value).into()
        }
        unsafe extern "system" fn AllowAllPages<Impl: IPrintPageRangeOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAllowCurrentPage<Impl: IPrintPageRangeOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowCurrentPage(value).into()
        }
        unsafe extern "system" fn AllowCurrentPage<Impl: IPrintPageRangeOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAllowCustomSetOfPages<Impl: IPrintPageRangeOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowCustomSetOfPages(value).into()
        }
        unsafe extern "system" fn AllowCustomSetOfPages<Impl: IPrintPageRangeOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
pub trait IPrintTaskImpl: Sized {
    fn Properties(&self) -> ::windows::core::Result<super::super::ApplicationModel::DataTransfer::DataPackagePropertySet>;
    fn Source(&self) -> ::windows::core::Result<IPrintDocumentSource>;
    fn Options(&self) -> ::windows::core::Result<PrintTaskOptions>;
    fn Previewing(&self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PrintTask, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePreviewing(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Submitting(&self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PrintTask, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSubmitting(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Progressing(&self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PrintTask, PrintTaskProgressingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveProgressing(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Completed(&self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PrintTask, PrintTaskCompletedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCompleted(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrintTask {
    const NAME: &'static str = "Windows.Graphics.Printing.IPrintTask";
}
#[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation", feature = "implement_exclusive"))]
impl IPrintTaskVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTaskImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintTaskVtbl {
        unsafe extern "system" fn Properties<Impl: IPrintTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Source<Impl: IPrintTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Options<Impl: IPrintTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Previewing<Impl: IPrintTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemovePreviewing<Impl: IPrintTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePreviewing(&*(&eventcookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Submitting<Impl: IPrintTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveSubmitting<Impl: IPrintTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSubmitting(&*(&eventcookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Progressing<Impl: IPrintTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveProgressing<Impl: IPrintTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveProgressing(&*(&eventcookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Completed<Impl: IPrintTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveCompleted<Impl: IPrintTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
pub trait IPrintTask2Impl: Sized {
    fn SetIsPreviewEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsPreviewEnabled(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintTask2 {
    const NAME: &'static str = "Windows.Graphics.Printing.IPrintTask2";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintTask2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTask2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintTask2Vtbl {
        unsafe extern "system" fn SetIsPreviewEnabled<Impl: IPrintTask2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsPreviewEnabled(value).into()
        }
        unsafe extern "system" fn IsPreviewEnabled<Impl: IPrintTask2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
pub trait IPrintTaskCompletedEventArgsImpl: Sized {
    fn Completion(&self) -> ::windows::core::Result<PrintTaskCompletion>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintTaskCompletedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.IPrintTaskCompletedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintTaskCompletedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTaskCompletedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintTaskCompletedEventArgsVtbl {
        unsafe extern "system" fn Completion<Impl: IPrintTaskCompletedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintTaskCompletion) -> ::windows::core::HRESULT {
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
pub trait IPrintTaskOptionsImpl: Sized {
    fn SetBordering(&self, value: PrintBordering) -> ::windows::core::Result<()>;
    fn Bordering(&self) -> ::windows::core::Result<PrintBordering>;
    fn GetPagePrintTicket(&self, printpageinfo: &::core::option::Option<PrintPageInfo>) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStream>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrintTaskOptions {
    const NAME: &'static str = "Windows.Graphics.Printing.IPrintTaskOptions";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IPrintTaskOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTaskOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintTaskOptionsVtbl {
        unsafe extern "system" fn SetBordering<Impl: IPrintTaskOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PrintBordering) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBordering(value).into()
        }
        unsafe extern "system" fn Bordering<Impl: IPrintTaskOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintBordering) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetPagePrintTicket<Impl: IPrintTaskOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, printpageinfo: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IPrintTaskOptions2Impl: Sized {
    fn PageRangeOptions(&self) -> ::windows::core::Result<PrintPageRangeOptions>;
    fn CustomPageRanges(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<PrintPageRange>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrintTaskOptions2 {
    const NAME: &'static str = "Windows.Graphics.Printing.IPrintTaskOptions2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPrintTaskOptions2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTaskOptions2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintTaskOptions2Vtbl {
        unsafe extern "system" fn PageRangeOptions<Impl: IPrintTaskOptions2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CustomPageRanges<Impl: IPrintTaskOptions2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IPrintTaskOptionsCoreImpl: Sized {
    fn GetPageDescription(&self, jobpagenumber: u32) -> ::windows::core::Result<PrintPageDescription>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IPrintTaskOptionsCore {
    const NAME: &'static str = "Windows.Graphics.Printing.IPrintTaskOptionsCore";
}
#[cfg(feature = "Foundation")]
impl IPrintTaskOptionsCoreVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTaskOptionsCoreImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintTaskOptionsCoreVtbl {
        unsafe extern "system" fn GetPageDescription<Impl: IPrintTaskOptionsCoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, jobpagenumber: u32, result__: *mut PrintPageDescription) -> ::windows::core::HRESULT {
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
pub trait IPrintTaskOptionsCorePropertiesImpl: Sized {
    fn SetMediaSize(&self, value: PrintMediaSize) -> ::windows::core::Result<()>;
    fn MediaSize(&self) -> ::windows::core::Result<PrintMediaSize>;
    fn SetMediaType(&self, value: PrintMediaType) -> ::windows::core::Result<()>;
    fn MediaType(&self) -> ::windows::core::Result<PrintMediaType>;
    fn SetOrientation(&self, value: PrintOrientation) -> ::windows::core::Result<()>;
    fn Orientation(&self) -> ::windows::core::Result<PrintOrientation>;
    fn SetPrintQuality(&self, value: PrintQuality) -> ::windows::core::Result<()>;
    fn PrintQuality(&self) -> ::windows::core::Result<PrintQuality>;
    fn SetColorMode(&self, value: PrintColorMode) -> ::windows::core::Result<()>;
    fn ColorMode(&self) -> ::windows::core::Result<PrintColorMode>;
    fn SetDuplex(&self, value: PrintDuplex) -> ::windows::core::Result<()>;
    fn Duplex(&self) -> ::windows::core::Result<PrintDuplex>;
    fn SetCollation(&self, value: PrintCollation) -> ::windows::core::Result<()>;
    fn Collation(&self) -> ::windows::core::Result<PrintCollation>;
    fn SetStaple(&self, value: PrintStaple) -> ::windows::core::Result<()>;
    fn Staple(&self) -> ::windows::core::Result<PrintStaple>;
    fn SetHolePunch(&self, value: PrintHolePunch) -> ::windows::core::Result<()>;
    fn HolePunch(&self) -> ::windows::core::Result<PrintHolePunch>;
    fn SetBinding(&self, value: PrintBinding) -> ::windows::core::Result<()>;
    fn Binding(&self) -> ::windows::core::Result<PrintBinding>;
    fn MinCopies(&self) -> ::windows::core::Result<u32>;
    fn MaxCopies(&self) -> ::windows::core::Result<u32>;
    fn SetNumberOfCopies(&self, value: u32) -> ::windows::core::Result<()>;
    fn NumberOfCopies(&self) -> ::windows::core::Result<u32>;
}
impl ::windows::core::RuntimeName for IPrintTaskOptionsCoreProperties {
    const NAME: &'static str = "Windows.Graphics.Printing.IPrintTaskOptionsCoreProperties";
}
impl IPrintTaskOptionsCorePropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTaskOptionsCorePropertiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintTaskOptionsCorePropertiesVtbl {
        unsafe extern "system" fn SetMediaSize<Impl: IPrintTaskOptionsCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PrintMediaSize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMediaSize(value).into()
        }
        unsafe extern "system" fn MediaSize<Impl: IPrintTaskOptionsCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintMediaSize) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMediaType<Impl: IPrintTaskOptionsCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PrintMediaType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMediaType(value).into()
        }
        unsafe extern "system" fn MediaType<Impl: IPrintTaskOptionsCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintMediaType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetOrientation<Impl: IPrintTaskOptionsCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PrintOrientation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOrientation(value).into()
        }
        unsafe extern "system" fn Orientation<Impl: IPrintTaskOptionsCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintOrientation) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPrintQuality<Impl: IPrintTaskOptionsCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PrintQuality) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrintQuality(value).into()
        }
        unsafe extern "system" fn PrintQuality<Impl: IPrintTaskOptionsCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintQuality) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetColorMode<Impl: IPrintTaskOptionsCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PrintColorMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColorMode(value).into()
        }
        unsafe extern "system" fn ColorMode<Impl: IPrintTaskOptionsCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintColorMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDuplex<Impl: IPrintTaskOptionsCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PrintDuplex) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDuplex(value).into()
        }
        unsafe extern "system" fn Duplex<Impl: IPrintTaskOptionsCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintDuplex) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCollation<Impl: IPrintTaskOptionsCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PrintCollation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCollation(value).into()
        }
        unsafe extern "system" fn Collation<Impl: IPrintTaskOptionsCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintCollation) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetStaple<Impl: IPrintTaskOptionsCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PrintStaple) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStaple(value).into()
        }
        unsafe extern "system" fn Staple<Impl: IPrintTaskOptionsCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintStaple) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHolePunch<Impl: IPrintTaskOptionsCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PrintHolePunch) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHolePunch(value).into()
        }
        unsafe extern "system" fn HolePunch<Impl: IPrintTaskOptionsCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintHolePunch) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetBinding<Impl: IPrintTaskOptionsCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PrintBinding) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBinding(value).into()
        }
        unsafe extern "system" fn Binding<Impl: IPrintTaskOptionsCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintBinding) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MinCopies<Impl: IPrintTaskOptionsCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MaxCopies<Impl: IPrintTaskOptionsCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetNumberOfCopies<Impl: IPrintTaskOptionsCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNumberOfCopies(value).into()
        }
        unsafe extern "system" fn NumberOfCopies<Impl: IPrintTaskOptionsCorePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
pub trait IPrintTaskOptionsCoreUIConfigurationImpl: Sized {
    fn DisplayedOptions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IPrintTaskOptionsCoreUIConfiguration {
    const NAME: &'static str = "Windows.Graphics.Printing.IPrintTaskOptionsCoreUIConfiguration";
}
#[cfg(feature = "Foundation_Collections")]
impl IPrintTaskOptionsCoreUIConfigurationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTaskOptionsCoreUIConfigurationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintTaskOptionsCoreUIConfigurationVtbl {
        unsafe extern "system" fn DisplayedOptions<Impl: IPrintTaskOptionsCoreUIConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IPrintTaskProgressingEventArgsImpl: Sized {
    fn DocumentPageCount(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintTaskProgressingEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.IPrintTaskProgressingEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintTaskProgressingEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTaskProgressingEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintTaskProgressingEventArgsVtbl {
        unsafe extern "system" fn DocumentPageCount<Impl: IPrintTaskProgressingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
pub trait IPrintTaskRequestImpl: Sized {
    fn Deadline(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn CreatePrintTask(&self, title: &::windows::core::HSTRING, handler: &::core::option::Option<PrintTaskSourceRequestedHandler>) -> ::windows::core::Result<PrintTask>;
    fn GetDeferral(&self) -> ::windows::core::Result<PrintTaskRequestedDeferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrintTaskRequest {
    const NAME: &'static str = "Windows.Graphics.Printing.IPrintTaskRequest";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPrintTaskRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTaskRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintTaskRequestVtbl {
        unsafe extern "system" fn Deadline<Impl: IPrintTaskRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreatePrintTask<Impl: IPrintTaskRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, title: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, handler: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeferral<Impl: IPrintTaskRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IPrintTaskRequestedDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintTaskRequestedDeferral {
    const NAME: &'static str = "Windows.Graphics.Printing.IPrintTaskRequestedDeferral";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintTaskRequestedDeferralVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTaskRequestedDeferralImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintTaskRequestedDeferralVtbl {
        unsafe extern "system" fn Complete<Impl: IPrintTaskRequestedDeferralImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait IPrintTaskRequestedEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<PrintTaskRequest>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintTaskRequestedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.IPrintTaskRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintTaskRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTaskRequestedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintTaskRequestedEventArgsVtbl {
        unsafe extern "system" fn Request<Impl: IPrintTaskRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IPrintTaskSourceRequestedArgsImpl: Sized {
    fn Deadline(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn SetSource(&self, source: &::core::option::Option<IPrintDocumentSource>) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<PrintTaskSourceRequestedDeferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrintTaskSourceRequestedArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.IPrintTaskSourceRequestedArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPrintTaskSourceRequestedArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTaskSourceRequestedArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintTaskSourceRequestedArgsVtbl {
        unsafe extern "system" fn Deadline<Impl: IPrintTaskSourceRequestedArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSource<Impl: IPrintTaskSourceRequestedArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSource(&*(&source as *const <IPrintDocumentSource as ::windows::core::Abi>::Abi as *const <IPrintDocumentSource as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetDeferral<Impl: IPrintTaskSourceRequestedArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IPrintTaskSourceRequestedDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintTaskSourceRequestedDeferral {
    const NAME: &'static str = "Windows.Graphics.Printing.IPrintTaskSourceRequestedDeferral";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintTaskSourceRequestedDeferralVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTaskSourceRequestedDeferralImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintTaskSourceRequestedDeferralVtbl {
        unsafe extern "system" fn Complete<Impl: IPrintTaskSourceRequestedDeferralImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait IPrintTaskTargetDeviceSupportImpl: Sized {
    fn SetIsPrinterTargetEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsPrinterTargetEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIs3DManufacturingTargetEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn Is3DManufacturingTargetEnabled(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintTaskTargetDeviceSupport {
    const NAME: &'static str = "Windows.Graphics.Printing.IPrintTaskTargetDeviceSupport";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintTaskTargetDeviceSupportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTaskTargetDeviceSupportImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintTaskTargetDeviceSupportVtbl {
        unsafe extern "system" fn SetIsPrinterTargetEnabled<Impl: IPrintTaskTargetDeviceSupportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsPrinterTargetEnabled(value).into()
        }
        unsafe extern "system" fn IsPrinterTargetEnabled<Impl: IPrintTaskTargetDeviceSupportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIs3DManufacturingTargetEnabled<Impl: IPrintTaskTargetDeviceSupportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIs3DManufacturingTargetEnabled(value).into()
        }
        unsafe extern "system" fn Is3DManufacturingTargetEnabled<Impl: IPrintTaskTargetDeviceSupportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
pub trait IStandardPrintTaskOptionsStaticImpl: Sized {
    fn MediaSize(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MediaType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Orientation(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PrintQuality(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ColorMode(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Duplex(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Collation(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Staple(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HolePunch(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Binding(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Copies(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NUp(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn InputBin(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStandardPrintTaskOptionsStatic {
    const NAME: &'static str = "Windows.Graphics.Printing.IStandardPrintTaskOptionsStatic";
}
#[cfg(feature = "implement_exclusive")]
impl IStandardPrintTaskOptionsStaticVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStandardPrintTaskOptionsStaticImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStandardPrintTaskOptionsStaticVtbl {
        unsafe extern "system" fn MediaSize<Impl: IStandardPrintTaskOptionsStaticImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MediaType<Impl: IStandardPrintTaskOptionsStaticImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Orientation<Impl: IStandardPrintTaskOptionsStaticImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PrintQuality<Impl: IStandardPrintTaskOptionsStaticImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ColorMode<Impl: IStandardPrintTaskOptionsStaticImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Duplex<Impl: IStandardPrintTaskOptionsStaticImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Collation<Impl: IStandardPrintTaskOptionsStaticImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Staple<Impl: IStandardPrintTaskOptionsStaticImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HolePunch<Impl: IStandardPrintTaskOptionsStaticImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Binding<Impl: IStandardPrintTaskOptionsStaticImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Copies<Impl: IStandardPrintTaskOptionsStaticImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn NUp<Impl: IStandardPrintTaskOptionsStaticImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn InputBin<Impl: IStandardPrintTaskOptionsStaticImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
pub trait IStandardPrintTaskOptionsStatic2Impl: Sized {
    fn Bordering(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStandardPrintTaskOptionsStatic2 {
    const NAME: &'static str = "Windows.Graphics.Printing.IStandardPrintTaskOptionsStatic2";
}
#[cfg(feature = "implement_exclusive")]
impl IStandardPrintTaskOptionsStatic2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStandardPrintTaskOptionsStatic2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStandardPrintTaskOptionsStatic2Vtbl {
        unsafe extern "system" fn Bordering<Impl: IStandardPrintTaskOptionsStatic2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
pub trait IStandardPrintTaskOptionsStatic3Impl: Sized {
    fn CustomPageRanges(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStandardPrintTaskOptionsStatic3 {
    const NAME: &'static str = "Windows.Graphics.Printing.IStandardPrintTaskOptionsStatic3";
}
#[cfg(feature = "implement_exclusive")]
impl IStandardPrintTaskOptionsStatic3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStandardPrintTaskOptionsStatic3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStandardPrintTaskOptionsStatic3Vtbl {
        unsafe extern "system" fn CustomPageRanges<Impl: IStandardPrintTaskOptionsStatic3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
