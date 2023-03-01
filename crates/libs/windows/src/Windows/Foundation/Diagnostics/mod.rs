#[doc(hidden)]
#[repr(transparent)]
pub struct IAsyncCausalityTracerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAsyncCausalityTracerStatics {
    type Vtable = IAsyncCausalityTracerStatics_Vtbl;
}
impl ::core::clone::Clone for IAsyncCausalityTracerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAsyncCausalityTracerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50850b26_267e_451b_a890_ab6a370245ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsyncCausalityTracerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TraceOperationCreation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: ::windows::core::GUID, operationid: u64, operationname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, relatedcontext: u64) -> ::windows::core::HRESULT,
    pub TraceOperationCompletion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: ::windows::core::GUID, operationid: u64, status: super::AsyncStatus) -> ::windows::core::HRESULT,
    pub TraceOperationRelation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: ::windows::core::GUID, operationid: u64, relation: CausalityRelation) -> ::windows::core::HRESULT,
    pub TraceSynchronousWorkStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: ::windows::core::GUID, operationid: u64, work: CausalitySynchronousWork) -> ::windows::core::HRESULT,
    pub TraceSynchronousWorkCompletion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tracelevel: CausalityTraceLevel, source: CausalitySource, work: CausalitySynchronousWork) -> ::windows::core::HRESULT,
    pub TracingStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::EventRegistrationToken) -> ::windows::core::HRESULT,
    pub RemoveTracingStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::EventRegistrationToken) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IErrorDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IErrorDetails {
    type Vtable = IErrorDetails_Vtbl;
}
impl ::core::clone::Clone for IErrorDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IErrorDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x378cbb01_2cc9_428f_8c55_2c990d463e8f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IErrorDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub LongDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub HelpUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IErrorDetailsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IErrorDetailsStatics {
    type Vtable = IErrorDetailsStatics_Vtbl;
}
impl ::core::clone::Clone for IErrorDetailsStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IErrorDetailsStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb7703750_0b1d_46c8_aa0e_4b8178e4fce9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IErrorDetailsStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateFromHResultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, errorcode: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
pub struct IErrorReportingSettings(::windows::core::IUnknown);
impl IErrorReportingSettings {
    pub fn SetErrorOptions(&self, value: ErrorOptions) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetErrorOptions)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetErrorOptions(&self) -> ::windows::core::Result<ErrorOptions> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ErrorOptions>();
            (::windows::core::Interface::vtable(this).GetErrorOptions)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IErrorReportingSettings, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::cmp::PartialEq for IErrorReportingSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IErrorReportingSettings {}
impl ::core::fmt::Debug for IErrorReportingSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IErrorReportingSettings").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IErrorReportingSettings {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{16369792-b03e-4ba1-8bb8-d28f4ab4d2c0}");
}
unsafe impl ::windows::core::Interface for IErrorReportingSettings {
    type Vtable = IErrorReportingSettings_Vtbl;
}
impl ::core::clone::Clone for IErrorReportingSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IErrorReportingSettings {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x16369792_b03e_4ba1_8bb8_d28f4ab4d2c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IErrorReportingSettings_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetErrorOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ErrorOptions) -> ::windows::core::HRESULT,
    pub GetErrorOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ErrorOptions) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
pub struct IFileLoggingSession(::windows::core::IUnknown);
impl IFileLoggingSession {
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AddLoggingChannel<P0>(&self, loggingchannel: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<ILoggingChannel>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddLoggingChannel)(::windows::core::Interface::as_raw(this), loggingchannel.try_into_param()?.abi()).ok() }
    }
    pub fn AddLoggingChannelWithLevel<P0>(&self, loggingchannel: P0, maxlevel: LoggingLevel) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<ILoggingChannel>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddLoggingChannelWithLevel)(::windows::core::Interface::as_raw(this), loggingchannel.try_into_param()?.abi(), maxlevel).ok() }
    }
    pub fn RemoveLoggingChannel<P0>(&self, loggingchannel: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<ILoggingChannel>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveLoggingChannel)(::windows::core::Interface::as_raw(this), loggingchannel.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn CloseAndSaveToFileAsync(&self) -> ::windows::core::Result<super::IAsyncOperation<super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::IAsyncOperation<super::super::Storage::StorageFile>>();
            (::windows::core::Interface::vtable(this).CloseAndSaveToFileAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LogFileGenerated(&self, handler: &super::TypedEventHandler<IFileLoggingSession, LogFileGeneratedEventArgs>) -> ::windows::core::Result<super::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).LogFileGenerated)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveLogFileGenerated(&self, token: super::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveLogFileGenerated)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
::windows::imp::interface_hierarchy!(IFileLoggingSession, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<super::IClosable> for IFileLoggingSession {}
impl ::core::cmp::PartialEq for IFileLoggingSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFileLoggingSession {}
impl ::core::fmt::Debug for IFileLoggingSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFileLoggingSession").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IFileLoggingSession {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{24c74216-fed2-404c-895f-1f9699cb02f7}");
}
unsafe impl ::windows::core::Interface for IFileLoggingSession {
    type Vtable = IFileLoggingSession_Vtbl;
}
impl ::core::clone::Clone for IFileLoggingSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IFileLoggingSession {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24c74216_fed2_404c_895f_1f9699cb02f7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileLoggingSession_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AddLoggingChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, loggingchannel: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddLoggingChannelWithLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, loggingchannel: *mut ::core::ffi::c_void, maxlevel: LoggingLevel) -> ::windows::core::HRESULT,
    pub RemoveLoggingChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, loggingchannel: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage")]
    pub CloseAndSaveToFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    CloseAndSaveToFileAsync: usize,
    pub LogFileGenerated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::EventRegistrationToken) -> ::windows::core::HRESULT,
    pub RemoveLogFileGenerated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::EventRegistrationToken) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFileLoggingSessionFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFileLoggingSessionFactory {
    type Vtable = IFileLoggingSessionFactory_Vtbl;
}
impl ::core::clone::Clone for IFileLoggingSessionFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IFileLoggingSessionFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeea08dce_8447_4daa_9133_12eb46f697d4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileLoggingSessionFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILogFileGeneratedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILogFileGeneratedEventArgs {
    type Vtable = ILogFileGeneratedEventArgs_Vtbl;
}
impl ::core::clone::Clone for ILogFileGeneratedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILogFileGeneratedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x269e976f_0d38_4c1a_b53f_b395d881df84);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILogFileGeneratedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Storage")]
    pub File: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    File: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILoggingActivity(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILoggingActivity {
    type Vtable = ILoggingActivity_Vtbl;
}
impl ::core::clone::Clone for ILoggingActivity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILoggingActivity {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc032941_b766_4cb5_9848_97ac6ba6d60c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingActivity_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILoggingActivity2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILoggingActivity2 {
    type Vtable = ILoggingActivity2_Vtbl;
}
impl ::core::clone::Clone for ILoggingActivity2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILoggingActivity2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x26c29808_6322_456a_af82_80c8642f178b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingActivity2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Channel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub StopActivity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stopeventname: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub StopActivityWithFields: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stopeventname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, fields: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub StopActivityWithFieldsAndOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stopeventname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, fields: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILoggingActivityFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILoggingActivityFactory {
    type Vtable = ILoggingActivityFactory_Vtbl;
}
impl ::core::clone::Clone for ILoggingActivityFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILoggingActivityFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b33b483_e10a_4c58_97d5_10fb451074fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingActivityFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateLoggingActivity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activityname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, loggingchannel: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateLoggingActivityWithLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activityname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, loggingchannel: *mut ::core::ffi::c_void, level: LoggingLevel, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
pub struct ILoggingChannel(::windows::core::IUnknown);
impl ILoggingChannel {
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Enabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).Enabled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Level(&self) -> ::windows::core::Result<LoggingLevel> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<LoggingLevel>();
            (::windows::core::Interface::vtable(this).Level)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LogMessage(&self, eventstring: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).LogMessage)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(eventstring)).ok() }
    }
    pub fn LogMessageWithLevel(&self, eventstring: &::windows::core::HSTRING, level: LoggingLevel) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).LogMessageWithLevel)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(eventstring), level).ok() }
    }
    pub fn LogValuePair(&self, value1: &::windows::core::HSTRING, value2: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).LogValuePair)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value1), value2).ok() }
    }
    pub fn LogValuePairWithLevel(&self, value1: &::windows::core::HSTRING, value2: i32, level: LoggingLevel) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).LogValuePairWithLevel)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value1), value2, level).ok() }
    }
    pub fn LoggingEnabled(&self, handler: &super::TypedEventHandler<ILoggingChannel, ::windows::core::IInspectable>) -> ::windows::core::Result<super::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).LoggingEnabled)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveLoggingEnabled(&self, token: super::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveLoggingEnabled)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
