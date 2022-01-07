#[cfg(feature = "implement_exclusive")]
pub trait IAddPagesEventArgsImpl: Sized {
    fn PrintTaskOptions(&self) -> ::windows::core::Result<super::super::super::Graphics::Printing::PrintTaskOptions>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAddPagesEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Printing.IAddPagesEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAddPagesEventArgsVtbl {
    pub const fn new<Impl: IAddPagesEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAddPagesEventArgsVtbl {
        unsafe extern "system" fn PrintTaskOptions<Impl: IAddPagesEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrintTaskOptions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAddPagesEventArgs>, base.5, PrintTaskOptions::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGetPreviewPageEventArgsImpl: Sized {
    fn PageNumber(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGetPreviewPageEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Printing.IGetPreviewPageEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IGetPreviewPageEventArgsVtbl {
    pub const fn new<Impl: IGetPreviewPageEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGetPreviewPageEventArgsVtbl {
        unsafe extern "system" fn PageNumber<Impl: IGetPreviewPageEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PageNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGetPreviewPageEventArgs>, base.5, PageNumber::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaginateEventArgsImpl: Sized {
    fn PrintTaskOptions(&self) -> ::windows::core::Result<super::super::super::Graphics::Printing::PrintTaskOptions>;
    fn CurrentPreviewPageNumber(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPaginateEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Printing.IPaginateEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPaginateEventArgsVtbl {
    pub const fn new<Impl: IPaginateEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPaginateEventArgsVtbl {
        unsafe extern "system" fn PrintTaskOptions<Impl: IPaginateEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrintTaskOptions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentPreviewPageNumber<Impl: IPaginateEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CurrentPreviewPageNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPaginateEventArgs>, base.5, PrintTaskOptions::<Impl, OFFSET>, CurrentPreviewPageNumber::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintDocumentImpl: Sized {
    fn DocumentSource(&self) -> ::windows::core::Result<super::super::super::Graphics::Printing::IPrintDocumentSource>;
    fn Paginate(&self, handler: &::core::option::Option<PaginateEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePaginate(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetPreviewPage(&self, handler: &::core::option::Option<GetPreviewPageEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveGetPreviewPage(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AddPages(&self, handler: &::core::option::Option<AddPagesEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveAddPages(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AddPage(&self, pagevisual: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn AddPagesComplete(&self) -> ::windows::core::Result<()>;
    fn SetPreviewPageCount(&self, count: i32, r#type: PreviewPageCountType) -> ::windows::core::Result<()>;
    fn SetPreviewPage(&self, pagenumber: i32, pagevisual: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn InvalidatePreview(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintDocument {
    const NAME: &'static str = "Windows.UI.Xaml.Printing.IPrintDocument";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintDocumentVtbl {
    pub const fn new<Impl: IPrintDocumentImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrintDocumentVtbl {
        unsafe extern "system" fn DocumentSource<Impl: IPrintDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DocumentSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Paginate<Impl: IPrintDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Paginate(&*(&handler as *const <PaginateEventHandler as ::windows::core::Abi>::Abi as *const <PaginateEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePaginate<Impl: IPrintDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemovePaginate(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetPreviewPage<Impl: IPrintDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPreviewPage(&*(&handler as *const <GetPreviewPageEventHandler as ::windows::core::Abi>::Abi as *const <GetPreviewPageEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveGetPreviewPage<Impl: IPrintDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveGetPreviewPage(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddPages<Impl: IPrintDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddPages(&*(&handler as *const <AddPagesEventHandler as ::windows::core::Abi>::Abi as *const <AddPagesEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAddPages<Impl: IPrintDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveAddPages(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddPage<Impl: IPrintDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pagevisual: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).AddPage(&*(&pagevisual as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddPagesComplete<Impl: IPrintDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).AddPagesComplete().into()
        }
        unsafe extern "system" fn SetPreviewPageCount<Impl: IPrintDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: i32, r#type: PreviewPageCountType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPreviewPageCount(count, r#type).into()
        }
        unsafe extern "system" fn SetPreviewPage<Impl: IPrintDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pagenumber: i32, pagevisual: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPreviewPage(pagenumber, &*(&pagevisual as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InvalidatePreview<Impl: IPrintDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).InvalidatePreview().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrintDocument>, base.5, DocumentSource::<Impl, OFFSET>, Paginate::<Impl, OFFSET>, RemovePaginate::<Impl, OFFSET>, GetPreviewPage::<Impl, OFFSET>, RemoveGetPreviewPage::<Impl, OFFSET>, AddPages::<Impl, OFFSET>, RemoveAddPages::<Impl, OFFSET>, AddPage::<Impl, OFFSET>, AddPagesComplete::<Impl, OFFSET>, SetPreviewPageCount::<Impl, OFFSET>, SetPreviewPage::<Impl, OFFSET>, InvalidatePreview::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintDocumentFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<PrintDocument>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintDocumentFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Printing.IPrintDocumentFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintDocumentFactoryVtbl {
    pub const fn new<Impl: IPrintDocumentFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrintDocumentFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IPrintDocumentFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrintDocumentFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintDocumentStaticsImpl: Sized {
    fn DocumentSourceProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintDocumentStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Printing.IPrintDocumentStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintDocumentStaticsVtbl {
    pub const fn new<Impl: IPrintDocumentStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPrintDocumentStaticsVtbl {
        unsafe extern "system" fn DocumentSourceProperty<Impl: IPrintDocumentStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DocumentSourceProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPrintDocumentStatics>, base.5, DocumentSourceProperty::<Impl, OFFSET>)
    }
}
