#[cfg(feature = "implement_exclusive")]
pub trait IActivatedDeferral_Impl: Sized {
    fn Complete(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IActivatedDeferral {
    const NAME: &'static str = "Windows.UI.WebUI.IActivatedDeferral";
}
#[cfg(feature = "implement_exclusive")]
impl IActivatedDeferral_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActivatedDeferral_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IActivatedDeferral_Vtbl {
        unsafe extern "system" fn Complete<Impl: IActivatedDeferral_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IActivatedDeferral, BASE_OFFSET>(), Complete: Complete::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActivatedDeferral as ::windows::core::Interface>::IID
    }
}
pub trait IActivatedEventArgsDeferral_Impl: Sized {
    fn ActivatedOperation(&mut self) -> ::windows::core::Result<ActivatedOperation>;
}
impl ::windows::core::RuntimeName for IActivatedEventArgsDeferral {
    const NAME: &'static str = "Windows.UI.WebUI.IActivatedEventArgsDeferral";
}
impl IActivatedEventArgsDeferral_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActivatedEventArgsDeferral_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IActivatedEventArgsDeferral_Vtbl {
        unsafe extern "system" fn ActivatedOperation<Impl: IActivatedEventArgsDeferral_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActivatedOperation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IActivatedEventArgsDeferral, BASE_OFFSET>(),
            ActivatedOperation: ActivatedOperation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActivatedEventArgsDeferral as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IActivatedOperation_Impl: Sized {
    fn GetDeferral(&mut self) -> ::windows::core::Result<ActivatedDeferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IActivatedOperation {
    const NAME: &'static str = "Windows.UI.WebUI.IActivatedOperation";
}
#[cfg(feature = "implement_exclusive")]
impl IActivatedOperation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActivatedOperation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IActivatedOperation_Vtbl {
        unsafe extern "system" fn GetDeferral<Impl: IActivatedOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IActivatedOperation, BASE_OFFSET>(), GetDeferral: GetDeferral::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActivatedOperation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Graphics_Printing", feature = "implement_exclusive"))]
pub trait IHtmlPrintDocumentSource_Impl: Sized + super::super::Graphics::Printing::IPrintDocumentSource_Impl {
    fn Content(&mut self) -> ::windows::core::Result<PrintContent>;
    fn SetContent(&mut self, value: PrintContent) -> ::windows::core::Result<()>;
    fn LeftMargin(&mut self) -> ::windows::core::Result<f32>;
    fn SetLeftMargin(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn TopMargin(&mut self) -> ::windows::core::Result<f32>;
    fn SetTopMargin(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn RightMargin(&mut self) -> ::windows::core::Result<f32>;
    fn SetRightMargin(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn BottomMargin(&mut self) -> ::windows::core::Result<f32>;
    fn SetBottomMargin(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn EnableHeaderFooter(&mut self) -> ::windows::core::Result<bool>;
    fn SetEnableHeaderFooter(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn ShrinkToFit(&mut self) -> ::windows::core::Result<bool>;
    fn SetShrinkToFit(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn PercentScale(&mut self) -> ::windows::core::Result<f32>;
    fn SetPercentScale(&mut self, scalepercent: f32) -> ::windows::core::Result<()>;
    fn PageRange(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TrySetPageRange(&mut self, strpagerange: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Graphics_Printing", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHtmlPrintDocumentSource {
    const NAME: &'static str = "Windows.UI.WebUI.IHtmlPrintDocumentSource";
}
#[cfg(all(feature = "Graphics_Printing", feature = "implement_exclusive"))]
impl IHtmlPrintDocumentSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHtmlPrintDocumentSource_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHtmlPrintDocumentSource_Vtbl {
        unsafe extern "system" fn Content<Impl: IHtmlPrintDocumentSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintContent) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Content() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContent<Impl: IHtmlPrintDocumentSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PrintContent) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContent(value).into()
        }
        unsafe extern "system" fn LeftMargin<Impl: IHtmlPrintDocumentSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LeftMargin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLeftMargin<Impl: IHtmlPrintDocumentSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLeftMargin(value).into()
        }
        unsafe extern "system" fn TopMargin<Impl: IHtmlPrintDocumentSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TopMargin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTopMargin<Impl: IHtmlPrintDocumentSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTopMargin(value).into()
        }
        unsafe extern "system" fn RightMargin<Impl: IHtmlPrintDocumentSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RightMargin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRightMargin<Impl: IHtmlPrintDocumentSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRightMargin(value).into()
        }
        unsafe extern "system" fn BottomMargin<Impl: IHtmlPrintDocumentSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BottomMargin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBottomMargin<Impl: IHtmlPrintDocumentSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBottomMargin(value).into()
        }
        unsafe extern "system" fn EnableHeaderFooter<Impl: IHtmlPrintDocumentSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableHeaderFooter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableHeaderFooter<Impl: IHtmlPrintDocumentSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnableHeaderFooter(value).into()
        }
        unsafe extern "system" fn ShrinkToFit<Impl: IHtmlPrintDocumentSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShrinkToFit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShrinkToFit<Impl: IHtmlPrintDocumentSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShrinkToFit(value).into()
        }
        unsafe extern "system" fn PercentScale<Impl: IHtmlPrintDocumentSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PercentScale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPercentScale<Impl: IHtmlPrintDocumentSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scalepercent: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPercentScale(scalepercent).into()
        }
        unsafe extern "system" fn PageRange<Impl: IHtmlPrintDocumentSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PageRange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrySetPageRange<Impl: IHtmlPrintDocumentSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpagerange: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrySetPageRange(&*(&strpagerange as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHtmlPrintDocumentSource, BASE_OFFSET>(),
            Content: Content::<Impl, IMPL_OFFSET>,
            SetContent: SetContent::<Impl, IMPL_OFFSET>,
            LeftMargin: LeftMargin::<Impl, IMPL_OFFSET>,
            SetLeftMargin: SetLeftMargin::<Impl, IMPL_OFFSET>,
            TopMargin: TopMargin::<Impl, IMPL_OFFSET>,
            SetTopMargin: SetTopMargin::<Impl, IMPL_OFFSET>,
            RightMargin: RightMargin::<Impl, IMPL_OFFSET>,
            SetRightMargin: SetRightMargin::<Impl, IMPL_OFFSET>,
            BottomMargin: BottomMargin::<Impl, IMPL_OFFSET>,
            SetBottomMargin: SetBottomMargin::<Impl, IMPL_OFFSET>,
            EnableHeaderFooter: EnableHeaderFooter::<Impl, IMPL_OFFSET>,
            SetEnableHeaderFooter: SetEnableHeaderFooter::<Impl, IMPL_OFFSET>,
            ShrinkToFit: ShrinkToFit::<Impl, IMPL_OFFSET>,
            SetShrinkToFit: SetShrinkToFit::<Impl, IMPL_OFFSET>,
            PercentScale: PercentScale::<Impl, IMPL_OFFSET>,
            SetPercentScale: SetPercentScale::<Impl, IMPL_OFFSET>,
            PageRange: PageRange::<Impl, IMPL_OFFSET>,
            TrySetPageRange: TrySetPageRange::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHtmlPrintDocumentSource as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation", feature = "implement_exclusive"))]
pub trait INewWebUIViewCreatedEventArgs_Impl: Sized {
    fn WebUIView(&mut self) -> ::windows::core::Result<WebUIView>;
    fn ActivatedEventArgs(&mut self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::IActivatedEventArgs>;
    fn HasPendingNavigate(&mut self) -> ::windows::core::Result<bool>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for INewWebUIViewCreatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.INewWebUIViewCreatedEventArgs";
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation", feature = "implement_exclusive"))]
impl INewWebUIViewCreatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INewWebUIViewCreatedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INewWebUIViewCreatedEventArgs_Vtbl {
        unsafe extern "system" fn WebUIView<Impl: INewWebUIViewCreatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WebUIView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivatedEventArgs<Impl: INewWebUIViewCreatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActivatedEventArgs() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasPendingNavigate<Impl: INewWebUIViewCreatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasPendingNavigate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: INewWebUIViewCreatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, INewWebUIViewCreatedEventArgs, BASE_OFFSET>(),
            WebUIView: WebUIView::<Impl, IMPL_OFFSET>,
            ActivatedEventArgs: ActivatedEventArgs::<Impl, IMPL_OFFSET>,
            HasPendingNavigate: HasPendingNavigate::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INewWebUIViewCreatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel", feature = "ApplicationModel_Activation", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IWebUIActivationStatics_Impl: Sized {
    fn Activated(&mut self, handler: &::core::option::Option<ActivatedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveActivated(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Suspending(&mut self, handler: &::core::option::Option<SuspendingEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSuspending(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Resuming(&mut self, handler: &::core::option::Option<ResumingEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveResuming(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Navigated(&mut self, handler: &::core::option::Option<NavigatedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveNavigated(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "ApplicationModel", feature = "ApplicationModel_Activation", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebUIActivationStatics {
    const NAME: &'static str = "Windows.UI.WebUI.IWebUIActivationStatics";
}
#[cfg(all(feature = "ApplicationModel", feature = "ApplicationModel_Activation", feature = "Foundation", feature = "implement_exclusive"))]
impl IWebUIActivationStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebUIActivationStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebUIActivationStatics_Vtbl {
        unsafe extern "system" fn Activated<Impl: IWebUIActivationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Activated(&*(&handler as *const <ActivatedEventHandler as ::windows::core::Abi>::Abi as *const <ActivatedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveActivated<Impl: IWebUIActivationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveActivated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Suspending<Impl: IWebUIActivationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Suspending(&*(&handler as *const <SuspendingEventHandler as ::windows::core::Abi>::Abi as *const <SuspendingEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSuspending<Impl: IWebUIActivationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSuspending(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Resuming<Impl: IWebUIActivationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Resuming(&*(&handler as *const <ResumingEventHandler as ::windows::core::Abi>::Abi as *const <ResumingEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveResuming<Impl: IWebUIActivationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveResuming(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Navigated<Impl: IWebUIActivationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Navigated(&*(&handler as *const <NavigatedEventHandler as ::windows::core::Abi>::Abi as *const <NavigatedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveNavigated<Impl: IWebUIActivationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveNavigated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebUIActivationStatics, BASE_OFFSET>(),
            Activated: Activated::<Impl, IMPL_OFFSET>,
            RemoveActivated: RemoveActivated::<Impl, IMPL_OFFSET>,
            Suspending: Suspending::<Impl, IMPL_OFFSET>,
            RemoveSuspending: RemoveSuspending::<Impl, IMPL_OFFSET>,
            Resuming: Resuming::<Impl, IMPL_OFFSET>,
            RemoveResuming: RemoveResuming::<Impl, IMPL_OFFSET>,
            Navigated: Navigated::<Impl, IMPL_OFFSET>,
            RemoveNavigated: RemoveNavigated::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebUIActivationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IWebUIActivationStatics2_Impl: Sized {
    fn LeavingBackground(&mut self, handler: &::core::option::Option<LeavingBackgroundEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLeavingBackground(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EnteredBackground(&mut self, handler: &::core::option::Option<EnteredBackgroundEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveEnteredBackground(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EnablePrelaunch(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebUIActivationStatics2 {
    const NAME: &'static str = "Windows.UI.WebUI.IWebUIActivationStatics2";
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "implement_exclusive"))]
impl IWebUIActivationStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebUIActivationStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebUIActivationStatics2_Vtbl {
        unsafe extern "system" fn LeavingBackground<Impl: IWebUIActivationStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LeavingBackground(&*(&handler as *const <LeavingBackgroundEventHandler as ::windows::core::Abi>::Abi as *const <LeavingBackgroundEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLeavingBackground<Impl: IWebUIActivationStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveLeavingBackground(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EnteredBackground<Impl: IWebUIActivationStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnteredBackground(&*(&handler as *const <EnteredBackgroundEventHandler as ::windows::core::Abi>::Abi as *const <EnteredBackgroundEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEnteredBackground<Impl: IWebUIActivationStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveEnteredBackground(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EnablePrelaunch<Impl: IWebUIActivationStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnablePrelaunch(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebUIActivationStatics2, BASE_OFFSET>(),
            LeavingBackground: LeavingBackground::<Impl, IMPL_OFFSET>,
            RemoveLeavingBackground: RemoveLeavingBackground::<Impl, IMPL_OFFSET>,
            EnteredBackground: EnteredBackground::<Impl, IMPL_OFFSET>,
            RemoveEnteredBackground: RemoveEnteredBackground::<Impl, IMPL_OFFSET>,
            EnablePrelaunch: EnablePrelaunch::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebUIActivationStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
pub trait IWebUIActivationStatics3_Impl: Sized {
    fn RequestRestartAsync(&mut self, launcharguments: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::ApplicationModel::Core::AppRestartFailureReason>>;
    fn RequestRestartForUserAsync(&mut self, user: &::core::option::Option<super::super::System::User>, launcharguments: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::ApplicationModel::Core::AppRestartFailureReason>>;
}
#[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebUIActivationStatics3 {
    const NAME: &'static str = "Windows.UI.WebUI.IWebUIActivationStatics3";
}
#[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl IWebUIActivationStatics3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebUIActivationStatics3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebUIActivationStatics3_Vtbl {
        unsafe extern "system" fn RequestRestartAsync<Impl: IWebUIActivationStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, launcharguments: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestRestartAsync(&*(&launcharguments as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestRestartForUserAsync<Impl: IWebUIActivationStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, launcharguments: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestRestartForUserAsync(&*(&user as *const <super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::System::User as ::windows::core::DefaultType>::DefaultType), &*(&launcharguments as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebUIActivationStatics3, BASE_OFFSET>(),
            RequestRestartAsync: RequestRestartAsync::<Impl, IMPL_OFFSET>,
            RequestRestartForUserAsync: RequestRestartForUserAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebUIActivationStatics3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IWebUIActivationStatics4_Impl: Sized {
    fn NewWebUIViewCreated(&mut self, handler: &::core::option::Option<super::super::Foundation::EventHandler<NewWebUIViewCreatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveNewWebUIViewCreated(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn BackgroundActivated(&mut self, handler: &::core::option::Option<BackgroundActivatedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBackgroundActivated(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebUIActivationStatics4 {
    const NAME: &'static str = "Windows.UI.WebUI.IWebUIActivationStatics4";
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation", feature = "implement_exclusive"))]
impl IWebUIActivationStatics4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebUIActivationStatics4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebUIActivationStatics4_Vtbl {
        unsafe extern "system" fn NewWebUIViewCreated<Impl: IWebUIActivationStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NewWebUIViewCreated(&*(&handler as *const <super::super::Foundation::EventHandler<NewWebUIViewCreatedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<NewWebUIViewCreatedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveNewWebUIViewCreated<Impl: IWebUIActivationStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveNewWebUIViewCreated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BackgroundActivated<Impl: IWebUIActivationStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BackgroundActivated(&*(&handler as *const <BackgroundActivatedEventHandler as ::windows::core::Abi>::Abi as *const <BackgroundActivatedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBackgroundActivated<Impl: IWebUIActivationStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveBackgroundActivated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebUIActivationStatics4, BASE_OFFSET>(),
            NewWebUIViewCreated: NewWebUIViewCreated::<Impl, IMPL_OFFSET>,
            RemoveNewWebUIViewCreated: RemoveNewWebUIViewCreated::<Impl, IMPL_OFFSET>,
            BackgroundActivated: BackgroundActivated::<Impl, IMPL_OFFSET>,
            RemoveBackgroundActivated: RemoveBackgroundActivated::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebUIActivationStatics4 as ::windows::core::Interface>::IID
    }
}
pub trait IWebUIBackgroundTaskInstance_Impl: Sized {
    fn Succeeded(&mut self) -> ::windows::core::Result<bool>;
    fn SetSucceeded(&mut self, succeeded: bool) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWebUIBackgroundTaskInstance {
    const NAME: &'static str = "Windows.UI.WebUI.IWebUIBackgroundTaskInstance";
}
impl IWebUIBackgroundTaskInstance_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebUIBackgroundTaskInstance_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebUIBackgroundTaskInstance_Vtbl {
        unsafe extern "system" fn Succeeded<Impl: IWebUIBackgroundTaskInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Succeeded() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSucceeded<Impl: IWebUIBackgroundTaskInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, succeeded: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSucceeded(succeeded).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebUIBackgroundTaskInstance, BASE_OFFSET>(),
            Succeeded: Succeeded::<Impl, IMPL_OFFSET>,
            SetSucceeded: SetSucceeded::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebUIBackgroundTaskInstance as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebUIBackgroundTaskInstanceStatics_Impl: Sized {
    fn Current(&mut self) -> ::windows::core::Result<IWebUIBackgroundTaskInstance>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebUIBackgroundTaskInstanceStatics {
    const NAME: &'static str = "Windows.UI.WebUI.IWebUIBackgroundTaskInstanceStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IWebUIBackgroundTaskInstanceStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebUIBackgroundTaskInstanceStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebUIBackgroundTaskInstanceStatics_Vtbl {
        unsafe extern "system" fn Current<Impl: IWebUIBackgroundTaskInstanceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Current() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebUIBackgroundTaskInstanceStatics, BASE_OFFSET>(),
            Current: Current::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebUIBackgroundTaskInstanceStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebUINavigatedDeferral_Impl: Sized {
    fn Complete(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebUINavigatedDeferral {
    const NAME: &'static str = "Windows.UI.WebUI.IWebUINavigatedDeferral";
}
#[cfg(feature = "implement_exclusive")]
impl IWebUINavigatedDeferral_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebUINavigatedDeferral_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebUINavigatedDeferral_Vtbl {
        unsafe extern "system" fn Complete<Impl: IWebUINavigatedDeferral_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IWebUINavigatedDeferral, BASE_OFFSET>(), Complete: Complete::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebUINavigatedDeferral as ::windows::core::Interface>::IID
    }
}
pub trait IWebUINavigatedEventArgs_Impl: Sized {
    fn NavigatedOperation(&mut self) -> ::windows::core::Result<WebUINavigatedOperation>;
}
impl ::windows::core::RuntimeName for IWebUINavigatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.IWebUINavigatedEventArgs";
}
impl IWebUINavigatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebUINavigatedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebUINavigatedEventArgs_Vtbl {
        unsafe extern "system" fn NavigatedOperation<Impl: IWebUINavigatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NavigatedOperation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebUINavigatedEventArgs, BASE_OFFSET>(),
            NavigatedOperation: NavigatedOperation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebUINavigatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebUINavigatedOperation_Impl: Sized {
    fn GetDeferral(&mut self) -> ::windows::core::Result<WebUINavigatedDeferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebUINavigatedOperation {
    const NAME: &'static str = "Windows.UI.WebUI.IWebUINavigatedOperation";
}
#[cfg(feature = "implement_exclusive")]
impl IWebUINavigatedOperation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebUINavigatedOperation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebUINavigatedOperation_Vtbl {
        unsafe extern "system" fn GetDeferral<Impl: IWebUINavigatedOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IWebUINavigatedOperation, BASE_OFFSET>(), GetDeferral: GetDeferral::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebUINavigatedOperation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IWebUIView_Impl: Sized {
    fn ApplicationViewId(&mut self) -> ::windows::core::Result<i32>;
    fn Closed(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<WebUIView, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Activated(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<WebUIView, super::super::ApplicationModel::Activation::IActivatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveActivated(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn IgnoreApplicationContentUriRulesNavigationRestrictions(&mut self) -> ::windows::core::Result<bool>;
    fn SetIgnoreApplicationContentUriRulesNavigationRestrictions(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebUIView {
    const NAME: &'static str = "Windows.UI.WebUI.IWebUIView";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IWebUIView_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebUIView_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebUIView_Vtbl {
        unsafe extern "system" fn ApplicationViewId<Impl: IWebUIView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplicationViewId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Closed<Impl: IWebUIView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Closed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<WebUIView, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<WebUIView, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveClosed<Impl: IWebUIView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveClosed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Activated<Impl: IWebUIView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Activated(&*(&handler as *const <super::super::Foundation::TypedEventHandler<WebUIView, super::super::ApplicationModel::Activation::IActivatedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<WebUIView, super::super::ApplicationModel::Activation::IActivatedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveActivated<Impl: IWebUIView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveActivated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IgnoreApplicationContentUriRulesNavigationRestrictions<Impl: IWebUIView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IgnoreApplicationContentUriRulesNavigationRestrictions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIgnoreApplicationContentUriRulesNavigationRestrictions<Impl: IWebUIView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIgnoreApplicationContentUriRulesNavigationRestrictions(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebUIView, BASE_OFFSET>(),
            ApplicationViewId: ApplicationViewId::<Impl, IMPL_OFFSET>,
            Closed: Closed::<Impl, IMPL_OFFSET>,
            RemoveClosed: RemoveClosed::<Impl, IMPL_OFFSET>,
            Activated: Activated::<Impl, IMPL_OFFSET>,
            RemoveActivated: RemoveActivated::<Impl, IMPL_OFFSET>,
            IgnoreApplicationContentUriRulesNavigationRestrictions: IgnoreApplicationContentUriRulesNavigationRestrictions::<Impl, IMPL_OFFSET>,
            SetIgnoreApplicationContentUriRulesNavigationRestrictions: SetIgnoreApplicationContentUriRulesNavigationRestrictions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebUIView as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IWebUIViewStatics_Impl: Sized {
    fn CreateAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WebUIView>>;
    fn CreateWithUriAsync(&mut self, uri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WebUIView>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebUIViewStatics {
    const NAME: &'static str = "Windows.UI.WebUI.IWebUIViewStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IWebUIViewStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebUIViewStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebUIViewStatics_Vtbl {
        unsafe extern "system" fn CreateAsync<Impl: IWebUIViewStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithUriAsync<Impl: IWebUIViewStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithUriAsync(&*(&uri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebUIViewStatics, BASE_OFFSET>(),
            CreateAsync: CreateAsync::<Impl, IMPL_OFFSET>,
            CreateWithUriAsync: CreateWithUriAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebUIViewStatics as ::windows::core::Interface>::IID
    }
}