::windows::imp::interface_hierarchy!(ILoggingChannel, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<super::IClosable> for ILoggingChannel {}
impl ::core::cmp::PartialEq for ILoggingChannel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILoggingChannel {}
impl ::core::fmt::Debug for ILoggingChannel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILoggingChannel").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ILoggingChannel {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{e9a50343-11d7-4f01-b5ca-cf495278c0a8}");
}
unsafe impl ::windows::core::Interface for ILoggingChannel {
    type Vtable = ILoggingChannel_Vtbl;
}
impl ::core::clone::Clone for ILoggingChannel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILoggingChannel {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe9a50343_11d7_4f01_b5ca_cf495278c0a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingChannel_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Level: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LoggingLevel) -> ::windows::core::HRESULT,
    pub LogMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventstring: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub LogMessageWithLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventstring: ::std::mem::MaybeUninit<::windows::core::HSTRING>, level: LoggingLevel) -> ::windows::core::HRESULT,
    pub LogValuePair: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value1: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value2: i32) -> ::windows::core::HRESULT,
    pub LogValuePairWithLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value1: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value2: i32, level: LoggingLevel) -> ::windows::core::HRESULT,
    pub LoggingEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::EventRegistrationToken) -> ::windows::core::HRESULT,
    pub RemoveLoggingEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::EventRegistrationToken) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILoggingChannel2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILoggingChannel2 {
    type Vtable = ILoggingChannel2_Vtbl;
}
impl ::core::clone::Clone for ILoggingChannel2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILoggingChannel2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f4c3cf3_0bac_45a5_9e33_baf3f3a246a5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingChannel2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILoggingChannelFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILoggingChannelFactory {
    type Vtable = ILoggingChannelFactory_Vtbl;
}
impl ::core::clone::Clone for ILoggingChannelFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILoggingChannelFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4edc5b9c_af80_4a9b_b0dc_398f9ae5207b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingChannelFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILoggingChannelFactory2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILoggingChannelFactory2 {
    type Vtable = ILoggingChannelFactory2_Vtbl;
}
impl ::core::clone::Clone for ILoggingChannelFactory2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILoggingChannelFactory2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4c6ef5dd_3b27_4dc9_99f0_299c6e4603a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingChannelFactory2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateWithOptionsAndId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, options: *mut ::core::ffi::c_void, id: ::windows::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILoggingChannelOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILoggingChannelOptions {
    type Vtable = ILoggingChannelOptions_Vtbl;
}
impl ::core::clone::Clone for ILoggingChannelOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILoggingChannelOptions {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc3e847ff_0ebb_4a53_8c54_dec24926cb2c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingChannelOptions_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Group: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILoggingChannelOptionsFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILoggingChannelOptionsFactory {
    type Vtable = ILoggingChannelOptionsFactory_Vtbl;
}
impl ::core::clone::Clone for ILoggingChannelOptionsFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILoggingChannelOptionsFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa93151da_7faf_4191_8755_5e86dc65d896);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingChannelOptionsFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, group: ::windows::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILoggingFields(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILoggingFields {
    type Vtable = ILoggingFields_Vtbl;
}
impl ::core::clone::Clone for ILoggingFields {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILoggingFields {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7f6b7af_762d_4579_83bd_52c23bc333bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingFields_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub BeginStruct: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub BeginStructWithTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, tags: i32) -> ::windows::core::HRESULT,
    pub EndStruct: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddEmpty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AddEmptyWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddEmptyWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddUInt8: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: u8) -> ::windows::core::HRESULT,
    pub AddUInt8WithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: u8, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddUInt8WithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: u8, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddUInt8Array: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value_array_size: u32, value: *const u8) -> ::windows::core::HRESULT,
    pub AddUInt8ArrayWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value_array_size: u32, value: *const u8, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddUInt8ArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value_array_size: u32, value: *const u8, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddInt16: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: i16) -> ::windows::core::HRESULT,
    pub AddInt16WithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: i16, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddInt16WithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: i16, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddInt16Array: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value_array_size: u32, value: *const i16) -> ::windows::core::HRESULT,
    pub AddInt16ArrayWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value_array_size: u32, value: *const i16, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddInt16ArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value_array_size: u32, value: *const i16, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddUInt16: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: u16) -> ::windows::core::HRESULT,
    pub AddUInt16WithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: u16, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddUInt16WithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: u16, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddUInt16Array: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value_array_size: u32, value: *const u16) -> ::windows::core::HRESULT,
    pub AddUInt16ArrayWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value_array_size: u32, value: *const u16, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddUInt16ArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value_array_size: u32, value: *const u16, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddInt32: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: i32) -> ::windows::core::HRESULT,
    pub AddInt32WithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: i32, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddInt32WithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: i32, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddInt32Array: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value_array_size: u32, value: *const i32) -> ::windows::core::HRESULT,
    pub AddInt32ArrayWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value_array_size: u32, value: *const i32, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddInt32ArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value_array_size: u32, value: *const i32, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddUInt32: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: u32) -> ::windows::core::HRESULT,
    pub AddUInt32WithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: u32, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddUInt32WithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: u32, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddUInt32Array: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value_array_size: u32, value: *const u32) -> ::windows::core::HRESULT,
    pub AddUInt32ArrayWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value_array_size: u32, value: *const u32, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddUInt32ArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value_array_size: u32, value: *const u32, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddInt64: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: i64) -> ::windows::core::HRESULT,
    pub AddInt64WithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: i64, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddInt64WithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: i64, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddInt64Array: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value_array_size: u32, value: *const i64) -> ::windows::core::HRESULT,
    pub AddInt64ArrayWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value_array_size: u32, value: *const i64, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddInt64ArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value_array_size: u32, value: *const i64, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddUInt64: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: u64) -> ::windows::core::HRESULT,
    pub AddUInt64WithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: u64, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddUInt64WithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: u64, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddUInt64Array: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value_array_size: u32, value: *const u64) -> ::windows::core::HRESULT,
    pub AddUInt64ArrayWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value_array_size: u32, value: *const u64, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddUInt64ArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value_array_size: u32, value: *const u64, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddSingle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: f32) -> ::windows::core::HRESULT,
    pub AddSingleWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: f32, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddSingleWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: f32, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddSingleArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value_array_size: u32, value: *const f32) -> ::windows::core::HRESULT,
    pub AddSingleArrayWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value_array_size: u32, value: *const f32, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddSingleArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value_array_size: u32, value: *const f32, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddDouble: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: f64) -> ::windows::core::HRESULT,
    pub AddDoubleWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: f64, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddDoubleWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: f64, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddDoubleArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value_array_size: u32, value: *const f64) -> ::windows::core::HRESULT,
    pub AddDoubleArrayWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value_array_size: u32, value: *const f64, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddDoubleArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value_array_size: u32, value: *const f64, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddChar16: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: u16) -> ::windows::core::HRESULT,
    pub AddChar16WithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: u16, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddChar16WithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: u16, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddChar16Array: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value_array_size: u32, value: *const u16) -> ::windows::core::HRESULT,
    pub AddChar16ArrayWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value_array_size: u32, value: *const u16, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddChar16ArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value_array_size: u32, value: *const u16, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddBoolean: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: bool) -> ::windows::core::HRESULT,
    pub AddBooleanWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: bool, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddBooleanWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: bool, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddBooleanArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value_array_size: u32, value: *const bool) -> ::windows::core::HRESULT,
    pub AddBooleanArrayWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value_array_size: u32, value: *const bool, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddBooleanArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value_array_size: u32, value: *const bool, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AddStringWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddStringWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddStringArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value_array_size: u32, value: *const ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AddStringArrayWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value_array_size: u32, value: *const ::std::mem::MaybeUninit<::windows::core::HSTRING>, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddStringArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value_array_size: u32, value: *const ::std::mem::MaybeUninit<::windows::core::HSTRING>, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub AddGuidWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: ::windows::core::GUID, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddGuidWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: ::windows::core::GUID, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddGuidArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value_array_size: u32, value: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub AddGuidArrayWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value_array_size: u32, value: *const ::windows::core::GUID, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddGuidArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value_array_size: u32, value: *const ::windows::core::GUID, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddDateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: super::DateTime) -> ::windows::core::HRESULT,
    pub AddDateTimeWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: super::DateTime, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddDateTimeWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: super::DateTime, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddDateTimeArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value_array_size: u32, value: *const super::DateTime) -> ::windows::core::HRESULT,
    pub AddDateTimeArrayWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value_array_size: u32, value: *const super::DateTime, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddDateTimeArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value_array_size: u32, value: *const super::DateTime, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddTimeSpan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: super::TimeSpan) -> ::windows::core::HRESULT,
    pub AddTimeSpanWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: super::TimeSpan, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddTimeSpanWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: super::TimeSpan, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddTimeSpanArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value_array_size: u32, value: *const super::TimeSpan) -> ::windows::core::HRESULT,
    pub AddTimeSpanArrayWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value_array_size: u32, value: *const super::TimeSpan, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddTimeSpanArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value_array_size: u32, value: *const super::TimeSpan, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: super::Point) -> ::windows::core::HRESULT,
    pub AddPointWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: super::Point, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddPointWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: super::Point, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddPointArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value_array_size: u32, value: *const super::Point) -> ::windows::core::HRESULT,
    pub AddPointArrayWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value_array_size: u32, value: *const super::Point, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddPointArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value_array_size: u32, value: *const super::Point, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: super::Size) -> ::windows::core::HRESULT,
    pub AddSizeWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: super::Size, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddSizeWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: super::Size, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddSizeArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value_array_size: u32, value: *const super::Size) -> ::windows::core::HRESULT,
    pub AddSizeArrayWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value_array_size: u32, value: *const super::Size, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddSizeArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value_array_size: u32, value: *const super::Size, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: super::Rect) -> ::windows::core::HRESULT,
    pub AddRectWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: super::Rect, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddRectWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value: super::Rect, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddRectArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value_array_size: u32, value: *const super::Rect) -> ::windows::core::HRESULT,
    pub AddRectArrayWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value_array_size: u32, value: *const super::Rect, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddRectArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, value_array_size: u32, value: *const super::Rect, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILoggingOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILoggingOptions {
    type Vtable = ILoggingOptions_Vtbl;
}
impl ::core::clone::Clone for ILoggingOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILoggingOptions {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x90bc7850_0192_4f5d_ac26_006adaca12d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingOptions_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Keywords: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT,
    pub SetKeywords: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i64) -> ::windows::core::HRESULT,
    pub Tags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub Task: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i16) -> ::windows::core::HRESULT,
    pub SetTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT,
    pub Opcode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LoggingOpcode) -> ::windows::core::HRESULT,
    pub SetOpcode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: LoggingOpcode) -> ::windows::core::HRESULT,
    pub ActivityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetActivityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub RelatedActivityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetRelatedActivityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILoggingOptionsFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILoggingOptionsFactory {
    type Vtable = ILoggingOptionsFactory_Vtbl;
}
impl ::core::clone::Clone for ILoggingOptionsFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILoggingOptionsFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd713c6cb_98ab_464b_9f22_a3268478368a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingOptionsFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateWithKeywords: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keywords: i64, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
pub struct ILoggingSession(::windows::core::IUnknown);
impl ILoggingSession {
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn SaveToFileAsync<P0>(&self, folder: P0, filename: &::windows::core::HSTRING) -> ::windows::core::Result<super::IAsyncOperation<super::super::Storage::StorageFile>>
    where
        P0: ::windows::core::TryIntoParam<super::super::Storage::IStorageFolder>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::IAsyncOperation<super::super::Storage::StorageFile>>();
            (::windows::core::Interface::vtable(this).SaveToFileAsync)(::windows::core::Interface::as_raw(this), folder.try_into_param()?.abi(), ::core::mem::transmute_copy(filename), &mut result__).from_abi(result__)
        }
    }
    pub fn AddLoggingChannel<P0>(&self, loggingchannel: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<ILoggingChannel>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddLoggingChannel)(::windows::core::Interface::as_raw(this), loggingchannel.try_into_param()?.abi()).ok() }
    }
    pub fn AddLoggingChannelWithLevel<P0>(&self, loggingchannel: P0, maxlevel: LoggingLevel) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<ILoggingChannel>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddLoggingChannelWithLevel)(::windows::core::Interface::as_raw(this), loggingchannel.try_into_param()?.abi(), maxlevel).ok() }
    }
    pub fn RemoveLoggingChannel<P0>(&self, loggingchannel: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<ILoggingChannel>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveLoggingChannel)(::windows::core::Interface::as_raw(this), loggingchannel.try_into_param()?.abi()).ok() }
    }
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
::windows::imp::interface_hierarchy!(ILoggingSession, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<super::IClosable> for ILoggingSession {}
impl ::core::cmp::PartialEq for ILoggingSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILoggingSession {}
impl ::core::fmt::Debug for ILoggingSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILoggingSession").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ILoggingSession {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{6221f306-9380-4ad7-baf5-41ea9310d768}");
}
unsafe impl ::windows::core::Interface for ILoggingSession {
    type Vtable = ILoggingSession_Vtbl;
}
impl ::core::clone::Clone for ILoggingSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILoggingSession {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6221f306_9380_4ad7_baf5_41ea9310d768);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingSession_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage")]
    pub SaveToFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folder: *mut ::core::ffi::c_void, filename: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    SaveToFileAsync: usize,
    pub AddLoggingChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, loggingchannel: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddLoggingChannelWithLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, loggingchannel: *mut ::core::ffi::c_void, maxlevel: LoggingLevel) -> ::windows::core::HRESULT,
    pub RemoveLoggingChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, loggingchannel: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILoggingSessionFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILoggingSessionFactory {
    type Vtable = ILoggingSessionFactory_Vtbl;
}
impl ::core::clone::Clone for ILoggingSessionFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILoggingSessionFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e937ee5_58fd_45e0_8c2f_a132eff95c1e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingSessionFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
pub struct ILoggingTarget(::windows::core::IUnknown);
impl ILoggingTarget {
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsEnabled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsEnabledWithLevel(&self, level: LoggingLevel) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsEnabledWithLevel)(::windows::core::Interface::as_raw(this), level, &mut result__).from_abi(result__)
        }
    }
    pub fn IsEnabledWithLevelAndKeywords(&self, level: LoggingLevel, keywords: i64) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsEnabledWithLevelAndKeywords)(::windows::core::Interface::as_raw(this), level, keywords, &mut result__).from_abi(result__)
        }
    }
    pub fn LogEvent(&self, eventname: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).LogEvent)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(eventname)).ok() }
    }
    pub fn LogEventWithFields(&self, eventname: &::windows::core::HSTRING, fields: &LoggingFields) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).LogEventWithFields)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(eventname), ::core::mem::transmute_copy(fields)).ok() }
    }
    pub fn LogEventWithFieldsAndLevel(&self, eventname: &::windows::core::HSTRING, fields: &LoggingFields, level: LoggingLevel) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).LogEventWithFieldsAndLevel)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(eventname), ::core::mem::transmute_copy(fields), level).ok() }
    }
    pub fn LogEventWithFieldsAndOptions(&self, eventname: &::windows::core::HSTRING, fields: &LoggingFields, level: LoggingLevel, options: &LoggingOptions) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).LogEventWithFieldsAndOptions)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(eventname), ::core::mem::transmute_copy(fields), level, ::core::mem::transmute_copy(options)).ok() }
    }
    pub fn StartActivity(&self, starteventname: &::windows::core::HSTRING) -> ::windows::core::Result<LoggingActivity> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<LoggingActivity>();
            (::windows::core::Interface::vtable(this).StartActivity)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(starteventname), &mut result__).from_abi(result__)
        }
    }
    pub fn StartActivityWithFields(&self, starteventname: &::windows::core::HSTRING, fields: &LoggingFields) -> ::windows::core::Result<LoggingActivity> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<LoggingActivity>();
            (::windows::core::Interface::vtable(this).StartActivityWithFields)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(starteventname), ::core::mem::transmute_copy(fields), &mut result__).from_abi(result__)
        }
    }
    pub fn StartActivityWithFieldsAndLevel(&self, starteventname: &::windows::core::HSTRING, fields: &LoggingFields, level: LoggingLevel) -> ::windows::core::Result<LoggingActivity> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<LoggingActivity>();
            (::windows::core::Interface::vtable(this).StartActivityWithFieldsAndLevel)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(starteventname), ::core::mem::transmute_copy(fields), level, &mut result__).from_abi(result__)
        }
    }
    pub fn StartActivityWithFieldsAndOptions(&self, starteventname: &::windows::core::HSTRING, fields: &LoggingFields, level: LoggingLevel, options: &LoggingOptions) -> ::windows::core::Result<LoggingActivity> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<LoggingActivity>();
            (::windows::core::Interface::vtable(this).StartActivityWithFieldsAndOptions)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(starteventname), ::core::mem::transmute_copy(fields), level, ::core::mem::transmute_copy(options), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(ILoggingTarget, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::cmp::PartialEq for ILoggingTarget {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILoggingTarget {}
impl ::core::fmt::Debug for ILoggingTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILoggingTarget").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ILoggingTarget {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{65f16c35-e388-4e26-b17a-f51cd3a83916}");
}
unsafe impl ::windows::core::Interface for ILoggingTarget {
    type Vtable = ILoggingTarget_Vtbl;
}
impl ::core::clone::Clone for ILoggingTarget {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILoggingTarget {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x65f16c35_e388_4e26_b17a_f51cd3a83916);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingTarget_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsEnabledWithLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, level: LoggingLevel, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsEnabledWithLevelAndKeywords: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, level: LoggingLevel, keywords: i64, result__: *mut bool) -> ::windows::core::HRESULT,
    pub LogEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventname: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub LogEventWithFields: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, fields: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub LogEventWithFieldsAndLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, fields: *mut ::core::ffi::c_void, level: LoggingLevel) -> ::windows::core::HRESULT,
    pub LogEventWithFieldsAndOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, fields: *mut ::core::ffi::c_void, level: LoggingLevel, options: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub StartActivity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, starteventname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub StartActivityWithFields: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, starteventname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, fields: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub StartActivityWithFieldsAndLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, starteventname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, fields: *mut ::core::ffi::c_void, level: LoggingLevel, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub StartActivityWithFieldsAndOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, starteventname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, fields: *mut ::core::ffi::c_void, level: LoggingLevel, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITracingStatusChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITracingStatusChangedEventArgs {
    type Vtable = ITracingStatusChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for ITracingStatusChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITracingStatusChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x410b7711_ff3b_477f_9c9a_d2efda302dc3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITracingStatusChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub TraceLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CausalityTraceLevel) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
