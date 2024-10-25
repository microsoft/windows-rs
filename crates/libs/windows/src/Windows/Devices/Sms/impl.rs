#[cfg(feature = "deprecated")]
pub trait ISmsBinaryMessage_Impl: Sized + windows_core::IUnknownImpl + ISmsMessage_Impl {
    fn Format(&self) -> windows_core::Result<SmsDataFormat>;
    fn SetFormat(&self, value: SmsDataFormat) -> windows_core::Result<()>;
    fn GetData(&self) -> windows_core::Result<windows_core::Array<u8>>;
    fn SetData(&self, value: &[u8]) -> windows_core::Result<()>;
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for ISmsBinaryMessage {
    const NAME: &'static str = "Windows.Devices.Sms.ISmsBinaryMessage";
}
#[cfg(feature = "deprecated")]
impl ISmsBinaryMessage_Vtbl {
    pub const fn new<Identity: ISmsBinaryMessage_Impl, const OFFSET: isize>() -> ISmsBinaryMessage_Vtbl {
        unsafe extern "system" fn Format<Identity: ISmsBinaryMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut SmsDataFormat) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISmsBinaryMessage_Impl::Format(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormat<Identity: ISmsBinaryMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: SmsDataFormat) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISmsBinaryMessage_Impl::SetFormat(this, value).into()
        }
        unsafe extern "system" fn GetData<Identity: ISmsBinaryMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISmsBinaryMessage_Impl::GetData(this) {
                Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    result__.write(ok_data__);
                    result_size__.write(ok_data_len__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetData<Identity: ISmsBinaryMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value_array_size: u32, value: *const u8) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISmsBinaryMessage_Impl::SetData(this, core::slice::from_raw_parts(core::mem::transmute_copy(&value), value_array_size as usize)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ISmsBinaryMessage, OFFSET>(),
            Format: Format::<Identity, OFFSET>,
            SetFormat: SetFormat::<Identity, OFFSET>,
            GetData: GetData::<Identity, OFFSET>,
            SetData: SetData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISmsBinaryMessage as windows_core::Interface>::IID
    }
}
#[cfg(feature = "deprecated")]
pub trait ISmsDevice_Impl: Sized + windows_core::IUnknownImpl {
    fn SendMessageAsync(&self, message: Option<&ISmsMessage>) -> windows_core::Result<SendSmsMessageOperation>;
    fn CalculateLength(&self, message: Option<&SmsTextMessage>) -> windows_core::Result<SmsEncodedLength>;
    fn AccountPhoneNumber(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn CellularClass(&self) -> windows_core::Result<CellularClass>;
    fn MessageStore(&self) -> windows_core::Result<SmsDeviceMessageStore>;
    fn DeviceStatus(&self) -> windows_core::Result<SmsDeviceStatus>;
    fn SmsMessageReceived(&self, eventhandler: Option<&SmsMessageReceivedEventHandler>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSmsMessageReceived(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
    fn SmsDeviceStatusChanged(&self, eventhandler: Option<&SmsDeviceStatusChangedEventHandler>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSmsDeviceStatusChanged(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for ISmsDevice {
    const NAME: &'static str = "Windows.Devices.Sms.ISmsDevice";
}
#[cfg(feature = "deprecated")]
impl ISmsDevice_Vtbl {
    pub const fn new<Identity: ISmsDevice_Impl, const OFFSET: isize>() -> ISmsDevice_Vtbl {
        unsafe extern "system" fn SendMessageAsync<Identity: ISmsDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, message: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISmsDevice_Impl::SendMessageAsync(this, windows_core::from_raw_borrowed(&message)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CalculateLength<Identity: ISmsDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, message: *mut core::ffi::c_void, result__: *mut SmsEncodedLength) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISmsDevice_Impl::CalculateLength(this, windows_core::from_raw_borrowed(&message)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AccountPhoneNumber<Identity: ISmsDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISmsDevice_Impl::AccountPhoneNumber(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CellularClass<Identity: ISmsDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut CellularClass) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISmsDevice_Impl::CellularClass(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MessageStore<Identity: ISmsDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISmsDevice_Impl::MessageStore(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceStatus<Identity: ISmsDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut SmsDeviceStatus) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISmsDevice_Impl::DeviceStatus(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SmsMessageReceived<Identity: ISmsDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventhandler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISmsDevice_Impl::SmsMessageReceived(this, windows_core::from_raw_borrowed(&eventhandler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSmsMessageReceived<Identity: ISmsDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISmsDevice_Impl::RemoveSmsMessageReceived(this, core::mem::transmute(&eventcookie)).into()
        }
        unsafe extern "system" fn SmsDeviceStatusChanged<Identity: ISmsDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventhandler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISmsDevice_Impl::SmsDeviceStatusChanged(this, windows_core::from_raw_borrowed(&eventhandler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSmsDeviceStatusChanged<Identity: ISmsDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISmsDevice_Impl::RemoveSmsDeviceStatusChanged(this, core::mem::transmute(&eventcookie)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ISmsDevice, OFFSET>(),
            SendMessageAsync: SendMessageAsync::<Identity, OFFSET>,
            CalculateLength: CalculateLength::<Identity, OFFSET>,
            AccountPhoneNumber: AccountPhoneNumber::<Identity, OFFSET>,
            CellularClass: CellularClass::<Identity, OFFSET>,
            MessageStore: MessageStore::<Identity, OFFSET>,
            DeviceStatus: DeviceStatus::<Identity, OFFSET>,
            SmsMessageReceived: SmsMessageReceived::<Identity, OFFSET>,
            RemoveSmsMessageReceived: RemoveSmsMessageReceived::<Identity, OFFSET>,
            SmsDeviceStatusChanged: SmsDeviceStatusChanged::<Identity, OFFSET>,
            RemoveSmsDeviceStatusChanged: RemoveSmsDeviceStatusChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISmsDevice as windows_core::Interface>::IID
    }
}
pub trait ISmsMessage_Impl: Sized + windows_core::IUnknownImpl {
    fn Id(&self) -> windows_core::Result<u32>;
    fn MessageClass(&self) -> windows_core::Result<SmsMessageClass>;
}
impl windows_core::RuntimeName for ISmsMessage {
    const NAME: &'static str = "Windows.Devices.Sms.ISmsMessage";
}
impl ISmsMessage_Vtbl {
    pub const fn new<Identity: ISmsMessage_Impl, const OFFSET: isize>() -> ISmsMessage_Vtbl {
        unsafe extern "system" fn Id<Identity: ISmsMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISmsMessage_Impl::Id(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MessageClass<Identity: ISmsMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut SmsMessageClass) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISmsMessage_Impl::MessageClass(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ISmsMessage, OFFSET>(),
            Id: Id::<Identity, OFFSET>,
            MessageClass: MessageClass::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISmsMessage as windows_core::Interface>::IID
    }
}
pub trait ISmsMessageBase_Impl: Sized + windows_core::IUnknownImpl {
    fn MessageType(&self) -> windows_core::Result<SmsMessageType>;
    fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn CellularClass(&self) -> windows_core::Result<CellularClass>;
    fn MessageClass(&self) -> windows_core::Result<SmsMessageClass>;
    fn SimIccId(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl windows_core::RuntimeName for ISmsMessageBase {
    const NAME: &'static str = "Windows.Devices.Sms.ISmsMessageBase";
}
impl ISmsMessageBase_Vtbl {
    pub const fn new<Identity: ISmsMessageBase_Impl, const OFFSET: isize>() -> ISmsMessageBase_Vtbl {
        unsafe extern "system" fn MessageType<Identity: ISmsMessageBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut SmsMessageType) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISmsMessageBase_Impl::MessageType(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceId<Identity: ISmsMessageBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISmsMessageBase_Impl::DeviceId(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CellularClass<Identity: ISmsMessageBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut CellularClass) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISmsMessageBase_Impl::CellularClass(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MessageClass<Identity: ISmsMessageBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut SmsMessageClass) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISmsMessageBase_Impl::MessageClass(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SimIccId<Identity: ISmsMessageBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISmsMessageBase_Impl::SimIccId(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ISmsMessageBase, OFFSET>(),
            MessageType: MessageType::<Identity, OFFSET>,
            DeviceId: DeviceId::<Identity, OFFSET>,
            CellularClass: CellularClass::<Identity, OFFSET>,
            MessageClass: MessageClass::<Identity, OFFSET>,
            SimIccId: SimIccId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISmsMessageBase as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
pub trait ISmsTextMessage_Impl: Sized + windows_core::IUnknownImpl + ISmsMessage_Impl {
    fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::DateTime>;
    fn PartReferenceId(&self) -> windows_core::Result<u32>;
    fn PartNumber(&self) -> windows_core::Result<u32>;
    fn PartCount(&self) -> windows_core::Result<u32>;
    fn To(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetTo(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn From(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetFrom(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn Body(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetBody(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn Encoding(&self) -> windows_core::Result<SmsEncoding>;
    fn SetEncoding(&self, value: SmsEncoding) -> windows_core::Result<()>;
    fn ToBinaryMessages(&self, format: SmsDataFormat) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<ISmsBinaryMessage>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
impl windows_core::RuntimeName for ISmsTextMessage {
    const NAME: &'static str = "Windows.Devices.Sms.ISmsTextMessage";
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
impl ISmsTextMessage_Vtbl {
    pub const fn new<Identity: ISmsTextMessage_Impl, const OFFSET: isize>() -> ISmsTextMessage_Vtbl {
        unsafe extern "system" fn Timestamp<Identity: ISmsTextMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISmsTextMessage_Impl::Timestamp(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PartReferenceId<Identity: ISmsTextMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISmsTextMessage_Impl::PartReferenceId(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PartNumber<Identity: ISmsTextMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISmsTextMessage_Impl::PartNumber(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PartCount<Identity: ISmsTextMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISmsTextMessage_Impl::PartCount(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn To<Identity: ISmsTextMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISmsTextMessage_Impl::To(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTo<Identity: ISmsTextMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISmsTextMessage_Impl::SetTo(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn From<Identity: ISmsTextMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISmsTextMessage_Impl::From(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFrom<Identity: ISmsTextMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISmsTextMessage_Impl::SetFrom(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Body<Identity: ISmsTextMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISmsTextMessage_Impl::Body(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBody<Identity: ISmsTextMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISmsTextMessage_Impl::SetBody(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Encoding<Identity: ISmsTextMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut SmsEncoding) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISmsTextMessage_Impl::Encoding(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEncoding<Identity: ISmsTextMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: SmsEncoding) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISmsTextMessage_Impl::SetEncoding(this, value).into()
        }
        unsafe extern "system" fn ToBinaryMessages<Identity: ISmsTextMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: SmsDataFormat, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISmsTextMessage_Impl::ToBinaryMessages(this, format) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ISmsTextMessage, OFFSET>(),
            Timestamp: Timestamp::<Identity, OFFSET>,
            PartReferenceId: PartReferenceId::<Identity, OFFSET>,
            PartNumber: PartNumber::<Identity, OFFSET>,
            PartCount: PartCount::<Identity, OFFSET>,
            To: To::<Identity, OFFSET>,
            SetTo: SetTo::<Identity, OFFSET>,
            From: From::<Identity, OFFSET>,
            SetFrom: SetFrom::<Identity, OFFSET>,
            Body: Body::<Identity, OFFSET>,
            SetBody: SetBody::<Identity, OFFSET>,
            Encoding: Encoding::<Identity, OFFSET>,
            SetEncoding: SetEncoding::<Identity, OFFSET>,
            ToBinaryMessages: ToBinaryMessages::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISmsTextMessage as windows_core::Interface>::IID
    }
}
