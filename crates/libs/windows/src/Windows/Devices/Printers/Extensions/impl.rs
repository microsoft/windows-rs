#[cfg(feature = "implement_exclusive")]
pub trait IPrint3DWorkflowImpl: Sized {
    fn DeviceID(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetPrintModelPackage(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn IsPrintReady(&self) -> ::windows::core::Result<bool>;
    fn SetIsPrintReady(&self, value: bool) -> ::windows::core::Result<()>;
    fn PrintRequested(&self, eventhandler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<Print3DWorkflow, Print3DWorkflowPrintRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePrintRequested(&self, eventcookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrint3DWorkflow {
    const NAME: &'static str = "Windows.Devices.Printers.Extensions.IPrint3DWorkflow";
}
#[cfg(feature = "implement_exclusive")]
impl IPrint3DWorkflowVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrint3DWorkflowImpl, const OFFSET: isize>() -> IPrint3DWorkflowVtbl {
        unsafe extern "system" fn DeviceID<Impl: IPrint3DWorkflowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceID() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrintModelPackage<Impl: IPrint3DWorkflowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPrintModelPackage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPrintReady<Impl: IPrint3DWorkflowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPrintReady() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsPrintReady<Impl: IPrint3DWorkflowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsPrintReady(value).into()
        }
        unsafe extern "system" fn PrintRequested<Impl: IPrint3DWorkflowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrintRequested(&*(&eventhandler as *const <super::super::super::Foundation::TypedEventHandler<Print3DWorkflow, Print3DWorkflowPrintRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<Print3DWorkflow, Print3DWorkflowPrintRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePrintRequested<Impl: IPrint3DWorkflowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePrintRequested(&*(&eventcookie as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrint3DWorkflow>, ::windows::core::GetTrustLevel, DeviceID::<Impl, OFFSET>, GetPrintModelPackage::<Impl, OFFSET>, IsPrintReady::<Impl, OFFSET>, SetIsPrintReady::<Impl, OFFSET>, PrintRequested::<Impl, OFFSET>, RemovePrintRequested::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrint3DWorkflow2Impl: Sized {
    fn PrinterChanged(&self, eventhandler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<Print3DWorkflow, Print3DWorkflowPrinterChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePrinterChanged(&self, eventcookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrint3DWorkflow2 {
    const NAME: &'static str = "Windows.Devices.Printers.Extensions.IPrint3DWorkflow2";
}
#[cfg(feature = "implement_exclusive")]
impl IPrint3DWorkflow2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrint3DWorkflow2Impl, const OFFSET: isize>() -> IPrint3DWorkflow2Vtbl {
        unsafe extern "system" fn PrinterChanged<Impl: IPrint3DWorkflow2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrinterChanged(&*(&eventhandler as *const <super::super::super::Foundation::TypedEventHandler<Print3DWorkflow, Print3DWorkflowPrinterChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<Print3DWorkflow, Print3DWorkflowPrinterChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePrinterChanged<Impl: IPrint3DWorkflow2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePrinterChanged(&*(&eventcookie as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrint3DWorkflow2>, ::windows::core::GetTrustLevel, PrinterChanged::<Impl, OFFSET>, RemovePrinterChanged::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrint3DWorkflowPrintRequestedEventArgsImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<Print3DWorkflowStatus>;
    fn SetExtendedStatus(&self, value: Print3DWorkflowDetail) -> ::windows::core::Result<()>;
    fn SetSource(&self, source: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn SetSourceChanged(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrint3DWorkflowPrintRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.Printers.Extensions.IPrint3DWorkflowPrintRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPrint3DWorkflowPrintRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrint3DWorkflowPrintRequestedEventArgsImpl, const OFFSET: isize>() -> IPrint3DWorkflowPrintRequestedEventArgsVtbl {
        unsafe extern "system" fn Status<Impl: IPrint3DWorkflowPrintRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Print3DWorkflowStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetExtendedStatus<Impl: IPrint3DWorkflowPrintRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: Print3DWorkflowDetail) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExtendedStatus(value).into()
        }
        unsafe extern "system" fn SetSource<Impl: IPrint3DWorkflowPrintRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSource(&*(&source as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetSourceChanged<Impl: IPrint3DWorkflowPrintRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSourceChanged(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrint3DWorkflowPrintRequestedEventArgs>, ::windows::core::GetTrustLevel, Status::<Impl, OFFSET>, SetExtendedStatus::<Impl, OFFSET>, SetSource::<Impl, OFFSET>, SetSourceChanged::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrint3DWorkflowPrinterChangedEventArgsImpl: Sized {
    fn NewDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrint3DWorkflowPrinterChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Printers.Extensions.IPrint3DWorkflowPrinterChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPrint3DWorkflowPrinterChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrint3DWorkflowPrinterChangedEventArgsImpl, const OFFSET: isize>() -> IPrint3DWorkflowPrinterChangedEventArgsVtbl {
        unsafe extern "system" fn NewDeviceId<Impl: IPrint3DWorkflowPrinterChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NewDeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrint3DWorkflowPrinterChangedEventArgs>, ::windows::core::GetTrustLevel, NewDeviceId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintExtensionContextStaticImpl: Sized {
    fn FromDeviceId(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintExtensionContextStatic {
    const NAME: &'static str = "Windows.Devices.Printers.Extensions.IPrintExtensionContextStatic";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintExtensionContextStaticVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintExtensionContextStaticImpl, const OFFSET: isize>() -> IPrintExtensionContextStaticVtbl {
        unsafe extern "system" fn FromDeviceId<Impl: IPrintExtensionContextStaticImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromDeviceId(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintExtensionContextStatic>, ::windows::core::GetTrustLevel, FromDeviceId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintNotificationEventDetailsImpl: Sized {
    fn PrinterName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EventData(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetEventData(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintNotificationEventDetails {
    const NAME: &'static str = "Windows.Devices.Printers.Extensions.IPrintNotificationEventDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintNotificationEventDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintNotificationEventDetailsImpl, const OFFSET: isize>() -> IPrintNotificationEventDetailsVtbl {
        unsafe extern "system" fn PrinterName<Impl: IPrintNotificationEventDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrinterName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventData<Impl: IPrintNotificationEventDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventData<Impl: IPrintNotificationEventDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEventData(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintNotificationEventDetails>, ::windows::core::GetTrustLevel, PrinterName::<Impl, OFFSET>, EventData::<Impl, OFFSET>, SetEventData::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintTaskConfigurationImpl: Sized {
    fn PrinterExtensionContext(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SaveRequested(&self, eventhandler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<PrintTaskConfiguration, PrintTaskConfigurationSaveRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSaveRequested(&self, eventcookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintTaskConfiguration {
    const NAME: &'static str = "Windows.Devices.Printers.Extensions.IPrintTaskConfiguration";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintTaskConfigurationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTaskConfigurationImpl, const OFFSET: isize>() -> IPrintTaskConfigurationVtbl {
        unsafe extern "system" fn PrinterExtensionContext<Impl: IPrintTaskConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrinterExtensionContext() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveRequested<Impl: IPrintTaskConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveRequested(&*(&eventhandler as *const <super::super::super::Foundation::TypedEventHandler<PrintTaskConfiguration, PrintTaskConfigurationSaveRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<PrintTaskConfiguration, PrintTaskConfigurationSaveRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSaveRequested<Impl: IPrintTaskConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSaveRequested(&*(&eventcookie as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintTaskConfiguration>, ::windows::core::GetTrustLevel, PrinterExtensionContext::<Impl, OFFSET>, SaveRequested::<Impl, OFFSET>, RemoveSaveRequested::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintTaskConfigurationSaveRequestImpl: Sized {
    fn Cancel(&self) -> ::windows::core::Result<()>;
    fn Save(&self, printerextensioncontext: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<PrintTaskConfigurationSaveRequestedDeferral>;
    fn Deadline(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintTaskConfigurationSaveRequest {
    const NAME: &'static str = "Windows.Devices.Printers.Extensions.IPrintTaskConfigurationSaveRequest";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintTaskConfigurationSaveRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTaskConfigurationSaveRequestImpl, const OFFSET: isize>() -> IPrintTaskConfigurationSaveRequestVtbl {
        unsafe extern "system" fn Cancel<Impl: IPrintTaskConfigurationSaveRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Cancel().into()
        }
        unsafe extern "system" fn Save<Impl: IPrintTaskConfigurationSaveRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, printerextensioncontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Save(&*(&printerextensioncontext as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetDeferral<Impl: IPrintTaskConfigurationSaveRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Deadline<Impl: IPrintTaskConfigurationSaveRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintTaskConfigurationSaveRequest>, ::windows::core::GetTrustLevel, Cancel::<Impl, OFFSET>, Save::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>, Deadline::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintTaskConfigurationSaveRequestedDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintTaskConfigurationSaveRequestedDeferral {
    const NAME: &'static str = "Windows.Devices.Printers.Extensions.IPrintTaskConfigurationSaveRequestedDeferral";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintTaskConfigurationSaveRequestedDeferralVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTaskConfigurationSaveRequestedDeferralImpl, const OFFSET: isize>() -> IPrintTaskConfigurationSaveRequestedDeferralVtbl {
        unsafe extern "system" fn Complete<Impl: IPrintTaskConfigurationSaveRequestedDeferralImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintTaskConfigurationSaveRequestedDeferral>, ::windows::core::GetTrustLevel, Complete::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintTaskConfigurationSaveRequestedEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<PrintTaskConfigurationSaveRequest>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPrintTaskConfigurationSaveRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.Printers.Extensions.IPrintTaskConfigurationSaveRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPrintTaskConfigurationSaveRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrintTaskConfigurationSaveRequestedEventArgsImpl, const OFFSET: isize>() -> IPrintTaskConfigurationSaveRequestedEventArgsVtbl {
        unsafe extern "system" fn Request<Impl: IPrintTaskConfigurationSaveRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPrintTaskConfigurationSaveRequestedEventArgs>, ::windows::core::GetTrustLevel, Request::<Impl, OFFSET>)
    }
}