pub struct AsyncCausalityTracer;
impl AsyncCausalityTracer {
    pub fn TraceOperationCreation(tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: ::windows::core::GUID, operationid: u64, operationname: &::windows::core::HSTRING, relatedcontext: u64) -> ::windows::core::Result<()> {
        Self::IAsyncCausalityTracerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).TraceOperationCreation)(::windows::core::Interface::as_raw(this), tracelevel, source, platformid, operationid, ::core::mem::transmute_copy(operationname), relatedcontext).ok() })
    }
    pub fn TraceOperationCompletion(tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: ::windows::core::GUID, operationid: u64, status: super::AsyncStatus) -> ::windows::core::Result<()> {
        Self::IAsyncCausalityTracerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).TraceOperationCompletion)(::windows::core::Interface::as_raw(this), tracelevel, source, platformid, operationid, status).ok() })
    }
    pub fn TraceOperationRelation(tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: ::windows::core::GUID, operationid: u64, relation: CausalityRelation) -> ::windows::core::Result<()> {
        Self::IAsyncCausalityTracerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).TraceOperationRelation)(::windows::core::Interface::as_raw(this), tracelevel, source, platformid, operationid, relation).ok() })
    }
    pub fn TraceSynchronousWorkStart(tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: ::windows::core::GUID, operationid: u64, work: CausalitySynchronousWork) -> ::windows::core::Result<()> {
        Self::IAsyncCausalityTracerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).TraceSynchronousWorkStart)(::windows::core::Interface::as_raw(this), tracelevel, source, platformid, operationid, work).ok() })
    }
    pub fn TraceSynchronousWorkCompletion(tracelevel: CausalityTraceLevel, source: CausalitySource, work: CausalitySynchronousWork) -> ::windows::core::Result<()> {
        Self::IAsyncCausalityTracerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).TraceSynchronousWorkCompletion)(::windows::core::Interface::as_raw(this), tracelevel, source, work).ok() })
    }
    pub fn TracingStatusChanged(handler: &super::EventHandler<TracingStatusChangedEventArgs>) -> ::windows::core::Result<super::EventRegistrationToken> {
        Self::IAsyncCausalityTracerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).TracingStatusChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        })
    }
    pub fn RemoveTracingStatusChanged(cookie: super::EventRegistrationToken) -> ::windows::core::Result<()> {
        Self::IAsyncCausalityTracerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveTracingStatusChanged)(::windows::core::Interface::as_raw(this), cookie).ok() })
    }
    #[doc(hidden)]
    pub fn IAsyncCausalityTracerStatics<R, F: FnOnce(&IAsyncCausalityTracerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<AsyncCausalityTracer, IAsyncCausalityTracerStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for AsyncCausalityTracer {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.AsyncCausalityTracer";
}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
pub struct ErrorDetails(::windows::core::IUnknown);
impl ErrorDetails {
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Description)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LongDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).LongDescription)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HelpUri(&self) -> ::windows::core::Result<super::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Uri>();
            (::windows::core::Interface::vtable(this).HelpUri)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateFromHResultAsync(errorcode: i32) -> ::windows::core::Result<super::IAsyncOperation<ErrorDetails>> {
        Self::IErrorDetailsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::IAsyncOperation<ErrorDetails>>();
            (::windows::core::Interface::vtable(this).CreateFromHResultAsync)(::windows::core::Interface::as_raw(this), errorcode, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IErrorDetailsStatics<R, F: FnOnce(&IErrorDetailsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<ErrorDetails, IErrorDetailsStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for ErrorDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ErrorDetails {}
impl ::core::fmt::Debug for ErrorDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ErrorDetails").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ErrorDetails {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.ErrorDetails;{378cbb01-2cc9-428f-8c55-2c990d463e8f})");
}
impl ::core::clone::Clone for ErrorDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ErrorDetails {
    type Vtable = IErrorDetails_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ErrorDetails {
    const IID: ::windows::core::GUID = <IErrorDetails as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ErrorDetails {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.ErrorDetails";
}
::windows::imp::interface_hierarchy!(ErrorDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ErrorDetails {}
unsafe impl ::core::marker::Sync for ErrorDetails {}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
pub struct FileLoggingSession(::windows::core::IUnknown);
impl FileLoggingSession {
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AddLoggingChannel<P0>(&self, loggingchannel: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<ILoggingChannel>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddLoggingChannel)(::windows::core::Interface::as_raw(this), loggingchannel.try_into_param()?.abi()).ok() }
    }
    pub fn AddLoggingChannelWithLevel<P0>(&self, loggingchannel: P0, maxlevel: LoggingLevel) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<ILoggingChannel>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddLoggingChannelWithLevel)(::windows::core::Interface::as_raw(this), loggingchannel.try_into_param()?.abi(), maxlevel).ok() }
    }
    pub fn RemoveLoggingChannel<P0>(&self, loggingchannel: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<ILoggingChannel>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveLoggingChannel)(::windows::core::Interface::as_raw(this), loggingchannel.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn CloseAndSaveToFileAsync(&self) -> ::windows::core::Result<super::IAsyncOperation<super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::IAsyncOperation<super::super::Storage::StorageFile>>();
            (::windows::core::Interface::vtable(this).CloseAndSaveToFileAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LogFileGenerated(&self, handler: &super::TypedEventHandler<IFileLoggingSession, LogFileGeneratedEventArgs>) -> ::windows::core::Result<super::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).LogFileGenerated)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveLogFileGenerated(&self, token: super::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveLogFileGenerated)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    pub fn Create(name: &::windows::core::HSTRING) -> ::windows::core::Result<FileLoggingSession> {
        Self::IFileLoggingSessionFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<FileLoggingSession>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IFileLoggingSessionFactory<R, F: FnOnce(&IFileLoggingSessionFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<FileLoggingSession, IFileLoggingSessionFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for FileLoggingSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FileLoggingSession {}
impl ::core::fmt::Debug for FileLoggingSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileLoggingSession").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for FileLoggingSession {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.FileLoggingSession;{24c74216-fed2-404c-895f-1f9699cb02f7})");
}
impl ::core::clone::Clone for FileLoggingSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for FileLoggingSession {
    type Vtable = IFileLoggingSession_Vtbl;
}
unsafe impl ::windows::core::ComInterface for FileLoggingSession {
    const IID: ::windows::core::GUID = <IFileLoggingSession as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for FileLoggingSession {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.FileLoggingSession";
}
::windows::imp::interface_hierarchy!(FileLoggingSession, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<super::IClosable> for FileLoggingSession {}
impl ::windows::core::CanTryInto<IFileLoggingSession> for FileLoggingSession {}
unsafe impl ::core::marker::Send for FileLoggingSession {}
unsafe impl ::core::marker::Sync for FileLoggingSession {}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
pub struct LogFileGeneratedEventArgs(::windows::core::IUnknown);
impl LogFileGeneratedEventArgs {
    #[doc = "*Required features: `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn File(&self) -> ::windows::core::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::StorageFile>();
            (::windows::core::Interface::vtable(this).File)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for LogFileGeneratedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LogFileGeneratedEventArgs {}
impl ::core::fmt::Debug for LogFileGeneratedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LogFileGeneratedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for LogFileGeneratedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.LogFileGeneratedEventArgs;{269e976f-0d38-4c1a-b53f-b395d881df84})");
}
impl ::core::clone::Clone for LogFileGeneratedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for LogFileGeneratedEventArgs {
    type Vtable = ILogFileGeneratedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for LogFileGeneratedEventArgs {
    const IID: ::windows::core::GUID = <ILogFileGeneratedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for LogFileGeneratedEventArgs {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.LogFileGeneratedEventArgs";
}
::windows::imp::interface_hierarchy!(LogFileGeneratedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for LogFileGeneratedEventArgs {}
unsafe impl ::core::marker::Sync for LogFileGeneratedEventArgs {}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
pub struct LoggingActivity(::windows::core::IUnknown);
impl LoggingActivity {
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Channel(&self) -> ::windows::core::Result<LoggingChannel> {
        let this = &::windows::core::ComInterface::cast::<ILoggingActivity2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<LoggingChannel>();
            (::windows::core::Interface::vtable(this).Channel)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StopActivity(&self, stopeventname: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ILoggingActivity2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StopActivity)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(stopeventname)).ok() }
    }
    pub fn StopActivityWithFields(&self, stopeventname: &::windows::core::HSTRING, fields: &LoggingFields) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ILoggingActivity2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StopActivityWithFields)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(stopeventname), ::core::mem::transmute_copy(fields)).ok() }
    }
    pub fn StopActivityWithFieldsAndOptions(&self, stopeventname: &::windows::core::HSTRING, fields: &LoggingFields, options: &LoggingOptions) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ILoggingActivity2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StopActivityWithFieldsAndOptions)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(stopeventname), ::core::mem::transmute_copy(fields), ::core::mem::transmute_copy(options)).ok() }
    }
    pub fn CreateLoggingActivity<P0>(activityname: &::windows::core::HSTRING, loggingchannel: P0) -> ::windows::core::Result<LoggingActivity>
    where
        P0: ::windows::core::TryIntoParam<ILoggingChannel>,
    {
        Self::ILoggingActivityFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<LoggingActivity>();
            (::windows::core::Interface::vtable(this).CreateLoggingActivity)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(activityname), loggingchannel.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateLoggingActivityWithLevel<P0>(activityname: &::windows::core::HSTRING, loggingchannel: P0, level: LoggingLevel) -> ::windows::core::Result<LoggingActivity>
    where
        P0: ::windows::core::TryIntoParam<ILoggingChannel>,
    {
        Self::ILoggingActivityFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<LoggingActivity>();
            (::windows::core::Interface::vtable(this).CreateLoggingActivityWithLevel)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(activityname), loggingchannel.try_into_param()?.abi(), level, &mut result__).from_abi(result__)
        })
    }
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsEnabled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsEnabledWithLevel(&self, level: LoggingLevel) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsEnabledWithLevel)(::windows::core::Interface::as_raw(this), level, &mut result__).from_abi(result__)
        }
    }
    pub fn IsEnabledWithLevelAndKeywords(&self, level: LoggingLevel, keywords: i64) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsEnabledWithLevelAndKeywords)(::windows::core::Interface::as_raw(this), level, keywords, &mut result__).from_abi(result__)
        }
    }
    pub fn LogEvent(&self, eventname: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ILoggingTarget>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).LogEvent)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(eventname)).ok() }
    }
    pub fn LogEventWithFields(&self, eventname: &::windows::core::HSTRING, fields: &LoggingFields) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ILoggingTarget>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).LogEventWithFields)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(eventname), ::core::mem::transmute_copy(fields)).ok() }
    }
    pub fn LogEventWithFieldsAndLevel(&self, eventname: &::windows::core::HSTRING, fields: &LoggingFields, level: LoggingLevel) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ILoggingTarget>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).LogEventWithFieldsAndLevel)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(eventname), ::core::mem::transmute_copy(fields), level).ok() }
    }
    pub fn LogEventWithFieldsAndOptions(&self, eventname: &::windows::core::HSTRING, fields: &LoggingFields, level: LoggingLevel, options: &LoggingOptions) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ILoggingTarget>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).LogEventWithFieldsAndOptions)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(eventname), ::core::mem::transmute_copy(fields), level, ::core::mem::transmute_copy(options)).ok() }
    }
    pub fn StartActivity(&self, starteventname: &::windows::core::HSTRING) -> ::windows::core::Result<LoggingActivity> {
        let this = &::windows::core::ComInterface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<LoggingActivity>();
            (::windows::core::Interface::vtable(this).StartActivity)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(starteventname), &mut result__).from_abi(result__)
        }
    }
    pub fn StartActivityWithFields(&self, starteventname: &::windows::core::HSTRING, fields: &LoggingFields) -> ::windows::core::Result<LoggingActivity> {
        let this = &::windows::core::ComInterface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<LoggingActivity>();
            (::windows::core::Interface::vtable(this).StartActivityWithFields)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(starteventname), ::core::mem::transmute_copy(fields), &mut result__).from_abi(result__)
        }
    }
    pub fn StartActivityWithFieldsAndLevel(&self, starteventname: &::windows::core::HSTRING, fields: &LoggingFields, level: LoggingLevel) -> ::windows::core::Result<LoggingActivity> {
        let this = &::windows::core::ComInterface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<LoggingActivity>();
            (::windows::core::Interface::vtable(this).StartActivityWithFieldsAndLevel)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(starteventname), ::core::mem::transmute_copy(fields), level, &mut result__).from_abi(result__)
        }
    }
    pub fn StartActivityWithFieldsAndOptions(&self, starteventname: &::windows::core::HSTRING, fields: &LoggingFields, level: LoggingLevel, options: &LoggingOptions) -> ::windows::core::Result<LoggingActivity> {
        let this = &::windows::core::ComInterface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<LoggingActivity>();
            (::windows::core::Interface::vtable(this).StartActivityWithFieldsAndOptions)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(starteventname), ::core::mem::transmute_copy(fields), level, ::core::mem::transmute_copy(options), &mut result__).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn ILoggingActivityFactory<R, F: FnOnce(&ILoggingActivityFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<LoggingActivity, ILoggingActivityFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for LoggingActivity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LoggingActivity {}
impl ::core::fmt::Debug for LoggingActivity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LoggingActivity").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for LoggingActivity {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.LoggingActivity;{bc032941-b766-4cb5-9848-97ac6ba6d60c})");
}
impl ::core::clone::Clone for LoggingActivity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for LoggingActivity {
    type Vtable = ILoggingActivity_Vtbl;
}
unsafe impl ::windows::core::ComInterface for LoggingActivity {
    const IID: ::windows::core::GUID = <ILoggingActivity as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for LoggingActivity {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.LoggingActivity";
}
::windows::imp::interface_hierarchy!(LoggingActivity, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<super::IClosable> for LoggingActivity {}
impl ::windows::core::CanTryInto<ILoggingTarget> for LoggingActivity {}
unsafe impl ::core::marker::Send for LoggingActivity {}
unsafe impl ::core::marker::Sync for LoggingActivity {}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
pub struct LoggingChannel(::windows::core::IUnknown);
impl LoggingChannel {
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Enabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).Enabled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Level(&self) -> ::windows::core::Result<LoggingLevel> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<LoggingLevel>();
            (::windows::core::Interface::vtable(this).Level)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LogMessage(&self, eventstring: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).LogMessage)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(eventstring)).ok() }
    }
    pub fn LogMessageWithLevel(&self, eventstring: &::windows::core::HSTRING, level: LoggingLevel) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).LogMessageWithLevel)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(eventstring), level).ok() }
    }
    pub fn LogValuePair(&self, value1: &::windows::core::HSTRING, value2: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).LogValuePair)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value1), value2).ok() }
    }
    pub fn LogValuePairWithLevel(&self, value1: &::windows::core::HSTRING, value2: i32, level: LoggingLevel) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).LogValuePairWithLevel)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value1), value2, level).ok() }
    }
    pub fn LoggingEnabled(&self, handler: &super::TypedEventHandler<ILoggingChannel, ::windows::core::IInspectable>) -> ::windows::core::Result<super::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).LoggingEnabled)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveLoggingEnabled(&self, token: super::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveLoggingEnabled)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::ComInterface::cast::<ILoggingChannel2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Create(name: &::windows::core::HSTRING) -> ::windows::core::Result<LoggingChannel> {
        Self::ILoggingChannelFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<LoggingChannel>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateWithOptions(name: &::windows::core::HSTRING, options: &LoggingChannelOptions) -> ::windows::core::Result<LoggingChannel> {
        Self::ILoggingChannelFactory2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<LoggingChannel>();
            (::windows::core::Interface::vtable(this).CreateWithOptions)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(options), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateWithOptionsAndId(name: &::windows::core::HSTRING, options: &LoggingChannelOptions, id: ::windows::core::GUID) -> ::windows::core::Result<LoggingChannel> {
        Self::ILoggingChannelFactory2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<LoggingChannel>();
            (::windows::core::Interface::vtable(this).CreateWithOptionsAndId)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(options), id, &mut result__).from_abi(result__)
        })
    }
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsEnabled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsEnabledWithLevel(&self, level: LoggingLevel) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsEnabledWithLevel)(::windows::core::Interface::as_raw(this), level, &mut result__).from_abi(result__)
        }
    }
    pub fn IsEnabledWithLevelAndKeywords(&self, level: LoggingLevel, keywords: i64) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsEnabledWithLevelAndKeywords)(::windows::core::Interface::as_raw(this), level, keywords, &mut result__).from_abi(result__)
        }
    }
    pub fn LogEvent(&self, eventname: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ILoggingTarget>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).LogEvent)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(eventname)).ok() }
    }
    pub fn LogEventWithFields(&self, eventname: &::windows::core::HSTRING, fields: &LoggingFields) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ILoggingTarget>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).LogEventWithFields)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(eventname), ::core::mem::transmute_copy(fields)).ok() }
    }
    pub fn LogEventWithFieldsAndLevel(&self, eventname: &::windows::core::HSTRING, fields: &LoggingFields, level: LoggingLevel) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ILoggingTarget>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).LogEventWithFieldsAndLevel)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(eventname), ::core::mem::transmute_copy(fields), level).ok() }
    }
    pub fn LogEventWithFieldsAndOptions(&self, eventname: &::windows::core::HSTRING, fields: &LoggingFields, level: LoggingLevel, options: &LoggingOptions) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ILoggingTarget>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).LogEventWithFieldsAndOptions)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(eventname), ::core::mem::transmute_copy(fields), level, ::core::mem::transmute_copy(options)).ok() }
    }
    pub fn StartActivity(&self, starteventname: &::windows::core::HSTRING) -> ::windows::core::Result<LoggingActivity> {
        let this = &::windows::core::ComInterface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<LoggingActivity>();
            (::windows::core::Interface::vtable(this).StartActivity)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(starteventname), &mut result__).from_abi(result__)
        }
    }
    pub fn StartActivityWithFields(&self, starteventname: &::windows::core::HSTRING, fields: &LoggingFields) -> ::windows::core::Result<LoggingActivity> {
        let this = &::windows::core::ComInterface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<LoggingActivity>();
            (::windows::core::Interface::vtable(this).StartActivityWithFields)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(starteventname), ::core::mem::transmute_copy(fields), &mut result__).from_abi(result__)
        }
    }
    pub fn StartActivityWithFieldsAndLevel(&self, starteventname: &::windows::core::HSTRING, fields: &LoggingFields, level: LoggingLevel) -> ::windows::core::Result<LoggingActivity> {
        let this = &::windows::core::ComInterface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<LoggingActivity>();
            (::windows::core::Interface::vtable(this).StartActivityWithFieldsAndLevel)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(starteventname), ::core::mem::transmute_copy(fields), level, &mut result__).from_abi(result__)
        }
    }
    pub fn StartActivityWithFieldsAndOptions(&self, starteventname: &::windows::core::HSTRING, fields: &LoggingFields, level: LoggingLevel, options: &LoggingOptions) -> ::windows::core::Result<LoggingActivity> {
        let this = &::windows::core::ComInterface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<LoggingActivity>();
            (::windows::core::Interface::vtable(this).StartActivityWithFieldsAndOptions)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(starteventname), ::core::mem::transmute_copy(fields), level, ::core::mem::transmute_copy(options), &mut result__).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn ILoggingChannelFactory<R, F: FnOnce(&ILoggingChannelFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<LoggingChannel, ILoggingChannelFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ILoggingChannelFactory2<R, F: FnOnce(&ILoggingChannelFactory2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<LoggingChannel, ILoggingChannelFactory2> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for LoggingChannel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LoggingChannel {}
impl ::core::fmt::Debug for LoggingChannel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LoggingChannel").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for LoggingChannel {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.LoggingChannel;{e9a50343-11d7-4f01-b5ca-cf495278c0a8})");
}
impl ::core::clone::Clone for LoggingChannel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for LoggingChannel {
    type Vtable = ILoggingChannel_Vtbl;
}
unsafe impl ::windows::core::ComInterface for LoggingChannel {
    const IID: ::windows::core::GUID = <ILoggingChannel as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for LoggingChannel {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.LoggingChannel";
}
::windows::imp::interface_hierarchy!(LoggingChannel, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<super::IClosable> for LoggingChannel {}
impl ::windows::core::CanTryInto<ILoggingChannel> for LoggingChannel {}
impl ::windows::core::CanTryInto<ILoggingTarget> for LoggingChannel {}
unsafe impl ::core::marker::Send for LoggingChannel {}
unsafe impl ::core::marker::Sync for LoggingChannel {}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
pub struct LoggingChannelOptions(::windows::core::IUnknown);
impl LoggingChannelOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<LoggingChannelOptions, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Group(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).Group)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetGroup(&self, value: ::windows::core::GUID) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetGroup)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Create(group: ::windows::core::GUID) -> ::windows::core::Result<LoggingChannelOptions> {
        Self::ILoggingChannelOptionsFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<LoggingChannelOptions>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), group, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILoggingChannelOptionsFactory<R, F: FnOnce(&ILoggingChannelOptionsFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<LoggingChannelOptions, ILoggingChannelOptionsFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for LoggingChannelOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LoggingChannelOptions {}
impl ::core::fmt::Debug for LoggingChannelOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LoggingChannelOptions").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for LoggingChannelOptions {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.LoggingChannelOptions;{c3e847ff-0ebb-4a53-8c54-dec24926cb2c})");
}
impl ::core::clone::Clone for LoggingChannelOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for LoggingChannelOptions {
    type Vtable = ILoggingChannelOptions_Vtbl;
}
unsafe impl ::windows::core::ComInterface for LoggingChannelOptions {
    const IID: ::windows::core::GUID = <ILoggingChannelOptions as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for LoggingChannelOptions {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.LoggingChannelOptions";
}
::windows::imp::interface_hierarchy!(LoggingChannelOptions, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for LoggingChannelOptions {}
unsafe impl ::core::marker::Sync for LoggingChannelOptions {}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
pub struct LoggingFields(::windows::core::IUnknown);
impl LoggingFields {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<LoggingFields, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Clear)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn BeginStruct(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).BeginStruct)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name)).ok() }
    }
    pub fn BeginStructWithTags(&self, name: &::windows::core::HSTRING, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).BeginStructWithTags)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), tags).ok() }
    }
    pub fn EndStruct(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).EndStruct)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn AddEmpty(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddEmpty)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name)).ok() }
    }
    pub fn AddEmptyWithFormat(&self, name: &::windows::core::HSTRING, format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddEmptyWithFormat)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), format).ok() }
    }
    pub fn AddEmptyWithFormatAndTags(&self, name: &::windows::core::HSTRING, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddEmptyWithFormatAndTags)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), format, tags).ok() }
    }
    pub fn AddUInt8(&self, name: &::windows::core::HSTRING, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddUInt8)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value).ok() }
    }
    pub fn AddUInt8WithFormat(&self, name: &::windows::core::HSTRING, value: u8, format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddUInt8WithFormat)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format).ok() }
    }
    pub fn AddUInt8WithFormatAndTags(&self, name: &::windows::core::HSTRING, value: u8, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddUInt8WithFormatAndTags)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format, tags).ok() }
    }
    pub fn AddUInt8Array(&self, name: &::windows::core::HSTRING, value: &[u8]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddUInt8Array)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr()).ok() }
    }
    pub fn AddUInt8ArrayWithFormat(&self, name: &::windows::core::HSTRING, value: &[u8], format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddUInt8ArrayWithFormat)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format).ok() }
    }
    pub fn AddUInt8ArrayWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &[u8], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddUInt8ArrayWithFormatAndTags)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format, tags).ok() }
    }
    pub fn AddInt16(&self, name: &::windows::core::HSTRING, value: i16) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddInt16)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value).ok() }
    }
    pub fn AddInt16WithFormat(&self, name: &::windows::core::HSTRING, value: i16, format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddInt16WithFormat)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format).ok() }
    }
    pub fn AddInt16WithFormatAndTags(&self, name: &::windows::core::HSTRING, value: i16, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddInt16WithFormatAndTags)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format, tags).ok() }
    }
    pub fn AddInt16Array(&self, name: &::windows::core::HSTRING, value: &[i16]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddInt16Array)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr()).ok() }
    }
    pub fn AddInt16ArrayWithFormat(&self, name: &::windows::core::HSTRING, value: &[i16], format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddInt16ArrayWithFormat)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format).ok() }
    }
    pub fn AddInt16ArrayWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &[i16], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddInt16ArrayWithFormatAndTags)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format, tags).ok() }
    }
    pub fn AddUInt16(&self, name: &::windows::core::HSTRING, value: u16) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddUInt16)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value).ok() }
    }
    pub fn AddUInt16WithFormat(&self, name: &::windows::core::HSTRING, value: u16, format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddUInt16WithFormat)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format).ok() }
    }
    pub fn AddUInt16WithFormatAndTags(&self, name: &::windows::core::HSTRING, value: u16, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddUInt16WithFormatAndTags)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format, tags).ok() }
    }
    pub fn AddUInt16Array(&self, name: &::windows::core::HSTRING, value: &[u16]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddUInt16Array)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr()).ok() }
    }
    pub fn AddUInt16ArrayWithFormat(&self, name: &::windows::core::HSTRING, value: &[u16], format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddUInt16ArrayWithFormat)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format).ok() }
    }
    pub fn AddUInt16ArrayWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &[u16], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddUInt16ArrayWithFormatAndTags)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format, tags).ok() }
    }
    pub fn AddInt32(&self, name: &::windows::core::HSTRING, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddInt32)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value).ok() }
    }
    pub fn AddInt32WithFormat(&self, name: &::windows::core::HSTRING, value: i32, format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddInt32WithFormat)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format).ok() }
    }
    pub fn AddInt32WithFormatAndTags(&self, name: &::windows::core::HSTRING, value: i32, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddInt32WithFormatAndTags)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format, tags).ok() }
    }
    pub fn AddInt32Array(&self, name: &::windows::core::HSTRING, value: &[i32]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddInt32Array)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr()).ok() }
    }
    pub fn AddInt32ArrayWithFormat(&self, name: &::windows::core::HSTRING, value: &[i32], format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddInt32ArrayWithFormat)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format).ok() }
    }
    pub fn AddInt32ArrayWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &[i32], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddInt32ArrayWithFormatAndTags)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format, tags).ok() }
    }
    pub fn AddUInt32(&self, name: &::windows::core::HSTRING, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddUInt32)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value).ok() }
    }
    pub fn AddUInt32WithFormat(&self, name: &::windows::core::HSTRING, value: u32, format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddUInt32WithFormat)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format).ok() }
    }
    pub fn AddUInt32WithFormatAndTags(&self, name: &::windows::core::HSTRING, value: u32, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddUInt32WithFormatAndTags)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format, tags).ok() }
    }
    pub fn AddUInt32Array(&self, name: &::windows::core::HSTRING, value: &[u32]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddUInt32Array)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr()).ok() }
    }
    pub fn AddUInt32ArrayWithFormat(&self, name: &::windows::core::HSTRING, value: &[u32], format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddUInt32ArrayWithFormat)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format).ok() }
    }
    pub fn AddUInt32ArrayWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &[u32], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddUInt32ArrayWithFormatAndTags)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format, tags).ok() }
    }
    pub fn AddInt64(&self, name: &::windows::core::HSTRING, value: i64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddInt64)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value).ok() }
    }
    pub fn AddInt64WithFormat(&self, name: &::windows::core::HSTRING, value: i64, format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddInt64WithFormat)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format).ok() }
    }
    pub fn AddInt64WithFormatAndTags(&self, name: &::windows::core::HSTRING, value: i64, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddInt64WithFormatAndTags)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format, tags).ok() }
    }
    pub fn AddInt64Array(&self, name: &::windows::core::HSTRING, value: &[i64]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddInt64Array)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr()).ok() }
    }
    pub fn AddInt64ArrayWithFormat(&self, name: &::windows::core::HSTRING, value: &[i64], format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddInt64ArrayWithFormat)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format).ok() }
    }
    pub fn AddInt64ArrayWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &[i64], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddInt64ArrayWithFormatAndTags)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format, tags).ok() }
    }
    pub fn AddUInt64(&self, name: &::windows::core::HSTRING, value: u64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddUInt64)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value).ok() }
    }
    pub fn AddUInt64WithFormat(&self, name: &::windows::core::HSTRING, value: u64, format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddUInt64WithFormat)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format).ok() }
    }
    pub fn AddUInt64WithFormatAndTags(&self, name: &::windows::core::HSTRING, value: u64, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddUInt64WithFormatAndTags)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format, tags).ok() }
    }
    pub fn AddUInt64Array(&self, name: &::windows::core::HSTRING, value: &[u64]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddUInt64Array)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr()).ok() }
    }
    pub fn AddUInt64ArrayWithFormat(&self, name: &::windows::core::HSTRING, value: &[u64], format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddUInt64ArrayWithFormat)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format).ok() }
    }
    pub fn AddUInt64ArrayWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &[u64], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddUInt64ArrayWithFormatAndTags)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format, tags).ok() }
    }
    pub fn AddSingle(&self, name: &::windows::core::HSTRING, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddSingle)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value).ok() }
    }
    pub fn AddSingleWithFormat(&self, name: &::windows::core::HSTRING, value: f32, format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddSingleWithFormat)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format).ok() }
    }
    pub fn AddSingleWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: f32, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddSingleWithFormatAndTags)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format, tags).ok() }
    }
    pub fn AddSingleArray(&self, name: &::windows::core::HSTRING, value: &[f32]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddSingleArray)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr()).ok() }
    }
    pub fn AddSingleArrayWithFormat(&self, name: &::windows::core::HSTRING, value: &[f32], format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddSingleArrayWithFormat)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format).ok() }
    }
    pub fn AddSingleArrayWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &[f32], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddSingleArrayWithFormatAndTags)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format, tags).ok() }
    }
    pub fn AddDouble(&self, name: &::windows::core::HSTRING, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddDouble)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value).ok() }
    }
    pub fn AddDoubleWithFormat(&self, name: &::windows::core::HSTRING, value: f64, format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddDoubleWithFormat)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format).ok() }
    }
    pub fn AddDoubleWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: f64, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddDoubleWithFormatAndTags)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format, tags).ok() }
    }
    pub fn AddDoubleArray(&self, name: &::windows::core::HSTRING, value: &[f64]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddDoubleArray)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr()).ok() }
    }
    pub fn AddDoubleArrayWithFormat(&self, name: &::windows::core::HSTRING, value: &[f64], format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddDoubleArrayWithFormat)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format).ok() }
    }
    pub fn AddDoubleArrayWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &[f64], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddDoubleArrayWithFormatAndTags)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format, tags).ok() }
    }
    pub fn AddChar16(&self, name: &::windows::core::HSTRING, value: u16) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddChar16)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value).ok() }
    }
    pub fn AddChar16WithFormat(&self, name: &::windows::core::HSTRING, value: u16, format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddChar16WithFormat)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format).ok() }
    }
    pub fn AddChar16WithFormatAndTags(&self, name: &::windows::core::HSTRING, value: u16, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddChar16WithFormatAndTags)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format, tags).ok() }
    }
    pub fn AddChar16Array(&self, name: &::windows::core::HSTRING, value: &[u16]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddChar16Array)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr()).ok() }
    }
    pub fn AddChar16ArrayWithFormat(&self, name: &::windows::core::HSTRING, value: &[u16], format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddChar16ArrayWithFormat)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format).ok() }
    }
    pub fn AddChar16ArrayWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &[u16], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddChar16ArrayWithFormatAndTags)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format, tags).ok() }
    }
    pub fn AddBoolean(&self, name: &::windows::core::HSTRING, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddBoolean)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value).ok() }
    }
    pub fn AddBooleanWithFormat(&self, name: &::windows::core::HSTRING, value: bool, format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddBooleanWithFormat)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format).ok() }
    }
    pub fn AddBooleanWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: bool, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddBooleanWithFormatAndTags)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format, tags).ok() }
    }
    pub fn AddBooleanArray(&self, name: &::windows::core::HSTRING, value: &[bool]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddBooleanArray)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr()).ok() }
    }
    pub fn AddBooleanArrayWithFormat(&self, name: &::windows::core::HSTRING, value: &[bool], format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddBooleanArrayWithFormat)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format).ok() }
    }
    pub fn AddBooleanArrayWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &[bool], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddBooleanArrayWithFormatAndTags)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format, tags).ok() }
    }
    pub fn AddString(&self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddString)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn AddStringWithFormat(&self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING, format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddStringWithFormat)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(value), format).ok() }
    }
    pub fn AddStringWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddStringWithFormatAndTags)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(value), format, tags).ok() }
    }
    pub fn AddStringArray(&self, name: &::windows::core::HSTRING, value: &[::windows::core::HSTRING]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddStringArray)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    pub fn AddStringArrayWithFormat(&self, name: &::windows::core::HSTRING, value: &[::windows::core::HSTRING], format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddStringArrayWithFormat)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format).ok() }
    }
    pub fn AddStringArrayWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &[::windows::core::HSTRING], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddStringArrayWithFormatAndTags)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    pub fn AddGuid(&self, name: &::windows::core::HSTRING, value: ::windows::core::GUID) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddGuid)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value).ok() }
    }
    pub fn AddGuidWithFormat(&self, name: &::windows::core::HSTRING, value: ::windows::core::GUID, format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddGuidWithFormat)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format).ok() }
    }
    pub fn AddGuidWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: ::windows::core::GUID, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddGuidWithFormatAndTags)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format, tags).ok() }
    }
    pub fn AddGuidArray(&self, name: &::windows::core::HSTRING, value: &[::windows::core::GUID]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddGuidArray)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr()).ok() }
    }
    pub fn AddGuidArrayWithFormat(&self, name: &::windows::core::HSTRING, value: &[::windows::core::GUID], format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddGuidArrayWithFormat)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format).ok() }
    }
    pub fn AddGuidArrayWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &[::windows::core::GUID], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddGuidArrayWithFormatAndTags)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format, tags).ok() }
    }
    pub fn AddDateTime(&self, name: &::windows::core::HSTRING, value: super::DateTime) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddDateTime)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value).ok() }
    }
    pub fn AddDateTimeWithFormat(&self, name: &::windows::core::HSTRING, value: super::DateTime, format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddDateTimeWithFormat)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format).ok() }
    }
    pub fn AddDateTimeWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: super::DateTime, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddDateTimeWithFormatAndTags)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format, tags).ok() }
    }
    pub fn AddDateTimeArray(&self, name: &::windows::core::HSTRING, value: &[super::DateTime]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddDateTimeArray)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr()).ok() }
    }
    pub fn AddDateTimeArrayWithFormat(&self, name: &::windows::core::HSTRING, value: &[super::DateTime], format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddDateTimeArrayWithFormat)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format).ok() }
    }
    pub fn AddDateTimeArrayWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &[super::DateTime], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddDateTimeArrayWithFormatAndTags)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format, tags).ok() }
    }
    pub fn AddTimeSpan(&self, name: &::windows::core::HSTRING, value: super::TimeSpan) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddTimeSpan)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value).ok() }
    }
    pub fn AddTimeSpanWithFormat(&self, name: &::windows::core::HSTRING, value: super::TimeSpan, format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddTimeSpanWithFormat)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format).ok() }
    }
    pub fn AddTimeSpanWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: super::TimeSpan, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddTimeSpanWithFormatAndTags)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format, tags).ok() }
    }
    pub fn AddTimeSpanArray(&self, name: &::windows::core::HSTRING, value: &[super::TimeSpan]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddTimeSpanArray)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr()).ok() }
    }
    pub fn AddTimeSpanArrayWithFormat(&self, name: &::windows::core::HSTRING, value: &[super::TimeSpan], format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddTimeSpanArrayWithFormat)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format).ok() }
    }
    pub fn AddTimeSpanArrayWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &[super::TimeSpan], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddTimeSpanArrayWithFormatAndTags)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format, tags).ok() }
    }
    pub fn AddPoint(&self, name: &::windows::core::HSTRING, value: super::Point) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddPoint)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value).ok() }
    }
    pub fn AddPointWithFormat(&self, name: &::windows::core::HSTRING, value: super::Point, format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddPointWithFormat)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format).ok() }
    }
    pub fn AddPointWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: super::Point, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddPointWithFormatAndTags)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format, tags).ok() }
    }
    pub fn AddPointArray(&self, name: &::windows::core::HSTRING, value: &[super::Point]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddPointArray)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr()).ok() }
    }
    pub fn AddPointArrayWithFormat(&self, name: &::windows::core::HSTRING, value: &[super::Point], format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddPointArrayWithFormat)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format).ok() }
    }
    pub fn AddPointArrayWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &[super::Point], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddPointArrayWithFormatAndTags)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format, tags).ok() }
    }
    pub fn AddSize(&self, name: &::windows::core::HSTRING, value: super::Size) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddSize)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value).ok() }
    }
    pub fn AddSizeWithFormat(&self, name: &::windows::core::HSTRING, value: super::Size, format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddSizeWithFormat)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format).ok() }
    }
    pub fn AddSizeWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: super::Size, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddSizeWithFormatAndTags)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format, tags).ok() }
    }
    pub fn AddSizeArray(&self, name: &::windows::core::HSTRING, value: &[super::Size]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddSizeArray)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr()).ok() }
    }
    pub fn AddSizeArrayWithFormat(&self, name: &::windows::core::HSTRING, value: &[super::Size], format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddSizeArrayWithFormat)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format).ok() }
    }
    pub fn AddSizeArrayWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &[super::Size], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddSizeArrayWithFormatAndTags)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format, tags).ok() }
    }
    pub fn AddRect(&self, name: &::windows::core::HSTRING, value: super::Rect) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddRect)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value).ok() }
    }
    pub fn AddRectWithFormat(&self, name: &::windows::core::HSTRING, value: super::Rect, format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddRectWithFormat)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format).ok() }
    }
    pub fn AddRectWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: super::Rect, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddRectWithFormatAndTags)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format, tags).ok() }
    }
    pub fn AddRectArray(&self, name: &::windows::core::HSTRING, value: &[super::Rect]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddRectArray)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr()).ok() }
    }
    pub fn AddRectArrayWithFormat(&self, name: &::windows::core::HSTRING, value: &[super::Rect], format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddRectArrayWithFormat)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format).ok() }
    }
    pub fn AddRectArrayWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &[super::Rect], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddRectArrayWithFormatAndTags)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format, tags).ok() }
    }
}
impl ::core::cmp::PartialEq for LoggingFields {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LoggingFields {}
impl ::core::fmt::Debug for LoggingFields {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LoggingFields").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for LoggingFields {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.LoggingFields;{d7f6b7af-762d-4579-83bd-52c23bc333bc})");
}
impl ::core::clone::Clone for LoggingFields {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for LoggingFields {
    type Vtable = ILoggingFields_Vtbl;
}
unsafe impl ::windows::core::ComInterface for LoggingFields {
    const IID: ::windows::core::GUID = <ILoggingFields as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for LoggingFields {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.LoggingFields";
}
::windows::imp::interface_hierarchy!(LoggingFields, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for LoggingFields {}
unsafe impl ::core::marker::Sync for LoggingFields {}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
pub struct LoggingOptions(::windows::core::IUnknown);
impl LoggingOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<LoggingOptions, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Keywords(&self) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i64>();
            (::windows::core::Interface::vtable(this).Keywords)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetKeywords(&self, value: i64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetKeywords)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Tags(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).Tags)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTags(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTags)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Task(&self) -> ::windows::core::Result<i16> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i16>();
            (::windows::core::Interface::vtable(this).Task)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTask(&self, value: i16) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTask)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Opcode(&self) -> ::windows::core::Result<LoggingOpcode> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<LoggingOpcode>();
            (::windows::core::Interface::vtable(this).Opcode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetOpcode(&self, value: LoggingOpcode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetOpcode)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ActivityId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).ActivityId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetActivityId(&self, value: ::windows::core::GUID) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetActivityId)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn RelatedActivityId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).RelatedActivityId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRelatedActivityId(&self, value: ::windows::core::GUID) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRelatedActivityId)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn CreateWithKeywords(keywords: i64) -> ::windows::core::Result<LoggingOptions> {
        Self::ILoggingOptionsFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<LoggingOptions>();
            (::windows::core::Interface::vtable(this).CreateWithKeywords)(::windows::core::Interface::as_raw(this), keywords, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILoggingOptionsFactory<R, F: FnOnce(&ILoggingOptionsFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<LoggingOptions, ILoggingOptionsFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for LoggingOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LoggingOptions {}
impl ::core::fmt::Debug for LoggingOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LoggingOptions").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for LoggingOptions {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.LoggingOptions;{90bc7850-0192-4f5d-ac26-006adaca12d8})");
}
impl ::core::clone::Clone for LoggingOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for LoggingOptions {
    type Vtable = ILoggingOptions_Vtbl;
}
unsafe impl ::windows::core::ComInterface for LoggingOptions {
    const IID: ::windows::core::GUID = <ILoggingOptions as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for LoggingOptions {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.LoggingOptions";
}
::windows::imp::interface_hierarchy!(LoggingOptions, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for LoggingOptions {}
unsafe impl ::core::marker::Sync for LoggingOptions {}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
pub struct LoggingSession(::windows::core::IUnknown);
impl LoggingSession {
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn SaveToFileAsync<P0>(&self, folder: P0, filename: &::windows::core::HSTRING) -> ::windows::core::Result<super::IAsyncOperation<super::super::Storage::StorageFile>>
    where
        P0: ::windows::core::TryIntoParam<super::super::Storage::IStorageFolder>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::IAsyncOperation<super::super::Storage::StorageFile>>();
            (::windows::core::Interface::vtable(this).SaveToFileAsync)(::windows::core::Interface::as_raw(this), folder.try_into_param()?.abi(), ::core::mem::transmute_copy(filename), &mut result__).from_abi(result__)
        }
    }
    pub fn AddLoggingChannel<P0>(&self, loggingchannel: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<ILoggingChannel>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddLoggingChannel)(::windows::core::Interface::as_raw(this), loggingchannel.try_into_param()?.abi()).ok() }
    }
    pub fn AddLoggingChannelWithLevel<P0>(&self, loggingchannel: P0, maxlevel: LoggingLevel) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<ILoggingChannel>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddLoggingChannelWithLevel)(::windows::core::Interface::as_raw(this), loggingchannel.try_into_param()?.abi(), maxlevel).ok() }
    }
    pub fn RemoveLoggingChannel<P0>(&self, loggingchannel: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<ILoggingChannel>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveLoggingChannel)(::windows::core::Interface::as_raw(this), loggingchannel.try_into_param()?.abi()).ok() }
    }
    pub fn Create(name: &::windows::core::HSTRING) -> ::windows::core::Result<LoggingSession> {
        Self::ILoggingSessionFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<LoggingSession>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILoggingSessionFactory<R, F: FnOnce(&ILoggingSessionFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<LoggingSession, ILoggingSessionFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for LoggingSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LoggingSession {}
impl ::core::fmt::Debug for LoggingSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LoggingSession").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for LoggingSession {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.LoggingSession;{6221f306-9380-4ad7-baf5-41ea9310d768})");
}
impl ::core::clone::Clone for LoggingSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for LoggingSession {
    type Vtable = ILoggingSession_Vtbl;
}
unsafe impl ::windows::core::ComInterface for LoggingSession {
    const IID: ::windows::core::GUID = <ILoggingSession as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for LoggingSession {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.LoggingSession";
}
::windows::imp::interface_hierarchy!(LoggingSession, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<super::IClosable> for LoggingSession {}
impl ::windows::core::CanTryInto<ILoggingSession> for LoggingSession {}
unsafe impl ::core::marker::Send for LoggingSession {}
unsafe impl ::core::marker::Sync for LoggingSession {}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
pub struct RuntimeBrokerErrorSettings(::windows::core::IUnknown);
impl RuntimeBrokerErrorSettings {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<RuntimeBrokerErrorSettings, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetErrorOptions(&self, value: ErrorOptions) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetErrorOptions)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetErrorOptions(&self) -> ::windows::core::Result<ErrorOptions> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ErrorOptions>();
            (::windows::core::Interface::vtable(this).GetErrorOptions)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for RuntimeBrokerErrorSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RuntimeBrokerErrorSettings {}
impl ::core::fmt::Debug for RuntimeBrokerErrorSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RuntimeBrokerErrorSettings").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for RuntimeBrokerErrorSettings {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.RuntimeBrokerErrorSettings;{16369792-b03e-4ba1-8bb8-d28f4ab4d2c0})");
}
impl ::core::clone::Clone for RuntimeBrokerErrorSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for RuntimeBrokerErrorSettings {
    type Vtable = IErrorReportingSettings_Vtbl;
}
unsafe impl ::windows::core::ComInterface for RuntimeBrokerErrorSettings {
    const IID: ::windows::core::GUID = <IErrorReportingSettings as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for RuntimeBrokerErrorSettings {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.RuntimeBrokerErrorSettings";
}
::windows::imp::interface_hierarchy!(RuntimeBrokerErrorSettings, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IErrorReportingSettings> for RuntimeBrokerErrorSettings {}
unsafe impl ::core::marker::Send for RuntimeBrokerErrorSettings {}
unsafe impl ::core::marker::Sync for RuntimeBrokerErrorSettings {}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
pub struct TracingStatusChangedEventArgs(::windows::core::IUnknown);
impl TracingStatusChangedEventArgs {
    pub fn Enabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).Enabled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TraceLevel(&self) -> ::windows::core::Result<CausalityTraceLevel> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CausalityTraceLevel>();
            (::windows::core::Interface::vtable(this).TraceLevel)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for TracingStatusChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TracingStatusChangedEventArgs {}
impl ::core::fmt::Debug for TracingStatusChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TracingStatusChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for TracingStatusChangedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.TracingStatusChangedEventArgs;{410b7711-ff3b-477f-9c9a-d2efda302dc3})");
}
impl ::core::clone::Clone for TracingStatusChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for TracingStatusChangedEventArgs {
    type Vtable = ITracingStatusChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for TracingStatusChangedEventArgs {
    const IID: ::windows::core::GUID = <ITracingStatusChangedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for TracingStatusChangedEventArgs {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.TracingStatusChangedEventArgs";
}
::windows::imp::interface_hierarchy!(TracingStatusChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for TracingStatusChangedEventArgs {}
unsafe impl ::core::marker::Sync for TracingStatusChangedEventArgs {}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CausalityRelation(pub i32);
impl CausalityRelation {
    pub const AssignDelegate: Self = Self(0i32);
    pub const Join: Self = Self(1i32);
    pub const Choice: Self = Self(2i32);
    pub const Cancel: Self = Self(3i32);
    pub const Error: Self = Self(4i32);
}
impl ::core::marker::Copy for CausalityRelation {}
impl ::core::clone::Clone for CausalityRelation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CausalityRelation {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CausalityRelation {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CausalityRelation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CausalityRelation").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CausalityRelation {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Foundation.Diagnostics.CausalityRelation;i4)");
}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CausalitySource(pub i32);
impl CausalitySource {
    pub const Application: Self = Self(0i32);
    pub const Library: Self = Self(1i32);
    pub const System: Self = Self(2i32);
}
impl ::core::marker::Copy for CausalitySource {}
impl ::core::clone::Clone for CausalitySource {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CausalitySource {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CausalitySource {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CausalitySource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CausalitySource").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CausalitySource {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Foundation.Diagnostics.CausalitySource;i4)");
}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CausalitySynchronousWork(pub i32);
impl CausalitySynchronousWork {
    pub const CompletionNotification: Self = Self(0i32);
    pub const ProgressNotification: Self = Self(1i32);
    pub const Execution: Self = Self(2i32);
}
impl ::core::marker::Copy for CausalitySynchronousWork {}
impl ::core::clone::Clone for CausalitySynchronousWork {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CausalitySynchronousWork {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CausalitySynchronousWork {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CausalitySynchronousWork {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CausalitySynchronousWork").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CausalitySynchronousWork {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Foundation.Diagnostics.CausalitySynchronousWork;i4)");
}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CausalityTraceLevel(pub i32);
impl CausalityTraceLevel {
    pub const Required: Self = Self(0i32);
    pub const Important: Self = Self(1i32);
    pub const Verbose: Self = Self(2i32);
}
impl ::core::marker::Copy for CausalityTraceLevel {}
impl ::core::clone::Clone for CausalityTraceLevel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CausalityTraceLevel {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CausalityTraceLevel {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CausalityTraceLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CausalityTraceLevel").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CausalityTraceLevel {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Foundation.Diagnostics.CausalityTraceLevel;i4)");
}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ErrorOptions(pub u32);
impl ErrorOptions {
    pub const None: Self = Self(0u32);
    pub const SuppressExceptions: Self = Self(1u32);
    pub const ForceExceptions: Self = Self(2u32);
    pub const UseSetErrorInfo: Self = Self(4u32);
    pub const SuppressSetErrorInfo: Self = Self(8u32);
}
impl ::core::marker::Copy for ErrorOptions {}
impl ::core::clone::Clone for ErrorOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ErrorOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for ErrorOptions {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for ErrorOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ErrorOptions").field(&self.0).finish()
    }
}
impl ErrorOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for ErrorOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for ErrorOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for ErrorOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for ErrorOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for ErrorOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows::core::RuntimeType for ErrorOptions {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Foundation.Diagnostics.ErrorOptions;u4)");
}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LoggingFieldFormat(pub i32);
impl LoggingFieldFormat {
    pub const Default: Self = Self(0i32);
    pub const Hidden: Self = Self(1i32);
    pub const String: Self = Self(2i32);
    pub const Boolean: Self = Self(3i32);
    pub const Hexadecimal: Self = Self(4i32);
    pub const ProcessId: Self = Self(5i32);
    pub const ThreadId: Self = Self(6i32);
    pub const Port: Self = Self(7i32);
    pub const Ipv4Address: Self = Self(8i32);
    pub const Ipv6Address: Self = Self(9i32);
    pub const SocketAddress: Self = Self(10i32);
    pub const Xml: Self = Self(11i32);
    pub const Json: Self = Self(12i32);
    pub const Win32Error: Self = Self(13i32);
    pub const NTStatus: Self = Self(14i32);
    pub const HResult: Self = Self(15i32);
    pub const FileTime: Self = Self(16i32);
    pub const Signed: Self = Self(17i32);
    pub const Unsigned: Self = Self(18i32);
}
impl ::core::marker::Copy for LoggingFieldFormat {}
impl ::core::clone::Clone for LoggingFieldFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LoggingFieldFormat {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for LoggingFieldFormat {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for LoggingFieldFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LoggingFieldFormat").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for LoggingFieldFormat {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Foundation.Diagnostics.LoggingFieldFormat;i4)");
}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LoggingLevel(pub i32);
impl LoggingLevel {
    pub const Verbose: Self = Self(0i32);
    pub const Information: Self = Self(1i32);
    pub const Warning: Self = Self(2i32);
    pub const Error: Self = Self(3i32);
    pub const Critical: Self = Self(4i32);
}
impl ::core::marker::Copy for LoggingLevel {}
impl ::core::clone::Clone for LoggingLevel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LoggingLevel {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for LoggingLevel {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for LoggingLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LoggingLevel").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for LoggingLevel {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Foundation.Diagnostics.LoggingLevel;i4)");
}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LoggingOpcode(pub i32);
impl LoggingOpcode {
    pub const Info: Self = Self(0i32);
    pub const Start: Self = Self(1i32);
    pub const Stop: Self = Self(2i32);
    pub const Reply: Self = Self(6i32);
    pub const Resume: Self = Self(7i32);
    pub const Suspend: Self = Self(8i32);
    pub const Send: Self = Self(9i32);
}
impl ::core::marker::Copy for LoggingOpcode {}
impl ::core::clone::Clone for LoggingOpcode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LoggingOpcode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for LoggingOpcode {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for LoggingOpcode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LoggingOpcode").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for LoggingOpcode {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Foundation.Diagnostics.LoggingOpcode;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
