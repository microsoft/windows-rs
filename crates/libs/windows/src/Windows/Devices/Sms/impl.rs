#[cfg(feature = "implement_exclusive")]
pub trait ISmsAppMessageImpl: Sized + ISmsMessageBaseImpl {
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn To(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTo(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn From(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Body(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetBody(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn CallbackNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCallbackNumber(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsDeliveryNotificationEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsDeliveryNotificationEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn RetryAttemptCount(&self) -> ::windows::core::Result<i32>;
    fn SetRetryAttemptCount(&self, value: i32) -> ::windows::core::Result<()>;
    fn Encoding(&self) -> ::windows::core::Result<SmsEncoding>;
    fn SetEncoding(&self, value: SmsEncoding) -> ::windows::core::Result<()>;
    fn PortNumber(&self) -> ::windows::core::Result<i32>;
    fn SetPortNumber(&self, value: i32) -> ::windows::core::Result<()>;
    fn TeleserviceId(&self) -> ::windows::core::Result<i32>;
    fn SetTeleserviceId(&self, value: i32) -> ::windows::core::Result<()>;
    fn ProtocolId(&self) -> ::windows::core::Result<i32>;
    fn SetProtocolId(&self, value: i32) -> ::windows::core::Result<()>;
    fn BinaryBody(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn SetBinaryBody(&self, value: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "deprecated")]
pub trait ISmsBinaryMessageImpl: Sized + ISmsMessageImpl {
    fn Format(&self) -> ::windows::core::Result<SmsDataFormat>;
    fn SetFormat(&self, value: SmsDataFormat) -> ::windows::core::Result<()>;
    fn GetData(&self) -> ::windows::core::Result<::windows::core::Array<u8>>;
    fn SetData(&self, value: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmsBroadcastMessageImpl: Sized + ISmsMessageBaseImpl {
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn To(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Body(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Channel(&self) -> ::windows::core::Result<i32>;
    fn GeographicalScope(&self) -> ::windows::core::Result<SmsGeographicalScope>;
    fn MessageCode(&self) -> ::windows::core::Result<i32>;
    fn UpdateNumber(&self) -> ::windows::core::Result<i32>;
    fn BroadcastType(&self) -> ::windows::core::Result<SmsBroadcastType>;
    fn IsEmergencyAlert(&self) -> ::windows::core::Result<bool>;
    fn IsUserPopupRequested(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "deprecated")]
pub trait ISmsDeviceImpl: Sized {
    fn SendMessageAsync(&self, message: &::core::option::Option<ISmsMessage>) -> ::windows::core::Result<SendSmsMessageOperation>;
    fn CalculateLength(&self, message: &::core::option::Option<SmsTextMessage>) -> ::windows::core::Result<SmsEncodedLength>;
    fn AccountPhoneNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CellularClass(&self) -> ::windows::core::Result<CellularClass>;
    fn MessageStore(&self) -> ::windows::core::Result<SmsDeviceMessageStore>;
    fn DeviceStatus(&self) -> ::windows::core::Result<SmsDeviceStatus>;
    fn SmsMessageReceived(&self, eventhandler: &::core::option::Option<SmsMessageReceivedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSmsMessageReceived(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SmsDeviceStatusChanged(&self, eventhandler: &::core::option::Option<SmsDeviceStatusChangedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSmsDeviceStatusChanged(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmsDevice2Impl: Sized {
    fn SmscAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSmscAddress(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ParentDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AccountPhoneNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CellularClass(&self) -> ::windows::core::Result<CellularClass>;
    fn DeviceStatus(&self) -> ::windows::core::Result<SmsDeviceStatus>;
    fn CalculateLength(&self, message: &::core::option::Option<ISmsMessageBase>) -> ::windows::core::Result<SmsEncodedLength>;
    fn SendMessageAndGetResultAsync(&self, message: &::core::option::Option<ISmsMessageBase>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmsSendMessageResult>>;
    fn DeviceStatusChanged(&self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SmsDevice2, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDeviceStatusChanged(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmsDevice2StaticsImpl: Sized {
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromId(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<SmsDevice2>;
    fn GetDefault(&self) -> ::windows::core::Result<SmsDevice2>;
    fn FromParentId(&self, parentdeviceid: &::windows::core::HSTRING) -> ::windows::core::Result<SmsDevice2>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISmsDeviceMessageStoreImpl: Sized {
    fn DeleteMessageAsync(&self, messageid: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DeleteMessagesAsync(&self, messagefilter: SmsMessageFilter) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn GetMessageAsync(&self, messageid: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ISmsMessage>>;
    fn GetMessagesAsync(&self, messagefilter: SmsMessageFilter) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Foundation::Collections::IVectorView<ISmsMessage>, i32>>;
    fn MaxMessages(&self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISmsDeviceStaticsImpl: Sized {
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmsDevice>>;
    fn GetDefaultAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmsDevice>>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISmsDeviceStatics2Impl: Sized {
    fn FromNetworkAccountIdAsync(&self, networkaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SmsDevice>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmsFilterRuleImpl: Sized {
    fn MessageType(&self) -> ::windows::core::Result<SmsMessageType>;
    fn ImsiPrefixes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn DeviceIds(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn SenderNumbers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn TextMessagePrefixes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn PortNumbers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<i32>>;
    fn CellularClass(&self) -> ::windows::core::Result<CellularClass>;
    fn SetCellularClass(&self, value: CellularClass) -> ::windows::core::Result<()>;
    fn ProtocolIds(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<i32>>;
    fn TeleserviceIds(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<i32>>;
    fn WapApplicationIds(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn WapContentTypes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn BroadcastTypes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SmsBroadcastType>>;
    fn BroadcastChannels(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<i32>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmsFilterRuleFactoryImpl: Sized {
    fn CreateFilterRule(&self, messagetype: SmsMessageType) -> ::windows::core::Result<SmsFilterRule>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmsFilterRulesImpl: Sized {
    fn ActionType(&self) -> ::windows::core::Result<SmsFilterActionType>;
    fn Rules(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SmsFilterRule>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmsFilterRulesFactoryImpl: Sized {
    fn CreateFilterRules(&self, actiontype: SmsFilterActionType) -> ::windows::core::Result<SmsFilterRules>;
}
pub trait ISmsMessageImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<u32>;
    fn MessageClass(&self) -> ::windows::core::Result<SmsMessageClass>;
}
pub trait ISmsMessageBaseImpl: Sized {
    fn MessageType(&self) -> ::windows::core::Result<SmsMessageType>;
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CellularClass(&self) -> ::windows::core::Result<CellularClass>;
    fn MessageClass(&self) -> ::windows::core::Result<SmsMessageClass>;
    fn SimIccId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISmsMessageReceivedEventArgsImpl: Sized {
    fn TextMessage(&self) -> ::windows::core::Result<SmsTextMessage>;
    fn BinaryMessage(&self) -> ::windows::core::Result<SmsBinaryMessage>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmsMessageReceivedTriggerDetailsImpl: Sized {
    fn MessageType(&self) -> ::windows::core::Result<SmsMessageType>;
    fn TextMessage(&self) -> ::windows::core::Result<SmsTextMessage2>;
    fn WapMessage(&self) -> ::windows::core::Result<SmsWapMessage>;
    fn AppMessage(&self) -> ::windows::core::Result<SmsAppMessage>;
    fn BroadcastMessage(&self) -> ::windows::core::Result<SmsBroadcastMessage>;
    fn VoicemailMessage(&self) -> ::windows::core::Result<SmsVoicemailMessage>;
    fn StatusMessage(&self) -> ::windows::core::Result<SmsStatusMessage>;
    fn Drop(&self) -> ::windows::core::Result<()>;
    fn Accept(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmsMessageRegistrationImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Unregister(&self) -> ::windows::core::Result<()>;
    fn MessageReceived(&self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SmsMessageRegistration, SmsMessageReceivedTriggerDetails>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMessageReceived(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmsMessageRegistrationStaticsImpl: Sized {
    fn AllRegistrations(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SmsMessageRegistration>>;
    fn Register(&self, id: &::windows::core::HSTRING, filterrules: &::core::option::Option<SmsFilterRules>) -> ::windows::core::Result<SmsMessageRegistration>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISmsReceivedEventDetailsImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MessageIndex(&self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISmsReceivedEventDetails2Impl: Sized {
    fn MessageClass(&self) -> ::windows::core::Result<SmsMessageClass>;
    fn BinaryMessage(&self) -> ::windows::core::Result<SmsBinaryMessage>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmsSendMessageResultImpl: Sized {
    fn IsSuccessful(&self) -> ::windows::core::Result<bool>;
    fn MessageReferenceNumbers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<i32>>;
    fn CellularClass(&self) -> ::windows::core::Result<CellularClass>;
    fn ModemErrorCode(&self) -> ::windows::core::Result<SmsModemErrorCode>;
    fn IsErrorTransient(&self) -> ::windows::core::Result<bool>;
    fn NetworkCauseCode(&self) -> ::windows::core::Result<i32>;
    fn TransportFailureCause(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmsStatusMessageImpl: Sized + ISmsMessageBaseImpl {
    fn To(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn From(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Body(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Status(&self) -> ::windows::core::Result<i32>;
    fn MessageReferenceNumber(&self) -> ::windows::core::Result<i32>;
    fn ServiceCenterTimestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn DischargeTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
}
#[cfg(feature = "deprecated")]
pub trait ISmsTextMessageImpl: Sized + ISmsMessageImpl {
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn PartReferenceId(&self) -> ::windows::core::Result<u32>;
    fn PartNumber(&self) -> ::windows::core::Result<u32>;
    fn PartCount(&self) -> ::windows::core::Result<u32>;
    fn To(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTo(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn From(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetFrom(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Body(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetBody(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Encoding(&self) -> ::windows::core::Result<SmsEncoding>;
    fn SetEncoding(&self, value: SmsEncoding) -> ::windows::core::Result<()>;
    fn ToBinaryMessages(&self, format: SmsDataFormat) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ISmsBinaryMessage>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmsTextMessage2Impl: Sized + ISmsMessageBaseImpl {
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn To(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTo(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn From(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Body(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetBody(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Encoding(&self) -> ::windows::core::Result<SmsEncoding>;
    fn SetEncoding(&self, value: SmsEncoding) -> ::windows::core::Result<()>;
    fn CallbackNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCallbackNumber(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsDeliveryNotificationEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsDeliveryNotificationEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn RetryAttemptCount(&self) -> ::windows::core::Result<i32>;
    fn SetRetryAttemptCount(&self, value: i32) -> ::windows::core::Result<()>;
    fn TeleserviceId(&self) -> ::windows::core::Result<i32>;
    fn ProtocolId(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISmsTextMessageStaticsImpl: Sized {
    fn FromBinaryMessage(&self, binarymessage: &::core::option::Option<SmsBinaryMessage>) -> ::windows::core::Result<SmsTextMessage>;
    fn FromBinaryData(&self, format: SmsDataFormat, value: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<SmsTextMessage>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmsVoicemailMessageImpl: Sized + ISmsMessageBaseImpl {
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn To(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Body(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MessageCount(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmsWapMessageImpl: Sized + ISmsMessageBaseImpl {
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn To(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn From(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ApplicationId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BinaryBody(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn Headers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>;
}
