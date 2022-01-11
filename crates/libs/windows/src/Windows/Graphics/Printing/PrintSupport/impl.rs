#[cfg(all(feature = "Devices_Printers", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPrintSupportExtensionSessionImpl: Sized {
    fn Printer(&self) -> ::windows::core::Result<super::super::super::Devices::Printers::IppPrintDevice>;
    fn PrintTicketValidationRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<PrintSupportExtensionSession, PrintSupportPrintTicketValidationRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePrintTicketValidationRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PrintDeviceCapabilitiesChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<PrintSupportExtensionSession, PrintSupportPrintDeviceCapabilitiesChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePrintDeviceCapabilitiesChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Start(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Devices_Printers", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrintSupportExtensionSession {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.IPrintSupportExtensionSession";
}
#[cfg(all(feature = "Devices_Printers", feature = "Foundation", feature = "implement_exclusive"))]
impl IPrintSupportExtensionSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSupportExtensionSessionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintSupportExtensionSessionVtbl {
        unsafe extern "system" fn Printer<Impl: IPrintSupportExtensionSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PrintTicketValidationRequested<Impl: IPrintSupportExtensionSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrintTicketValidationRequested(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<PrintSupportExtensionSession, PrintSupportPrintTicketValidationRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<PrintSupportExtensionSession, PrintSupportPrintTicketValidationRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePrintTicketValidationRequested<Impl: IPrintSupportExtensionSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePrintTicketValidationRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PrintDeviceCapabilitiesChanged<Impl: IPrintSupportExtensionSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrintDeviceCapabilitiesChanged(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<PrintSupportExtensionSession, PrintSupportPrintDeviceCapabilitiesChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<PrintSupportExtensionSession, PrintSupportPrintDeviceCapabilitiesChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePrintDeviceCapabilitiesChanged<Impl: IPrintSupportExtensionSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePrintDeviceCapabilitiesChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Start<Impl: IPrintSupportExtensionSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IPrintSupportExtensionSession>,
            ::windows::core::GetTrustLevel,
            Printer::<Impl, IMPL_OFFSET>,
            PrintTicketValidationRequested::<Impl, IMPL_OFFSET>,
            RemovePrintTicketValidationRequested::<Impl, IMPL_OFFSET>,
            PrintDeviceCapabilitiesChanged::<Impl, IMPL_OFFSET>,
            RemovePrintDeviceCapabilitiesChanged::<Impl, IMPL_OFFSET>,
            Start::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSupportExtensionSession as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintSupportExtensionTriggerDetailsImpl: Sized {
    fn Session(&self) -> ::windows::core::Result<PrintSupportExtensionSession>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintSupportExtensionTriggerDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.IPrintSupportExtensionTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintSupportExtensionTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSupportExtensionTriggerDetailsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintSupportExtensionTriggerDetailsVtbl {
        unsafe extern "system" fn Session<Impl: IPrintSupportExtensionTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintSupportExtensionTriggerDetails>, ::windows::core::GetTrustLevel, Session::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSupportExtensionTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPrintSupportPrintDeviceCapabilitiesChangedEventArgsImpl: Sized {
    fn GetCurrentPrintDeviceCapabilities(&self) -> ::windows::core::Result<super::super::super::Data::Xml::Dom::XmlDocument>;
    fn UpdatePrintDeviceCapabilities(&self, updatedpdc: &::core::option::Option<super::super::super::Data::Xml::Dom::XmlDocument>) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrintSupportPrintDeviceCapabilitiesChangedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.IPrintSupportPrintDeviceCapabilitiesChangedEventArgs";
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation", feature = "implement_exclusive"))]
impl IPrintSupportPrintDeviceCapabilitiesChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSupportPrintDeviceCapabilitiesChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintSupportPrintDeviceCapabilitiesChangedEventArgsVtbl {
        unsafe extern "system" fn GetCurrentPrintDeviceCapabilities<Impl: IPrintSupportPrintDeviceCapabilitiesChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentPrintDeviceCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdatePrintDeviceCapabilities<Impl: IPrintSupportPrintDeviceCapabilitiesChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, updatedpdc: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdatePrintDeviceCapabilities(&*(&updatedpdc as *const <super::super::super::Data::Xml::Dom::XmlDocument as ::windows::core::Abi>::Abi as *const <super::super::super::Data::Xml::Dom::XmlDocument as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetDeferral<Impl: IPrintSupportPrintDeviceCapabilitiesChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintSupportPrintDeviceCapabilitiesChangedEventArgs>, ::windows::core::GetTrustLevel, GetCurrentPrintDeviceCapabilities::<Impl, IMPL_OFFSET>, UpdatePrintDeviceCapabilities::<Impl, IMPL_OFFSET>, GetDeferral::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSupportPrintDeviceCapabilitiesChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Printing_PrintTicket", feature = "implement_exclusive"))]
pub trait IPrintSupportPrintTicketValidationRequestedEventArgsImpl: Sized {
    fn PrintTicket(&self) -> ::windows::core::Result<super::PrintTicket::WorkflowPrintTicket>;
    fn SetPrintTicketValidationStatus(&self, status: WorkflowPrintTicketValidationStatus) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Printing_PrintTicket", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrintSupportPrintTicketValidationRequestedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.IPrintSupportPrintTicketValidationRequestedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Printing_PrintTicket", feature = "implement_exclusive"))]
impl IPrintSupportPrintTicketValidationRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSupportPrintTicketValidationRequestedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintSupportPrintTicketValidationRequestedEventArgsVtbl {
        unsafe extern "system" fn PrintTicket<Impl: IPrintSupportPrintTicketValidationRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrintTicket() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrintTicketValidationStatus<Impl: IPrintSupportPrintTicketValidationRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: WorkflowPrintTicketValidationStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrintTicketValidationStatus(status).into()
        }
        unsafe extern "system" fn GetDeferral<Impl: IPrintSupportPrintTicketValidationRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintSupportPrintTicketValidationRequestedEventArgs>, ::windows::core::GetTrustLevel, PrintTicket::<Impl, IMPL_OFFSET>, SetPrintTicketValidationStatus::<Impl, IMPL_OFFSET>, GetDeferral::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSupportPrintTicketValidationRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel", feature = "Devices_Printers", feature = "implement_exclusive"))]
pub trait IPrintSupportSessionInfoImpl: Sized {
    fn SourceAppInfo(&self) -> ::windows::core::Result<super::super::super::ApplicationModel::AppInfo>;
    fn Printer(&self) -> ::windows::core::Result<super::super::super::Devices::Printers::IppPrintDevice>;
}
#[cfg(all(feature = "ApplicationModel", feature = "Devices_Printers", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrintSupportSessionInfo {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.IPrintSupportSessionInfo";
}
#[cfg(all(feature = "ApplicationModel", feature = "Devices_Printers", feature = "implement_exclusive"))]
impl IPrintSupportSessionInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSupportSessionInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintSupportSessionInfoVtbl {
        unsafe extern "system" fn SourceAppInfo<Impl: IPrintSupportSessionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceAppInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Printer<Impl: IPrintSupportSessionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintSupportSessionInfo>, ::windows::core::GetTrustLevel, SourceAppInfo::<Impl, IMPL_OFFSET>, Printer::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSupportSessionInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPrintSupportSettingsActivatedEventArgsImpl: Sized {
    fn Session(&self) -> ::windows::core::Result<PrintSupportSettingsUISession>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrintSupportSettingsActivatedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.IPrintSupportSettingsActivatedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPrintSupportSettingsActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSupportSettingsActivatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintSupportSettingsActivatedEventArgsVtbl {
        unsafe extern "system" fn Session<Impl: IPrintSupportSettingsActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeferral<Impl: IPrintSupportSettingsActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintSupportSettingsActivatedEventArgs>, ::windows::core::GetTrustLevel, Session::<Impl, IMPL_OFFSET>, GetDeferral::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSupportSettingsActivatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Graphics_Printing_PrintTicket", feature = "implement_exclusive"))]
pub trait IPrintSupportSettingsUISessionImpl: Sized {
    fn SessionPrintTicket(&self) -> ::windows::core::Result<super::PrintTicket::WorkflowPrintTicket>;
    fn DocumentTitle(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LaunchKind(&self) -> ::windows::core::Result<SettingsLaunchKind>;
    fn UpdatePrintTicket(&self, printticket: &::core::option::Option<super::PrintTicket::WorkflowPrintTicket>) -> ::windows::core::Result<()>;
    fn SessionInfo(&self) -> ::windows::core::Result<PrintSupportSessionInfo>;
}
#[cfg(all(feature = "Graphics_Printing_PrintTicket", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrintSupportSettingsUISession {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.IPrintSupportSettingsUISession";
}
#[cfg(all(feature = "Graphics_Printing_PrintTicket", feature = "implement_exclusive"))]
impl IPrintSupportSettingsUISessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSupportSettingsUISessionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintSupportSettingsUISessionVtbl {
        unsafe extern "system" fn SessionPrintTicket<Impl: IPrintSupportSettingsUISessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionPrintTicket() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DocumentTitle<Impl: IPrintSupportSettingsUISessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DocumentTitle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LaunchKind<Impl: IPrintSupportSettingsUISessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SettingsLaunchKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LaunchKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdatePrintTicket<Impl: IPrintSupportSettingsUISessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, printticket: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdatePrintTicket(&*(&printticket as *const <super::PrintTicket::WorkflowPrintTicket as ::windows::core::Abi>::Abi as *const <super::PrintTicket::WorkflowPrintTicket as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SessionInfo<Impl: IPrintSupportSettingsUISessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintSupportSettingsUISession>, ::windows::core::GetTrustLevel, SessionPrintTicket::<Impl, IMPL_OFFSET>, DocumentTitle::<Impl, IMPL_OFFSET>, LaunchKind::<Impl, IMPL_OFFSET>, UpdatePrintTicket::<Impl, IMPL_OFFSET>, SessionInfo::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSupportSettingsUISession as ::windows::core::Interface>::IID
    }
}
