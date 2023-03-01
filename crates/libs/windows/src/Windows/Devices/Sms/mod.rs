#[doc(hidden)]
#[repr(transparent)]
pub struct ISmsAppMessage(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISmsAppMessage {
    type Vtable = ISmsAppMessage_Vtbl;
}
impl ::core::clone::Clone for ISmsAppMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISmsAppMessage {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8bb8494_d3a0_4a0a_86d7_291033a8cf54);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsAppMessage_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub To: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub From: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Body: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetBody: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub CallbackNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetCallbackNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IsDeliveryNotificationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsDeliveryNotificationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub RetryAttemptCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetRetryAttemptCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub Encoding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmsEncoding) -> ::windows::core::HRESULT,
    pub SetEncoding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SmsEncoding) -> ::windows::core::HRESULT,
    pub PortNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetPortNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub TeleserviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetTeleserviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub ProtocolId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetProtocolId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub BinaryBody: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    BinaryBody: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetBinaryBody: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetBinaryBody: usize,
}
#[doc = "*Required features: `\"Devices_Sms\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISmsBinaryMessage(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl ISmsBinaryMessage {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Format(&self) -> ::windows::core::Result<SmsDataFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsDataFormat>();
            (::windows::core::Interface::vtable(this).Format)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetFormat(&self, value: SmsDataFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetFormat)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn GetData(&self) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetData)(::windows::core::Interface::as_raw(this), ::windows::core::Array::<u8>::set_abi_len(::std::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetData(&self, value: &[u8]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetData)(::windows::core::Interface::as_raw(this), value.len() as u32, value.as_ptr()).ok() }
    }
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<ISmsMessage>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MessageClass(&self) -> ::windows::core::Result<SmsMessageClass> {
        let this = &::windows::core::ComInterface::cast::<ISmsMessage>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsMessageClass>();
            (::windows::core::Interface::vtable(this).MessageClass)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
::windows::imp::interface_hierarchy!(ISmsBinaryMessage, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
impl windows::core::CanTryInto<ISmsMessage> for ISmsBinaryMessage {}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for ISmsBinaryMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for ISmsBinaryMessage {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for ISmsBinaryMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISmsBinaryMessage").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeType for ISmsBinaryMessage {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{5bf4e813-3b53-4c6e-b61a-d86a63755650}");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ISmsBinaryMessage {
    type Vtable = ISmsBinaryMessage_Vtbl;
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for ISmsBinaryMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::ComInterface for ISmsBinaryMessage {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5bf4e813_3b53_4c6e_b61a_d86a63755650);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISmsBinaryMessage_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub Format: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmsDataFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Format: usize,
    #[cfg(feature = "deprecated")]
    pub SetFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SmsDataFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetFormat: usize,
    #[cfg(feature = "deprecated")]
    pub GetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    GetData: usize,
    #[cfg(feature = "deprecated")]
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u8) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetData: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmsBroadcastMessage(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISmsBroadcastMessage {
    type Vtable = ISmsBroadcastMessage_Vtbl;
}
impl ::core::clone::Clone for ISmsBroadcastMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISmsBroadcastMessage {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75aebbf1_e4b7_4874_a09c_2956e592f957);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsBroadcastMessage_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub To: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Body: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Channel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub GeographicalScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmsGeographicalScope) -> ::windows::core::HRESULT,
    pub MessageCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub UpdateNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub BroadcastType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmsBroadcastType) -> ::windows::core::HRESULT,
    pub IsEmergencyAlert: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsUserPopupRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Devices_Sms\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISmsDevice(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl ISmsDevice {
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SendMessageAsync<P0>(&self, message: P0) -> ::windows::core::Result<SendSmsMessageOperation>
    where
        P0: ::windows::core::TryIntoParam<ISmsMessage>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SendSmsMessageOperation>();
            (::windows::core::Interface::vtable(this).SendMessageAsync)(::windows::core::Interface::as_raw(this), message.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn CalculateLength(&self, message: &SmsTextMessage) -> ::windows::core::Result<SmsEncodedLength> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsEncodedLength>();
            (::windows::core::Interface::vtable(this).CalculateLength)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(message), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn AccountPhoneNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).AccountPhoneNumber)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn CellularClass(&self) -> ::windows::core::Result<CellularClass> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CellularClass>();
            (::windows::core::Interface::vtable(this).CellularClass)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn MessageStore(&self) -> ::windows::core::Result<SmsDeviceMessageStore> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsDeviceMessageStore>();
            (::windows::core::Interface::vtable(this).MessageStore)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn DeviceStatus(&self) -> ::windows::core::Result<SmsDeviceStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsDeviceStatus>();
            (::windows::core::Interface::vtable(this).DeviceStatus)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SmsMessageReceived(&self, eventhandler: &SmsMessageReceivedEventHandler) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).SmsMessageReceived)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(eventhandler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveSmsMessageReceived(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveSmsMessageReceived)(::windows::core::Interface::as_raw(this), eventcookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SmsDeviceStatusChanged(&self, eventhandler: &SmsDeviceStatusChangedEventHandler) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).SmsDeviceStatusChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(eventhandler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveSmsDeviceStatusChanged(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveSmsDeviceStatusChanged)(::windows::core::Interface::as_raw(this), eventcookie).ok() }
    }
}
#[cfg(feature = "deprecated")]
::windows::imp::interface_hierarchy!(ISmsDevice, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for ISmsDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for ISmsDevice {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for ISmsDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISmsDevice").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeType for ISmsDevice {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{091791ed-872b-4eec-9c72-ab11627b34ec}");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ISmsDevice {
    type Vtable = ISmsDevice_Vtbl;
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for ISmsDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::ComInterface for ISmsDevice {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x091791ed_872b_4eec_9c72_ab11627b34ec);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISmsDevice_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SendMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SendMessageAsync: usize,
    #[cfg(feature = "deprecated")]
    pub CalculateLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: *mut ::core::ffi::c_void, result__: *mut SmsEncodedLength) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CalculateLength: usize,
    #[cfg(feature = "deprecated")]
    pub AccountPhoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AccountPhoneNumber: usize,
    #[cfg(feature = "deprecated")]
    pub CellularClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CellularClass) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CellularClass: usize,
    #[cfg(feature = "deprecated")]
    pub MessageStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    MessageStore: usize,
    #[cfg(feature = "deprecated")]
    pub DeviceStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmsDeviceStatus) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DeviceStatus: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SmsMessageReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SmsMessageReceived: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveSmsMessageReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveSmsMessageReceived: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SmsDeviceStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SmsDeviceStatusChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveSmsDeviceStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveSmsDeviceStatusChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmsDevice2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISmsDevice2 {
    type Vtable = ISmsDevice2_Vtbl;
}
impl ::core::clone::Clone for ISmsDevice2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISmsDevice2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbd8a5c13_e522_46cb_b8d5_9ead30fb6c47);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsDevice2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SmscAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetSmscAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ParentDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AccountPhoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub CellularClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CellularClass) -> ::windows::core::HRESULT,
    pub DeviceStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmsDeviceStatus) -> ::windows::core::HRESULT,
    pub CalculateLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: *mut ::core::ffi::c_void, result__: *mut SmsEncodedLength) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SendMessageAndGetResultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SendMessageAndGetResultAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeviceStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeviceStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDeviceStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDeviceStatusChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmsDevice2Statics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISmsDevice2Statics {
    type Vtable = ISmsDevice2Statics_Vtbl;
}
impl ::core::clone::Clone for ISmsDevice2Statics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISmsDevice2Statics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x65c78325_1031_491e_8fb6_ef9991afe363);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsDevice2Statics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub FromId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FromParentId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parentdeviceid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISmsDeviceMessageStore(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ISmsDeviceMessageStore {
    type Vtable = ISmsDeviceMessageStore_Vtbl;
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for ISmsDeviceMessageStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::ComInterface for ISmsDeviceMessageStore {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9889f253_f188_4427_8d54_ce0c2423c5c1);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISmsDeviceMessageStore_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub DeleteMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messageid: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    DeleteMessageAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub DeleteMessagesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messagefilter: SmsMessageFilter, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    DeleteMessagesAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub GetMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messageid: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    GetMessageAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub GetMessagesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messagefilter: SmsMessageFilter, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    GetMessagesAsync: usize,
    #[cfg(feature = "deprecated")]
    pub MaxMessages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    MaxMessages: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISmsDeviceStatics(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ISmsDeviceStatics {
    type Vtable = ISmsDeviceStatics_Vtbl;
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for ISmsDeviceStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::ComInterface for ISmsDeviceStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf88d07ea_d815_4dd1_a234_4520ce4604a4);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISmsDeviceStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    GetDeviceSelector: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    FromIdAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    GetDefaultAsync: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISmsDeviceStatics2(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ISmsDeviceStatics2 {
    type Vtable = ISmsDeviceStatics2_Vtbl;
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for ISmsDeviceStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::ComInterface for ISmsDeviceStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ca11c87_0873_4caf_8a7d_bd471e8586d1);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISmsDeviceStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub FromNetworkAccountIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, networkaccountid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    FromNetworkAccountIdAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmsFilterRule(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISmsFilterRule {
    type Vtable = ISmsFilterRule_Vtbl;
}
impl ::core::clone::Clone for ISmsFilterRule {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISmsFilterRule {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x40e32fae_b049_4fbc_afe9_e2a610eff55c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsFilterRule_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub MessageType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmsMessageType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ImsiPrefixes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ImsiPrefixes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub DeviceIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DeviceIds: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SenderNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SenderNumbers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub TextMessagePrefixes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TextMessagePrefixes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub PortNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PortNumbers: usize,
    pub CellularClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CellularClass) -> ::windows::core::HRESULT,
    pub SetCellularClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: CellularClass) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ProtocolIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ProtocolIds: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub TeleserviceIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TeleserviceIds: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub WapApplicationIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    WapApplicationIds: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub WapContentTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    WapContentTypes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub BroadcastTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    BroadcastTypes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub BroadcastChannels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    BroadcastChannels: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmsFilterRuleFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISmsFilterRuleFactory {
    type Vtable = ISmsFilterRuleFactory_Vtbl;
}
impl ::core::clone::Clone for ISmsFilterRuleFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISmsFilterRuleFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00c36508_6296_4f29_9aad_8920ceba3ce8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsFilterRuleFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateFilterRule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messagetype: SmsMessageType, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmsFilterRules(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISmsFilterRules {
    type Vtable = ISmsFilterRules_Vtbl;
}
impl ::core::clone::Clone for ISmsFilterRules {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISmsFilterRules {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e47eafb_79cd_4881_9894_55a4135b23fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsFilterRules_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ActionType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmsFilterActionType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Rules: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Rules: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmsFilterRulesFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISmsFilterRulesFactory {
    type Vtable = ISmsFilterRulesFactory_Vtbl;
}
impl ::core::clone::Clone for ISmsFilterRulesFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISmsFilterRulesFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa09924ed_6e2e_4530_9fde_465d02eed00e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsFilterRulesFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateFilterRules: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, actiontype: SmsFilterActionType, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Devices_Sms\"`*"]
#[repr(transparent)]
pub struct ISmsMessage(::windows::core::IUnknown);
impl ISmsMessage {
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MessageClass(&self) -> ::windows::core::Result<SmsMessageClass> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsMessageClass>();
            (::windows::core::Interface::vtable(this).MessageClass)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(ISmsMessage, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::cmp::PartialEq for ISmsMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISmsMessage {}
impl ::core::fmt::Debug for ISmsMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISmsMessage").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ISmsMessage {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{ed3c5e28-6984-4b07-811d-8d5906ed3cea}");
}
unsafe impl ::windows::core::Interface for ISmsMessage {
    type Vtable = ISmsMessage_Vtbl;
}
impl ::core::clone::Clone for ISmsMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISmsMessage {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xed3c5e28_6984_4b07_811d_8d5906ed3cea);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsMessage_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub MessageClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmsMessageClass) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Devices_Sms\"`*"]
#[repr(transparent)]
pub struct ISmsMessageBase(::windows::core::IUnknown);
impl ISmsMessageBase {
    pub fn MessageType(&self) -> ::windows::core::Result<SmsMessageType> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsMessageType>();
            (::windows::core::Interface::vtable(this).MessageType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DeviceId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CellularClass(&self) -> ::windows::core::Result<CellularClass> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CellularClass>();
            (::windows::core::Interface::vtable(this).CellularClass)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MessageClass(&self) -> ::windows::core::Result<SmsMessageClass> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsMessageClass>();
            (::windows::core::Interface::vtable(this).MessageClass)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SimIccId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).SimIccId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(ISmsMessageBase, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::cmp::PartialEq for ISmsMessageBase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISmsMessageBase {}
impl ::core::fmt::Debug for ISmsMessageBase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISmsMessageBase").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ISmsMessageBase {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{2cf0fe30-fe50-4fc6-aa88-4ccfe27a29ea}");
}
unsafe impl ::windows::core::Interface for ISmsMessageBase {
    type Vtable = ISmsMessageBase_Vtbl;
}
impl ::core::clone::Clone for ISmsMessageBase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISmsMessageBase {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2cf0fe30_fe50_4fc6_aa88_4ccfe27a29ea);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsMessageBase_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub MessageType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmsMessageType) -> ::windows::core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub CellularClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CellularClass) -> ::windows::core::HRESULT,
    pub MessageClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmsMessageClass) -> ::windows::core::HRESULT,
    pub SimIccId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISmsMessageReceivedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ISmsMessageReceivedEventArgs {
    type Vtable = ISmsMessageReceivedEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for ISmsMessageReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::ComInterface for ISmsMessageReceivedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08e80a98_b8e5_41c1_a3d8_d3abfae22675);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISmsMessageReceivedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub TextMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TextMessage: usize,
    #[cfg(feature = "deprecated")]
    pub BinaryMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    BinaryMessage: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmsMessageReceivedTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISmsMessageReceivedTriggerDetails {
    type Vtable = ISmsMessageReceivedTriggerDetails_Vtbl;
}
impl ::core::clone::Clone for ISmsMessageReceivedTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISmsMessageReceivedTriggerDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2bcfcbd4_2657_4128_ad5f_e3877132bdb1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsMessageReceivedTriggerDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub MessageType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmsMessageType) -> ::windows::core::HRESULT,
    pub TextMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub WapMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AppMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub BroadcastMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub VoicemailMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub StatusMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Drop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Accept: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmsMessageRegistration(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISmsMessageRegistration {
    type Vtable = ISmsMessageRegistration_Vtbl;
}
impl ::core::clone::Clone for ISmsMessageRegistration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISmsMessageRegistration {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1720503e_f34f_446b_83b3_0ff19923b409);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsMessageRegistration_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Unregister: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MessageReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MessageReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMessageReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMessageReceived: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmsMessageRegistrationStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISmsMessageRegistrationStatics {
    type Vtable = ISmsMessageRegistrationStatics_Vtbl;
}
impl ::core::clone::Clone for ISmsMessageRegistrationStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISmsMessageRegistrationStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63a05464_2898_4778_a03c_6f994907d63a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsMessageRegistrationStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub AllRegistrations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AllRegistrations: usize,
    pub Register: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::std::mem::MaybeUninit<::windows::core::HSTRING>, filterrules: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISmsReceivedEventDetails(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ISmsReceivedEventDetails {
    type Vtable = ISmsReceivedEventDetails_Vtbl;
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for ISmsReceivedEventDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::ComInterface for ISmsReceivedEventDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5bb50f15_e46d_4c82_847d_5a0304c1d53d);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISmsReceivedEventDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DeviceId: usize,
    #[cfg(feature = "deprecated")]
    pub MessageIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    MessageIndex: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISmsReceivedEventDetails2(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ISmsReceivedEventDetails2 {
    type Vtable = ISmsReceivedEventDetails2_Vtbl;
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for ISmsReceivedEventDetails2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::ComInterface for ISmsReceivedEventDetails2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x40e05c86_a7b4_4771_9ae7_0b5ffb12c03a);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISmsReceivedEventDetails2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub MessageClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmsMessageClass) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    MessageClass: usize,
    #[cfg(feature = "deprecated")]
    pub BinaryMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    BinaryMessage: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmsSendMessageResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISmsSendMessageResult {
    type Vtable = ISmsSendMessageResult_Vtbl;
}
impl ::core::clone::Clone for ISmsSendMessageResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISmsSendMessageResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb139af2_78c9_4feb_9622_452328088d62);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsSendMessageResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsSuccessful: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub MessageReferenceNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MessageReferenceNumbers: usize,
    pub CellularClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CellularClass) -> ::windows::core::HRESULT,
    pub ModemErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmsModemErrorCode) -> ::windows::core::HRESULT,
    pub IsErrorTransient: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub NetworkCauseCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub TransportFailureCause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmsStatusMessage(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISmsStatusMessage {
    type Vtable = ISmsStatusMessage_Vtbl;
}
impl ::core::clone::Clone for ISmsStatusMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISmsStatusMessage {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe6d28342_b70b_4677_9379_c9783fdff8f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsStatusMessage_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub To: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub From: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Body: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub MessageReferenceNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ServiceCenterTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ServiceCenterTimestamp: usize,
    #[cfg(feature = "Foundation")]
    pub DischargeTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DischargeTime: usize,
}
#[doc = "*Required features: `\"Devices_Sms\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISmsTextMessage(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl ISmsTextMessage {
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::DateTime>();
            (::windows::core::Interface::vtable(this).Timestamp)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn PartReferenceId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).PartReferenceId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn PartNumber(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).PartNumber)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn PartCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).PartCount)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn To(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).To)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetTo(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTo)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn From(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).From)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetFrom(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetFrom)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Body(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Body)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetBody(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBody)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Encoding(&self) -> ::windows::core::Result<SmsEncoding> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsEncoding>();
            (::windows::core::Interface::vtable(this).Encoding)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetEncoding(&self, value: SmsEncoding) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetEncoding)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn ToBinaryMessages(&self, format: SmsDataFormat) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ISmsBinaryMessage>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<ISmsBinaryMessage>>();
            (::windows::core::Interface::vtable(this).ToBinaryMessages)(::windows::core::Interface::as_raw(this), format, &mut result__).from_abi(result__)
        }
    }
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<ISmsMessage>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MessageClass(&self) -> ::windows::core::Result<SmsMessageClass> {
        let this = &::windows::core::ComInterface::cast::<ISmsMessage>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsMessageClass>();
            (::windows::core::Interface::vtable(this).MessageClass)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
::windows::imp::interface_hierarchy!(ISmsTextMessage, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
impl windows::core::CanTryInto<ISmsMessage> for ISmsTextMessage {}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for ISmsTextMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for ISmsTextMessage {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for ISmsTextMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISmsTextMessage").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeType for ISmsTextMessage {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{d61c904c-a495-487f-9a6f-971548c5bc9f}");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ISmsTextMessage {
    type Vtable = ISmsTextMessage_Vtbl;
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for ISmsTextMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::ComInterface for ISmsTextMessage {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd61c904c_a495_487f_9a6f_971548c5bc9f);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISmsTextMessage_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    Timestamp: usize,
    #[cfg(feature = "deprecated")]
    pub PartReferenceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PartReferenceId: usize,
    #[cfg(feature = "deprecated")]
    pub PartNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PartNumber: usize,
    #[cfg(feature = "deprecated")]
    pub PartCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PartCount: usize,
    #[cfg(feature = "deprecated")]
    pub To: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    To: usize,
    #[cfg(feature = "deprecated")]
    pub SetTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetTo: usize,
    #[cfg(feature = "deprecated")]
    pub From: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    From: usize,
    #[cfg(feature = "deprecated")]
    pub SetFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetFrom: usize,
    #[cfg(feature = "deprecated")]
    pub Body: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Body: usize,
    #[cfg(feature = "deprecated")]
    pub SetBody: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetBody: usize,
    #[cfg(feature = "deprecated")]
    pub Encoding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmsEncoding) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Encoding: usize,
    #[cfg(feature = "deprecated")]
    pub SetEncoding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SmsEncoding) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetEncoding: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub ToBinaryMessages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: SmsDataFormat, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    ToBinaryMessages: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmsTextMessage2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISmsTextMessage2 {
    type Vtable = ISmsTextMessage2_Vtbl;
}
impl ::core::clone::Clone for ISmsTextMessage2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISmsTextMessage2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x22a0d893_4555_4755_b5a1_e7fd84955f8d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsTextMessage2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub To: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub From: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Body: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetBody: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Encoding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SmsEncoding) -> ::windows::core::HRESULT,
    pub SetEncoding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SmsEncoding) -> ::windows::core::HRESULT,
    pub CallbackNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetCallbackNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IsDeliveryNotificationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsDeliveryNotificationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub RetryAttemptCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetRetryAttemptCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub TeleserviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub ProtocolId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISmsTextMessageStatics(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ISmsTextMessageStatics {
    type Vtable = ISmsTextMessageStatics_Vtbl;
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for ISmsTextMessageStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::ComInterface for ISmsTextMessageStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f68c5ed_3ccc_47a3_8c55_380d3b010892);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISmsTextMessageStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub FromBinaryMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, binarymessage: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    FromBinaryMessage: usize,
    #[cfg(feature = "deprecated")]
    pub FromBinaryData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: SmsDataFormat, value_array_size: u32, value: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    FromBinaryData: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmsVoicemailMessage(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISmsVoicemailMessage {
    type Vtable = ISmsVoicemailMessage_Vtbl;
}
impl ::core::clone::Clone for ISmsVoicemailMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISmsVoicemailMessage {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x271aa0a6_95b1_44ff_bcb8_b8fdd7e08bc3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsVoicemailMessage_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub To: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Body: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MessageCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MessageCount: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmsWapMessage(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISmsWapMessage {
    type Vtable = ISmsWapMessage_Vtbl;
}
impl ::core::clone::Clone for ISmsWapMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISmsWapMessage {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd937743_7a55_4d3b_9021_f22e022d09c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsWapMessage_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub To: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub From: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ApplicationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub BinaryBody: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    BinaryBody: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Headers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Headers: usize,
}
#[doc = "*Required features: `\"Devices_Sms\"`, `\"Foundation\"`, `\"deprecated\"`*"]
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
#[repr(transparent)]
pub struct DeleteSmsMessageOperation(::windows::core::IUnknown);
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl DeleteSmsMessageOperation {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetCompleted(&self, handler: &super::super::Foundation::AsyncActionCompletedHandler) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCompleted)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Completed(&self) -> ::windows::core::Result<super::super::Foundation::AsyncActionCompletedHandler> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::AsyncActionCompletedHandler>();
            (::windows::core::Interface::vtable(this).Completed)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetResults(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).GetResults)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Status(&self) -> ::windows::core::Result<super::super::Foundation::AsyncStatus> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::AsyncStatus>();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HRESULT>();
            (::windows::core::Interface::vtable(this).ErrorCode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Cancel(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Cancel)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::core::cmp::PartialEq for DeleteSmsMessageOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::core::cmp::Eq for DeleteSmsMessageOperation {}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::core::fmt::Debug for DeleteSmsMessageOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeleteSmsMessageOperation").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::windows::core::RuntimeType for DeleteSmsMessageOperation {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.DeleteSmsMessageOperation;{5a648006-843a-4da9-865b-9d26e5dfad7b})");
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::core::clone::Clone for DeleteSmsMessageOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
unsafe impl ::windows::core::Interface for DeleteSmsMessageOperation {
    type Vtable = super::super::Foundation::IAsyncAction_Vtbl;
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
unsafe impl ::windows::core::ComInterface for DeleteSmsMessageOperation {
    const IID: ::windows::core::GUID = <super::super::Foundation::IAsyncAction as ::windows::core::ComInterface>::IID;
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::windows::core::RuntimeName for DeleteSmsMessageOperation {
    const NAME: &'static str = "Windows.Devices.Sms.DeleteSmsMessageOperation";
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl DeleteSmsMessageOperation {
    pub fn get(&self) -> ::windows::core::Result<()> {
        if self.Status()? == super::super::Foundation::AsyncStatus::Started {
            let (_waiter, signaler) = ::windows::imp::Waiter::new()?;
            self.SetCompleted(&super::super::Foundation::AsyncActionCompletedHandler::new(move |_sender, _args| {
                unsafe {
                    signaler.signal();
                }
                Ok(())
            }))?;
        }
        self.GetResults()
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::std::future::Future for DeleteSmsMessageOperation {
    type Output = ::windows::core::Result<()>;
    fn poll(self: ::std::pin::Pin<&mut Self>, context: &mut ::std::task::Context) -> ::std::task::Poll<Self::Output> {
        if self.Status()? == super::super::Foundation::AsyncStatus::Started {
            let waker = context.waker().clone();
            let _ = self.SetCompleted(&super::super::Foundation::AsyncActionCompletedHandler::new(move |_sender, _args| {
                waker.wake_by_ref();
                Ok(())
            }));
            ::std::task::Poll::Pending
        } else {
            ::std::task::Poll::Ready(self.GetResults())
        }
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
::windows::imp::interface_hierarchy!(DeleteSmsMessageOperation, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::windows::core::CanTryInto<super::super::Foundation::IAsyncAction> for DeleteSmsMessageOperation {}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::windows::core::CanTryInto<super::super::Foundation::IAsyncInfo> for DeleteSmsMessageOperation {}
#[doc = "*Required features: `\"Devices_Sms\"`, `\"Foundation\"`, `\"deprecated\"`*"]
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
#[repr(transparent)]
pub struct DeleteSmsMessagesOperation(::windows::core::IUnknown);
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl DeleteSmsMessagesOperation {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetCompleted(&self, handler: &super::super::Foundation::AsyncActionCompletedHandler) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCompleted)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Completed(&self) -> ::windows::core::Result<super::super::Foundation::AsyncActionCompletedHandler> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::AsyncActionCompletedHandler>();
            (::windows::core::Interface::vtable(this).Completed)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetResults(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).GetResults)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Status(&self) -> ::windows::core::Result<super::super::Foundation::AsyncStatus> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::AsyncStatus>();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HRESULT>();
            (::windows::core::Interface::vtable(this).ErrorCode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Cancel(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Cancel)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::core::cmp::PartialEq for DeleteSmsMessagesOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::core::cmp::Eq for DeleteSmsMessagesOperation {}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::core::fmt::Debug for DeleteSmsMessagesOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeleteSmsMessagesOperation").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::windows::core::RuntimeType for DeleteSmsMessagesOperation {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.DeleteSmsMessagesOperation;{5a648006-843a-4da9-865b-9d26e5dfad7b})");
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::core::clone::Clone for DeleteSmsMessagesOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
unsafe impl ::windows::core::Interface for DeleteSmsMessagesOperation {
    type Vtable = super::super::Foundation::IAsyncAction_Vtbl;
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
unsafe impl ::windows::core::ComInterface for DeleteSmsMessagesOperation {
    const IID: ::windows::core::GUID = <super::super::Foundation::IAsyncAction as ::windows::core::ComInterface>::IID;
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::windows::core::RuntimeName for DeleteSmsMessagesOperation {
    const NAME: &'static str = "Windows.Devices.Sms.DeleteSmsMessagesOperation";
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl DeleteSmsMessagesOperation {
    pub fn get(&self) -> ::windows::core::Result<()> {
        if self.Status()? == super::super::Foundation::AsyncStatus::Started {
            let (_waiter, signaler) = ::windows::imp::Waiter::new()?;
            self.SetCompleted(&super::super::Foundation::AsyncActionCompletedHandler::new(move |_sender, _args| {
                unsafe {
                    signaler.signal();
                }
                Ok(())
            }))?;
        }
        self.GetResults()
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::std::future::Future for DeleteSmsMessagesOperation {
    type Output = ::windows::core::Result<()>;
    fn poll(self: ::std::pin::Pin<&mut Self>, context: &mut ::std::task::Context) -> ::std::task::Poll<Self::Output> {
        if self.Status()? == super::super::Foundation::AsyncStatus::Started {
            let waker = context.waker().clone();
            let _ = self.SetCompleted(&super::super::Foundation::AsyncActionCompletedHandler::new(move |_sender, _args| {
                waker.wake_by_ref();
                Ok(())
            }));
            ::std::task::Poll::Pending
        } else {
            ::std::task::Poll::Ready(self.GetResults())
        }
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
::windows::imp::interface_hierarchy!(DeleteSmsMessagesOperation, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::windows::core::CanTryInto<super::super::Foundation::IAsyncAction> for DeleteSmsMessagesOperation {}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::windows::core::CanTryInto<super::super::Foundation::IAsyncInfo> for DeleteSmsMessagesOperation {}
#[doc = "*Required features: `\"Devices_Sms\"`, `\"Foundation\"`, `\"deprecated\"`*"]
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
#[repr(transparent)]
pub struct GetSmsDeviceOperation(::windows::core::IUnknown);
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl GetSmsDeviceOperation {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Status(&self) -> ::windows::core::Result<super::super::Foundation::AsyncStatus> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::AsyncStatus>();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HRESULT>();
            (::windows::core::Interface::vtable(this).ErrorCode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Cancel(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Cancel)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetCompleted(&self, handler: &super::super::Foundation::AsyncOperationCompletedHandler<SmsDevice>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCompleted)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Completed(&self) -> ::windows::core::Result<super::super::Foundation::AsyncOperationCompletedHandler<SmsDevice>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::AsyncOperationCompletedHandler<SmsDevice>>();
            (::windows::core::Interface::vtable(this).Completed)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetResults(&self) -> ::windows::core::Result<SmsDevice> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsDevice>();
            (::windows::core::Interface::vtable(this).GetResults)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::core::cmp::PartialEq for GetSmsDeviceOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::core::cmp::Eq for GetSmsDeviceOperation {}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::core::fmt::Debug for GetSmsDeviceOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GetSmsDeviceOperation").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::windows::core::RuntimeType for GetSmsDeviceOperation {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.GetSmsDeviceOperation;pinterface({9fc2b0bb-e446-44e2-aa61-9cab8f636af2};rc(Windows.Devices.Sms.SmsDevice;{091791ed-872b-4eec-9c72-ab11627b34ec})))");
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::core::clone::Clone for GetSmsDeviceOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
unsafe impl ::windows::core::Interface for GetSmsDeviceOperation {
    type Vtable = super::super::Foundation::IAsyncOperation_Vtbl<SmsDevice>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
unsafe impl ::windows::core::ComInterface for GetSmsDeviceOperation {
    const IID: ::windows::core::GUID = <super::super::Foundation::IAsyncOperation<SmsDevice> as ::windows::core::ComInterface>::IID;
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::windows::core::RuntimeName for GetSmsDeviceOperation {
    const NAME: &'static str = "Windows.Devices.Sms.GetSmsDeviceOperation";
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl GetSmsDeviceOperation {
    pub fn get(&self) -> ::windows::core::Result<SmsDevice> {
        if self.Status()? == super::super::Foundation::AsyncStatus::Started {
            let (_waiter, signaler) = ::windows::imp::Waiter::new()?;
            self.SetCompleted(&super::super::Foundation::AsyncOperationCompletedHandler::new(move |_sender, _args| {
                unsafe {
                    signaler.signal();
                }
                Ok(())
            }))?;
        }
        self.GetResults()
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::std::future::Future for GetSmsDeviceOperation {
    type Output = ::windows::core::Result<SmsDevice>;
    fn poll(self: ::std::pin::Pin<&mut Self>, context: &mut ::std::task::Context) -> ::std::task::Poll<Self::Output> {
        if self.Status()? == super::super::Foundation::AsyncStatus::Started {
            let waker = context.waker().clone();
            let _ = self.SetCompleted(&super::super::Foundation::AsyncOperationCompletedHandler::new(move |_sender, _args| {
                waker.wake_by_ref();
                Ok(())
            }));
            ::std::task::Poll::Pending
        } else {
            ::std::task::Poll::Ready(self.GetResults())
        }
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
::windows::imp::interface_hierarchy!(GetSmsDeviceOperation, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::windows::core::CanTryInto<super::super::Foundation::IAsyncInfo> for GetSmsDeviceOperation {}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::windows::core::CanTryInto<super::super::Foundation::IAsyncOperation<SmsDevice>> for GetSmsDeviceOperation {}
#[doc = "*Required features: `\"Devices_Sms\"`, `\"Foundation\"`, `\"deprecated\"`*"]
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
#[repr(transparent)]
pub struct GetSmsMessageOperation(::windows::core::IUnknown);
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl GetSmsMessageOperation {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Status(&self) -> ::windows::core::Result<super::super::Foundation::AsyncStatus> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::AsyncStatus>();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HRESULT>();
            (::windows::core::Interface::vtable(this).ErrorCode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Cancel(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Cancel)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetCompleted(&self, handler: &super::super::Foundation::AsyncOperationCompletedHandler<ISmsMessage>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCompleted)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Completed(&self) -> ::windows::core::Result<super::super::Foundation::AsyncOperationCompletedHandler<ISmsMessage>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::AsyncOperationCompletedHandler<ISmsMessage>>();
            (::windows::core::Interface::vtable(this).Completed)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetResults(&self) -> ::windows::core::Result<ISmsMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ISmsMessage>();
            (::windows::core::Interface::vtable(this).GetResults)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::core::cmp::PartialEq for GetSmsMessageOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::core::cmp::Eq for GetSmsMessageOperation {}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::core::fmt::Debug for GetSmsMessageOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GetSmsMessageOperation").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::windows::core::RuntimeType for GetSmsMessageOperation {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.GetSmsMessageOperation;pinterface({9fc2b0bb-e446-44e2-aa61-9cab8f636af2};{ed3c5e28-6984-4b07-811d-8d5906ed3cea}))");
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::core::clone::Clone for GetSmsMessageOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
unsafe impl ::windows::core::Interface for GetSmsMessageOperation {
    type Vtable = super::super::Foundation::IAsyncOperation_Vtbl<ISmsMessage>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
unsafe impl ::windows::core::ComInterface for GetSmsMessageOperation {
    const IID: ::windows::core::GUID = <super::super::Foundation::IAsyncOperation<ISmsMessage> as ::windows::core::ComInterface>::IID;
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::windows::core::RuntimeName for GetSmsMessageOperation {
    const NAME: &'static str = "Windows.Devices.Sms.GetSmsMessageOperation";
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl GetSmsMessageOperation {
    pub fn get(&self) -> ::windows::core::Result<ISmsMessage> {
        if self.Status()? == super::super::Foundation::AsyncStatus::Started {
            let (_waiter, signaler) = ::windows::imp::Waiter::new()?;
            self.SetCompleted(&super::super::Foundation::AsyncOperationCompletedHandler::new(move |_sender, _args| {
                unsafe {
                    signaler.signal();
                }
                Ok(())
            }))?;
        }
        self.GetResults()
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::std::future::Future for GetSmsMessageOperation {
    type Output = ::windows::core::Result<ISmsMessage>;
    fn poll(self: ::std::pin::Pin<&mut Self>, context: &mut ::std::task::Context) -> ::std::task::Poll<Self::Output> {
        if self.Status()? == super::super::Foundation::AsyncStatus::Started {
            let waker = context.waker().clone();
            let _ = self.SetCompleted(&super::super::Foundation::AsyncOperationCompletedHandler::new(move |_sender, _args| {
                waker.wake_by_ref();
                Ok(())
            }));
            ::std::task::Poll::Pending
        } else {
            ::std::task::Poll::Ready(self.GetResults())
        }
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
::windows::imp::interface_hierarchy!(GetSmsMessageOperation, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::windows::core::CanTryInto<super::super::Foundation::IAsyncInfo> for GetSmsMessageOperation {}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::windows::core::CanTryInto<super::super::Foundation::IAsyncOperation<ISmsMessage>> for GetSmsMessageOperation {}
#[doc = "*Required features: `\"Devices_Sms\"`, `\"Foundation_Collections\"`, `\"deprecated\"`*"]
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
#[repr(transparent)]
pub struct GetSmsMessagesOperation(::windows::core::IUnknown);
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
impl GetSmsMessagesOperation {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Status(&self) -> ::windows::core::Result<super::super::Foundation::AsyncStatus> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::AsyncStatus>();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HRESULT>();
            (::windows::core::Interface::vtable(this).ErrorCode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Cancel(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Cancel)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetProgress(&self, handler: &super::super::Foundation::AsyncOperationProgressHandler<super::super::Foundation::Collections::IVectorView<ISmsMessage>, i32>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetProgress)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Progress(&self) -> ::windows::core::Result<super::super::Foundation::AsyncOperationProgressHandler<super::super::Foundation::Collections::IVectorView<ISmsMessage>, i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::AsyncOperationProgressHandler<super::super::Foundation::Collections::IVectorView<ISmsMessage>, i32>>();
            (::windows::core::Interface::vtable(this).Progress)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetCompleted(&self, handler: &super::super::Foundation::AsyncOperationWithProgressCompletedHandler<super::super::Foundation::Collections::IVectorView<ISmsMessage>, i32>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCompleted)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Completed(&self) -> ::windows::core::Result<super::super::Foundation::AsyncOperationWithProgressCompletedHandler<super::super::Foundation::Collections::IVectorView<ISmsMessage>, i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::AsyncOperationWithProgressCompletedHandler<super::super::Foundation::Collections::IVectorView<ISmsMessage>, i32>>();
            (::windows::core::Interface::vtable(this).Completed)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetResults(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ISmsMessage>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<ISmsMessage>>();
            (::windows::core::Interface::vtable(this).GetResults)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
impl ::core::cmp::PartialEq for GetSmsMessagesOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
impl ::core::cmp::Eq for GetSmsMessagesOperation {}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
impl ::core::fmt::Debug for GetSmsMessagesOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GetSmsMessagesOperation").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
impl ::windows::core::RuntimeType for GetSmsMessagesOperation {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.GetSmsMessagesOperation;pinterface({b5d036d7-e297-498f-ba60-0289e76e23dd};pinterface({bbe1fa4c-b0e3-4583-baef-1f1b2e483e56};{ed3c5e28-6984-4b07-811d-8d5906ed3cea});i4))");
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
impl ::core::clone::Clone for GetSmsMessagesOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
unsafe impl ::windows::core::Interface for GetSmsMessagesOperation {
    type Vtable = super::super::Foundation::IAsyncOperationWithProgress_Vtbl<super::super::Foundation::Collections::IVectorView<ISmsMessage>, i32>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
unsafe impl ::windows::core::ComInterface for GetSmsMessagesOperation {
    const IID: ::windows::core::GUID = <super::super::Foundation::IAsyncOperationWithProgress<super::super::Foundation::Collections::IVectorView<ISmsMessage>, i32> as ::windows::core::ComInterface>::IID;
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
impl ::windows::core::RuntimeName for GetSmsMessagesOperation {
    const NAME: &'static str = "Windows.Devices.Sms.GetSmsMessagesOperation";
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
impl GetSmsMessagesOperation {
    pub fn get(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ISmsMessage>> {
        if self.Status()? == super::super::Foundation::AsyncStatus::Started {
            let (_waiter, signaler) = ::windows::imp::Waiter::new()?;
            self.SetCompleted(&super::super::Foundation::AsyncOperationWithProgressCompletedHandler::new(move |_sender, _args| {
                unsafe {
                    signaler.signal();
                }
                Ok(())
            }))?;
        }
        self.GetResults()
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
impl ::std::future::Future for GetSmsMessagesOperation {
    type Output = ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ISmsMessage>>;
    fn poll(self: ::std::pin::Pin<&mut Self>, context: &mut ::std::task::Context) -> ::std::task::Poll<Self::Output> {
        if self.Status()? == super::super::Foundation::AsyncStatus::Started {
            let waker = context.waker().clone();
            let _ = self.SetCompleted(&super::super::Foundation::AsyncOperationWithProgressCompletedHandler::new(move |_sender, _args| {
                waker.wake_by_ref();
                Ok(())
            }));
            ::std::task::Poll::Pending
        } else {
            ::std::task::Poll::Ready(self.GetResults())
        }
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
::windows::imp::interface_hierarchy!(GetSmsMessagesOperation, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
impl ::windows::core::CanTryInto<super::super::Foundation::IAsyncInfo> for GetSmsMessagesOperation {}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
impl ::windows::core::CanTryInto<super::super::Foundation::IAsyncOperationWithProgress<super::super::Foundation::Collections::IVectorView<ISmsMessage>, i32>> for GetSmsMessagesOperation {}
#[doc = "*Required features: `\"Devices_Sms\"`, `\"Foundation\"`, `\"deprecated\"`*"]
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
#[repr(transparent)]
pub struct SendSmsMessageOperation(::windows::core::IUnknown);
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl SendSmsMessageOperation {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetCompleted(&self, handler: &super::super::Foundation::AsyncActionCompletedHandler) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCompleted)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Completed(&self) -> ::windows::core::Result<super::super::Foundation::AsyncActionCompletedHandler> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::AsyncActionCompletedHandler>();
            (::windows::core::Interface::vtable(this).Completed)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetResults(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).GetResults)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Status(&self) -> ::windows::core::Result<super::super::Foundation::AsyncStatus> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::AsyncStatus>();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HRESULT>();
            (::windows::core::Interface::vtable(this).ErrorCode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Cancel(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Cancel)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::core::cmp::PartialEq for SendSmsMessageOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::core::cmp::Eq for SendSmsMessageOperation {}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::core::fmt::Debug for SendSmsMessageOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SendSmsMessageOperation").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::windows::core::RuntimeType for SendSmsMessageOperation {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.SendSmsMessageOperation;{5a648006-843a-4da9-865b-9d26e5dfad7b})");
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::core::clone::Clone for SendSmsMessageOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
unsafe impl ::windows::core::Interface for SendSmsMessageOperation {
    type Vtable = super::super::Foundation::IAsyncAction_Vtbl;
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
unsafe impl ::windows::core::ComInterface for SendSmsMessageOperation {
    const IID: ::windows::core::GUID = <super::super::Foundation::IAsyncAction as ::windows::core::ComInterface>::IID;
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::windows::core::RuntimeName for SendSmsMessageOperation {
    const NAME: &'static str = "Windows.Devices.Sms.SendSmsMessageOperation";
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl SendSmsMessageOperation {
    pub fn get(&self) -> ::windows::core::Result<()> {
        if self.Status()? == super::super::Foundation::AsyncStatus::Started {
            let (_waiter, signaler) = ::windows::imp::Waiter::new()?;
            self.SetCompleted(&super::super::Foundation::AsyncActionCompletedHandler::new(move |_sender, _args| {
                unsafe {
                    signaler.signal();
                }
                Ok(())
            }))?;
        }
        self.GetResults()
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::std::future::Future for SendSmsMessageOperation {
    type Output = ::windows::core::Result<()>;
    fn poll(self: ::std::pin::Pin<&mut Self>, context: &mut ::std::task::Context) -> ::std::task::Poll<Self::Output> {
        if self.Status()? == super::super::Foundation::AsyncStatus::Started {
            let waker = context.waker().clone();
            let _ = self.SetCompleted(&super::super::Foundation::AsyncActionCompletedHandler::new(move |_sender, _args| {
                waker.wake_by_ref();
                Ok(())
            }));
            ::std::task::Poll::Pending
        } else {
            ::std::task::Poll::Ready(self.GetResults())
        }
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
::windows::imp::interface_hierarchy!(SendSmsMessageOperation, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::windows::core::CanTryInto<super::super::Foundation::IAsyncAction> for SendSmsMessageOperation {}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::windows::core::CanTryInto<super::super::Foundation::IAsyncInfo> for SendSmsMessageOperation {}
#[doc = "*Required features: `\"Devices_Sms\"`*"]
#[repr(transparent)]
pub struct SmsAppMessage(::windows::core::IUnknown);
impl SmsAppMessage {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<SmsAppMessage, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::DateTime>();
            (::windows::core::Interface::vtable(this).Timestamp)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn To(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).To)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTo(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTo)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn From(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).From)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Body(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Body)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetBody(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBody)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn CallbackNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).CallbackNumber)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCallbackNumber(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCallbackNumber)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn IsDeliveryNotificationEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsDeliveryNotificationEnabled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsDeliveryNotificationEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsDeliveryNotificationEnabled)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn RetryAttemptCount(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).RetryAttemptCount)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRetryAttemptCount(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRetryAttemptCount)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Encoding(&self) -> ::windows::core::Result<SmsEncoding> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsEncoding>();
            (::windows::core::Interface::vtable(this).Encoding)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetEncoding(&self, value: SmsEncoding) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetEncoding)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn PortNumber(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).PortNumber)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPortNumber(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPortNumber)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn TeleserviceId(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).TeleserviceId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTeleserviceId(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTeleserviceId)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ProtocolId(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).ProtocolId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetProtocolId(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetProtocolId)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn BinaryBody(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::Streams::IBuffer>();
            (::windows::core::Interface::vtable(this).BinaryBody)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetBinaryBody<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBinaryBody)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn MessageType(&self) -> ::windows::core::Result<SmsMessageType> {
        let this = &::windows::core::ComInterface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsMessageType>();
            (::windows::core::Interface::vtable(this).MessageType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DeviceId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CellularClass(&self) -> ::windows::core::Result<CellularClass> {
        let this = &::windows::core::ComInterface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CellularClass>();
            (::windows::core::Interface::vtable(this).CellularClass)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MessageClass(&self) -> ::windows::core::Result<SmsMessageClass> {
        let this = &::windows::core::ComInterface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsMessageClass>();
            (::windows::core::Interface::vtable(this).MessageClass)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SimIccId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).SimIccId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for SmsAppMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmsAppMessage {}
impl ::core::fmt::Debug for SmsAppMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsAppMessage").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SmsAppMessage {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.SmsAppMessage;{e8bb8494-d3a0-4a0a-86d7-291033a8cf54})");
}
impl ::core::clone::Clone for SmsAppMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SmsAppMessage {
    type Vtable = ISmsAppMessage_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SmsAppMessage {
    const IID: ::windows::core::GUID = <ISmsAppMessage as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SmsAppMessage {
    const NAME: &'static str = "Windows.Devices.Sms.SmsAppMessage";
}
::windows::imp::interface_hierarchy!(SmsAppMessage, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<ISmsMessageBase> for SmsAppMessage {}
unsafe impl ::core::marker::Send for SmsAppMessage {}
unsafe impl ::core::marker::Sync for SmsAppMessage {}
#[doc = "*Required features: `\"Devices_Sms\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SmsBinaryMessage(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl SmsBinaryMessage {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<SmsBinaryMessage, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Format(&self) -> ::windows::core::Result<SmsDataFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsDataFormat>();
            (::windows::core::Interface::vtable(this).Format)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetFormat(&self, value: SmsDataFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetFormat)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn GetData(&self) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetData)(::windows::core::Interface::as_raw(this), ::windows::core::Array::<u8>::set_abi_len(::std::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetData(&self, value: &[u8]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetData)(::windows::core::Interface::as_raw(this), value.len() as u32, value.as_ptr()).ok() }
    }
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<ISmsMessage>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MessageClass(&self) -> ::windows::core::Result<SmsMessageClass> {
        let this = &::windows::core::ComInterface::cast::<ISmsMessage>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsMessageClass>();
            (::windows::core::Interface::vtable(this).MessageClass)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for SmsBinaryMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for SmsBinaryMessage {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SmsBinaryMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsBinaryMessage").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeType for SmsBinaryMessage {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.SmsBinaryMessage;{5bf4e813-3b53-4c6e-b61a-d86a63755650})");
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SmsBinaryMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for SmsBinaryMessage {
    type Vtable = ISmsBinaryMessage_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::ComInterface for SmsBinaryMessage {
    const IID: ::windows::core::GUID = <ISmsBinaryMessage as ::windows::core::ComInterface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for SmsBinaryMessage {
    const NAME: &'static str = "Windows.Devices.Sms.SmsBinaryMessage";
}
#[cfg(feature = "deprecated")]
::windows::imp::interface_hierarchy!(SmsBinaryMessage, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::windows::core::CanTryInto<ISmsBinaryMessage> for SmsBinaryMessage {}
#[cfg(feature = "deprecated")]
impl ::windows::core::CanTryInto<ISmsMessage> for SmsBinaryMessage {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for SmsBinaryMessage {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for SmsBinaryMessage {}
#[doc = "*Required features: `\"Devices_Sms\"`*"]
#[repr(transparent)]
pub struct SmsBroadcastMessage(::windows::core::IUnknown);
impl SmsBroadcastMessage {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::DateTime>();
            (::windows::core::Interface::vtable(this).Timestamp)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn To(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).To)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Body(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Body)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Channel(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).Channel)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GeographicalScope(&self) -> ::windows::core::Result<SmsGeographicalScope> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsGeographicalScope>();
            (::windows::core::Interface::vtable(this).GeographicalScope)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MessageCode(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).MessageCode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn UpdateNumber(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).UpdateNumber)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn BroadcastType(&self) -> ::windows::core::Result<SmsBroadcastType> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsBroadcastType>();
            (::windows::core::Interface::vtable(this).BroadcastType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsEmergencyAlert(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsEmergencyAlert)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsUserPopupRequested(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsUserPopupRequested)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MessageType(&self) -> ::windows::core::Result<SmsMessageType> {
        let this = &::windows::core::ComInterface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsMessageType>();
            (::windows::core::Interface::vtable(this).MessageType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DeviceId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CellularClass(&self) -> ::windows::core::Result<CellularClass> {
        let this = &::windows::core::ComInterface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CellularClass>();
            (::windows::core::Interface::vtable(this).CellularClass)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MessageClass(&self) -> ::windows::core::Result<SmsMessageClass> {
        let this = &::windows::core::ComInterface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsMessageClass>();
            (::windows::core::Interface::vtable(this).MessageClass)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SimIccId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).SimIccId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for SmsBroadcastMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmsBroadcastMessage {}
impl ::core::fmt::Debug for SmsBroadcastMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsBroadcastMessage").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SmsBroadcastMessage {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.SmsBroadcastMessage;{75aebbf1-e4b7-4874-a09c-2956e592f957})");
}
impl ::core::clone::Clone for SmsBroadcastMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SmsBroadcastMessage {
    type Vtable = ISmsBroadcastMessage_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SmsBroadcastMessage {
    const IID: ::windows::core::GUID = <ISmsBroadcastMessage as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SmsBroadcastMessage {
    const NAME: &'static str = "Windows.Devices.Sms.SmsBroadcastMessage";
}
::windows::imp::interface_hierarchy!(SmsBroadcastMessage, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<ISmsMessageBase> for SmsBroadcastMessage {}
unsafe impl ::core::marker::Send for SmsBroadcastMessage {}
unsafe impl ::core::marker::Sync for SmsBroadcastMessage {}
#[doc = "*Required features: `\"Devices_Sms\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SmsDevice(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl SmsDevice {
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SendMessageAsync<P0>(&self, message: P0) -> ::windows::core::Result<SendSmsMessageOperation>
    where
        P0: ::windows::core::TryIntoParam<ISmsMessage>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SendSmsMessageOperation>();
            (::windows::core::Interface::vtable(this).SendMessageAsync)(::windows::core::Interface::as_raw(this), message.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn CalculateLength(&self, message: &SmsTextMessage) -> ::windows::core::Result<SmsEncodedLength> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsEncodedLength>();
            (::windows::core::Interface::vtable(this).CalculateLength)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(message), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn AccountPhoneNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).AccountPhoneNumber)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn CellularClass(&self) -> ::windows::core::Result<CellularClass> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CellularClass>();
            (::windows::core::Interface::vtable(this).CellularClass)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn MessageStore(&self) -> ::windows::core::Result<SmsDeviceMessageStore> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsDeviceMessageStore>();
            (::windows::core::Interface::vtable(this).MessageStore)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn DeviceStatus(&self) -> ::windows::core::Result<SmsDeviceStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsDeviceStatus>();
            (::windows::core::Interface::vtable(this).DeviceStatus)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SmsMessageReceived(&self, eventhandler: &SmsMessageReceivedEventHandler) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).SmsMessageReceived)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(eventhandler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveSmsMessageReceived(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveSmsMessageReceived)(::windows::core::Interface::as_raw(this), eventcookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SmsDeviceStatusChanged(&self, eventhandler: &SmsDeviceStatusChangedEventHandler) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).SmsDeviceStatusChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(eventhandler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveSmsDeviceStatusChanged(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveSmsDeviceStatusChanged)(::windows::core::Interface::as_raw(this), eventcookie).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISmsDeviceStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetDeviceSelector)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn FromIdAsync(deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmsDevice>> {
        Self::ISmsDeviceStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<SmsDevice>>();
            (::windows::core::Interface::vtable(this).FromIdAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn GetDefaultAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmsDevice>> {
        Self::ISmsDeviceStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<SmsDevice>>();
            (::windows::core::Interface::vtable(this).GetDefaultAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn FromNetworkAccountIdAsync(networkaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmsDevice>> {
        Self::ISmsDeviceStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<SmsDevice>>();
            (::windows::core::Interface::vtable(this).FromNetworkAccountIdAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(networkaccountid), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn ISmsDeviceStatics<R, F: FnOnce(&ISmsDeviceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<SmsDevice, ISmsDeviceStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn ISmsDeviceStatics2<R, F: FnOnce(&ISmsDeviceStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<SmsDevice, ISmsDeviceStatics2> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for SmsDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for SmsDevice {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SmsDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsDevice").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeType for SmsDevice {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.SmsDevice;{091791ed-872b-4eec-9c72-ab11627b34ec})");
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SmsDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for SmsDevice {
    type Vtable = ISmsDevice_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::ComInterface for SmsDevice {
    const IID: ::windows::core::GUID = <ISmsDevice as ::windows::core::ComInterface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for SmsDevice {
    const NAME: &'static str = "Windows.Devices.Sms.SmsDevice";
}
#[cfg(feature = "deprecated")]
::windows::imp::interface_hierarchy!(SmsDevice, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::windows::core::CanTryInto<ISmsDevice> for SmsDevice {}
#[doc = "*Required features: `\"Devices_Sms\"`*"]
#[repr(transparent)]
pub struct SmsDevice2(::windows::core::IUnknown);
impl SmsDevice2 {
    pub fn SmscAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).SmscAddress)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSmscAddress(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSmscAddress)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DeviceId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ParentDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ParentDeviceId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AccountPhoneNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).AccountPhoneNumber)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CellularClass(&self) -> ::windows::core::Result<CellularClass> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CellularClass>();
            (::windows::core::Interface::vtable(this).CellularClass)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DeviceStatus(&self) -> ::windows::core::Result<SmsDeviceStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsDeviceStatus>();
            (::windows::core::Interface::vtable(this).DeviceStatus)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CalculateLength<P0>(&self, message: P0) -> ::windows::core::Result<SmsEncodedLength>
    where
        P0: ::windows::core::TryIntoParam<ISmsMessageBase>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsEncodedLength>();
            (::windows::core::Interface::vtable(this).CalculateLength)(::windows::core::Interface::as_raw(this), message.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SendMessageAndGetResultAsync<P0>(&self, message: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmsSendMessageResult>>
    where
        P0: ::windows::core::TryIntoParam<ISmsMessageBase>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<SmsSendMessageResult>>();
            (::windows::core::Interface::vtable(this).SendMessageAndGetResultAsync)(::windows::core::Interface::as_raw(this), message.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeviceStatusChanged(&self, eventhandler: &super::super::Foundation::TypedEventHandler<SmsDevice2, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).DeviceStatusChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(eventhandler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDeviceStatusChanged(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveDeviceStatusChanged)(::windows::core::Interface::as_raw(this), eventcookie).ok() }
    }
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISmsDevice2Statics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetDeviceSelector)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn FromId(deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<SmsDevice2> {
        Self::ISmsDevice2Statics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsDevice2>();
            (::windows::core::Interface::vtable(this).FromId)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), &mut result__).from_abi(result__)
        })
    }
    pub fn GetDefault() -> ::windows::core::Result<SmsDevice2> {
        Self::ISmsDevice2Statics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsDevice2>();
            (::windows::core::Interface::vtable(this).GetDefault)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn FromParentId(parentdeviceid: &::windows::core::HSTRING) -> ::windows::core::Result<SmsDevice2> {
        Self::ISmsDevice2Statics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsDevice2>();
            (::windows::core::Interface::vtable(this).FromParentId)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(parentdeviceid), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISmsDevice2Statics<R, F: FnOnce(&ISmsDevice2Statics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<SmsDevice2, ISmsDevice2Statics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for SmsDevice2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmsDevice2 {}
impl ::core::fmt::Debug for SmsDevice2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsDevice2").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SmsDevice2 {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.SmsDevice2;{bd8a5c13-e522-46cb-b8d5-9ead30fb6c47})");
}
impl ::core::clone::Clone for SmsDevice2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SmsDevice2 {
    type Vtable = ISmsDevice2_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SmsDevice2 {
    const IID: ::windows::core::GUID = <ISmsDevice2 as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SmsDevice2 {
    const NAME: &'static str = "Windows.Devices.Sms.SmsDevice2";
}
::windows::imp::interface_hierarchy!(SmsDevice2, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Devices_Sms\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SmsDeviceMessageStore(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl SmsDeviceMessageStore {
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn DeleteMessageAsync(&self, messageid: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).DeleteMessageAsync)(::windows::core::Interface::as_raw(this), messageid, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn DeleteMessagesAsync(&self, messagefilter: SmsMessageFilter) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).DeleteMessagesAsync)(::windows::core::Interface::as_raw(this), messagefilter, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn GetMessageAsync(&self, messageid: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ISmsMessage>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<ISmsMessage>>();
            (::windows::core::Interface::vtable(this).GetMessageAsync)(::windows::core::Interface::as_raw(this), messageid, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn GetMessagesAsync(&self, messagefilter: SmsMessageFilter) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Foundation::Collections::IVectorView<ISmsMessage>, i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<super::super::Foundation::Collections::IVectorView<ISmsMessage>, i32>>();
            (::windows::core::Interface::vtable(this).GetMessagesAsync)(::windows::core::Interface::as_raw(this), messagefilter, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn MaxMessages(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).MaxMessages)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for SmsDeviceMessageStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for SmsDeviceMessageStore {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SmsDeviceMessageStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsDeviceMessageStore").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeType for SmsDeviceMessageStore {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.SmsDeviceMessageStore;{9889f253-f188-4427-8d54-ce0c2423c5c1})");
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SmsDeviceMessageStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for SmsDeviceMessageStore {
    type Vtable = ISmsDeviceMessageStore_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::ComInterface for SmsDeviceMessageStore {
    const IID: ::windows::core::GUID = <ISmsDeviceMessageStore as ::windows::core::ComInterface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for SmsDeviceMessageStore {
    const NAME: &'static str = "Windows.Devices.Sms.SmsDeviceMessageStore";
}
#[cfg(feature = "deprecated")]
::windows::imp::interface_hierarchy!(SmsDeviceMessageStore, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Devices_Sms\"`*"]
#[repr(transparent)]
pub struct SmsFilterRule(::windows::core::IUnknown);
impl SmsFilterRule {
    pub fn MessageType(&self) -> ::windows::core::Result<SmsMessageType> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsMessageType>();
            (::windows::core::Interface::vtable(this).MessageType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ImsiPrefixes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>();
            (::windows::core::Interface::vtable(this).ImsiPrefixes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn DeviceIds(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>();
            (::windows::core::Interface::vtable(this).DeviceIds)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SenderNumbers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>();
            (::windows::core::Interface::vtable(this).SenderNumbers)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn TextMessagePrefixes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>();
            (::windows::core::Interface::vtable(this).TextMessagePrefixes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn PortNumbers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<i32>>();
            (::windows::core::Interface::vtable(this).PortNumbers)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CellularClass(&self) -> ::windows::core::Result<CellularClass> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CellularClass>();
            (::windows::core::Interface::vtable(this).CellularClass)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCellularClass(&self, value: CellularClass) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCellularClass)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ProtocolIds(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<i32>>();
            (::windows::core::Interface::vtable(this).ProtocolIds)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn TeleserviceIds(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<i32>>();
            (::windows::core::Interface::vtable(this).TeleserviceIds)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn WapApplicationIds(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>();
            (::windows::core::Interface::vtable(this).WapApplicationIds)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn WapContentTypes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>();
            (::windows::core::Interface::vtable(this).WapContentTypes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn BroadcastTypes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SmsBroadcastType>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<SmsBroadcastType>>();
            (::windows::core::Interface::vtable(this).BroadcastTypes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn BroadcastChannels(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<i32>>();
            (::windows::core::Interface::vtable(this).BroadcastChannels)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateFilterRule(messagetype: SmsMessageType) -> ::windows::core::Result<SmsFilterRule> {
        Self::ISmsFilterRuleFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsFilterRule>();
            (::windows::core::Interface::vtable(this).CreateFilterRule)(::windows::core::Interface::as_raw(this), messagetype, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISmsFilterRuleFactory<R, F: FnOnce(&ISmsFilterRuleFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<SmsFilterRule, ISmsFilterRuleFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for SmsFilterRule {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmsFilterRule {}
impl ::core::fmt::Debug for SmsFilterRule {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsFilterRule").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SmsFilterRule {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.SmsFilterRule;{40e32fae-b049-4fbc-afe9-e2a610eff55c})");
}
impl ::core::clone::Clone for SmsFilterRule {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SmsFilterRule {
    type Vtable = ISmsFilterRule_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SmsFilterRule {
    const IID: ::windows::core::GUID = <ISmsFilterRule as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SmsFilterRule {
    const NAME: &'static str = "Windows.Devices.Sms.SmsFilterRule";
}
::windows::imp::interface_hierarchy!(SmsFilterRule, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for SmsFilterRule {}
unsafe impl ::core::marker::Sync for SmsFilterRule {}
#[doc = "*Required features: `\"Devices_Sms\"`*"]
#[repr(transparent)]
pub struct SmsFilterRules(::windows::core::IUnknown);
impl SmsFilterRules {
    pub fn ActionType(&self) -> ::windows::core::Result<SmsFilterActionType> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsFilterActionType>();
            (::windows::core::Interface::vtable(this).ActionType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Rules(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SmsFilterRule>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<SmsFilterRule>>();
            (::windows::core::Interface::vtable(this).Rules)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateFilterRules(actiontype: SmsFilterActionType) -> ::windows::core::Result<SmsFilterRules> {
        Self::ISmsFilterRulesFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsFilterRules>();
            (::windows::core::Interface::vtable(this).CreateFilterRules)(::windows::core::Interface::as_raw(this), actiontype, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISmsFilterRulesFactory<R, F: FnOnce(&ISmsFilterRulesFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<SmsFilterRules, ISmsFilterRulesFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for SmsFilterRules {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmsFilterRules {}
impl ::core::fmt::Debug for SmsFilterRules {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsFilterRules").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SmsFilterRules {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.SmsFilterRules;{4e47eafb-79cd-4881-9894-55a4135b23fa})");
}
impl ::core::clone::Clone for SmsFilterRules {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SmsFilterRules {
    type Vtable = ISmsFilterRules_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SmsFilterRules {
    const IID: ::windows::core::GUID = <ISmsFilterRules as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SmsFilterRules {
    const NAME: &'static str = "Windows.Devices.Sms.SmsFilterRules";
}
::windows::imp::interface_hierarchy!(SmsFilterRules, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for SmsFilterRules {}
unsafe impl ::core::marker::Sync for SmsFilterRules {}
#[doc = "*Required features: `\"Devices_Sms\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SmsMessageReceivedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl SmsMessageReceivedEventArgs {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn TextMessage(&self) -> ::windows::core::Result<SmsTextMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsTextMessage>();
            (::windows::core::Interface::vtable(this).TextMessage)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn BinaryMessage(&self) -> ::windows::core::Result<SmsBinaryMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsBinaryMessage>();
            (::windows::core::Interface::vtable(this).BinaryMessage)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for SmsMessageReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for SmsMessageReceivedEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SmsMessageReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsMessageReceivedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeType for SmsMessageReceivedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.SmsMessageReceivedEventArgs;{08e80a98-b8e5-41c1-a3d8-d3abfae22675})");
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SmsMessageReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for SmsMessageReceivedEventArgs {
    type Vtable = ISmsMessageReceivedEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::ComInterface for SmsMessageReceivedEventArgs {
    const IID: ::windows::core::GUID = <ISmsMessageReceivedEventArgs as ::windows::core::ComInterface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for SmsMessageReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sms.SmsMessageReceivedEventArgs";
}
#[cfg(feature = "deprecated")]
::windows::imp::interface_hierarchy!(SmsMessageReceivedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Devices_Sms\"`*"]
#[repr(transparent)]
pub struct SmsMessageReceivedTriggerDetails(::windows::core::IUnknown);
impl SmsMessageReceivedTriggerDetails {
    pub fn MessageType(&self) -> ::windows::core::Result<SmsMessageType> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsMessageType>();
            (::windows::core::Interface::vtable(this).MessageType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TextMessage(&self) -> ::windows::core::Result<SmsTextMessage2> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsTextMessage2>();
            (::windows::core::Interface::vtable(this).TextMessage)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn WapMessage(&self) -> ::windows::core::Result<SmsWapMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsWapMessage>();
            (::windows::core::Interface::vtable(this).WapMessage)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AppMessage(&self) -> ::windows::core::Result<SmsAppMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsAppMessage>();
            (::windows::core::Interface::vtable(this).AppMessage)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn BroadcastMessage(&self) -> ::windows::core::Result<SmsBroadcastMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsBroadcastMessage>();
            (::windows::core::Interface::vtable(this).BroadcastMessage)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn VoicemailMessage(&self) -> ::windows::core::Result<SmsVoicemailMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsVoicemailMessage>();
            (::windows::core::Interface::vtable(this).VoicemailMessage)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StatusMessage(&self) -> ::windows::core::Result<SmsStatusMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsStatusMessage>();
            (::windows::core::Interface::vtable(this).StatusMessage)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Drop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Drop)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Accept(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Accept)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::cmp::PartialEq for SmsMessageReceivedTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmsMessageReceivedTriggerDetails {}
impl ::core::fmt::Debug for SmsMessageReceivedTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsMessageReceivedTriggerDetails").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SmsMessageReceivedTriggerDetails {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.SmsMessageReceivedTriggerDetails;{2bcfcbd4-2657-4128-ad5f-e3877132bdb1})");
}
impl ::core::clone::Clone for SmsMessageReceivedTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SmsMessageReceivedTriggerDetails {
    type Vtable = ISmsMessageReceivedTriggerDetails_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SmsMessageReceivedTriggerDetails {
    const IID: ::windows::core::GUID = <ISmsMessageReceivedTriggerDetails as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SmsMessageReceivedTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Sms.SmsMessageReceivedTriggerDetails";
}
::windows::imp::interface_hierarchy!(SmsMessageReceivedTriggerDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for SmsMessageReceivedTriggerDetails {}
unsafe impl ::core::marker::Sync for SmsMessageReceivedTriggerDetails {}
#[doc = "*Required features: `\"Devices_Sms\"`*"]
#[repr(transparent)]
pub struct SmsMessageRegistration(::windows::core::IUnknown);
impl SmsMessageRegistration {
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Unregister(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Unregister)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MessageReceived(&self, eventhandler: &super::super::Foundation::TypedEventHandler<SmsMessageRegistration, SmsMessageReceivedTriggerDetails>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).MessageReceived)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(eventhandler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMessageReceived(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveMessageReceived)(::windows::core::Interface::as_raw(this), eventcookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AllRegistrations() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SmsMessageRegistration>> {
        Self::ISmsMessageRegistrationStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<SmsMessageRegistration>>();
            (::windows::core::Interface::vtable(this).AllRegistrations)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Register(id: &::windows::core::HSTRING, filterrules: &SmsFilterRules) -> ::windows::core::Result<SmsMessageRegistration> {
        Self::ISmsMessageRegistrationStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsMessageRegistration>();
            (::windows::core::Interface::vtable(this).Register)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(id), ::core::mem::transmute_copy(filterrules), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISmsMessageRegistrationStatics<R, F: FnOnce(&ISmsMessageRegistrationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<SmsMessageRegistration, ISmsMessageRegistrationStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for SmsMessageRegistration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmsMessageRegistration {}
impl ::core::fmt::Debug for SmsMessageRegistration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsMessageRegistration").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SmsMessageRegistration {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.SmsMessageRegistration;{1720503e-f34f-446b-83b3-0ff19923b409})");
}
impl ::core::clone::Clone for SmsMessageRegistration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SmsMessageRegistration {
    type Vtable = ISmsMessageRegistration_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SmsMessageRegistration {
    const IID: ::windows::core::GUID = <ISmsMessageRegistration as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SmsMessageRegistration {
    const NAME: &'static str = "Windows.Devices.Sms.SmsMessageRegistration";
}
::windows::imp::interface_hierarchy!(SmsMessageRegistration, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Devices_Sms\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SmsReceivedEventDetails(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl SmsReceivedEventDetails {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DeviceId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn MessageIndex(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).MessageIndex)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn MessageClass(&self) -> ::windows::core::Result<SmsMessageClass> {
        let this = &::windows::core::ComInterface::cast::<ISmsReceivedEventDetails2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsMessageClass>();
            (::windows::core::Interface::vtable(this).MessageClass)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn BinaryMessage(&self) -> ::windows::core::Result<SmsBinaryMessage> {
        let this = &::windows::core::ComInterface::cast::<ISmsReceivedEventDetails2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsBinaryMessage>();
            (::windows::core::Interface::vtable(this).BinaryMessage)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for SmsReceivedEventDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for SmsReceivedEventDetails {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SmsReceivedEventDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsReceivedEventDetails").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeType for SmsReceivedEventDetails {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.SmsReceivedEventDetails;{5bb50f15-e46d-4c82-847d-5a0304c1d53d})");
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SmsReceivedEventDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for SmsReceivedEventDetails {
    type Vtable = ISmsReceivedEventDetails_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::ComInterface for SmsReceivedEventDetails {
    const IID: ::windows::core::GUID = <ISmsReceivedEventDetails as ::windows::core::ComInterface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for SmsReceivedEventDetails {
    const NAME: &'static str = "Windows.Devices.Sms.SmsReceivedEventDetails";
}
#[cfg(feature = "deprecated")]
::windows::imp::interface_hierarchy!(SmsReceivedEventDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for SmsReceivedEventDetails {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for SmsReceivedEventDetails {}
#[doc = "*Required features: `\"Devices_Sms\"`*"]
#[repr(transparent)]
pub struct SmsSendMessageResult(::windows::core::IUnknown);
impl SmsSendMessageResult {
    pub fn IsSuccessful(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsSuccessful)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn MessageReferenceNumbers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<i32>>();
            (::windows::core::Interface::vtable(this).MessageReferenceNumbers)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CellularClass(&self) -> ::windows::core::Result<CellularClass> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CellularClass>();
            (::windows::core::Interface::vtable(this).CellularClass)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ModemErrorCode(&self) -> ::windows::core::Result<SmsModemErrorCode> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsModemErrorCode>();
            (::windows::core::Interface::vtable(this).ModemErrorCode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsErrorTransient(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsErrorTransient)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NetworkCauseCode(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).NetworkCauseCode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TransportFailureCause(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).TransportFailureCause)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for SmsSendMessageResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmsSendMessageResult {}
impl ::core::fmt::Debug for SmsSendMessageResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsSendMessageResult").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SmsSendMessageResult {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.SmsSendMessageResult;{db139af2-78c9-4feb-9622-452328088d62})");
}
impl ::core::clone::Clone for SmsSendMessageResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SmsSendMessageResult {
    type Vtable = ISmsSendMessageResult_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SmsSendMessageResult {
    const IID: ::windows::core::GUID = <ISmsSendMessageResult as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SmsSendMessageResult {
    const NAME: &'static str = "Windows.Devices.Sms.SmsSendMessageResult";
}
::windows::imp::interface_hierarchy!(SmsSendMessageResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for SmsSendMessageResult {}
unsafe impl ::core::marker::Sync for SmsSendMessageResult {}
#[doc = "*Required features: `\"Devices_Sms\"`*"]
#[repr(transparent)]
pub struct SmsStatusMessage(::windows::core::IUnknown);
impl SmsStatusMessage {
    pub fn MessageType(&self) -> ::windows::core::Result<SmsMessageType> {
        let this = &::windows::core::ComInterface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsMessageType>();
            (::windows::core::Interface::vtable(this).MessageType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DeviceId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CellularClass(&self) -> ::windows::core::Result<CellularClass> {
        let this = &::windows::core::ComInterface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CellularClass>();
            (::windows::core::Interface::vtable(this).CellularClass)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MessageClass(&self) -> ::windows::core::Result<SmsMessageClass> {
        let this = &::windows::core::ComInterface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsMessageClass>();
            (::windows::core::Interface::vtable(this).MessageClass)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SimIccId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).SimIccId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn To(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).To)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn From(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).From)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Body(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Body)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Status(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MessageReferenceNumber(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).MessageReferenceNumber)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ServiceCenterTimestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::DateTime>();
            (::windows::core::Interface::vtable(this).ServiceCenterTimestamp)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DischargeTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::DateTime>();
            (::windows::core::Interface::vtable(this).DischargeTime)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for SmsStatusMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmsStatusMessage {}
impl ::core::fmt::Debug for SmsStatusMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsStatusMessage").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SmsStatusMessage {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.SmsStatusMessage;{e6d28342-b70b-4677-9379-c9783fdff8f4})");
}
impl ::core::clone::Clone for SmsStatusMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SmsStatusMessage {
    type Vtable = ISmsStatusMessage_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SmsStatusMessage {
    const IID: ::windows::core::GUID = <ISmsStatusMessage as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SmsStatusMessage {
    const NAME: &'static str = "Windows.Devices.Sms.SmsStatusMessage";
}
::windows::imp::interface_hierarchy!(SmsStatusMessage, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<ISmsMessageBase> for SmsStatusMessage {}
unsafe impl ::core::marker::Send for SmsStatusMessage {}
unsafe impl ::core::marker::Sync for SmsStatusMessage {}
#[doc = "*Required features: `\"Devices_Sms\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SmsTextMessage(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl SmsTextMessage {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<SmsTextMessage, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<ISmsMessage>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MessageClass(&self) -> ::windows::core::Result<SmsMessageClass> {
        let this = &::windows::core::ComInterface::cast::<ISmsMessage>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsMessageClass>();
            (::windows::core::Interface::vtable(this).MessageClass)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::DateTime>();
            (::windows::core::Interface::vtable(this).Timestamp)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn PartReferenceId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).PartReferenceId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn PartNumber(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).PartNumber)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn PartCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).PartCount)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn To(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).To)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetTo(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTo)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn From(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).From)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetFrom(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetFrom)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Body(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Body)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetBody(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBody)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Encoding(&self) -> ::windows::core::Result<SmsEncoding> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsEncoding>();
            (::windows::core::Interface::vtable(this).Encoding)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetEncoding(&self, value: SmsEncoding) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetEncoding)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn ToBinaryMessages(&self, format: SmsDataFormat) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ISmsBinaryMessage>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<ISmsBinaryMessage>>();
            (::windows::core::Interface::vtable(this).ToBinaryMessages)(::windows::core::Interface::as_raw(this), format, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn FromBinaryMessage(binarymessage: &SmsBinaryMessage) -> ::windows::core::Result<SmsTextMessage> {
        Self::ISmsTextMessageStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsTextMessage>();
            (::windows::core::Interface::vtable(this).FromBinaryMessage)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(binarymessage), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn FromBinaryData(format: SmsDataFormat, value: &[u8]) -> ::windows::core::Result<SmsTextMessage> {
        Self::ISmsTextMessageStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsTextMessage>();
            (::windows::core::Interface::vtable(this).FromBinaryData)(::windows::core::Interface::as_raw(this), format, value.len() as u32, value.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn ISmsTextMessageStatics<R, F: FnOnce(&ISmsTextMessageStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<SmsTextMessage, ISmsTextMessageStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for SmsTextMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for SmsTextMessage {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SmsTextMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsTextMessage").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeType for SmsTextMessage {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.SmsTextMessage;{d61c904c-a495-487f-9a6f-971548c5bc9f})");
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SmsTextMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for SmsTextMessage {
    type Vtable = ISmsTextMessage_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::ComInterface for SmsTextMessage {
    const IID: ::windows::core::GUID = <ISmsTextMessage as ::windows::core::ComInterface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for SmsTextMessage {
    const NAME: &'static str = "Windows.Devices.Sms.SmsTextMessage";
}
#[cfg(feature = "deprecated")]
::windows::imp::interface_hierarchy!(SmsTextMessage, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::windows::core::CanTryInto<ISmsMessage> for SmsTextMessage {}
#[cfg(feature = "deprecated")]
impl ::windows::core::CanTryInto<ISmsTextMessage> for SmsTextMessage {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for SmsTextMessage {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for SmsTextMessage {}
#[doc = "*Required features: `\"Devices_Sms\"`*"]
#[repr(transparent)]
pub struct SmsTextMessage2(::windows::core::IUnknown);
impl SmsTextMessage2 {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<SmsTextMessage2, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn MessageType(&self) -> ::windows::core::Result<SmsMessageType> {
        let this = &::windows::core::ComInterface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsMessageType>();
            (::windows::core::Interface::vtable(this).MessageType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DeviceId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CellularClass(&self) -> ::windows::core::Result<CellularClass> {
        let this = &::windows::core::ComInterface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CellularClass>();
            (::windows::core::Interface::vtable(this).CellularClass)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MessageClass(&self) -> ::windows::core::Result<SmsMessageClass> {
        let this = &::windows::core::ComInterface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsMessageClass>();
            (::windows::core::Interface::vtable(this).MessageClass)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SimIccId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).SimIccId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::DateTime>();
            (::windows::core::Interface::vtable(this).Timestamp)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn To(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).To)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTo(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTo)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn From(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).From)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Body(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Body)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetBody(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBody)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Encoding(&self) -> ::windows::core::Result<SmsEncoding> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsEncoding>();
            (::windows::core::Interface::vtable(this).Encoding)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetEncoding(&self, value: SmsEncoding) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetEncoding)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn CallbackNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).CallbackNumber)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCallbackNumber(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCallbackNumber)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn IsDeliveryNotificationEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsDeliveryNotificationEnabled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsDeliveryNotificationEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsDeliveryNotificationEnabled)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn RetryAttemptCount(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).RetryAttemptCount)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRetryAttemptCount(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRetryAttemptCount)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn TeleserviceId(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).TeleserviceId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ProtocolId(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).ProtocolId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for SmsTextMessage2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmsTextMessage2 {}
impl ::core::fmt::Debug for SmsTextMessage2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsTextMessage2").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SmsTextMessage2 {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.SmsTextMessage2;{22a0d893-4555-4755-b5a1-e7fd84955f8d})");
}
impl ::core::clone::Clone for SmsTextMessage2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SmsTextMessage2 {
    type Vtable = ISmsTextMessage2_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SmsTextMessage2 {
    const IID: ::windows::core::GUID = <ISmsTextMessage2 as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SmsTextMessage2 {
    const NAME: &'static str = "Windows.Devices.Sms.SmsTextMessage2";
}
::windows::imp::interface_hierarchy!(SmsTextMessage2, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<ISmsMessageBase> for SmsTextMessage2 {}
unsafe impl ::core::marker::Send for SmsTextMessage2 {}
unsafe impl ::core::marker::Sync for SmsTextMessage2 {}
#[doc = "*Required features: `\"Devices_Sms\"`*"]
#[repr(transparent)]
pub struct SmsVoicemailMessage(::windows::core::IUnknown);
impl SmsVoicemailMessage {
    pub fn MessageType(&self) -> ::windows::core::Result<SmsMessageType> {
        let this = &::windows::core::ComInterface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsMessageType>();
            (::windows::core::Interface::vtable(this).MessageType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DeviceId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CellularClass(&self) -> ::windows::core::Result<CellularClass> {
        let this = &::windows::core::ComInterface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CellularClass>();
            (::windows::core::Interface::vtable(this).CellularClass)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MessageClass(&self) -> ::windows::core::Result<SmsMessageClass> {
        let this = &::windows::core::ComInterface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsMessageClass>();
            (::windows::core::Interface::vtable(this).MessageClass)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SimIccId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).SimIccId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::DateTime>();
            (::windows::core::Interface::vtable(this).Timestamp)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn To(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).To)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Body(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Body)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MessageCount(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<i32>>();
            (::windows::core::Interface::vtable(this).MessageCount)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for SmsVoicemailMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmsVoicemailMessage {}
impl ::core::fmt::Debug for SmsVoicemailMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsVoicemailMessage").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SmsVoicemailMessage {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.SmsVoicemailMessage;{271aa0a6-95b1-44ff-bcb8-b8fdd7e08bc3})");
}
impl ::core::clone::Clone for SmsVoicemailMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SmsVoicemailMessage {
    type Vtable = ISmsVoicemailMessage_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SmsVoicemailMessage {
    const IID: ::windows::core::GUID = <ISmsVoicemailMessage as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SmsVoicemailMessage {
    const NAME: &'static str = "Windows.Devices.Sms.SmsVoicemailMessage";
}
::windows::imp::interface_hierarchy!(SmsVoicemailMessage, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<ISmsMessageBase> for SmsVoicemailMessage {}
unsafe impl ::core::marker::Send for SmsVoicemailMessage {}
unsafe impl ::core::marker::Sync for SmsVoicemailMessage {}
#[doc = "*Required features: `\"Devices_Sms\"`*"]
#[repr(transparent)]
pub struct SmsWapMessage(::windows::core::IUnknown);
impl SmsWapMessage {
    pub fn MessageType(&self) -> ::windows::core::Result<SmsMessageType> {
        let this = &::windows::core::ComInterface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsMessageType>();
            (::windows::core::Interface::vtable(this).MessageType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DeviceId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CellularClass(&self) -> ::windows::core::Result<CellularClass> {
        let this = &::windows::core::ComInterface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CellularClass>();
            (::windows::core::Interface::vtable(this).CellularClass)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MessageClass(&self) -> ::windows::core::Result<SmsMessageClass> {
        let this = &::windows::core::ComInterface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsMessageClass>();
            (::windows::core::Interface::vtable(this).MessageClass)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SimIccId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).SimIccId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::DateTime>();
            (::windows::core::Interface::vtable(this).Timestamp)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn To(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).To)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn From(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).From)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ApplicationId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ApplicationId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ContentType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn BinaryBody(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::Streams::IBuffer>();
            (::windows::core::Interface::vtable(this).BinaryBody)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Headers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>();
            (::windows::core::Interface::vtable(this).Headers)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for SmsWapMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmsWapMessage {}
impl ::core::fmt::Debug for SmsWapMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsWapMessage").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SmsWapMessage {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.SmsWapMessage;{cd937743-7a55-4d3b-9021-f22e022d09c5})");
}
impl ::core::clone::Clone for SmsWapMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SmsWapMessage {
    type Vtable = ISmsWapMessage_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SmsWapMessage {
    const IID: ::windows::core::GUID = <ISmsWapMessage as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SmsWapMessage {
    const NAME: &'static str = "Windows.Devices.Sms.SmsWapMessage";
}
::windows::imp::interface_hierarchy!(SmsWapMessage, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<ISmsMessageBase> for SmsWapMessage {}
unsafe impl ::core::marker::Send for SmsWapMessage {}
unsafe impl ::core::marker::Sync for SmsWapMessage {}
#[doc = "*Required features: `\"Devices_Sms\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CellularClass(pub i32);
impl CellularClass {
    pub const None: Self = Self(0i32);
    pub const Gsm: Self = Self(1i32);
    pub const Cdma: Self = Self(2i32);
}
impl ::core::marker::Copy for CellularClass {}
impl ::core::clone::Clone for CellularClass {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CellularClass {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CellularClass {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CellularClass {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CellularClass").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CellularClass {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Sms.CellularClass;i4)");
}
#[doc = "*Required features: `\"Devices_Sms\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SmsBroadcastType(pub i32);
impl SmsBroadcastType {
    pub const Other: Self = Self(0i32);
    pub const CmasPresidential: Self = Self(1i32);
    pub const CmasExtreme: Self = Self(2i32);
    pub const CmasSevere: Self = Self(3i32);
    pub const CmasAmber: Self = Self(4i32);
    pub const CmasTest: Self = Self(5i32);
    pub const EUAlert1: Self = Self(6i32);
    pub const EUAlert2: Self = Self(7i32);
    pub const EUAlert3: Self = Self(8i32);
    pub const EUAlertAmber: Self = Self(9i32);
    pub const EUAlertInfo: Self = Self(10i32);
    pub const EtwsEarthquake: Self = Self(11i32);
    pub const EtwsTsunami: Self = Self(12i32);
    pub const EtwsTsunamiAndEarthquake: Self = Self(13i32);
    pub const LatAlertLocal: Self = Self(14i32);
}
impl ::core::marker::Copy for SmsBroadcastType {}
impl ::core::clone::Clone for SmsBroadcastType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmsBroadcastType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SmsBroadcastType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SmsBroadcastType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsBroadcastType").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SmsBroadcastType {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Sms.SmsBroadcastType;i4)");
}
#[doc = "*Required features: `\"Devices_Sms\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SmsDataFormat(pub i32);
impl SmsDataFormat {
    pub const Unknown: Self = Self(0i32);
    pub const CdmaSubmit: Self = Self(1i32);
    pub const GsmSubmit: Self = Self(2i32);
    pub const CdmaDeliver: Self = Self(3i32);
    pub const GsmDeliver: Self = Self(4i32);
}
impl ::core::marker::Copy for SmsDataFormat {}
impl ::core::clone::Clone for SmsDataFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmsDataFormat {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SmsDataFormat {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SmsDataFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsDataFormat").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SmsDataFormat {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Sms.SmsDataFormat;i4)");
}
#[doc = "*Required features: `\"Devices_Sms\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SmsDeviceStatus(pub i32);
impl SmsDeviceStatus {
    pub const Off: Self = Self(0i32);
    pub const Ready: Self = Self(1i32);
    pub const SimNotInserted: Self = Self(2i32);
    pub const BadSim: Self = Self(3i32);
    pub const DeviceFailure: Self = Self(4i32);
    pub const SubscriptionNotActivated: Self = Self(5i32);
    pub const DeviceLocked: Self = Self(6i32);
    pub const DeviceBlocked: Self = Self(7i32);
}
impl ::core::marker::Copy for SmsDeviceStatus {}
impl ::core::clone::Clone for SmsDeviceStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmsDeviceStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SmsDeviceStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SmsDeviceStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsDeviceStatus").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SmsDeviceStatus {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Sms.SmsDeviceStatus;i4)");
}
#[doc = "*Required features: `\"Devices_Sms\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SmsEncoding(pub i32);
impl SmsEncoding {
    pub const Unknown: Self = Self(0i32);
    pub const Optimal: Self = Self(1i32);
    pub const SevenBitAscii: Self = Self(2i32);
    pub const Unicode: Self = Self(3i32);
    pub const GsmSevenBit: Self = Self(4i32);
    pub const EightBit: Self = Self(5i32);
    pub const Latin: Self = Self(6i32);
    pub const Korean: Self = Self(7i32);
    pub const IA5: Self = Self(8i32);
    pub const ShiftJis: Self = Self(9i32);
    pub const LatinHebrew: Self = Self(10i32);
}
impl ::core::marker::Copy for SmsEncoding {}
impl ::core::clone::Clone for SmsEncoding {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmsEncoding {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SmsEncoding {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SmsEncoding {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsEncoding").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SmsEncoding {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Sms.SmsEncoding;i4)");
}
#[doc = "*Required features: `\"Devices_Sms\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SmsFilterActionType(pub i32);
impl SmsFilterActionType {
    pub const AcceptImmediately: Self = Self(0i32);
    pub const Drop: Self = Self(1i32);
    pub const Peek: Self = Self(2i32);
    pub const Accept: Self = Self(3i32);
}
impl ::core::marker::Copy for SmsFilterActionType {}
impl ::core::clone::Clone for SmsFilterActionType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmsFilterActionType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SmsFilterActionType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SmsFilterActionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsFilterActionType").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SmsFilterActionType {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Sms.SmsFilterActionType;i4)");
}
#[doc = "*Required features: `\"Devices_Sms\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SmsGeographicalScope(pub i32);
impl SmsGeographicalScope {
    pub const None: Self = Self(0i32);
    pub const CellWithImmediateDisplay: Self = Self(1i32);
    pub const LocationArea: Self = Self(2i32);
    pub const Plmn: Self = Self(3i32);
    pub const Cell: Self = Self(4i32);
}
impl ::core::marker::Copy for SmsGeographicalScope {}
impl ::core::clone::Clone for SmsGeographicalScope {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmsGeographicalScope {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SmsGeographicalScope {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SmsGeographicalScope {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsGeographicalScope").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SmsGeographicalScope {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Sms.SmsGeographicalScope;i4)");
}
#[doc = "*Required features: `\"Devices_Sms\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SmsMessageClass(pub i32);
impl SmsMessageClass {
    pub const None: Self = Self(0i32);
    pub const Class0: Self = Self(1i32);
    pub const Class1: Self = Self(2i32);
    pub const Class2: Self = Self(3i32);
    pub const Class3: Self = Self(4i32);
}
impl ::core::marker::Copy for SmsMessageClass {}
impl ::core::clone::Clone for SmsMessageClass {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmsMessageClass {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SmsMessageClass {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SmsMessageClass {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsMessageClass").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SmsMessageClass {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Sms.SmsMessageClass;i4)");
}
#[doc = "*Required features: `\"Devices_Sms\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SmsMessageFilter(pub i32);
#[cfg(feature = "deprecated")]
impl SmsMessageFilter {
    pub const All: Self = Self(0i32);
    pub const Unread: Self = Self(1i32);
    pub const Read: Self = Self(2i32);
    pub const Sent: Self = Self(3i32);
    pub const Draft: Self = Self(4i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for SmsMessageFilter {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SmsMessageFilter {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for SmsMessageFilter {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
impl ::windows::core::TypeKind for SmsMessageFilter {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SmsMessageFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsMessageFilter").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeType for SmsMessageFilter {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Sms.SmsMessageFilter;i4)");
}
#[doc = "*Required features: `\"Devices_Sms\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SmsMessageType(pub i32);
impl SmsMessageType {
    pub const Binary: Self = Self(0i32);
    pub const Text: Self = Self(1i32);
    pub const Wap: Self = Self(2i32);
    pub const App: Self = Self(3i32);
    pub const Broadcast: Self = Self(4i32);
    pub const Voicemail: Self = Self(5i32);
    pub const Status: Self = Self(6i32);
}
impl ::core::marker::Copy for SmsMessageType {}
impl ::core::clone::Clone for SmsMessageType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmsMessageType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SmsMessageType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SmsMessageType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsMessageType").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SmsMessageType {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Sms.SmsMessageType;i4)");
}
#[doc = "*Required features: `\"Devices_Sms\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SmsModemErrorCode(pub i32);
impl SmsModemErrorCode {
    pub const Other: Self = Self(0i32);
    pub const MessagingNetworkError: Self = Self(1i32);
    pub const SmsOperationNotSupportedByDevice: Self = Self(2i32);
    pub const SmsServiceNotSupportedByNetwork: Self = Self(3i32);
    pub const DeviceFailure: Self = Self(4i32);
    pub const MessageNotEncodedProperly: Self = Self(5i32);
    pub const MessageTooLarge: Self = Self(6i32);
    pub const DeviceNotReady: Self = Self(7i32);
    pub const NetworkNotReady: Self = Self(8i32);
    pub const InvalidSmscAddress: Self = Self(9i32);
    pub const NetworkFailure: Self = Self(10i32);
    pub const FixedDialingNumberRestricted: Self = Self(11i32);
}
impl ::core::marker::Copy for SmsModemErrorCode {}
impl ::core::clone::Clone for SmsModemErrorCode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SmsModemErrorCode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SmsModemErrorCode {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SmsModemErrorCode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsModemErrorCode").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SmsModemErrorCode {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Sms.SmsModemErrorCode;i4)");
}
#[repr(C)]
#[doc = "*Required features: `\"Devices_Sms\"`*"]
pub struct SmsEncodedLength {
    pub SegmentCount: u32,
    pub CharacterCountLastSegment: u32,
    pub CharactersPerSegment: u32,
    pub ByteCountLastSegment: u32,
    pub BytesPerSegment: u32,
}
impl ::core::marker::Copy for SmsEncodedLength {}
impl ::core::clone::Clone for SmsEncodedLength {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SmsEncodedLength {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SmsEncodedLength").field("SegmentCount", &self.SegmentCount).field("CharacterCountLastSegment", &self.CharacterCountLastSegment).field("CharactersPerSegment", &self.CharactersPerSegment).field("ByteCountLastSegment", &self.ByteCountLastSegment).field("BytesPerSegment", &self.BytesPerSegment).finish()
    }
}
impl ::windows::core::TypeKind for SmsEncodedLength {
    type TypeKind = ::windows::core::CopyType;
}
impl ::windows::core::RuntimeType for SmsEncodedLength {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"struct(Windows.Devices.Sms.SmsEncodedLength;u4;u4;u4;u4;u4)");
}
impl ::core::cmp::PartialEq for SmsEncodedLength {
    fn eq(&self, other: &Self) -> bool {
        self.SegmentCount == other.SegmentCount && self.CharacterCountLastSegment == other.CharacterCountLastSegment && self.CharactersPerSegment == other.CharactersPerSegment && self.ByteCountLastSegment == other.ByteCountLastSegment && self.BytesPerSegment == other.BytesPerSegment
    }
}
impl ::core::cmp::Eq for SmsEncodedLength {}
impl ::core::default::Default for SmsEncodedLength {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Devices_Sms\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SmsDeviceStatusChangedEventHandler(pub ::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl SmsDeviceStatusChangedEventHandler {
    pub fn new<F: FnMut(::core::option::Option<&SmsDevice>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = SmsDeviceStatusChangedEventHandlerBox::<F> { vtable: &SmsDeviceStatusChangedEventHandlerBox::<F>::VTABLE, count: ::windows::imp::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Invoke(&self, sender: &SmsDevice) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(sender)).ok() }
    }
}
#[cfg(feature = "deprecated")]
#[repr(C)]
struct SmsDeviceStatusChangedEventHandlerBox<F: FnMut(::core::option::Option<&SmsDevice>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const SmsDeviceStatusChangedEventHandler_Vtbl,
    invoke: F,
    count: ::windows::imp::RefCount,
}
#[cfg(feature = "deprecated")]
impl<F: FnMut(::core::option::Option<&SmsDevice>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> SmsDeviceStatusChangedEventHandlerBox<F> {
    const VTABLE: SmsDeviceStatusChangedEventHandler_Vtbl = SmsDeviceStatusChangedEventHandler_Vtbl {
        base__: ::windows::core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<SmsDeviceStatusChangedEventHandler as ::windows::core::ComInterface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::ComInterface>::IID || iid == &<::windows::imp::IAgileObject as ::windows::core::ComInterface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::std::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::windows::core::from_raw_borrowed(&sender)).into()
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for SmsDeviceStatusChangedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for SmsDeviceStatusChangedEventHandler {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SmsDeviceStatusChangedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsDeviceStatusChangedEventHandler").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for SmsDeviceStatusChangedEventHandler {
    type Vtable = SmsDeviceStatusChangedEventHandler_Vtbl;
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SmsDeviceStatusChangedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::ComInterface for SmsDeviceStatusChangedEventHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x982b1162_3dd7_4618_af89_0c272d5d06d8);
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeType for SmsDeviceStatusChangedEventHandler {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{982b1162-3dd7-4618-af89-0c272d5d06d8}");
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct SmsDeviceStatusChangedEventHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "deprecated")]
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Invoke: usize,
}
#[doc = "*Required features: `\"Devices_Sms\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SmsMessageReceivedEventHandler(pub ::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl SmsMessageReceivedEventHandler {
    pub fn new<F: FnMut(::core::option::Option<&SmsDevice>, ::core::option::Option<&SmsMessageReceivedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = SmsMessageReceivedEventHandlerBox::<F> { vtable: &SmsMessageReceivedEventHandlerBox::<F>::VTABLE, count: ::windows::imp::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Invoke(&self, sender: &SmsDevice, e: &SmsMessageReceivedEventArgs) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(sender), ::core::mem::transmute_copy(e)).ok() }
    }
}
#[cfg(feature = "deprecated")]
#[repr(C)]
struct SmsMessageReceivedEventHandlerBox<F: FnMut(::core::option::Option<&SmsDevice>, ::core::option::Option<&SmsMessageReceivedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const SmsMessageReceivedEventHandler_Vtbl,
    invoke: F,
    count: ::windows::imp::RefCount,
}
#[cfg(feature = "deprecated")]
impl<F: FnMut(::core::option::Option<&SmsDevice>, ::core::option::Option<&SmsMessageReceivedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> SmsMessageReceivedEventHandlerBox<F> {
    const VTABLE: SmsMessageReceivedEventHandler_Vtbl = SmsMessageReceivedEventHandler_Vtbl {
        base__: ::windows::core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<SmsMessageReceivedEventHandler as ::windows::core::ComInterface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::ComInterface>::IID || iid == &<::windows::imp::IAgileObject as ::windows::core::ComInterface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::std::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::windows::core::from_raw_borrowed(&sender), ::windows::core::from_raw_borrowed(&e)).into()
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for SmsMessageReceivedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for SmsMessageReceivedEventHandler {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SmsMessageReceivedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsMessageReceivedEventHandler").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for SmsMessageReceivedEventHandler {
    type Vtable = SmsMessageReceivedEventHandler_Vtbl;
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SmsMessageReceivedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::ComInterface for SmsMessageReceivedEventHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0b7ad409_ec2d_47ce_a253_732beeebcacd);
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeType for SmsMessageReceivedEventHandler {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{0b7ad409-ec2d-47ce-a253-732beeebcacd}");
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct SmsMessageReceivedEventHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "deprecated")]
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Invoke: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
