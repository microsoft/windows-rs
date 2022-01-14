#[cfg(all(feature = "Devices_Printers", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPrintSupportExtensionSession_Impl: Sized {
    fn Printer(&mut self) -> ::windows::core::Result<super::super::super::Devices::Printers::IppPrintDevice>;
    fn PrintTicketValidationRequested(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<PrintSupportExtensionSession, PrintSupportPrintTicketValidationRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePrintTicketValidationRequested(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PrintDeviceCapabilitiesChanged(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<PrintSupportExtensionSession, PrintSupportPrintDeviceCapabilitiesChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePrintDeviceCapabilitiesChanged(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Start(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Devices_Printers", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrintSupportExtensionSession {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.IPrintSupportExtensionSession";
}
#[cfg(all(feature = "Devices_Printers", feature = "Foundation", feature = "implement_exclusive"))]
impl IPrintSupportExtensionSession_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSupportExtensionSession_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintSupportExtensionSession_Vtbl {
        unsafe extern "system" fn Printer<Impl: IPrintSupportExtensionSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PrintTicketValidationRequested<Impl: IPrintSupportExtensionSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemovePrintTicketValidationRequested<Impl: IPrintSupportExtensionSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePrintTicketValidationRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PrintDeviceCapabilitiesChanged<Impl: IPrintSupportExtensionSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemovePrintDeviceCapabilitiesChanged<Impl: IPrintSupportExtensionSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePrintDeviceCapabilitiesChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Start<Impl: IPrintSupportExtensionSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintSupportExtensionSession, BASE_OFFSET>(),
            Printer: Printer::<Impl, IMPL_OFFSET>,
            PrintTicketValidationRequested: PrintTicketValidationRequested::<Impl, IMPL_OFFSET>,
            RemovePrintTicketValidationRequested: RemovePrintTicketValidationRequested::<Impl, IMPL_OFFSET>,
            PrintDeviceCapabilitiesChanged: PrintDeviceCapabilitiesChanged::<Impl, IMPL_OFFSET>,
            RemovePrintDeviceCapabilitiesChanged: RemovePrintDeviceCapabilitiesChanged::<Impl, IMPL_OFFSET>,
            Start: Start::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSupportExtensionSession as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintSupportExtensionTriggerDetails_Impl: Sized {
    fn Session(&mut self) -> ::windows::core::Result<PrintSupportExtensionSession>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintSupportExtensionTriggerDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.IPrintSupportExtensionTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintSupportExtensionTriggerDetails_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSupportExtensionTriggerDetails_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintSupportExtensionTriggerDetails_Vtbl {
        unsafe extern "system" fn Session<Impl: IPrintSupportExtensionTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintSupportExtensionTriggerDetails, BASE_OFFSET>(),
            Session: Session::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSupportExtensionTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPrintSupportPrintDeviceCapabilitiesChangedEventArgs_Impl: Sized {
    fn GetCurrentPrintDeviceCapabilities(&mut self) -> ::windows::core::Result<super::super::super::Data::Xml::Dom::XmlDocument>;
    fn UpdatePrintDeviceCapabilities(&mut self, updatedpdc: &::core::option::Option<super::super::super::Data::Xml::Dom::XmlDocument>) -> ::windows::core::Result<()>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrintSupportPrintDeviceCapabilitiesChangedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.IPrintSupportPrintDeviceCapabilitiesChangedEventArgs";
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation", feature = "implement_exclusive"))]
impl IPrintSupportPrintDeviceCapabilitiesChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSupportPrintDeviceCapabilitiesChangedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintSupportPrintDeviceCapabilitiesChangedEventArgs_Vtbl {
        unsafe extern "system" fn GetCurrentPrintDeviceCapabilities<Impl: IPrintSupportPrintDeviceCapabilitiesChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UpdatePrintDeviceCapabilities<Impl: IPrintSupportPrintDeviceCapabilitiesChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, updatedpdc: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdatePrintDeviceCapabilities(&*(&updatedpdc as *const <super::super::super::Data::Xml::Dom::XmlDocument as ::windows::core::Abi>::Abi as *const <super::super::super::Data::Xml::Dom::XmlDocument as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetDeferral<Impl: IPrintSupportPrintDeviceCapabilitiesChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintSupportPrintDeviceCapabilitiesChangedEventArgs, BASE_OFFSET>(),
            GetCurrentPrintDeviceCapabilities: GetCurrentPrintDeviceCapabilities::<Impl, IMPL_OFFSET>,
            UpdatePrintDeviceCapabilities: UpdatePrintDeviceCapabilities::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSupportPrintDeviceCapabilitiesChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Printing_PrintTicket", feature = "implement_exclusive"))]
pub trait IPrintSupportPrintTicketValidationRequestedEventArgs_Impl: Sized {
    fn PrintTicket(&mut self) -> ::windows::core::Result<super::PrintTicket::WorkflowPrintTicket>;
    fn SetPrintTicketValidationStatus(&mut self, status: WorkflowPrintTicketValidationStatus) -> ::windows::core::Result<()>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Printing_PrintTicket", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrintSupportPrintTicketValidationRequestedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.IPrintSupportPrintTicketValidationRequestedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Printing_PrintTicket", feature = "implement_exclusive"))]
impl IPrintSupportPrintTicketValidationRequestedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSupportPrintTicketValidationRequestedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintSupportPrintTicketValidationRequestedEventArgs_Vtbl {
        unsafe extern "system" fn PrintTicket<Impl: IPrintSupportPrintTicketValidationRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPrintTicketValidationStatus<Impl: IPrintSupportPrintTicketValidationRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: WorkflowPrintTicketValidationStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrintTicketValidationStatus(status).into()
        }
        unsafe extern "system" fn GetDeferral<Impl: IPrintSupportPrintTicketValidationRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintSupportPrintTicketValidationRequestedEventArgs, BASE_OFFSET>(),
            PrintTicket: PrintTicket::<Impl, IMPL_OFFSET>,
            SetPrintTicketValidationStatus: SetPrintTicketValidationStatus::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSupportPrintTicketValidationRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel", feature = "Devices_Printers", feature = "implement_exclusive"))]
pub trait IPrintSupportSessionInfo_Impl: Sized {
    fn SourceAppInfo(&mut self) -> ::windows::core::Result<super::super::super::ApplicationModel::AppInfo>;
    fn Printer(&mut self) -> ::windows::core::Result<super::super::super::Devices::Printers::IppPrintDevice>;
}
#[cfg(all(feature = "ApplicationModel", feature = "Devices_Printers", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrintSupportSessionInfo {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.IPrintSupportSessionInfo";
}
#[cfg(all(feature = "ApplicationModel", feature = "Devices_Printers", feature = "implement_exclusive"))]
impl IPrintSupportSessionInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSupportSessionInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintSupportSessionInfo_Vtbl {
        unsafe extern "system" fn SourceAppInfo<Impl: IPrintSupportSessionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Printer<Impl: IPrintSupportSessionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintSupportSessionInfo, BASE_OFFSET>(),
            SourceAppInfo: SourceAppInfo::<Impl, IMPL_OFFSET>,
            Printer: Printer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSupportSessionInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPrintSupportSettingsActivatedEventArgs_Impl: Sized {
    fn Session(&mut self) -> ::windows::core::Result<PrintSupportSettingsUISession>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrintSupportSettingsActivatedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.IPrintSupportSettingsActivatedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPrintSupportSettingsActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSupportSettingsActivatedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintSupportSettingsActivatedEventArgs_Vtbl {
        unsafe extern "system" fn Session<Impl: IPrintSupportSettingsActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeferral<Impl: IPrintSupportSettingsActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintSupportSettingsActivatedEventArgs, BASE_OFFSET>(),
            Session: Session::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSupportSettingsActivatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Graphics_Printing_PrintTicket", feature = "implement_exclusive"))]
pub trait IPrintSupportSettingsUISession_Impl: Sized {
    fn SessionPrintTicket(&mut self) -> ::windows::core::Result<super::PrintTicket::WorkflowPrintTicket>;
    fn DocumentTitle(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LaunchKind(&mut self) -> ::windows::core::Result<SettingsLaunchKind>;
    fn UpdatePrintTicket(&mut self, printticket: &::core::option::Option<super::PrintTicket::WorkflowPrintTicket>) -> ::windows::core::Result<()>;
    fn SessionInfo(&mut self) -> ::windows::core::Result<PrintSupportSessionInfo>;
}
#[cfg(all(feature = "Graphics_Printing_PrintTicket", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPrintSupportSettingsUISession {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.IPrintSupportSettingsUISession";
}
#[cfg(all(feature = "Graphics_Printing_PrintTicket", feature = "implement_exclusive"))]
impl IPrintSupportSettingsUISession_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintSupportSettingsUISession_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrintSupportSettingsUISession_Vtbl {
        unsafe extern "system" fn SessionPrintTicket<Impl: IPrintSupportSettingsUISession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DocumentTitle<Impl: IPrintSupportSettingsUISession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LaunchKind<Impl: IPrintSupportSettingsUISession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SettingsLaunchKind) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UpdatePrintTicket<Impl: IPrintSupportSettingsUISession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, printticket: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdatePrintTicket(&*(&printticket as *const <super::PrintTicket::WorkflowPrintTicket as ::windows::core::Abi>::Abi as *const <super::PrintTicket::WorkflowPrintTicket as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SessionInfo<Impl: IPrintSupportSettingsUISession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPrintSupportSettingsUISession, BASE_OFFSET>(),
            SessionPrintTicket: SessionPrintTicket::<Impl, IMPL_OFFSET>,
            DocumentTitle: DocumentTitle::<Impl, IMPL_OFFSET>,
            LaunchKind: LaunchKind::<Impl, IMPL_OFFSET>,
            UpdatePrintTicket: UpdatePrintTicket::<Impl, IMPL_OFFSET>,
            SessionInfo: SessionInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrintSupportSettingsUISession as ::windows::core::Interface>::IID
    }
}
