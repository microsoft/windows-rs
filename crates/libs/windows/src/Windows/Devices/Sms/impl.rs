#[cfg(feature = "deprecated")]
pub trait ISmsBinaryMessage_Impl: Sized + ISmsMessage_Impl {
    fn Format(&mut self) -> ::windows::core::Result<SmsDataFormat>;
    fn SetFormat(&mut self, value: SmsDataFormat) -> ::windows::core::Result<()>;
    fn GetData(&mut self) -> ::windows::core::Result<::windows::core::Array<u8>>;
    fn SetData(&mut self, value: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for ISmsBinaryMessage {
    const NAME: &'static str = "Windows.Devices.Sms.ISmsBinaryMessage";
}
#[cfg(feature = "deprecated")]
impl ISmsBinaryMessage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmsBinaryMessage_Impl, const OFFSET: isize>() -> ISmsBinaryMessage_Vtbl {
        unsafe extern "system" fn Format<Identity: ::windows::core::IUnknownImpl, Impl: ISmsBinaryMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SmsDataFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Format() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormat<Identity: ::windows::core::IUnknownImpl, Impl: ISmsBinaryMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: SmsDataFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFormat(value).into()
        }
        unsafe extern "system" fn GetData<Identity: ::windows::core::IUnknownImpl, Impl: ISmsBinaryMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetData() {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    *result__ = ok_data__;
                    *result_size__ = ok_data_len__;
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetData<Identity: ::windows::core::IUnknownImpl, Impl: ISmsBinaryMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetData(::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISmsBinaryMessage, OFFSET>(),
            Format: Format::<Identity, Impl, OFFSET>,
            SetFormat: SetFormat::<Identity, Impl, OFFSET>,
            GetData: GetData::<Identity, Impl, OFFSET>,
            SetData: SetData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmsBinaryMessage as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
pub trait ISmsDevice_Impl: Sized {
    fn SendMessageAsync(&mut self, message: &::core::option::Option<ISmsMessage>) -> ::windows::core::Result<SendSmsMessageOperation>;
    fn CalculateLength(&mut self, message: &::core::option::Option<SmsTextMessage>) -> ::windows::core::Result<SmsEncodedLength>;
    fn AccountPhoneNumber(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CellularClass(&mut self) -> ::windows::core::Result<CellularClass>;
    fn MessageStore(&mut self) -> ::windows::core::Result<SmsDeviceMessageStore>;
    fn DeviceStatus(&mut self) -> ::windows::core::Result<SmsDeviceStatus>;
    fn SmsMessageReceived(&mut self, eventhandler: &::core::option::Option<SmsMessageReceivedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSmsMessageReceived(&mut self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SmsDeviceStatusChanged(&mut self, eventhandler: &::core::option::Option<SmsDeviceStatusChangedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSmsDeviceStatusChanged(&mut self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::windows::core::RuntimeName for ISmsDevice {
    const NAME: &'static str = "Windows.Devices.Sms.ISmsDevice";
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ISmsDevice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmsDevice_Impl, const OFFSET: isize>() -> ISmsDevice_Vtbl {
        unsafe extern "system" fn SendMessageAsync<Identity: ::windows::core::IUnknownImpl, Impl: ISmsDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SendMessageAsync(&*(&message as *const <ISmsMessage as ::windows::core::Abi>::Abi as *const <ISmsMessage as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CalculateLength<Identity: ::windows::core::IUnknownImpl, Impl: ISmsDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: ::windows::core::RawPtr, result__: *mut SmsEncodedLength) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CalculateLength(&*(&message as *const <SmsTextMessage as ::windows::core::Abi>::Abi as *const <SmsTextMessage as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AccountPhoneNumber<Identity: ::windows::core::IUnknownImpl, Impl: ISmsDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AccountPhoneNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CellularClass<Identity: ::windows::core::IUnknownImpl, Impl: ISmsDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CellularClass) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CellularClass() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MessageStore<Identity: ::windows::core::IUnknownImpl, Impl: ISmsDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MessageStore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceStatus<Identity: ::windows::core::IUnknownImpl, Impl: ISmsDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SmsDeviceStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DeviceStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SmsMessageReceived<Identity: ::windows::core::IUnknownImpl, Impl: ISmsDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SmsMessageReceived(&*(&eventhandler as *const <SmsMessageReceivedEventHandler as ::windows::core::Abi>::Abi as *const <SmsMessageReceivedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSmsMessageReceived<Identity: ::windows::core::IUnknownImpl, Impl: ISmsDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveSmsMessageReceived(&*(&eventcookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SmsDeviceStatusChanged<Identity: ::windows::core::IUnknownImpl, Impl: ISmsDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SmsDeviceStatusChanged(&*(&eventhandler as *const <SmsDeviceStatusChangedEventHandler as ::windows::core::Abi>::Abi as *const <SmsDeviceStatusChangedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSmsDeviceStatusChanged<Identity: ::windows::core::IUnknownImpl, Impl: ISmsDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveSmsDeviceStatusChanged(&*(&eventcookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISmsDevice, OFFSET>(),
            SendMessageAsync: SendMessageAsync::<Identity, Impl, OFFSET>,
            CalculateLength: CalculateLength::<Identity, Impl, OFFSET>,
            AccountPhoneNumber: AccountPhoneNumber::<Identity, Impl, OFFSET>,
            CellularClass: CellularClass::<Identity, Impl, OFFSET>,
            MessageStore: MessageStore::<Identity, Impl, OFFSET>,
            DeviceStatus: DeviceStatus::<Identity, Impl, OFFSET>,
            SmsMessageReceived: SmsMessageReceived::<Identity, Impl, OFFSET>,
            RemoveSmsMessageReceived: RemoveSmsMessageReceived::<Identity, Impl, OFFSET>,
            SmsDeviceStatusChanged: SmsDeviceStatusChanged::<Identity, Impl, OFFSET>,
            RemoveSmsDeviceStatusChanged: RemoveSmsDeviceStatusChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmsDevice as ::windows::core::Interface>::IID
    }
}
pub trait ISmsMessage_Impl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<u32>;
    fn MessageClass(&mut self) -> ::windows::core::Result<SmsMessageClass>;
}
impl ::windows::core::RuntimeName for ISmsMessage {
    const NAME: &'static str = "Windows.Devices.Sms.ISmsMessage";
}
impl ISmsMessage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmsMessage_Impl, const OFFSET: isize>() -> ISmsMessage_Vtbl {
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl, Impl: ISmsMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MessageClass<Identity: ::windows::core::IUnknownImpl, Impl: ISmsMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SmsMessageClass) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MessageClass() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISmsMessage, OFFSET>(),
            Id: Id::<Identity, Impl, OFFSET>,
            MessageClass: MessageClass::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmsMessage as ::windows::core::Interface>::IID
    }
}
pub trait ISmsMessageBase_Impl: Sized {
    fn MessageType(&mut self) -> ::windows::core::Result<SmsMessageType>;
    fn DeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CellularClass(&mut self) -> ::windows::core::Result<CellularClass>;
    fn MessageClass(&mut self) -> ::windows::core::Result<SmsMessageClass>;
    fn SimIccId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for ISmsMessageBase {
    const NAME: &'static str = "Windows.Devices.Sms.ISmsMessageBase";
}
impl ISmsMessageBase_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmsMessageBase_Impl, const OFFSET: isize>() -> ISmsMessageBase_Vtbl {
        unsafe extern "system" fn MessageType<Identity: ::windows::core::IUnknownImpl, Impl: ISmsMessageBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SmsMessageType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MessageType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceId<Identity: ::windows::core::IUnknownImpl, Impl: ISmsMessageBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CellularClass<Identity: ::windows::core::IUnknownImpl, Impl: ISmsMessageBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CellularClass) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CellularClass() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MessageClass<Identity: ::windows::core::IUnknownImpl, Impl: ISmsMessageBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SmsMessageClass) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MessageClass() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SimIccId<Identity: ::windows::core::IUnknownImpl, Impl: ISmsMessageBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SimIccId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISmsMessageBase, OFFSET>(),
            MessageType: MessageType::<Identity, Impl, OFFSET>,
            DeviceId: DeviceId::<Identity, Impl, OFFSET>,
            CellularClass: CellularClass::<Identity, Impl, OFFSET>,
            MessageClass: MessageClass::<Identity, Impl, OFFSET>,
            SimIccId: SimIccId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmsMessageBase as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated"))]
pub trait ISmsTextMessage_Impl: Sized + ISmsMessage_Impl {
    fn Timestamp(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn PartReferenceId(&mut self) -> ::windows::core::Result<u32>;
    fn PartNumber(&mut self) -> ::windows::core::Result<u32>;
    fn PartCount(&mut self) -> ::windows::core::Result<u32>;
    fn To(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTo(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn From(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetFrom(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Body(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetBody(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Encoding(&mut self) -> ::windows::core::Result<SmsEncoding>;
    fn SetEncoding(&mut self, value: SmsEncoding) -> ::windows::core::Result<()>;
    fn ToBinaryMessages(&mut self, format: SmsDataFormat) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ISmsBinaryMessage>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated"))]
impl ::windows::core::RuntimeName for ISmsTextMessage {
    const NAME: &'static str = "Windows.Devices.Sms.ISmsTextMessage";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated"))]
impl ISmsTextMessage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmsTextMessage_Impl, const OFFSET: isize>() -> ISmsTextMessage_Vtbl {
        unsafe extern "system" fn Timestamp<Identity: ::windows::core::IUnknownImpl, Impl: ISmsTextMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PartReferenceId<Identity: ::windows::core::IUnknownImpl, Impl: ISmsTextMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PartReferenceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PartNumber<Identity: ::windows::core::IUnknownImpl, Impl: ISmsTextMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PartNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PartCount<Identity: ::windows::core::IUnknownImpl, Impl: ISmsTextMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PartCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn To<Identity: ::windows::core::IUnknownImpl, Impl: ISmsTextMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).To() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTo<Identity: ::windows::core::IUnknownImpl, Impl: ISmsTextMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTo(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn From<Identity: ::windows::core::IUnknownImpl, Impl: ISmsTextMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).From() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFrom<Identity: ::windows::core::IUnknownImpl, Impl: ISmsTextMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFrom(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Body<Identity: ::windows::core::IUnknownImpl, Impl: ISmsTextMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Body() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBody<Identity: ::windows::core::IUnknownImpl, Impl: ISmsTextMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBody(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Encoding<Identity: ::windows::core::IUnknownImpl, Impl: ISmsTextMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SmsEncoding) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Encoding() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEncoding<Identity: ::windows::core::IUnknownImpl, Impl: ISmsTextMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: SmsEncoding) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEncoding(value).into()
        }
        unsafe extern "system" fn ToBinaryMessages<Identity: ::windows::core::IUnknownImpl, Impl: ISmsTextMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: SmsDataFormat, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ToBinaryMessages(format) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISmsTextMessage, OFFSET>(),
            Timestamp: Timestamp::<Identity, Impl, OFFSET>,
            PartReferenceId: PartReferenceId::<Identity, Impl, OFFSET>,
            PartNumber: PartNumber::<Identity, Impl, OFFSET>,
            PartCount: PartCount::<Identity, Impl, OFFSET>,
            To: To::<Identity, Impl, OFFSET>,
            SetTo: SetTo::<Identity, Impl, OFFSET>,
            From: From::<Identity, Impl, OFFSET>,
            SetFrom: SetFrom::<Identity, Impl, OFFSET>,
            Body: Body::<Identity, Impl, OFFSET>,
            SetBody: SetBody::<Identity, Impl, OFFSET>,
            Encoding: Encoding::<Identity, Impl, OFFSET>,
            SetEncoding: SetEncoding::<Identity, Impl, OFFSET>,
            ToBinaryMessages: ToBinaryMessages::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmsTextMessage as ::windows::core::Interface>::IID
    }
}
