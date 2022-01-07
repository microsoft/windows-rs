#[cfg(feature = "implement_exclusive")]
pub trait IActivatedDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IActivatedDeferral {
    const NAME: &'static str = "Windows.UI.WebUI.IActivatedDeferral";
}
#[cfg(feature = "implement_exclusive")]
impl IActivatedDeferralVtbl {
    pub const fn new<Impl: IActivatedDeferralImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IActivatedDeferralVtbl {
        unsafe extern "system" fn Complete<Impl: IActivatedDeferralImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IActivatedDeferral>, base.5, Complete::<Impl, OFFSET>)
    }
}
pub trait IActivatedEventArgsDeferralImpl: Sized {
    fn ActivatedOperation(&self) -> ::windows::core::Result<ActivatedOperation>;
}
impl ::windows::core::RuntimeName for IActivatedEventArgsDeferral {
    const NAME: &'static str = "Windows.UI.WebUI.IActivatedEventArgsDeferral";
}
impl IActivatedEventArgsDeferralVtbl {
    pub const fn new<Impl: IActivatedEventArgsDeferralImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IActivatedEventArgsDeferralVtbl {
        unsafe extern "system" fn ActivatedOperation<Impl: IActivatedEventArgsDeferralImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ActivatedOperation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IActivatedEventArgsDeferral>, base.5, ActivatedOperation::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IActivatedOperationImpl: Sized {
    fn GetDeferral(&self) -> ::windows::core::Result<ActivatedDeferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IActivatedOperation {
    const NAME: &'static str = "Windows.UI.WebUI.IActivatedOperation";
}
#[cfg(feature = "implement_exclusive")]
impl IActivatedOperationVtbl {
    pub const fn new<Impl: IActivatedOperationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IActivatedOperationVtbl {
        unsafe extern "system" fn GetDeferral<Impl: IActivatedOperationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IActivatedOperation>, base.5, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "Graphics_Printing", feature = "implement_exclusive"))]
pub trait IHtmlPrintDocumentSourceImpl: Sized + IPrintDocumentSourceImpl {
    fn Content(&self) -> ::windows::core::Result<PrintContent>;
    fn SetContent(&self, value: PrintContent) -> ::windows::core::Result<()>;
    fn LeftMargin(&self) -> ::windows::core::Result<f32>;
    fn SetLeftMargin(&self, value: f32) -> ::windows::core::Result<()>;
    fn TopMargin(&self) -> ::windows::core::Result<f32>;
    fn SetTopMargin(&self, value: f32) -> ::windows::core::Result<()>;
    fn RightMargin(&self) -> ::windows::core::Result<f32>;
    fn SetRightMargin(&self, value: f32) -> ::windows::core::Result<()>;
    fn BottomMargin(&self) -> ::windows::core::Result<f32>;
    fn SetBottomMargin(&self, value: f32) -> ::windows::core::Result<()>;
    fn EnableHeaderFooter(&self) -> ::windows::core::Result<bool>;
    fn SetEnableHeaderFooter(&self, value: bool) -> ::windows::core::Result<()>;
    fn ShrinkToFit(&self) -> ::windows::core::Result<bool>;
    fn SetShrinkToFit(&self, value: bool) -> ::windows::core::Result<()>;
    fn PercentScale(&self) -> ::windows::core::Result<f32>;
    fn SetPercentScale(&self, scalepercent: f32) -> ::windows::core::Result<()>;
    fn PageRange(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TrySetPageRange(&self, strpagerange: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Graphics_Printing", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHtmlPrintDocumentSource {
    const NAME: &'static str = "Windows.UI.WebUI.IHtmlPrintDocumentSource";
}
#[cfg(all(feature = "Graphics_Printing", feature = "implement_exclusive"))]
impl IHtmlPrintDocumentSourceVtbl {
    pub const fn new<Impl: IHtmlPrintDocumentSourceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHtmlPrintDocumentSourceVtbl {
        unsafe extern "system" fn Content<Impl: IHtmlPrintDocumentSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut PrintContent) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Content() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContent<Impl: IHtmlPrintDocumentSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: PrintContent) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetContent(value).into()
        }
        unsafe extern "system" fn LeftMargin<Impl: IHtmlPrintDocumentSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LeftMargin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLeftMargin<Impl: IHtmlPrintDocumentSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetLeftMargin(value).into()
        }
        unsafe extern "system" fn TopMargin<Impl: IHtmlPrintDocumentSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TopMargin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTopMargin<Impl: IHtmlPrintDocumentSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTopMargin(value).into()
        }
        unsafe extern "system" fn RightMargin<Impl: IHtmlPrintDocumentSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RightMargin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRightMargin<Impl: IHtmlPrintDocumentSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRightMargin(value).into()
        }
        unsafe extern "system" fn BottomMargin<Impl: IHtmlPrintDocumentSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BottomMargin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBottomMargin<Impl: IHtmlPrintDocumentSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetBottomMargin(value).into()
        }
        unsafe extern "system" fn EnableHeaderFooter<Impl: IHtmlPrintDocumentSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnableHeaderFooter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableHeaderFooter<Impl: IHtmlPrintDocumentSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetEnableHeaderFooter(value).into()
        }
        unsafe extern "system" fn ShrinkToFit<Impl: IHtmlPrintDocumentSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ShrinkToFit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShrinkToFit<Impl: IHtmlPrintDocumentSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetShrinkToFit(value).into()
        }
        unsafe extern "system" fn PercentScale<Impl: IHtmlPrintDocumentSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PercentScale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPercentScale<Impl: IHtmlPrintDocumentSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scalepercent: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPercentScale(scalepercent).into()
        }
        unsafe extern "system" fn PageRange<Impl: IHtmlPrintDocumentSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PageRange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrySetPageRange<Impl: IHtmlPrintDocumentSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strpagerange: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TrySetPageRange(&*(&strpagerange as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IHtmlPrintDocumentSource>,
            base.5,
            Content::<Impl, OFFSET>,
            SetContent::<Impl, OFFSET>,
            LeftMargin::<Impl, OFFSET>,
            SetLeftMargin::<Impl, OFFSET>,
            TopMargin::<Impl, OFFSET>,
            SetTopMargin::<Impl, OFFSET>,
            RightMargin::<Impl, OFFSET>,
            SetRightMargin::<Impl, OFFSET>,
            BottomMargin::<Impl, OFFSET>,
            SetBottomMargin::<Impl, OFFSET>,
            EnableHeaderFooter::<Impl, OFFSET>,
            SetEnableHeaderFooter::<Impl, OFFSET>,
            ShrinkToFit::<Impl, OFFSET>,
            SetShrinkToFit::<Impl, OFFSET>,
            PercentScale::<Impl, OFFSET>,
            SetPercentScale::<Impl, OFFSET>,
            PageRange::<Impl, OFFSET>,
            TrySetPageRange::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INewWebUIViewCreatedEventArgsImpl: Sized {
    fn WebUIView(&self) -> ::windows::core::Result<WebUIView>;
    fn ActivatedEventArgs(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::IActivatedEventArgs>;
    fn HasPendingNavigate(&self) -> ::windows::core::Result<bool>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INewWebUIViewCreatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.INewWebUIViewCreatedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl INewWebUIViewCreatedEventArgsVtbl {
    pub const fn new<Impl: INewWebUIViewCreatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INewWebUIViewCreatedEventArgsVtbl {
        unsafe extern "system" fn WebUIView<Impl: INewWebUIViewCreatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WebUIView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivatedEventArgs<Impl: INewWebUIViewCreatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ActivatedEventArgs() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasPendingNavigate<Impl: INewWebUIViewCreatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HasPendingNavigate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: INewWebUIViewCreatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<INewWebUIViewCreatedEventArgs>, base.5, WebUIView::<Impl, OFFSET>, ActivatedEventArgs::<Impl, OFFSET>, HasPendingNavigate::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebUIActivationStaticsImpl: Sized {
    fn Activated(&self, handler: &::core::option::Option<ActivatedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveActivated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Suspending(&self, handler: &::core::option::Option<SuspendingEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSuspending(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Resuming(&self, handler: &::core::option::Option<ResumingEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveResuming(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Navigated(&self, handler: &::core::option::Option<NavigatedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveNavigated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebUIActivationStatics {
    const NAME: &'static str = "Windows.UI.WebUI.IWebUIActivationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IWebUIActivationStaticsVtbl {
    pub const fn new<Impl: IWebUIActivationStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebUIActivationStaticsVtbl {
        unsafe extern "system" fn Activated<Impl: IWebUIActivationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Activated(&*(&handler as *const <ActivatedEventHandler as ::windows::core::Abi>::Abi as *const <ActivatedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveActivated<Impl: IWebUIActivationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveActivated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Suspending<Impl: IWebUIActivationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Suspending(&*(&handler as *const <SuspendingEventHandler as ::windows::core::Abi>::Abi as *const <SuspendingEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSuspending<Impl: IWebUIActivationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveSuspending(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Resuming<Impl: IWebUIActivationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Resuming(&*(&handler as *const <ResumingEventHandler as ::windows::core::Abi>::Abi as *const <ResumingEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveResuming<Impl: IWebUIActivationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveResuming(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Navigated<Impl: IWebUIActivationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Navigated(&*(&handler as *const <NavigatedEventHandler as ::windows::core::Abi>::Abi as *const <NavigatedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveNavigated<Impl: IWebUIActivationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveNavigated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWebUIActivationStatics>, base.5, Activated::<Impl, OFFSET>, RemoveActivated::<Impl, OFFSET>, Suspending::<Impl, OFFSET>, RemoveSuspending::<Impl, OFFSET>, Resuming::<Impl, OFFSET>, RemoveResuming::<Impl, OFFSET>, Navigated::<Impl, OFFSET>, RemoveNavigated::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebUIActivationStatics2Impl: Sized {
    fn LeavingBackground(&self, handler: &::core::option::Option<LeavingBackgroundEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLeavingBackground(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EnteredBackground(&self, handler: &::core::option::Option<EnteredBackgroundEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveEnteredBackground(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EnablePrelaunch(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebUIActivationStatics2 {
    const NAME: &'static str = "Windows.UI.WebUI.IWebUIActivationStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IWebUIActivationStatics2Vtbl {
    pub const fn new<Impl: IWebUIActivationStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebUIActivationStatics2Vtbl {
        unsafe extern "system" fn LeavingBackground<Impl: IWebUIActivationStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LeavingBackground(&*(&handler as *const <LeavingBackgroundEventHandler as ::windows::core::Abi>::Abi as *const <LeavingBackgroundEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLeavingBackground<Impl: IWebUIActivationStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveLeavingBackground(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EnteredBackground<Impl: IWebUIActivationStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnteredBackground(&*(&handler as *const <EnteredBackgroundEventHandler as ::windows::core::Abi>::Abi as *const <EnteredBackgroundEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEnteredBackground<Impl: IWebUIActivationStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveEnteredBackground(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EnablePrelaunch<Impl: IWebUIActivationStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).EnablePrelaunch(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWebUIActivationStatics2>, base.5, LeavingBackground::<Impl, OFFSET>, RemoveLeavingBackground::<Impl, OFFSET>, EnteredBackground::<Impl, OFFSET>, RemoveEnteredBackground::<Impl, OFFSET>, EnablePrelaunch::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebUIActivationStatics3Impl: Sized {
    fn RequestRestartAsync(&self, launcharguments: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::ApplicationModel::Core::AppRestartFailureReason>>;
    fn RequestRestartForUserAsync(&self, user: &::core::option::Option<super::super::System::User>, launcharguments: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::ApplicationModel::Core::AppRestartFailureReason>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebUIActivationStatics3 {
    const NAME: &'static str = "Windows.UI.WebUI.IWebUIActivationStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IWebUIActivationStatics3Vtbl {
    pub const fn new<Impl: IWebUIActivationStatics3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebUIActivationStatics3Vtbl {
        unsafe extern "system" fn RequestRestartAsync<Impl: IWebUIActivationStatics3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, launcharguments: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestRestartAsync(&*(&launcharguments as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestRestartForUserAsync<Impl: IWebUIActivationStatics3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, launcharguments: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestRestartForUserAsync(&*(&user as *const <super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::System::User as ::windows::core::DefaultType>::DefaultType), &*(&launcharguments as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWebUIActivationStatics3>, base.5, RequestRestartAsync::<Impl, OFFSET>, RequestRestartForUserAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebUIActivationStatics4Impl: Sized {
    fn NewWebUIViewCreated(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<NewWebUIViewCreatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveNewWebUIViewCreated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn BackgroundActivated(&self, handler: &::core::option::Option<BackgroundActivatedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBackgroundActivated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebUIActivationStatics4 {
    const NAME: &'static str = "Windows.UI.WebUI.IWebUIActivationStatics4";
}
#[cfg(feature = "implement_exclusive")]
impl IWebUIActivationStatics4Vtbl {
    pub const fn new<Impl: IWebUIActivationStatics4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebUIActivationStatics4Vtbl {
        unsafe extern "system" fn NewWebUIViewCreated<Impl: IWebUIActivationStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NewWebUIViewCreated(&*(&handler as *const <super::super::Foundation::EventHandler<NewWebUIViewCreatedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<NewWebUIViewCreatedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveNewWebUIViewCreated<Impl: IWebUIActivationStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveNewWebUIViewCreated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BackgroundActivated<Impl: IWebUIActivationStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BackgroundActivated(&*(&handler as *const <BackgroundActivatedEventHandler as ::windows::core::Abi>::Abi as *const <BackgroundActivatedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBackgroundActivated<Impl: IWebUIActivationStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveBackgroundActivated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWebUIActivationStatics4>, base.5, NewWebUIViewCreated::<Impl, OFFSET>, RemoveNewWebUIViewCreated::<Impl, OFFSET>, BackgroundActivated::<Impl, OFFSET>, RemoveBackgroundActivated::<Impl, OFFSET>)
    }
}
pub trait IWebUIBackgroundTaskInstanceImpl: Sized {
    fn Succeeded(&self) -> ::windows::core::Result<bool>;
    fn SetSucceeded(&self, succeeded: bool) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWebUIBackgroundTaskInstance {
    const NAME: &'static str = "Windows.UI.WebUI.IWebUIBackgroundTaskInstance";
}
impl IWebUIBackgroundTaskInstanceVtbl {
    pub const fn new<Impl: IWebUIBackgroundTaskInstanceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebUIBackgroundTaskInstanceVtbl {
        unsafe extern "system" fn Succeeded<Impl: IWebUIBackgroundTaskInstanceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Succeeded() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSucceeded<Impl: IWebUIBackgroundTaskInstanceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, succeeded: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSucceeded(succeeded).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWebUIBackgroundTaskInstance>, base.5, Succeeded::<Impl, OFFSET>, SetSucceeded::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebUIBackgroundTaskInstanceStaticsImpl: Sized {
    fn Current(&self) -> ::windows::core::Result<IWebUIBackgroundTaskInstance>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebUIBackgroundTaskInstanceStatics {
    const NAME: &'static str = "Windows.UI.WebUI.IWebUIBackgroundTaskInstanceStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IWebUIBackgroundTaskInstanceStaticsVtbl {
    pub const fn new<Impl: IWebUIBackgroundTaskInstanceStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebUIBackgroundTaskInstanceStaticsVtbl {
        unsafe extern "system" fn Current<Impl: IWebUIBackgroundTaskInstanceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Current() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWebUIBackgroundTaskInstanceStatics>, base.5, Current::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebUINavigatedDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebUINavigatedDeferral {
    const NAME: &'static str = "Windows.UI.WebUI.IWebUINavigatedDeferral";
}
#[cfg(feature = "implement_exclusive")]
impl IWebUINavigatedDeferralVtbl {
    pub const fn new<Impl: IWebUINavigatedDeferralImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebUINavigatedDeferralVtbl {
        unsafe extern "system" fn Complete<Impl: IWebUINavigatedDeferralImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWebUINavigatedDeferral>, base.5, Complete::<Impl, OFFSET>)
    }
}
pub trait IWebUINavigatedEventArgsImpl: Sized {
    fn NavigatedOperation(&self) -> ::windows::core::Result<WebUINavigatedOperation>;
}
impl ::windows::core::RuntimeName for IWebUINavigatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.IWebUINavigatedEventArgs";
}
impl IWebUINavigatedEventArgsVtbl {
    pub const fn new<Impl: IWebUINavigatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebUINavigatedEventArgsVtbl {
        unsafe extern "system" fn NavigatedOperation<Impl: IWebUINavigatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NavigatedOperation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWebUINavigatedEventArgs>, base.5, NavigatedOperation::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebUINavigatedOperationImpl: Sized {
    fn GetDeferral(&self) -> ::windows::core::Result<WebUINavigatedDeferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebUINavigatedOperation {
    const NAME: &'static str = "Windows.UI.WebUI.IWebUINavigatedOperation";
}
#[cfg(feature = "implement_exclusive")]
impl IWebUINavigatedOperationVtbl {
    pub const fn new<Impl: IWebUINavigatedOperationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebUINavigatedOperationVtbl {
        unsafe extern "system" fn GetDeferral<Impl: IWebUINavigatedOperationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWebUINavigatedOperation>, base.5, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebUIViewImpl: Sized {
    fn ApplicationViewId(&self) -> ::windows::core::Result<i32>;
    fn Closed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<WebUIView, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Activated(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<WebUIView, super::super::ApplicationModel::Activation::IActivatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveActivated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn IgnoreApplicationContentUriRulesNavigationRestrictions(&self) -> ::windows::core::Result<bool>;
    fn SetIgnoreApplicationContentUriRulesNavigationRestrictions(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebUIView {
    const NAME: &'static str = "Windows.UI.WebUI.IWebUIView";
}
#[cfg(feature = "implement_exclusive")]
impl IWebUIViewVtbl {
    pub const fn new<Impl: IWebUIViewImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebUIViewVtbl {
        unsafe extern "system" fn ApplicationViewId<Impl: IWebUIViewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ApplicationViewId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Closed<Impl: IWebUIViewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Closed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<WebUIView, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<WebUIView, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveClosed<Impl: IWebUIViewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveClosed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Activated<Impl: IWebUIViewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Activated(&*(&handler as *const <super::super::Foundation::TypedEventHandler<WebUIView, super::super::ApplicationModel::Activation::IActivatedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<WebUIView, super::super::ApplicationModel::Activation::IActivatedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveActivated<Impl: IWebUIViewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveActivated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IgnoreApplicationContentUriRulesNavigationRestrictions<Impl: IWebUIViewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IgnoreApplicationContentUriRulesNavigationRestrictions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIgnoreApplicationContentUriRulesNavigationRestrictions<Impl: IWebUIViewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIgnoreApplicationContentUriRulesNavigationRestrictions(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWebUIView>, base.5, ApplicationViewId::<Impl, OFFSET>, Closed::<Impl, OFFSET>, RemoveClosed::<Impl, OFFSET>, Activated::<Impl, OFFSET>, RemoveActivated::<Impl, OFFSET>, IgnoreApplicationContentUriRulesNavigationRestrictions::<Impl, OFFSET>, SetIgnoreApplicationContentUriRulesNavigationRestrictions::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebUIViewStaticsImpl: Sized {
    fn CreateAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WebUIView>>;
    fn CreateWithUriAsync(&self, uri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WebUIView>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebUIViewStatics {
    const NAME: &'static str = "Windows.UI.WebUI.IWebUIViewStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IWebUIViewStaticsVtbl {
    pub const fn new<Impl: IWebUIViewStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebUIViewStaticsVtbl {
        unsafe extern "system" fn CreateAsync<Impl: IWebUIViewStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithUriAsync<Impl: IWebUIViewStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateWithUriAsync(&*(&uri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWebUIViewStatics>, base.5, CreateAsync::<Impl, OFFSET>, CreateWithUriAsync::<Impl, OFFSET>)
    }
}
