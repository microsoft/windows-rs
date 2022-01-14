#[cfg(all(feature = "Graphics_Printing", feature = "implement_exclusive"))]
pub trait IAddPagesEventArgs_Impl: Sized {
    fn PrintTaskOptions(&mut self) -> ::windows::core::Result<super::super::super::Graphics::Printing::PrintTaskOptions>;
}
#[cfg(all(feature = "Graphics_Printing", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAddPagesEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Printing.IAddPagesEventArgs";
}
#[cfg(all(feature = "Graphics_Printing", feature = "implement_exclusive"))]
impl IAddPagesEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAddPagesEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAddPagesEventArgs_Vtbl {
        unsafe extern "system" fn PrintTaskOptions<Impl: IAddPagesEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrintTaskOptions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAddPagesEventArgs, BASE_OFFSET>(),
            PrintTaskOptions: PrintTaskOptions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAddPagesEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGetPreviewPageEventArgs_Impl: Sized {
    fn PageNumber(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGetPreviewPageEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Printing.IGetPreviewPageEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IGetPreviewPageEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGetPreviewPageEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGetPreviewPageEventArgs_Vtbl {
        unsafe extern "system" fn PageNumber<Impl: IGetPreviewPageEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PageNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IGetPreviewPageEventArgs, BASE_OFFSET>(), PageNumber: PageNumber::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGetPreviewPageEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Graphics_Printing", feature = "implement_exclusive"))]
pub trait IPaginateEventArgs_Impl: Sized {
    fn PrintTaskOptions(&mut self) -> ::windows::core::Result<super::super::super::Graphics::Printing::PrintTaskOptions>;
    fn CurrentPreviewPageNumber(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Graphics_Printing", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPaginateEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Printing.IPaginateEventArgs";
}
#[cfg(all(feature = "Graphics_Printing", feature = "implement_exclusive"))]
impl IPaginateEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPaginateEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPaginateEventArgs_Vtbl {
        unsafe extern "system" fn PrintTaskOptions<Impl: IPaginateEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrintTaskOptions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentPreviewPageNumber<Impl: IPaginateEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentPreviewPageNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPaginateEventArgs, BASE_OFFSET>(),
            PrintTaskOptions: PrintTaskOptions::<Impl, IMPL_OFFSET>,
            CurrentPreviewPageNumber: CurrentPreviewPageNumber::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPaginateEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Printing", feature = "implement_exclusive"))]
pub trait IPrintDocument_Impl: Sized {
    fn DocumentSource(&mut self) -> ::windows::core::Result<super::super::super::Graphics::Printing::IPrintDocumentSource>;
    fn Paginate(&mut self, handler: &::core::option::Option<PaginateEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePaginate(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetPreviewPage(&mut self, handler: &::core::option::Option<GetPreviewPageEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveGetPreviewPage(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AddPages(&mut self, handler: &::core::option::Option<AddPagesEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveAddPages(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AddPage(&mut self, pagevisual: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn AddPagesComplete(&mut self) -> ::windows::core::Result<()>;
    fn SetPreviewPageCount(&mut self, count: i32, r#type: PreviewPageCountType) -> ::windows::core::Result<()>;
    fn SetPreviewPage(&mut self, pagenumber: i32, pagevisual: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn InvalidatePreview(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Printing", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrintDocument {
    const NAME: &'static str = "Windows.UI.Xaml.Printing.IPrintDocument";
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Printing", feature = "implement_exclusive"))]
impl IPrintDocument_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintDocument_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintDocument_Vtbl {
        unsafe extern "system" fn DocumentSource<Impl: IPrintDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DocumentSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Paginate<Impl: IPrintDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Paginate(&*(&handler as *const <PaginateEventHandler as ::windows::core::Abi>::Abi as *const <PaginateEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePaginate<Impl: IPrintDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePaginate(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetPreviewPage<Impl: IPrintDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPreviewPage(&*(&handler as *const <GetPreviewPageEventHandler as ::windows::core::Abi>::Abi as *const <GetPreviewPageEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveGetPreviewPage<Impl: IPrintDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveGetPreviewPage(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddPages<Impl: IPrintDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddPages(&*(&handler as *const <AddPagesEventHandler as ::windows::core::Abi>::Abi as *const <AddPagesEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAddPages<Impl: IPrintDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAddPages(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddPage<Impl: IPrintDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagevisual: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPage(&*(&pagevisual as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddPagesComplete<Impl: IPrintDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPagesComplete().into()
        }
        unsafe extern "system" fn SetPreviewPageCount<Impl: IPrintDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: i32, r#type: PreviewPageCountType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPreviewPageCount(count, r#type).into()
        }
        unsafe extern "system" fn SetPreviewPage<Impl: IPrintDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagenumber: i32, pagevisual: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPreviewPage(pagenumber, &*(&pagevisual as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InvalidatePreview<Impl: IPrintDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InvalidatePreview().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintDocument, BASE_OFFSET>(),
            DocumentSource: DocumentSource::<Impl, IMPL_OFFSET>,
            Paginate: Paginate::<Impl, IMPL_OFFSET>,
            RemovePaginate: RemovePaginate::<Impl, IMPL_OFFSET>,
            GetPreviewPage: GetPreviewPage::<Impl, IMPL_OFFSET>,
            RemoveGetPreviewPage: RemoveGetPreviewPage::<Impl, IMPL_OFFSET>,
            AddPages: AddPages::<Impl, IMPL_OFFSET>,
            RemoveAddPages: RemoveAddPages::<Impl, IMPL_OFFSET>,
            AddPage: AddPage::<Impl, IMPL_OFFSET>,
            AddPagesComplete: AddPagesComplete::<Impl, IMPL_OFFSET>,
            SetPreviewPageCount: SetPreviewPageCount::<Impl, IMPL_OFFSET>,
            SetPreviewPage: SetPreviewPage::<Impl, IMPL_OFFSET>,
            InvalidatePreview: InvalidatePreview::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintDocument as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintDocumentFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<PrintDocument>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintDocumentFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Printing.IPrintDocumentFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintDocumentFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintDocumentFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintDocumentFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IPrintDocumentFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintDocumentFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintDocumentFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintDocumentStatics_Impl: Sized {
    fn DocumentSourceProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintDocumentStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Printing.IPrintDocumentStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintDocumentStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintDocumentStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintDocumentStatics_Vtbl {
        unsafe extern "system" fn DocumentSourceProperty<Impl: IPrintDocumentStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DocumentSourceProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintDocumentStatics, BASE_OFFSET>(),
            DocumentSourceProperty: DocumentSourceProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintDocumentStatics as ::windows::core::Interface>::IID
    }
}
