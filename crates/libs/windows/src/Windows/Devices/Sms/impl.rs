#[doc = "*Required features: `\"Devices_Sms\"`, `\"deprecated\"`, `\"implement\"`*"]
#[cfg(feature = "deprecated")]
pub trait ISmsBinaryMessage_Impl: Sized + ISmsMessage_Impl {
    fn Format(&self) -> ::windows_core::Result<SmsDataFormat>;
    fn SetFormat(&self, value: SmsDataFormat) -> ::windows_core::Result<()>;
    fn GetData(&self) -> ::windows_core::Result<::windows_core::Array<u8>>;
    fn SetData(&self, value: &[u8]) -> ::windows_core::Result<()>;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for ISmsBinaryMessage {
    const NAME: &'static str = "Windows.Devices.Sms.ISmsBinaryMessage";
}
#[cfg(feature = "deprecated")]
impl ISmsBinaryMessage_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISmsBinaryMessage_Impl, const OFFSET: isize>() -> ISmsBinaryMessage_Vtbl {
        unsafe extern "system" fn Format<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISmsBinaryMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SmsDataFormat) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Format() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormat<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISmsBinaryMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: SmsDataFormat) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFormat(value).into()
        }
        unsafe extern "system" fn GetData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISmsBinaryMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetData() {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    ::core::ptr::write(result__, ok_data__);
                    ::core::ptr::write(result_size__, ok_data_len__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISmsBinaryMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u8) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetData(::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as usize)).into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, ISmsBinaryMessage, OFFSET>(),
            Format: Format::<Identity, Impl, OFFSET>,
            SetFormat: SetFormat::<Identity, Impl, OFFSET>,
            GetData: GetData::<Identity, Impl, OFFSET>,
            SetData: SetData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISmsBinaryMessage as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Devices_Sms\"`, `\"Foundation\"`, `\"deprecated\"`, `\"implement\"`*"]
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
pub trait ISmsDevice_Impl: Sized {
    fn SendMessageAsync(&self, message: ::core::option::Option<&ISmsMessage>) -> ::windows_core::Result<SendSmsMessageOperation>;
    fn CalculateLength(&self, message: ::core::option::Option<&SmsTextMessage>) -> ::windows_core::Result<SmsEncodedLength>;
    fn AccountPhoneNumber(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn CellularClass(&self) -> ::windows_core::Result<CellularClass>;
    fn MessageStore(&self) -> ::windows_core::Result<SmsDeviceMessageStore>;
    fn DeviceStatus(&self) -> ::windows_core::Result<SmsDeviceStatus>;
    fn SmsMessageReceived(&self, eventhandler: ::core::option::Option<&SmsMessageReceivedEventHandler>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSmsMessageReceived(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn SmsDeviceStatusChanged(&self, eventhandler: ::core::option::Option<&SmsDeviceStatusChangedEventHandler>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSmsDeviceStatusChanged(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::windows_core::RuntimeName for ISmsDevice {
    const NAME: &'static str = "Windows.Devices.Sms.ISmsDevice";
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ISmsDevice_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISmsDevice_Impl, const OFFSET: isize>() -> ISmsDevice_Vtbl {
        unsafe extern "system" fn SendMessageAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISmsDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SendMessageAsync(::windows_core::from_raw_borrowed(&message)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CalculateLength<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISmsDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: *mut ::core::ffi::c_void, result__: *mut SmsEncodedLength) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CalculateLength(::windows_core::from_raw_borrowed(&message)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AccountPhoneNumber<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISmsDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AccountPhoneNumber() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CellularClass<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISmsDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CellularClass) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CellularClass() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MessageStore<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISmsDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MessageStore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISmsDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SmsDeviceStatus) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DeviceStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SmsMessageReceived<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISmsDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SmsMessageReceived(::windows_core::from_raw_borrowed(&eventhandler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSmsMessageReceived<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISmsDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveSmsMessageReceived(::core::mem::transmute(&eventcookie)).into()
        }
        unsafe extern "system" fn SmsDeviceStatusChanged<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISmsDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SmsDeviceStatusChanged(::windows_core::from_raw_borrowed(&eventhandler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSmsDeviceStatusChanged<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISmsDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveSmsDeviceStatusChanged(::core::mem::transmute(&eventcookie)).into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, ISmsDevice, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISmsDevice as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Devices_Sms\"`, `\"implement\"`*"]
pub trait ISmsMessage_Impl: Sized {
    fn Id(&self) -> ::windows_core::Result<u32>;
    fn MessageClass(&self) -> ::windows_core::Result<SmsMessageClass>;
}
impl ::windows_core::RuntimeName for ISmsMessage {
    const NAME: &'static str = "Windows.Devices.Sms.ISmsMessage";
}
impl ISmsMessage_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISmsMessage_Impl, const OFFSET: isize>() -> ISmsMessage_Vtbl {
        unsafe extern "system" fn Id<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISmsMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Id() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MessageClass<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISmsMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SmsMessageClass) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MessageClass() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, ISmsMessage, OFFSET>(),
            Id: Id::<Identity, Impl, OFFSET>,
            MessageClass: MessageClass::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISmsMessage as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Devices_Sms\"`, `\"implement\"`*"]
pub trait ISmsMessageBase_Impl: Sized {
    fn MessageType(&self) -> ::windows_core::Result<SmsMessageType>;
    fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn CellularClass(&self) -> ::windows_core::Result<CellularClass>;
    fn MessageClass(&self) -> ::windows_core::Result<SmsMessageClass>;
    fn SimIccId(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
}
impl ::windows_core::RuntimeName for ISmsMessageBase {
    const NAME: &'static str = "Windows.Devices.Sms.ISmsMessageBase";
}
impl ISmsMessageBase_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISmsMessageBase_Impl, const OFFSET: isize>() -> ISmsMessageBase_Vtbl {
        unsafe extern "system" fn MessageType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISmsMessageBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SmsMessageType) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MessageType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISmsMessageBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CellularClass<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISmsMessageBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CellularClass) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CellularClass() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MessageClass<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISmsMessageBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SmsMessageClass) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MessageClass() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SimIccId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISmsMessageBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SimIccId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, ISmsMessageBase, OFFSET>(),
            MessageType: MessageType::<Identity, Impl, OFFSET>,
            DeviceId: DeviceId::<Identity, Impl, OFFSET>,
            CellularClass: CellularClass::<Identity, Impl, OFFSET>,
            MessageClass: MessageClass::<Identity, Impl, OFFSET>,
            SimIccId: SimIccId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISmsMessageBase as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Devices_Sms\"`, `\"Foundation_Collections\"`, `\"deprecated\"`, `\"implement\"`*"]
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
pub trait ISmsTextMessage_Impl: Sized + ISmsMessage_Impl {
    fn Timestamp(&self) -> ::windows_core::Result<super::super::Foundation::DateTime>;
    fn PartReferenceId(&self) -> ::windows_core::Result<u32>;
    fn PartNumber(&self) -> ::windows_core::Result<u32>;
    fn PartCount(&self) -> ::windows_core::Result<u32>;
    fn To(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn SetTo(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn From(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn SetFrom(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn Body(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn SetBody(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn Encoding(&self) -> ::windows_core::Result<SmsEncoding>;
    fn SetEncoding(&self, value: SmsEncoding) -> ::windows_core::Result<()>;
    fn ToBinaryMessages(&self, format: SmsDataFormat) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<ISmsBinaryMessage>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
impl ::windows_core::RuntimeName for ISmsTextMessage {
    const NAME: &'static str = "Windows.Devices.Sms.ISmsTextMessage";
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
impl ISmsTextMessage_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISmsTextMessage_Impl, const OFFSET: isize>() -> ISmsTextMessage_Vtbl {
        unsafe extern "system" fn Timestamp<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISmsTextMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PartReferenceId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISmsTextMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PartReferenceId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PartNumber<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISmsTextMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PartNumber() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PartCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISmsTextMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PartCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn To<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISmsTextMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.To() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISmsTextMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTo(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn From<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISmsTextMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.From() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFrom<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISmsTextMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFrom(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Body<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISmsTextMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Body() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBody<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISmsTextMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBody(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Encoding<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISmsTextMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SmsEncoding) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Encoding() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEncoding<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISmsTextMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: SmsEncoding) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEncoding(value).into()
        }
        unsafe extern "system" fn ToBinaryMessages<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISmsTextMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: SmsDataFormat, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ToBinaryMessages(format) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, ISmsTextMessage, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISmsTextMessage as ::windows_core::ComInterface>::IID
    }
}
