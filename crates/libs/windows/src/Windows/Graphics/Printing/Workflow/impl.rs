#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowBackgroundSessionImpl: Sized {
    fn SetupRequested(&self, setupeventhandler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<PrintWorkflowBackgroundSession, PrintWorkflowBackgroundSetupRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSetupRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Submitted(&self, submittedeventhandler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<PrintWorkflowBackgroundSession, PrintWorkflowSubmittedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSubmitted(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Status(&self) -> ::windows::core::Result<PrintWorkflowSessionStatus>;
    fn Start(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintWorkflowBackgroundSession {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowBackgroundSession";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintWorkflowBackgroundSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowBackgroundSessionImpl, const OFFSET: isize>() -> IPrintWorkflowBackgroundSessionVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintWorkflowBackgroundSession>, ::windows::core::GetTrustLevel, SetupRequested::<Impl, OFFSET>, RemoveSetupRequested::<Impl, OFFSET>, Submitted::<Impl, OFFSET>, RemoveSubmitted::<Impl, OFFSET>, Status::<Impl, OFFSET>, Start::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowBackgroundSetupRequestedEventArgsImpl: Sized {
    fn GetUserPrintTicketAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::PrintTicket::WorkflowPrintTicket>>;
    fn Configuration(&self) -> ::windows::core::Result<PrintWorkflowConfiguration>;
    fn SetRequiresUI(&self) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintWorkflowBackgroundSetupRequestedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowBackgroundSetupRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintWorkflowBackgroundSetupRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowBackgroundSetupRequestedEventArgsImpl, const OFFSET: isize>() -> IPrintWorkflowBackgroundSetupRequestedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintWorkflowBackgroundSetupRequestedEventArgs>, ::windows::core::GetTrustLevel, GetUserPrintTicketAsync::<Impl, OFFSET>, Configuration::<Impl, OFFSET>, SetRequiresUI::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowConfigurationImpl: Sized {
    fn SourceAppDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn JobTitle(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SessionId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintWorkflowConfiguration {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowConfiguration";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintWorkflowConfigurationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowConfigurationImpl, const OFFSET: isize>() -> IPrintWorkflowConfigurationVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintWorkflowConfiguration>, ::windows::core::GetTrustLevel, SourceAppDisplayName::<Impl, OFFSET>, JobTitle::<Impl, OFFSET>, SessionId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowConfiguration2Impl: Sized {
    fn AbortPrintFlow(&self, reason: PrintWorkflowJobAbortReason) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintWorkflowConfiguration2 {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowConfiguration2";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintWorkflowConfiguration2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowConfiguration2Impl, const OFFSET: isize>() -> IPrintWorkflowConfiguration2Vtbl {
        unsafe extern "system" fn AbortPrintFlow<Impl: IPrintWorkflowConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reason: PrintWorkflowJobAbortReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AbortPrintFlow(reason).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintWorkflowConfiguration2>, ::windows::core::GetTrustLevel, AbortPrintFlow::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowForegroundSessionImpl: Sized {
    fn SetupRequested(&self, setupeventhandler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<PrintWorkflowForegroundSession, PrintWorkflowForegroundSetupRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSetupRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn XpsDataAvailable(&self, xpsdataavailableeventhandler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<PrintWorkflowForegroundSession, PrintWorkflowXpsDataAvailableEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveXpsDataAvailable(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Status(&self) -> ::windows::core::Result<PrintWorkflowSessionStatus>;
    fn Start(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintWorkflowForegroundSession {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowForegroundSession";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintWorkflowForegroundSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowForegroundSessionImpl, const OFFSET: isize>() -> IPrintWorkflowForegroundSessionVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintWorkflowForegroundSession>, ::windows::core::GetTrustLevel, SetupRequested::<Impl, OFFSET>, RemoveSetupRequested::<Impl, OFFSET>, XpsDataAvailable::<Impl, OFFSET>, RemoveXpsDataAvailable::<Impl, OFFSET>, Status::<Impl, OFFSET>, Start::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowForegroundSetupRequestedEventArgsImpl: Sized {
    fn GetUserPrintTicketAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::PrintTicket::WorkflowPrintTicket>>;
    fn Configuration(&self) -> ::windows::core::Result<PrintWorkflowConfiguration>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintWorkflowForegroundSetupRequestedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowForegroundSetupRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintWorkflowForegroundSetupRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowForegroundSetupRequestedEventArgsImpl, const OFFSET: isize>() -> IPrintWorkflowForegroundSetupRequestedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintWorkflowForegroundSetupRequestedEventArgs>, ::windows::core::GetTrustLevel, GetUserPrintTicketAsync::<Impl, OFFSET>, Configuration::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowJobActivatedEventArgsImpl: Sized {
    fn Session(&self) -> ::windows::core::Result<PrintWorkflowJobUISession>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintWorkflowJobActivatedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowJobActivatedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintWorkflowJobActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowJobActivatedEventArgsImpl, const OFFSET: isize>() -> IPrintWorkflowJobActivatedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintWorkflowJobActivatedEventArgs>, ::windows::core::GetTrustLevel, Session::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowJobBackgroundSessionImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<PrintWorkflowSessionStatus>;
    fn JobStarting(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<PrintWorkflowJobBackgroundSession, PrintWorkflowJobStartingEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveJobStarting(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PdlModificationRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<PrintWorkflowJobBackgroundSession, PrintWorkflowPdlModificationRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePdlModificationRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Start(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintWorkflowJobBackgroundSession {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowJobBackgroundSession";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintWorkflowJobBackgroundSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowJobBackgroundSessionImpl, const OFFSET: isize>() -> IPrintWorkflowJobBackgroundSessionVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintWorkflowJobBackgroundSession>, ::windows::core::GetTrustLevel, Status::<Impl, OFFSET>, JobStarting::<Impl, OFFSET>, RemoveJobStarting::<Impl, OFFSET>, PdlModificationRequested::<Impl, OFFSET>, RemovePdlModificationRequested::<Impl, OFFSET>, Start::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowJobNotificationEventArgsImpl: Sized {
    fn Configuration(&self) -> ::windows::core::Result<PrintWorkflowConfiguration>;
    fn PrinterJob(&self) -> ::windows::core::Result<PrintWorkflowPrinterJob>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintWorkflowJobNotificationEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowJobNotificationEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintWorkflowJobNotificationEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowJobNotificationEventArgsImpl, const OFFSET: isize>() -> IPrintWorkflowJobNotificationEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintWorkflowJobNotificationEventArgs>, ::windows::core::GetTrustLevel, Configuration::<Impl, OFFSET>, PrinterJob::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowJobStartingEventArgsImpl: Sized {
    fn Configuration(&self) -> ::windows::core::Result<PrintWorkflowConfiguration>;
    fn Printer(&self) -> ::windows::core::Result<super::super::super::Devices::Printers::IppPrintDevice>;
    fn SetSkipSystemRendering(&self) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintWorkflowJobStartingEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowJobStartingEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintWorkflowJobStartingEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowJobStartingEventArgsImpl, const OFFSET: isize>() -> IPrintWorkflowJobStartingEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintWorkflowJobStartingEventArgs>, ::windows::core::GetTrustLevel, Configuration::<Impl, OFFSET>, Printer::<Impl, OFFSET>, SetSkipSystemRendering::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowJobTriggerDetailsImpl: Sized {
    fn PrintWorkflowJobSession(&self) -> ::windows::core::Result<PrintWorkflowJobBackgroundSession>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintWorkflowJobTriggerDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowJobTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintWorkflowJobTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowJobTriggerDetailsImpl, const OFFSET: isize>() -> IPrintWorkflowJobTriggerDetailsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintWorkflowJobTriggerDetails>, ::windows::core::GetTrustLevel, PrintWorkflowJobSession::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowJobUISessionImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<PrintWorkflowSessionStatus>;
    fn PdlDataAvailable(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<PrintWorkflowJobUISession, PrintWorkflowPdlDataAvailableEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePdlDataAvailable(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn JobNotification(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<PrintWorkflowJobUISession, PrintWorkflowJobNotificationEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveJobNotification(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Start(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintWorkflowJobUISession {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowJobUISession";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintWorkflowJobUISessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowJobUISessionImpl, const OFFSET: isize>() -> IPrintWorkflowJobUISessionVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintWorkflowJobUISession>, ::windows::core::GetTrustLevel, Status::<Impl, OFFSET>, PdlDataAvailable::<Impl, OFFSET>, RemovePdlDataAvailable::<Impl, OFFSET>, JobNotification::<Impl, OFFSET>, RemoveJobNotification::<Impl, OFFSET>, Start::<Impl, OFFSET>)
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowObjectModelSourceFileContentImpl, const OFFSET: isize>() -> IPrintWorkflowObjectModelSourceFileContentVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintWorkflowObjectModelSourceFileContent>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowObjectModelSourceFileContentFactoryImpl: Sized {
    fn CreateInstance(&self, xpsstream: &::core::option::Option<super::super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<PrintWorkflowObjectModelSourceFileContent>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintWorkflowObjectModelSourceFileContentFactory {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowObjectModelSourceFileContentFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintWorkflowObjectModelSourceFileContentFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowObjectModelSourceFileContentFactoryImpl, const OFFSET: isize>() -> IPrintWorkflowObjectModelSourceFileContentFactoryVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintWorkflowObjectModelSourceFileContentFactory>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, OFFSET>)
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowObjectModelTargetPackageImpl, const OFFSET: isize>() -> IPrintWorkflowObjectModelTargetPackageVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintWorkflowObjectModelTargetPackage>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowPdlConverterImpl: Sized {
    fn ConvertPdlAsync(&self, printticket: &::core::option::Option<super::PrintTicket::WorkflowPrintTicket>, inputstream: &::core::option::Option<super::super::super::Storage::Streams::IInputStream>, outputstream: &::core::option::Option<super::super::super::Storage::Streams::IOutputStream>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintWorkflowPdlConverter {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowPdlConverter";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintWorkflowPdlConverterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowPdlConverterImpl, const OFFSET: isize>() -> IPrintWorkflowPdlConverterVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintWorkflowPdlConverter>, ::windows::core::GetTrustLevel, ConvertPdlAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowPdlDataAvailableEventArgsImpl: Sized {
    fn Configuration(&self) -> ::windows::core::Result<PrintWorkflowConfiguration>;
    fn PrinterJob(&self) -> ::windows::core::Result<PrintWorkflowPrinterJob>;
    fn SourceContent(&self) -> ::windows::core::Result<PrintWorkflowPdlSourceContent>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintWorkflowPdlDataAvailableEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowPdlDataAvailableEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintWorkflowPdlDataAvailableEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowPdlDataAvailableEventArgsImpl, const OFFSET: isize>() -> IPrintWorkflowPdlDataAvailableEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintWorkflowPdlDataAvailableEventArgs>, ::windows::core::GetTrustLevel, Configuration::<Impl, OFFSET>, PrinterJob::<Impl, OFFSET>, SourceContent::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowPdlModificationRequestedEventArgsImpl: Sized {
    fn Configuration(&self) -> ::windows::core::Result<PrintWorkflowConfiguration>;
    fn PrinterJob(&self) -> ::windows::core::Result<PrintWorkflowPrinterJob>;
    fn SourceContent(&self) -> ::windows::core::Result<PrintWorkflowPdlSourceContent>;
    fn UILauncher(&self) -> ::windows::core::Result<PrintWorkflowUILauncher>;
    fn CreateJobOnPrinter(&self, targetcontenttype: &::windows::core::HSTRING) -> ::windows::core::Result<PrintWorkflowPdlTargetStream>;
    fn CreateJobOnPrinterWithAttributes(&self, jobattributes: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, super::super::super::Devices::Printers::IppAttributeValue>>>, targetcontenttype: &::windows::core::HSTRING) -> ::windows::core::Result<PrintWorkflowPdlTargetStream>;
    fn CreateJobOnPrinterWithAttributesBuffer(&self, jobattributesbuffer: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>, targetcontenttype: &::windows::core::HSTRING) -> ::windows::core::Result<PrintWorkflowPdlTargetStream>;
    fn GetPdlConverter(&self, conversiontype: PrintWorkflowPdlConversionType) -> ::windows::core::Result<PrintWorkflowPdlConverter>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintWorkflowPdlModificationRequestedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowPdlModificationRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintWorkflowPdlModificationRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowPdlModificationRequestedEventArgsImpl, const OFFSET: isize>() -> IPrintWorkflowPdlModificationRequestedEventArgsVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IPrintWorkflowPdlModificationRequestedEventArgs>,
            ::windows::core::GetTrustLevel,
            Configuration::<Impl, OFFSET>,
            PrinterJob::<Impl, OFFSET>,
            SourceContent::<Impl, OFFSET>,
            UILauncher::<Impl, OFFSET>,
            CreateJobOnPrinter::<Impl, OFFSET>,
            CreateJobOnPrinterWithAttributes::<Impl, OFFSET>,
            CreateJobOnPrinterWithAttributesBuffer::<Impl, OFFSET>,
            GetPdlConverter::<Impl, OFFSET>,
            GetDeferral::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowPdlSourceContentImpl: Sized {
    fn ContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetInputStream(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IInputStream>;
    fn GetContentFileAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::StorageFile>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintWorkflowPdlSourceContent {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowPdlSourceContent";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintWorkflowPdlSourceContentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowPdlSourceContentImpl, const OFFSET: isize>() -> IPrintWorkflowPdlSourceContentVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintWorkflowPdlSourceContent>, ::windows::core::GetTrustLevel, ContentType::<Impl, OFFSET>, GetInputStream::<Impl, OFFSET>, GetContentFileAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowPdlTargetStreamImpl: Sized {
    fn GetOutputStream(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IOutputStream>;
    fn CompleteStreamSubmission(&self, status: PrintWorkflowSubmittedStatus) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintWorkflowPdlTargetStream {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowPdlTargetStream";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintWorkflowPdlTargetStreamVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowPdlTargetStreamImpl, const OFFSET: isize>() -> IPrintWorkflowPdlTargetStreamVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintWorkflowPdlTargetStream>, ::windows::core::GetTrustLevel, GetOutputStream::<Impl, OFFSET>, CompleteStreamSubmission::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowPrinterJobImpl: Sized {
    fn JobId(&self) -> ::windows::core::Result<i32>;
    fn Printer(&self) -> ::windows::core::Result<super::super::super::Devices::Printers::IppPrintDevice>;
    fn GetJobStatus(&self) -> ::windows::core::Result<PrintWorkflowPrinterJobStatus>;
    fn GetJobPrintTicket(&self) -> ::windows::core::Result<super::PrintTicket::WorkflowPrintTicket>;
    fn GetJobAttributesAsBuffer(&self, attributenames: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn GetJobAttributes(&self, attributenames: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, super::super::super::Devices::Printers::IppAttributeValue>>;
    fn SetJobAttributesFromBuffer(&self, jobattributesbuffer: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::Devices::Printers::IppSetAttributesResult>;
    fn SetJobAttributes(&self, jobattributes: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, super::super::super::Devices::Printers::IppAttributeValue>>>) -> ::windows::core::Result<super::super::super::Devices::Printers::IppSetAttributesResult>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintWorkflowPrinterJob {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowPrinterJob";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintWorkflowPrinterJobVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowPrinterJobImpl, const OFFSET: isize>() -> IPrintWorkflowPrinterJobVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IPrintWorkflowPrinterJob>,
            ::windows::core::GetTrustLevel,
            JobId::<Impl, OFFSET>,
            Printer::<Impl, OFFSET>,
            GetJobStatus::<Impl, OFFSET>,
            GetJobPrintTicket::<Impl, OFFSET>,
            GetJobAttributesAsBuffer::<Impl, OFFSET>,
            GetJobAttributes::<Impl, OFFSET>,
            SetJobAttributesFromBuffer::<Impl, OFFSET>,
            SetJobAttributes::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowSourceContentImpl: Sized {
    fn GetJobPrintTicketAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::PrintTicket::WorkflowPrintTicket>>;
    fn GetSourceSpoolDataAsStreamContent(&self) -> ::windows::core::Result<PrintWorkflowSpoolStreamContent>;
    fn GetSourceSpoolDataAsXpsObjectModel(&self) -> ::windows::core::Result<PrintWorkflowObjectModelSourceFileContent>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintWorkflowSourceContent {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowSourceContent";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintWorkflowSourceContentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowSourceContentImpl, const OFFSET: isize>() -> IPrintWorkflowSourceContentVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintWorkflowSourceContent>, ::windows::core::GetTrustLevel, GetJobPrintTicketAsync::<Impl, OFFSET>, GetSourceSpoolDataAsStreamContent::<Impl, OFFSET>, GetSourceSpoolDataAsXpsObjectModel::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowSpoolStreamContentImpl: Sized {
    fn GetInputStream(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IInputStream>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintWorkflowSpoolStreamContent {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowSpoolStreamContent";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintWorkflowSpoolStreamContentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowSpoolStreamContentImpl, const OFFSET: isize>() -> IPrintWorkflowSpoolStreamContentVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintWorkflowSpoolStreamContent>, ::windows::core::GetTrustLevel, GetInputStream::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowStreamTargetImpl: Sized {
    fn GetOutputStream(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IOutputStream>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintWorkflowStreamTarget {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowStreamTarget";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintWorkflowStreamTargetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowStreamTargetImpl, const OFFSET: isize>() -> IPrintWorkflowStreamTargetVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintWorkflowStreamTarget>, ::windows::core::GetTrustLevel, GetOutputStream::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowSubmittedEventArgsImpl: Sized {
    fn Operation(&self) -> ::windows::core::Result<PrintWorkflowSubmittedOperation>;
    fn GetTarget(&self, jobprintticket: &::core::option::Option<super::PrintTicket::WorkflowPrintTicket>) -> ::windows::core::Result<PrintWorkflowTarget>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintWorkflowSubmittedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowSubmittedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintWorkflowSubmittedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowSubmittedEventArgsImpl, const OFFSET: isize>() -> IPrintWorkflowSubmittedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintWorkflowSubmittedEventArgs>, ::windows::core::GetTrustLevel, Operation::<Impl, OFFSET>, GetTarget::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowSubmittedOperationImpl: Sized {
    fn Complete(&self, status: PrintWorkflowSubmittedStatus) -> ::windows::core::Result<()>;
    fn Configuration(&self) -> ::windows::core::Result<PrintWorkflowConfiguration>;
    fn XpsContent(&self) -> ::windows::core::Result<PrintWorkflowSourceContent>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintWorkflowSubmittedOperation {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowSubmittedOperation";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintWorkflowSubmittedOperationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowSubmittedOperationImpl, const OFFSET: isize>() -> IPrintWorkflowSubmittedOperationVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintWorkflowSubmittedOperation>, ::windows::core::GetTrustLevel, Complete::<Impl, OFFSET>, Configuration::<Impl, OFFSET>, XpsContent::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowTargetImpl: Sized {
    fn TargetAsStream(&self) -> ::windows::core::Result<PrintWorkflowStreamTarget>;
    fn TargetAsXpsObjectModelPackage(&self) -> ::windows::core::Result<PrintWorkflowObjectModelTargetPackage>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintWorkflowTarget {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowTarget";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintWorkflowTargetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowTargetImpl, const OFFSET: isize>() -> IPrintWorkflowTargetVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintWorkflowTarget>, ::windows::core::GetTrustLevel, TargetAsStream::<Impl, OFFSET>, TargetAsXpsObjectModelPackage::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowTriggerDetailsImpl: Sized {
    fn PrintWorkflowSession(&self) -> ::windows::core::Result<PrintWorkflowBackgroundSession>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintWorkflowTriggerDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintWorkflowTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowTriggerDetailsImpl, const OFFSET: isize>() -> IPrintWorkflowTriggerDetailsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintWorkflowTriggerDetails>, ::windows::core::GetTrustLevel, PrintWorkflowSession::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "implement_exclusive"))]
pub trait IPrintWorkflowUIActivatedEventArgsImpl: Sized + IActivatedEventArgsImpl + IActivatedEventArgsWithUserImpl {
    fn PrintWorkflowSession(&self) -> ::windows::core::Result<PrintWorkflowForegroundSession>;
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrintWorkflowUIActivatedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowUIActivatedEventArgs";
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "implement_exclusive"))]
impl IPrintWorkflowUIActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowUIActivatedEventArgsImpl, const OFFSET: isize>() -> IPrintWorkflowUIActivatedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintWorkflowUIActivatedEventArgs>, ::windows::core::GetTrustLevel, PrintWorkflowSession::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowUILauncherImpl: Sized {
    fn IsUILaunchEnabled(&self) -> ::windows::core::Result<bool>;
    fn LaunchAndCompleteUIAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<PrintWorkflowUICompletionStatus>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintWorkflowUILauncher {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowUILauncher";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintWorkflowUILauncherVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowUILauncherImpl, const OFFSET: isize>() -> IPrintWorkflowUILauncherVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintWorkflowUILauncher>, ::windows::core::GetTrustLevel, IsUILaunchEnabled::<Impl, OFFSET>, LaunchAndCompleteUIAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintWorkflowXpsDataAvailableEventArgsImpl: Sized {
    fn Operation(&self) -> ::windows::core::Result<PrintWorkflowSubmittedOperation>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintWorkflowXpsDataAvailableEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.IPrintWorkflowXpsDataAvailableEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintWorkflowXpsDataAvailableEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintWorkflowXpsDataAvailableEventArgsImpl, const OFFSET: isize>() -> IPrintWorkflowXpsDataAvailableEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintWorkflowXpsDataAvailableEventArgs>, ::windows::core::GetTrustLevel, Operation::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
