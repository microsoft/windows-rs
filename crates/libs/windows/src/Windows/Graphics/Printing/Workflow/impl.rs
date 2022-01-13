#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPrintWorkflowBackgroundSessionImpl: Sized {
    fn SetupRequested(&mut self, setupeventhandler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<PrintWorkflowBackgroundSession, PrintWorkflowBackgroundSetupRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSetupRequested(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Submitted(&mut self, submittedeventhandler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<PrintWorkflowBackgroundSession, PrintWorkflowSubmittedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSubmitted(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Status(&mut self) -> ::windows::core::Result<PrintWorkflowSessionStatus>;
    fn Start(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrintWorkflowBackgroundSession {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowBackgroundSession";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPrintWorkflowBackgroundSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowBackgroundSessionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintWorkflowBackgroundSessionVtbl {
        unsafe extern "system" fn SetupRequested<Impl: IPrintWorkflowBackgroundSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, setupeventhandler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetupRequested(&*(&setupeventhandler as *const <super::super::super::Foundation::TypedEventHandler<PrintWorkflowBackgroundSession, PrintWorkflowBackgroundSetupRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<PrintWorkflowBackgroundSession, PrintWorkflowBackgroundSetupRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSetupRequested<Impl: IPrintWorkflowBackgroundSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSetupRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Submitted<Impl: IPrintWorkflowBackgroundSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, submittedeventhandler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Submitted(&*(&submittedeventhandler as *const <super::super::super::Foundation::TypedEventHandler<PrintWorkflowBackgroundSession, PrintWorkflowSubmittedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<PrintWorkflowBackgroundSession, PrintWorkflowSubmittedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSubmitted<Impl: IPrintWorkflowBackgroundSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSubmitted(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Status<Impl: IPrintWorkflowBackgroundSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintWorkflowSessionStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Start<Impl: IPrintWorkflowBackgroundSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintWorkflowBackgroundSession, BASE_OFFSET>(),
            SetupRequested: SetupRequested::<Impl, IMPL_OFFSET>,
            RemoveSetupRequested: RemoveSetupRequested::<Impl, IMPL_OFFSET>,
            Submitted: Submitted::<Impl, IMPL_OFFSET>,
            RemoveSubmitted: RemoveSubmitted::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
            Start: Start::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintWorkflowBackgroundSession as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Printing_PrintTicket", feature = "implement_exclusive"))]
pub trait IPrintWorkflowBackgroundSetupRequestedEventArgsImpl: Sized {
    fn GetUserPrintTicketAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::PrintTicket::WorkflowPrintTicket>>;
    fn Configuration(&mut self) -> ::windows::core::Result<PrintWorkflowConfiguration>;
    fn SetRequiresUI(&mut self) -> ::windows::core::Result<()>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Printing_PrintTicket", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrintWorkflowBackgroundSetupRequestedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowBackgroundSetupRequestedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Printing_PrintTicket", feature = "implement_exclusive"))]
impl IPrintWorkflowBackgroundSetupRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowBackgroundSetupRequestedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintWorkflowBackgroundSetupRequestedEventArgsVtbl {
        unsafe extern "system" fn GetUserPrintTicketAsync<Impl: IPrintWorkflowBackgroundSetupRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUserPrintTicketAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Configuration<Impl: IPrintWorkflowBackgroundSetupRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Configuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRequiresUI<Impl: IPrintWorkflowBackgroundSetupRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRequiresUI().into()
        }
        unsafe extern "system" fn GetDeferral<Impl: IPrintWorkflowBackgroundSetupRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintWorkflowBackgroundSetupRequestedEventArgs, BASE_OFFSET>(),
            GetUserPrintTicketAsync: GetUserPrintTicketAsync::<Impl, IMPL_OFFSET>,
            Configuration: Configuration::<Impl, IMPL_OFFSET>,
            SetRequiresUI: SetRequiresUI::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintWorkflowBackgroundSetupRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowConfigurationImpl: Sized {
    fn SourceAppDisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn JobTitle(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SessionId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintWorkflowConfiguration {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowConfiguration";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintWorkflowConfigurationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowConfigurationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintWorkflowConfigurationVtbl {
        unsafe extern "system" fn SourceAppDisplayName<Impl: IPrintWorkflowConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceAppDisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JobTitle<Impl: IPrintWorkflowConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).JobTitle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionId<Impl: IPrintWorkflowConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintWorkflowConfiguration, BASE_OFFSET>(),
            SourceAppDisplayName: SourceAppDisplayName::<Impl, IMPL_OFFSET>,
            JobTitle: JobTitle::<Impl, IMPL_OFFSET>,
            SessionId: SessionId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintWorkflowConfiguration as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowConfiguration2Impl: Sized {
    fn AbortPrintFlow(&mut self, reason: PrintWorkflowJobAbortReason) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintWorkflowConfiguration2 {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowConfiguration2";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintWorkflowConfiguration2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowConfiguration2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintWorkflowConfiguration2Vtbl {
        unsafe extern "system" fn AbortPrintFlow<Impl: IPrintWorkflowConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reason: PrintWorkflowJobAbortReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AbortPrintFlow(reason).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintWorkflowConfiguration2, BASE_OFFSET>(),
            AbortPrintFlow: AbortPrintFlow::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintWorkflowConfiguration2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPrintWorkflowForegroundSessionImpl: Sized {
    fn SetupRequested(&mut self, setupeventhandler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<PrintWorkflowForegroundSession, PrintWorkflowForegroundSetupRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSetupRequested(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn XpsDataAvailable(&mut self, xpsdataavailableeventhandler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<PrintWorkflowForegroundSession, PrintWorkflowXpsDataAvailableEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveXpsDataAvailable(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Status(&mut self) -> ::windows::core::Result<PrintWorkflowSessionStatus>;
    fn Start(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrintWorkflowForegroundSession {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowForegroundSession";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPrintWorkflowForegroundSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowForegroundSessionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintWorkflowForegroundSessionVtbl {
        unsafe extern "system" fn SetupRequested<Impl: IPrintWorkflowForegroundSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, setupeventhandler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetupRequested(&*(&setupeventhandler as *const <super::super::super::Foundation::TypedEventHandler<PrintWorkflowForegroundSession, PrintWorkflowForegroundSetupRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<PrintWorkflowForegroundSession, PrintWorkflowForegroundSetupRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSetupRequested<Impl: IPrintWorkflowForegroundSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSetupRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn XpsDataAvailable<Impl: IPrintWorkflowForegroundSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xpsdataavailableeventhandler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XpsDataAvailable(&*(&xpsdataavailableeventhandler as *const <super::super::super::Foundation::TypedEventHandler<PrintWorkflowForegroundSession, PrintWorkflowXpsDataAvailableEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<PrintWorkflowForegroundSession, PrintWorkflowXpsDataAvailableEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveXpsDataAvailable<Impl: IPrintWorkflowForegroundSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveXpsDataAvailable(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Status<Impl: IPrintWorkflowForegroundSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintWorkflowSessionStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Start<Impl: IPrintWorkflowForegroundSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintWorkflowForegroundSession, BASE_OFFSET>(),
            SetupRequested: SetupRequested::<Impl, IMPL_OFFSET>,
            RemoveSetupRequested: RemoveSetupRequested::<Impl, IMPL_OFFSET>,
            XpsDataAvailable: XpsDataAvailable::<Impl, IMPL_OFFSET>,
            RemoveXpsDataAvailable: RemoveXpsDataAvailable::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
            Start: Start::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintWorkflowForegroundSession as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Printing_PrintTicket", feature = "implement_exclusive"))]
pub trait IPrintWorkflowForegroundSetupRequestedEventArgsImpl: Sized {
    fn GetUserPrintTicketAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::PrintTicket::WorkflowPrintTicket>>;
    fn Configuration(&mut self) -> ::windows::core::Result<PrintWorkflowConfiguration>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Printing_PrintTicket", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrintWorkflowForegroundSetupRequestedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowForegroundSetupRequestedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Printing_PrintTicket", feature = "implement_exclusive"))]
impl IPrintWorkflowForegroundSetupRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowForegroundSetupRequestedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintWorkflowForegroundSetupRequestedEventArgsVtbl {
        unsafe extern "system" fn GetUserPrintTicketAsync<Impl: IPrintWorkflowForegroundSetupRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUserPrintTicketAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Configuration<Impl: IPrintWorkflowForegroundSetupRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Configuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IPrintWorkflowForegroundSetupRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintWorkflowForegroundSetupRequestedEventArgs, BASE_OFFSET>(),
            GetUserPrintTicketAsync: GetUserPrintTicketAsync::<Impl, IMPL_OFFSET>,
            Configuration: Configuration::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintWorkflowForegroundSetupRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowJobActivatedEventArgsImpl: Sized {
    fn Session(&mut self) -> ::windows::core::Result<PrintWorkflowJobUISession>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintWorkflowJobActivatedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowJobActivatedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintWorkflowJobActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowJobActivatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintWorkflowJobActivatedEventArgsVtbl {
        unsafe extern "system" fn Session<Impl: IPrintWorkflowJobActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Session() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintWorkflowJobActivatedEventArgs, BASE_OFFSET>(),
            Session: Session::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintWorkflowJobActivatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPrintWorkflowJobBackgroundSessionImpl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<PrintWorkflowSessionStatus>;
    fn JobStarting(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<PrintWorkflowJobBackgroundSession, PrintWorkflowJobStartingEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveJobStarting(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PdlModificationRequested(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<PrintWorkflowJobBackgroundSession, PrintWorkflowPdlModificationRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePdlModificationRequested(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Start(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrintWorkflowJobBackgroundSession {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowJobBackgroundSession";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPrintWorkflowJobBackgroundSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowJobBackgroundSessionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintWorkflowJobBackgroundSessionVtbl {
        unsafe extern "system" fn Status<Impl: IPrintWorkflowJobBackgroundSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintWorkflowSessionStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JobStarting<Impl: IPrintWorkflowJobBackgroundSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).JobStarting(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<PrintWorkflowJobBackgroundSession, PrintWorkflowJobStartingEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<PrintWorkflowJobBackgroundSession, PrintWorkflowJobStartingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveJobStarting<Impl: IPrintWorkflowJobBackgroundSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveJobStarting(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PdlModificationRequested<Impl: IPrintWorkflowJobBackgroundSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PdlModificationRequested(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<PrintWorkflowJobBackgroundSession, PrintWorkflowPdlModificationRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<PrintWorkflowJobBackgroundSession, PrintWorkflowPdlModificationRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePdlModificationRequested<Impl: IPrintWorkflowJobBackgroundSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePdlModificationRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Start<Impl: IPrintWorkflowJobBackgroundSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintWorkflowJobBackgroundSession, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            JobStarting: JobStarting::<Impl, IMPL_OFFSET>,
            RemoveJobStarting: RemoveJobStarting::<Impl, IMPL_OFFSET>,
            PdlModificationRequested: PdlModificationRequested::<Impl, IMPL_OFFSET>,
            RemovePdlModificationRequested: RemovePdlModificationRequested::<Impl, IMPL_OFFSET>,
            Start: Start::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintWorkflowJobBackgroundSession as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPrintWorkflowJobNotificationEventArgsImpl: Sized {
    fn Configuration(&mut self) -> ::windows::core::Result<PrintWorkflowConfiguration>;
    fn PrinterJob(&mut self) -> ::windows::core::Result<PrintWorkflowPrinterJob>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrintWorkflowJobNotificationEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowJobNotificationEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPrintWorkflowJobNotificationEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowJobNotificationEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintWorkflowJobNotificationEventArgsVtbl {
        unsafe extern "system" fn Configuration<Impl: IPrintWorkflowJobNotificationEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Configuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrinterJob<Impl: IPrintWorkflowJobNotificationEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrinterJob() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IPrintWorkflowJobNotificationEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintWorkflowJobNotificationEventArgs, BASE_OFFSET>(),
            Configuration: Configuration::<Impl, IMPL_OFFSET>,
            PrinterJob: PrinterJob::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintWorkflowJobNotificationEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Printers", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPrintWorkflowJobStartingEventArgsImpl: Sized {
    fn Configuration(&mut self) -> ::windows::core::Result<PrintWorkflowConfiguration>;
    fn Printer(&mut self) -> ::windows::core::Result<super::super::super::Devices::Printers::IppPrintDevice>;
    fn SetSkipSystemRendering(&mut self) -> ::windows::core::Result<()>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Devices_Printers", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrintWorkflowJobStartingEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowJobStartingEventArgs";
}
#[cfg(all(feature = "Devices_Printers", feature = "Foundation", feature = "implement_exclusive"))]
impl IPrintWorkflowJobStartingEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowJobStartingEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintWorkflowJobStartingEventArgsVtbl {
        unsafe extern "system" fn Configuration<Impl: IPrintWorkflowJobStartingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Configuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Printer<Impl: IPrintWorkflowJobStartingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Printer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSkipSystemRendering<Impl: IPrintWorkflowJobStartingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSkipSystemRendering().into()
        }
        unsafe extern "system" fn GetDeferral<Impl: IPrintWorkflowJobStartingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintWorkflowJobStartingEventArgs, BASE_OFFSET>(),
            Configuration: Configuration::<Impl, IMPL_OFFSET>,
            Printer: Printer::<Impl, IMPL_OFFSET>,
            SetSkipSystemRendering: SetSkipSystemRendering::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintWorkflowJobStartingEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowJobTriggerDetailsImpl: Sized {
    fn PrintWorkflowJobSession(&mut self) -> ::windows::core::Result<PrintWorkflowJobBackgroundSession>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintWorkflowJobTriggerDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowJobTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintWorkflowJobTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowJobTriggerDetailsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintWorkflowJobTriggerDetailsVtbl {
        unsafe extern "system" fn PrintWorkflowJobSession<Impl: IPrintWorkflowJobTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrintWorkflowJobSession() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintWorkflowJobTriggerDetails, BASE_OFFSET>(),
            PrintWorkflowJobSession: PrintWorkflowJobSession::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintWorkflowJobTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPrintWorkflowJobUISessionImpl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<PrintWorkflowSessionStatus>;
    fn PdlDataAvailable(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<PrintWorkflowJobUISession, PrintWorkflowPdlDataAvailableEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePdlDataAvailable(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn JobNotification(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<PrintWorkflowJobUISession, PrintWorkflowJobNotificationEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveJobNotification(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Start(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrintWorkflowJobUISession {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowJobUISession";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPrintWorkflowJobUISessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowJobUISessionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintWorkflowJobUISessionVtbl {
        unsafe extern "system" fn Status<Impl: IPrintWorkflowJobUISessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintWorkflowSessionStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PdlDataAvailable<Impl: IPrintWorkflowJobUISessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PdlDataAvailable(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<PrintWorkflowJobUISession, PrintWorkflowPdlDataAvailableEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<PrintWorkflowJobUISession, PrintWorkflowPdlDataAvailableEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePdlDataAvailable<Impl: IPrintWorkflowJobUISessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePdlDataAvailable(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn JobNotification<Impl: IPrintWorkflowJobUISessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).JobNotification(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<PrintWorkflowJobUISession, PrintWorkflowJobNotificationEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<PrintWorkflowJobUISession, PrintWorkflowJobNotificationEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveJobNotification<Impl: IPrintWorkflowJobUISessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveJobNotification(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Start<Impl: IPrintWorkflowJobUISessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintWorkflowJobUISession, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            PdlDataAvailable: PdlDataAvailable::<Impl, IMPL_OFFSET>,
            RemovePdlDataAvailable: RemovePdlDataAvailable::<Impl, IMPL_OFFSET>,
            JobNotification: JobNotification::<Impl, IMPL_OFFSET>,
            RemoveJobNotification: RemoveJobNotification::<Impl, IMPL_OFFSET>,
            Start: Start::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintWorkflowJobUISession as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowObjectModelSourceFileContentImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintWorkflowObjectModelSourceFileContent {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowObjectModelSourceFileContent";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintWorkflowObjectModelSourceFileContentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowObjectModelSourceFileContentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintWorkflowObjectModelSourceFileContentVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintWorkflowObjectModelSourceFileContent, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintWorkflowObjectModelSourceFileContent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IPrintWorkflowObjectModelSourceFileContentFactoryImpl: Sized {
    fn CreateInstance(&mut self, xpsstream: &::core::option::Option<super::super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<PrintWorkflowObjectModelSourceFileContent>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrintWorkflowObjectModelSourceFileContentFactory {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowObjectModelSourceFileContentFactory";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IPrintWorkflowObjectModelSourceFileContentFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowObjectModelSourceFileContentFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintWorkflowObjectModelSourceFileContentFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IPrintWorkflowObjectModelSourceFileContentFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xpsstream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&xpsstream as *const <super::super::super::Storage::Streams::IInputStream as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IInputStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintWorkflowObjectModelSourceFileContentFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintWorkflowObjectModelSourceFileContentFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowObjectModelTargetPackageImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintWorkflowObjectModelTargetPackage {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowObjectModelTargetPackage";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintWorkflowObjectModelTargetPackageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowObjectModelTargetPackageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintWorkflowObjectModelTargetPackageVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintWorkflowObjectModelTargetPackage, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintWorkflowObjectModelTargetPackage as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Printing_PrintTicket", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IPrintWorkflowPdlConverterImpl: Sized {
    fn ConvertPdlAsync(&mut self, printticket: &::core::option::Option<super::PrintTicket::WorkflowPrintTicket>, inputstream: &::core::option::Option<super::super::super::Storage::Streams::IInputStream>, outputstream: &::core::option::Option<super::super::super::Storage::Streams::IOutputStream>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Printing_PrintTicket", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrintWorkflowPdlConverter {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowPdlConverter";
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Printing_PrintTicket", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IPrintWorkflowPdlConverterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowPdlConverterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintWorkflowPdlConverterVtbl {
        unsafe extern "system" fn ConvertPdlAsync<Impl: IPrintWorkflowPdlConverterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, printticket: ::windows::core::RawPtr, inputstream: ::windows::core::RawPtr, outputstream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConvertPdlAsync(
                &*(&printticket as *const <super::PrintTicket::WorkflowPrintTicket as ::windows::core::Abi>::Abi as *const <super::PrintTicket::WorkflowPrintTicket as ::windows::core::DefaultType>::DefaultType),
                &*(&inputstream as *const <super::super::super::Storage::Streams::IInputStream as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IInputStream as ::windows::core::DefaultType>::DefaultType),
                &*(&outputstream as *const <super::super::super::Storage::Streams::IOutputStream as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IOutputStream as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintWorkflowPdlConverter, BASE_OFFSET>(),
            ConvertPdlAsync: ConvertPdlAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintWorkflowPdlConverter as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPrintWorkflowPdlDataAvailableEventArgsImpl: Sized {
    fn Configuration(&mut self) -> ::windows::core::Result<PrintWorkflowConfiguration>;
    fn PrinterJob(&mut self) -> ::windows::core::Result<PrintWorkflowPrinterJob>;
    fn SourceContent(&mut self) -> ::windows::core::Result<PrintWorkflowPdlSourceContent>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrintWorkflowPdlDataAvailableEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowPdlDataAvailableEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPrintWorkflowPdlDataAvailableEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowPdlDataAvailableEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintWorkflowPdlDataAvailableEventArgsVtbl {
        unsafe extern "system" fn Configuration<Impl: IPrintWorkflowPdlDataAvailableEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Configuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrinterJob<Impl: IPrintWorkflowPdlDataAvailableEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrinterJob() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SourceContent<Impl: IPrintWorkflowPdlDataAvailableEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceContent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IPrintWorkflowPdlDataAvailableEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintWorkflowPdlDataAvailableEventArgs, BASE_OFFSET>(),
            Configuration: Configuration::<Impl, IMPL_OFFSET>,
            PrinterJob: PrinterJob::<Impl, IMPL_OFFSET>,
            SourceContent: SourceContent::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintWorkflowPdlDataAvailableEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Printers", feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IPrintWorkflowPdlModificationRequestedEventArgsImpl: Sized {
    fn Configuration(&mut self) -> ::windows::core::Result<PrintWorkflowConfiguration>;
    fn PrinterJob(&mut self) -> ::windows::core::Result<PrintWorkflowPrinterJob>;
    fn SourceContent(&mut self) -> ::windows::core::Result<PrintWorkflowPdlSourceContent>;
    fn UILauncher(&mut self) -> ::windows::core::Result<PrintWorkflowUILauncher>;
    fn CreateJobOnPrinter(&mut self, targetcontenttype: &::windows::core::HSTRING) -> ::windows::core::Result<PrintWorkflowPdlTargetStream>;
    fn CreateJobOnPrinterWithAttributes(&mut self, jobattributes: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, super::super::super::Devices::Printers::IppAttributeValue>>>, targetcontenttype: &::windows::core::HSTRING) -> ::windows::core::Result<PrintWorkflowPdlTargetStream>;
    fn CreateJobOnPrinterWithAttributesBuffer(&mut self, jobattributesbuffer: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>, targetcontenttype: &::windows::core::HSTRING) -> ::windows::core::Result<PrintWorkflowPdlTargetStream>;
    fn GetPdlConverter(&mut self, conversiontype: PrintWorkflowPdlConversionType) -> ::windows::core::Result<PrintWorkflowPdlConverter>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Devices_Printers", feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrintWorkflowPdlModificationRequestedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowPdlModificationRequestedEventArgs";
}
#[cfg(all(feature = "Devices_Printers", feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IPrintWorkflowPdlModificationRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowPdlModificationRequestedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintWorkflowPdlModificationRequestedEventArgsVtbl {
        unsafe extern "system" fn Configuration<Impl: IPrintWorkflowPdlModificationRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Configuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrinterJob<Impl: IPrintWorkflowPdlModificationRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrinterJob() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SourceContent<Impl: IPrintWorkflowPdlModificationRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceContent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UILauncher<Impl: IPrintWorkflowPdlModificationRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UILauncher() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateJobOnPrinter<Impl: IPrintWorkflowPdlModificationRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetcontenttype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateJobOnPrinter(&*(&targetcontenttype as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateJobOnPrinterWithAttributes<Impl: IPrintWorkflowPdlModificationRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, jobattributes: ::windows::core::RawPtr, targetcontenttype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateJobOnPrinterWithAttributes(
                &*(&jobattributes as *const <super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, super::super::super::Devices::Printers::IppAttributeValue>> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, super::super::super::Devices::Printers::IppAttributeValue>> as ::windows::core::DefaultType>::DefaultType),
                &*(&targetcontenttype as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateJobOnPrinterWithAttributesBuffer<Impl: IPrintWorkflowPdlModificationRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, jobattributesbuffer: ::windows::core::RawPtr, targetcontenttype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateJobOnPrinterWithAttributesBuffer(&*(&jobattributesbuffer as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType), &*(&targetcontenttype as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPdlConverter<Impl: IPrintWorkflowPdlModificationRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, conversiontype: PrintWorkflowPdlConversionType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPdlConverter(conversiontype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IPrintWorkflowPdlModificationRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintWorkflowPdlModificationRequestedEventArgs, BASE_OFFSET>(),
            Configuration: Configuration::<Impl, IMPL_OFFSET>,
            PrinterJob: PrinterJob::<Impl, IMPL_OFFSET>,
            SourceContent: SourceContent::<Impl, IMPL_OFFSET>,
            UILauncher: UILauncher::<Impl, IMPL_OFFSET>,
            CreateJobOnPrinter: CreateJobOnPrinter::<Impl, IMPL_OFFSET>,
            CreateJobOnPrinterWithAttributes: CreateJobOnPrinterWithAttributes::<Impl, IMPL_OFFSET>,
            CreateJobOnPrinterWithAttributesBuffer: CreateJobOnPrinterWithAttributesBuffer::<Impl, IMPL_OFFSET>,
            GetPdlConverter: GetPdlConverter::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintWorkflowPdlModificationRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IPrintWorkflowPdlSourceContentImpl: Sized {
    fn ContentType(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetInputStream(&mut self) -> ::windows::core::Result<super::super::super::Storage::Streams::IInputStream>;
    fn GetContentFileAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::StorageFile>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrintWorkflowPdlSourceContent {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowPdlSourceContent";
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IPrintWorkflowPdlSourceContentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowPdlSourceContentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintWorkflowPdlSourceContentVtbl {
        unsafe extern "system" fn ContentType<Impl: IPrintWorkflowPdlSourceContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputStream<Impl: IPrintWorkflowPdlSourceContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputStream() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContentFileAsync<Impl: IPrintWorkflowPdlSourceContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContentFileAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintWorkflowPdlSourceContent, BASE_OFFSET>(),
            ContentType: ContentType::<Impl, IMPL_OFFSET>,
            GetInputStream: GetInputStream::<Impl, IMPL_OFFSET>,
            GetContentFileAsync: GetContentFileAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintWorkflowPdlSourceContent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IPrintWorkflowPdlTargetStreamImpl: Sized {
    fn GetOutputStream(&mut self) -> ::windows::core::Result<super::super::super::Storage::Streams::IOutputStream>;
    fn CompleteStreamSubmission(&mut self, status: PrintWorkflowSubmittedStatus) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrintWorkflowPdlTargetStream {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowPdlTargetStream";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IPrintWorkflowPdlTargetStreamVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowPdlTargetStreamImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintWorkflowPdlTargetStreamVtbl {
        unsafe extern "system" fn GetOutputStream<Impl: IPrintWorkflowPdlTargetStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutputStream() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompleteStreamSubmission<Impl: IPrintWorkflowPdlTargetStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: PrintWorkflowSubmittedStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CompleteStreamSubmission(status).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintWorkflowPdlTargetStream, BASE_OFFSET>(),
            GetOutputStream: GetOutputStream::<Impl, IMPL_OFFSET>,
            CompleteStreamSubmission: CompleteStreamSubmission::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintWorkflowPdlTargetStream as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Printers", feature = "Foundation_Collections", feature = "Graphics_Printing_PrintTicket", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IPrintWorkflowPrinterJobImpl: Sized {
    fn JobId(&mut self) -> ::windows::core::Result<i32>;
    fn Printer(&mut self) -> ::windows::core::Result<super::super::super::Devices::Printers::IppPrintDevice>;
    fn GetJobStatus(&mut self) -> ::windows::core::Result<PrintWorkflowPrinterJobStatus>;
    fn GetJobPrintTicket(&mut self) -> ::windows::core::Result<super::PrintTicket::WorkflowPrintTicket>;
    fn GetJobAttributesAsBuffer(&mut self, attributenames: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn GetJobAttributes(&mut self, attributenames: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, super::super::super::Devices::Printers::IppAttributeValue>>;
    fn SetJobAttributesFromBuffer(&mut self, jobattributesbuffer: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::Devices::Printers::IppSetAttributesResult>;
    fn SetJobAttributes(&mut self, jobattributes: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, super::super::super::Devices::Printers::IppAttributeValue>>>) -> ::windows::core::Result<super::super::super::Devices::Printers::IppSetAttributesResult>;
}
#[cfg(all(feature = "Devices_Printers", feature = "Foundation_Collections", feature = "Graphics_Printing_PrintTicket", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrintWorkflowPrinterJob {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowPrinterJob";
}
#[cfg(all(feature = "Devices_Printers", feature = "Foundation_Collections", feature = "Graphics_Printing_PrintTicket", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IPrintWorkflowPrinterJobVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowPrinterJobImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintWorkflowPrinterJobVtbl {
        unsafe extern "system" fn JobId<Impl: IPrintWorkflowPrinterJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).JobId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Printer<Impl: IPrintWorkflowPrinterJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Printer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetJobStatus<Impl: IPrintWorkflowPrinterJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PrintWorkflowPrinterJobStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetJobStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetJobPrintTicket<Impl: IPrintWorkflowPrinterJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetJobPrintTicket() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetJobAttributesAsBuffer<Impl: IPrintWorkflowPrinterJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributenames: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetJobAttributesAsBuffer(&*(&attributenames as *const <super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetJobAttributes<Impl: IPrintWorkflowPrinterJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributenames: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetJobAttributes(&*(&attributenames as *const <super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJobAttributesFromBuffer<Impl: IPrintWorkflowPrinterJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, jobattributesbuffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetJobAttributesFromBuffer(&*(&jobattributesbuffer as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJobAttributes<Impl: IPrintWorkflowPrinterJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, jobattributes: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetJobAttributes(&*(&jobattributes as *const <super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, super::super::super::Devices::Printers::IppAttributeValue>> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, super::super::super::Devices::Printers::IppAttributeValue>> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintWorkflowPrinterJob, BASE_OFFSET>(),
            JobId: JobId::<Impl, IMPL_OFFSET>,
            Printer: Printer::<Impl, IMPL_OFFSET>,
            GetJobStatus: GetJobStatus::<Impl, IMPL_OFFSET>,
            GetJobPrintTicket: GetJobPrintTicket::<Impl, IMPL_OFFSET>,
            GetJobAttributesAsBuffer: GetJobAttributesAsBuffer::<Impl, IMPL_OFFSET>,
            GetJobAttributes: GetJobAttributes::<Impl, IMPL_OFFSET>,
            SetJobAttributesFromBuffer: SetJobAttributesFromBuffer::<Impl, IMPL_OFFSET>,
            SetJobAttributes: SetJobAttributes::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintWorkflowPrinterJob as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Printing_PrintTicket", feature = "implement_exclusive"))]
pub trait IPrintWorkflowSourceContentImpl: Sized {
    fn GetJobPrintTicketAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::PrintTicket::WorkflowPrintTicket>>;
    fn GetSourceSpoolDataAsStreamContent(&mut self) -> ::windows::core::Result<PrintWorkflowSpoolStreamContent>;
    fn GetSourceSpoolDataAsXpsObjectModel(&mut self) -> ::windows::core::Result<PrintWorkflowObjectModelSourceFileContent>;
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Printing_PrintTicket", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrintWorkflowSourceContent {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowSourceContent";
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Printing_PrintTicket", feature = "implement_exclusive"))]
impl IPrintWorkflowSourceContentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowSourceContentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintWorkflowSourceContentVtbl {
        unsafe extern "system" fn GetJobPrintTicketAsync<Impl: IPrintWorkflowSourceContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetJobPrintTicketAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceSpoolDataAsStreamContent<Impl: IPrintWorkflowSourceContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSourceSpoolDataAsStreamContent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceSpoolDataAsXpsObjectModel<Impl: IPrintWorkflowSourceContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSourceSpoolDataAsXpsObjectModel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintWorkflowSourceContent, BASE_OFFSET>(),
            GetJobPrintTicketAsync: GetJobPrintTicketAsync::<Impl, IMPL_OFFSET>,
            GetSourceSpoolDataAsStreamContent: GetSourceSpoolDataAsStreamContent::<Impl, IMPL_OFFSET>,
            GetSourceSpoolDataAsXpsObjectModel: GetSourceSpoolDataAsXpsObjectModel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintWorkflowSourceContent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IPrintWorkflowSpoolStreamContentImpl: Sized {
    fn GetInputStream(&mut self) -> ::windows::core::Result<super::super::super::Storage::Streams::IInputStream>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrintWorkflowSpoolStreamContent {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowSpoolStreamContent";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IPrintWorkflowSpoolStreamContentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowSpoolStreamContentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintWorkflowSpoolStreamContentVtbl {
        unsafe extern "system" fn GetInputStream<Impl: IPrintWorkflowSpoolStreamContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputStream() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintWorkflowSpoolStreamContent, BASE_OFFSET>(),
            GetInputStream: GetInputStream::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintWorkflowSpoolStreamContent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IPrintWorkflowStreamTargetImpl: Sized {
    fn GetOutputStream(&mut self) -> ::windows::core::Result<super::super::super::Storage::Streams::IOutputStream>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrintWorkflowStreamTarget {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowStreamTarget";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IPrintWorkflowStreamTargetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowStreamTargetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintWorkflowStreamTargetVtbl {
        unsafe extern "system" fn GetOutputStream<Impl: IPrintWorkflowStreamTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutputStream() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintWorkflowStreamTarget, BASE_OFFSET>(),
            GetOutputStream: GetOutputStream::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintWorkflowStreamTarget as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Printing_PrintTicket", feature = "implement_exclusive"))]
pub trait IPrintWorkflowSubmittedEventArgsImpl: Sized {
    fn Operation(&mut self) -> ::windows::core::Result<PrintWorkflowSubmittedOperation>;
    fn GetTarget(&mut self, jobprintticket: &::core::option::Option<super::PrintTicket::WorkflowPrintTicket>) -> ::windows::core::Result<PrintWorkflowTarget>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Printing_PrintTicket", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrintWorkflowSubmittedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowSubmittedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Printing_PrintTicket", feature = "implement_exclusive"))]
impl IPrintWorkflowSubmittedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowSubmittedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintWorkflowSubmittedEventArgsVtbl {
        unsafe extern "system" fn Operation<Impl: IPrintWorkflowSubmittedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Operation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTarget<Impl: IPrintWorkflowSubmittedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, jobprintticket: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTarget(&*(&jobprintticket as *const <super::PrintTicket::WorkflowPrintTicket as ::windows::core::Abi>::Abi as *const <super::PrintTicket::WorkflowPrintTicket as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IPrintWorkflowSubmittedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintWorkflowSubmittedEventArgs, BASE_OFFSET>(),
            Operation: Operation::<Impl, IMPL_OFFSET>,
            GetTarget: GetTarget::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintWorkflowSubmittedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowSubmittedOperationImpl: Sized {
    fn Complete(&mut self, status: PrintWorkflowSubmittedStatus) -> ::windows::core::Result<()>;
    fn Configuration(&mut self) -> ::windows::core::Result<PrintWorkflowConfiguration>;
    fn XpsContent(&mut self) -> ::windows::core::Result<PrintWorkflowSourceContent>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintWorkflowSubmittedOperation {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowSubmittedOperation";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintWorkflowSubmittedOperationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowSubmittedOperationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintWorkflowSubmittedOperationVtbl {
        unsafe extern "system" fn Complete<Impl: IPrintWorkflowSubmittedOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: PrintWorkflowSubmittedStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Complete(status).into()
        }
        unsafe extern "system" fn Configuration<Impl: IPrintWorkflowSubmittedOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Configuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn XpsContent<Impl: IPrintWorkflowSubmittedOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XpsContent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintWorkflowSubmittedOperation, BASE_OFFSET>(),
            Complete: Complete::<Impl, IMPL_OFFSET>,
            Configuration: Configuration::<Impl, IMPL_OFFSET>,
            XpsContent: XpsContent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintWorkflowSubmittedOperation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowTargetImpl: Sized {
    fn TargetAsStream(&mut self) -> ::windows::core::Result<PrintWorkflowStreamTarget>;
    fn TargetAsXpsObjectModelPackage(&mut self) -> ::windows::core::Result<PrintWorkflowObjectModelTargetPackage>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintWorkflowTarget {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowTarget";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintWorkflowTargetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowTargetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintWorkflowTargetVtbl {
        unsafe extern "system" fn TargetAsStream<Impl: IPrintWorkflowTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetAsStream() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TargetAsXpsObjectModelPackage<Impl: IPrintWorkflowTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetAsXpsObjectModelPackage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintWorkflowTarget, BASE_OFFSET>(),
            TargetAsStream: TargetAsStream::<Impl, IMPL_OFFSET>,
            TargetAsXpsObjectModelPackage: TargetAsXpsObjectModelPackage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintWorkflowTarget as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowTriggerDetailsImpl: Sized {
    fn PrintWorkflowSession(&mut self) -> ::windows::core::Result<PrintWorkflowBackgroundSession>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintWorkflowTriggerDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintWorkflowTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowTriggerDetailsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintWorkflowTriggerDetailsVtbl {
        unsafe extern "system" fn PrintWorkflowSession<Impl: IPrintWorkflowTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrintWorkflowSession() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintWorkflowTriggerDetails, BASE_OFFSET>(),
            PrintWorkflowSession: PrintWorkflowSession::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintWorkflowTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "System", feature = "implement_exclusive"))]
pub trait IPrintWorkflowUIActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl + IActivatedEventArgsWithUserImpl {
    fn PrintWorkflowSession(&mut self) -> ::windows::core::Result<PrintWorkflowForegroundSession>;
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrintWorkflowUIActivatedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowUIActivatedEventArgs";
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "System", feature = "implement_exclusive"))]
impl IPrintWorkflowUIActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowUIActivatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintWorkflowUIActivatedEventArgsVtbl {
        unsafe extern "system" fn PrintWorkflowSession<Impl: IPrintWorkflowUIActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrintWorkflowSession() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintWorkflowUIActivatedEventArgs, BASE_OFFSET>(),
            PrintWorkflowSession: PrintWorkflowSession::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintWorkflowUIActivatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPrintWorkflowUILauncherImpl: Sized {
    fn IsUILaunchEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn LaunchAndCompleteUIAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<PrintWorkflowUICompletionStatus>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrintWorkflowUILauncher {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowUILauncher";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPrintWorkflowUILauncherVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowUILauncherImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintWorkflowUILauncherVtbl {
        unsafe extern "system" fn IsUILaunchEnabled<Impl: IPrintWorkflowUILauncherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsUILaunchEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LaunchAndCompleteUIAsync<Impl: IPrintWorkflowUILauncherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LaunchAndCompleteUIAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintWorkflowUILauncher, BASE_OFFSET>(),
            IsUILaunchEnabled: IsUILaunchEnabled::<Impl, IMPL_OFFSET>,
            LaunchAndCompleteUIAsync: LaunchAndCompleteUIAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintWorkflowUILauncher as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPrintWorkflowXpsDataAvailableEventArgsImpl: Sized {
    fn Operation(&mut self) -> ::windows::core::Result<PrintWorkflowSubmittedOperation>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrintWorkflowXpsDataAvailableEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowXpsDataAvailableEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPrintWorkflowXpsDataAvailableEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowXpsDataAvailableEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintWorkflowXpsDataAvailableEventArgsVtbl {
        unsafe extern "system" fn Operation<Impl: IPrintWorkflowXpsDataAvailableEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Operation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IPrintWorkflowXpsDataAvailableEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintWorkflowXpsDataAvailableEventArgs, BASE_OFFSET>(),
            Operation: Operation::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintWorkflowXpsDataAvailableEventArgs as ::windows::core::Interface>::IID
    }
}
