#[doc(hidden)]
#[repr(transparent)]
pub struct IAsyncCausalityTracerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAsyncCausalityTracerStatics {
    type Vtable = IAsyncCausalityTracerStatics_Vtbl;
}
impl ::core::clone::Clone for IAsyncCausalityTracerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAsyncCausalityTracerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x50850b26_267e_451b_a890_ab6a370245ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsyncCausalityTracerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TraceOperationCreation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: ::windows_core::GUID, operationid: u64, operationname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, relatedcontext: u64) -> ::windows_core::HRESULT,
    pub TraceOperationCompletion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: ::windows_core::GUID, operationid: u64, status: super::AsyncStatus) -> ::windows_core::HRESULT,
    pub TraceOperationRelation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: ::windows_core::GUID, operationid: u64, relation: CausalityRelation) -> ::windows_core::HRESULT,
    pub TraceSynchronousWorkStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: ::windows_core::GUID, operationid: u64, work: CausalitySynchronousWork) -> ::windows_core::HRESULT,
    pub TraceSynchronousWorkCompletion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tracelevel: CausalityTraceLevel, source: CausalitySource, work: CausalitySynchronousWork) -> ::windows_core::HRESULT,
    pub TracingStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveTracingStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IErrorDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IErrorDetails {
    type Vtable = IErrorDetails_Vtbl;
}
impl ::core::clone::Clone for IErrorDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IErrorDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x378cbb01_2cc9_428f_8c55_2c990d463e8f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IErrorDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub LongDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub HelpUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IErrorDetailsStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IErrorDetailsStatics {
    type Vtable = IErrorDetailsStatics_Vtbl;
}
impl ::core::clone::Clone for IErrorDetailsStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IErrorDetailsStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb7703750_0b1d_46c8_aa0e_4b8178e4fce9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IErrorDetailsStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateFromHResultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, errorcode: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
pub struct IErrorReportingSettings(::windows_core::IUnknown);
impl IErrorReportingSettings {
    pub fn SetErrorOptions(&self, value: ErrorOptions) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetErrorOptions)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetErrorOptions(&self) -> ::windows_core::Result<ErrorOptions> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetErrorOptions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IErrorReportingSettings, ::windows_core::IUnknown, ::windows_core::IInspectable);
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
impl ::windows_core::RuntimeType for IErrorReportingSettings {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{16369792-b03e-4ba1-8bb8-d28f4ab4d2c0}");
}
unsafe impl ::windows_core::Interface for IErrorReportingSettings {
    type Vtable = IErrorReportingSettings_Vtbl;
}
impl ::core::clone::Clone for IErrorReportingSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IErrorReportingSettings {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x16369792_b03e_4ba1_8bb8_d28f4ab4d2c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IErrorReportingSettings_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetErrorOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ErrorOptions) -> ::windows_core::HRESULT,
    pub GetErrorOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ErrorOptions) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
pub struct IFileLoggingSession(::windows_core::IUnknown);
impl IFileLoggingSession {
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AddLoggingChannel<P0>(&self, loggingchannel: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<ILoggingChannel>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddLoggingChannel)(::windows_core::Interface::as_raw(this), loggingchannel.try_into_param()?.abi()).ok() }
    }
    pub fn AddLoggingChannelWithLevel<P0>(&self, loggingchannel: P0, maxlevel: LoggingLevel) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<ILoggingChannel>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddLoggingChannelWithLevel)(::windows_core::Interface::as_raw(this), loggingchannel.try_into_param()?.abi(), maxlevel).ok() }
    }
    pub fn RemoveLoggingChannel<P0>(&self, loggingchannel: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<ILoggingChannel>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveLoggingChannel)(::windows_core::Interface::as_raw(this), loggingchannel.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn CloseAndSaveToFileAsync(&self) -> ::windows_core::Result<super::IAsyncOperation<super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CloseAndSaveToFileAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LogFileGenerated<P0>(&self, handler: P0) -> ::windows_core::Result<super::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::TypedEventHandler<IFileLoggingSession, LogFileGeneratedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LogFileGenerated)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveLogFileGenerated(&self, token: super::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveLogFileGenerated)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
::windows_core::imp::interface_hierarchy!(IFileLoggingSession, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<super::IClosable> for IFileLoggingSession {}
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
impl ::windows_core::RuntimeType for IFileLoggingSession {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{24c74216-fed2-404c-895f-1f9699cb02f7}");
}
unsafe impl ::windows_core::Interface for IFileLoggingSession {
    type Vtable = IFileLoggingSession_Vtbl;
}
impl ::core::clone::Clone for IFileLoggingSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IFileLoggingSession {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x24c74216_fed2_404c_895f_1f9699cb02f7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileLoggingSession_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AddLoggingChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, loggingchannel: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AddLoggingChannelWithLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, loggingchannel: *mut ::core::ffi::c_void, maxlevel: LoggingLevel) -> ::windows_core::HRESULT,
    pub RemoveLoggingChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, loggingchannel: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage")]
    pub CloseAndSaveToFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    CloseAndSaveToFileAsync: usize,
    pub LogFileGenerated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveLogFileGenerated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFileLoggingSessionFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFileLoggingSessionFactory {
    type Vtable = IFileLoggingSessionFactory_Vtbl;
}
impl ::core::clone::Clone for IFileLoggingSessionFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IFileLoggingSessionFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeea08dce_8447_4daa_9133_12eb46f697d4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileLoggingSessionFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILogFileGeneratedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILogFileGeneratedEventArgs {
    type Vtable = ILogFileGeneratedEventArgs_Vtbl;
}
impl ::core::clone::Clone for ILogFileGeneratedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILogFileGeneratedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x269e976f_0d38_4c1a_b53f_b395d881df84);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILogFileGeneratedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage")]
    pub File: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    File: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILoggingActivity(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILoggingActivity {
    type Vtable = ILoggingActivity_Vtbl;
}
impl ::core::clone::Clone for ILoggingActivity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILoggingActivity {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbc032941_b766_4cb5_9848_97ac6ba6d60c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingActivity_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILoggingActivity2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILoggingActivity2 {
    type Vtable = ILoggingActivity2_Vtbl;
}
impl ::core::clone::Clone for ILoggingActivity2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILoggingActivity2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x26c29808_6322_456a_af82_80c8642f178b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingActivity2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Channel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub StopActivity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stopeventname: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub StopActivityWithFields: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stopeventname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, fields: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub StopActivityWithFieldsAndOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stopeventname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, fields: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILoggingActivityFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILoggingActivityFactory {
    type Vtable = ILoggingActivityFactory_Vtbl;
}
impl ::core::clone::Clone for ILoggingActivityFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILoggingActivityFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6b33b483_e10a_4c58_97d5_10fb451074fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingActivityFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateLoggingActivity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activityname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, loggingchannel: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateLoggingActivityWithLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activityname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, loggingchannel: *mut ::core::ffi::c_void, level: LoggingLevel, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
pub struct ILoggingChannel(::windows_core::IUnknown);
impl ILoggingChannel {
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Enabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Enabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Level(&self) -> ::windows_core::Result<LoggingLevel> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Level)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LogMessage(&self, eventstring: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).LogMessage)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(eventstring)).ok() }
    }
    pub fn LogMessageWithLevel(&self, eventstring: &::windows_core::HSTRING, level: LoggingLevel) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).LogMessageWithLevel)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(eventstring), level).ok() }
    }
    pub fn LogValuePair(&self, value1: &::windows_core::HSTRING, value2: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).LogValuePair)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value1), value2).ok() }
    }
    pub fn LogValuePairWithLevel(&self, value1: &::windows_core::HSTRING, value2: i32, level: LoggingLevel) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).LogValuePairWithLevel)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value1), value2, level).ok() }
    }
    pub fn LoggingEnabled<P0>(&self, handler: P0) -> ::windows_core::Result<super::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::TypedEventHandler<ILoggingChannel, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LoggingEnabled)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveLoggingEnabled(&self, token: super::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveLoggingEnabled)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
::windows_core::imp::interface_hierarchy!(ILoggingChannel, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<super::IClosable> for ILoggingChannel {}
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
impl ::windows_core::RuntimeType for ILoggingChannel {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{e9a50343-11d7-4f01-b5ca-cf495278c0a8}");
}
unsafe impl ::windows_core::Interface for ILoggingChannel {
    type Vtable = ILoggingChannel_Vtbl;
}
impl ::core::clone::Clone for ILoggingChannel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILoggingChannel {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe9a50343_11d7_4f01_b5ca_cf495278c0a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingChannel_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Level: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LoggingLevel) -> ::windows_core::HRESULT,
    pub LogMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventstring: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub LogMessageWithLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventstring: ::std::mem::MaybeUninit<::windows_core::HSTRING>, level: LoggingLevel) -> ::windows_core::HRESULT,
    pub LogValuePair: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value1: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value2: i32) -> ::windows_core::HRESULT,
    pub LogValuePairWithLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value1: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value2: i32, level: LoggingLevel) -> ::windows_core::HRESULT,
    pub LoggingEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveLoggingEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILoggingChannel2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILoggingChannel2 {
    type Vtable = ILoggingChannel2_Vtbl;
}
impl ::core::clone::Clone for ILoggingChannel2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILoggingChannel2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9f4c3cf3_0bac_45a5_9e33_baf3f3a246a5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingChannel2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILoggingChannelFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILoggingChannelFactory {
    type Vtable = ILoggingChannelFactory_Vtbl;
}
impl ::core::clone::Clone for ILoggingChannelFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILoggingChannelFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4edc5b9c_af80_4a9b_b0dc_398f9ae5207b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingChannelFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILoggingChannelFactory2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILoggingChannelFactory2 {
    type Vtable = ILoggingChannelFactory2_Vtbl;
}
impl ::core::clone::Clone for ILoggingChannelFactory2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILoggingChannelFactory2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4c6ef5dd_3b27_4dc9_99f0_299c6e4603a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingChannelFactory2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateWithOptionsAndId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, options: *mut ::core::ffi::c_void, id: ::windows_core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILoggingChannelOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILoggingChannelOptions {
    type Vtable = ILoggingChannelOptions_Vtbl;
}
impl ::core::clone::Clone for ILoggingChannelOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILoggingChannelOptions {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc3e847ff_0ebb_4a53_8c54_dec24926cb2c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingChannelOptions_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Group: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub SetGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILoggingChannelOptionsFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILoggingChannelOptionsFactory {
    type Vtable = ILoggingChannelOptionsFactory_Vtbl;
}
impl ::core::clone::Clone for ILoggingChannelOptionsFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILoggingChannelOptionsFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa93151da_7faf_4191_8755_5e86dc65d896);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingChannelOptionsFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, group: ::windows_core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILoggingFields(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILoggingFields {
    type Vtable = ILoggingFields_Vtbl;
}
impl ::core::clone::Clone for ILoggingFields {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILoggingFields {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd7f6b7af_762d_4579_83bd_52c23bc333bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingFields_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub BeginStruct: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub BeginStructWithTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, tags: i32) -> ::windows_core::HRESULT,
    pub EndStruct: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AddEmpty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AddEmptyWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, format: LoggingFieldFormat) -> ::windows_core::HRESULT,
    pub AddEmptyWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, format: LoggingFieldFormat, tags: i32) -> ::windows_core::HRESULT,
    pub AddUInt8: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: u8) -> ::windows_core::HRESULT,
    pub AddUInt8WithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: u8, format: LoggingFieldFormat) -> ::windows_core::HRESULT,
    pub AddUInt8WithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: u8, format: LoggingFieldFormat, tags: i32) -> ::windows_core::HRESULT,
    pub AddUInt8Array: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value_array_size: u32, value: *const u8) -> ::windows_core::HRESULT,
    pub AddUInt8ArrayWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value_array_size: u32, value: *const u8, format: LoggingFieldFormat) -> ::windows_core::HRESULT,
    pub AddUInt8ArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value_array_size: u32, value: *const u8, format: LoggingFieldFormat, tags: i32) -> ::windows_core::HRESULT,
    pub AddInt16: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: i16) -> ::windows_core::HRESULT,
    pub AddInt16WithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: i16, format: LoggingFieldFormat) -> ::windows_core::HRESULT,
    pub AddInt16WithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: i16, format: LoggingFieldFormat, tags: i32) -> ::windows_core::HRESULT,
    pub AddInt16Array: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value_array_size: u32, value: *const i16) -> ::windows_core::HRESULT,
    pub AddInt16ArrayWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value_array_size: u32, value: *const i16, format: LoggingFieldFormat) -> ::windows_core::HRESULT,
    pub AddInt16ArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value_array_size: u32, value: *const i16, format: LoggingFieldFormat, tags: i32) -> ::windows_core::HRESULT,
    pub AddUInt16: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: u16) -> ::windows_core::HRESULT,
    pub AddUInt16WithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: u16, format: LoggingFieldFormat) -> ::windows_core::HRESULT,
    pub AddUInt16WithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: u16, format: LoggingFieldFormat, tags: i32) -> ::windows_core::HRESULT,
    pub AddUInt16Array: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value_array_size: u32, value: *const u16) -> ::windows_core::HRESULT,
    pub AddUInt16ArrayWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value_array_size: u32, value: *const u16, format: LoggingFieldFormat) -> ::windows_core::HRESULT,
    pub AddUInt16ArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value_array_size: u32, value: *const u16, format: LoggingFieldFormat, tags: i32) -> ::windows_core::HRESULT,
    pub AddInt32: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: i32) -> ::windows_core::HRESULT,
    pub AddInt32WithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: i32, format: LoggingFieldFormat) -> ::windows_core::HRESULT,
    pub AddInt32WithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: i32, format: LoggingFieldFormat, tags: i32) -> ::windows_core::HRESULT,
    pub AddInt32Array: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value_array_size: u32, value: *const i32) -> ::windows_core::HRESULT,
    pub AddInt32ArrayWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value_array_size: u32, value: *const i32, format: LoggingFieldFormat) -> ::windows_core::HRESULT,
    pub AddInt32ArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value_array_size: u32, value: *const i32, format: LoggingFieldFormat, tags: i32) -> ::windows_core::HRESULT,
    pub AddUInt32: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: u32) -> ::windows_core::HRESULT,
    pub AddUInt32WithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: u32, format: LoggingFieldFormat) -> ::windows_core::HRESULT,
    pub AddUInt32WithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: u32, format: LoggingFieldFormat, tags: i32) -> ::windows_core::HRESULT,
    pub AddUInt32Array: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value_array_size: u32, value: *const u32) -> ::windows_core::HRESULT,
    pub AddUInt32ArrayWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value_array_size: u32, value: *const u32, format: LoggingFieldFormat) -> ::windows_core::HRESULT,
    pub AddUInt32ArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value_array_size: u32, value: *const u32, format: LoggingFieldFormat, tags: i32) -> ::windows_core::HRESULT,
    pub AddInt64: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: i64) -> ::windows_core::HRESULT,
    pub AddInt64WithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: i64, format: LoggingFieldFormat) -> ::windows_core::HRESULT,
    pub AddInt64WithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: i64, format: LoggingFieldFormat, tags: i32) -> ::windows_core::HRESULT,
    pub AddInt64Array: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value_array_size: u32, value: *const i64) -> ::windows_core::HRESULT,
    pub AddInt64ArrayWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value_array_size: u32, value: *const i64, format: LoggingFieldFormat) -> ::windows_core::HRESULT,
    pub AddInt64ArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value_array_size: u32, value: *const i64, format: LoggingFieldFormat, tags: i32) -> ::windows_core::HRESULT,
    pub AddUInt64: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: u64) -> ::windows_core::HRESULT,
    pub AddUInt64WithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: u64, format: LoggingFieldFormat) -> ::windows_core::HRESULT,
    pub AddUInt64WithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: u64, format: LoggingFieldFormat, tags: i32) -> ::windows_core::HRESULT,
    pub AddUInt64Array: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value_array_size: u32, value: *const u64) -> ::windows_core::HRESULT,
    pub AddUInt64ArrayWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value_array_size: u32, value: *const u64, format: LoggingFieldFormat) -> ::windows_core::HRESULT,
    pub AddUInt64ArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value_array_size: u32, value: *const u64, format: LoggingFieldFormat, tags: i32) -> ::windows_core::HRESULT,
    pub AddSingle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: f32) -> ::windows_core::HRESULT,
    pub AddSingleWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: f32, format: LoggingFieldFormat) -> ::windows_core::HRESULT,
    pub AddSingleWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: f32, format: LoggingFieldFormat, tags: i32) -> ::windows_core::HRESULT,
    pub AddSingleArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value_array_size: u32, value: *const f32) -> ::windows_core::HRESULT,
    pub AddSingleArrayWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value_array_size: u32, value: *const f32, format: LoggingFieldFormat) -> ::windows_core::HRESULT,
    pub AddSingleArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value_array_size: u32, value: *const f32, format: LoggingFieldFormat, tags: i32) -> ::windows_core::HRESULT,
    pub AddDouble: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: f64) -> ::windows_core::HRESULT,
    pub AddDoubleWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: f64, format: LoggingFieldFormat) -> ::windows_core::HRESULT,
    pub AddDoubleWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: f64, format: LoggingFieldFormat, tags: i32) -> ::windows_core::HRESULT,
    pub AddDoubleArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value_array_size: u32, value: *const f64) -> ::windows_core::HRESULT,
    pub AddDoubleArrayWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value_array_size: u32, value: *const f64, format: LoggingFieldFormat) -> ::windows_core::HRESULT,
    pub AddDoubleArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value_array_size: u32, value: *const f64, format: LoggingFieldFormat, tags: i32) -> ::windows_core::HRESULT,
    pub AddChar16: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: u16) -> ::windows_core::HRESULT,
    pub AddChar16WithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: u16, format: LoggingFieldFormat) -> ::windows_core::HRESULT,
    pub AddChar16WithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: u16, format: LoggingFieldFormat, tags: i32) -> ::windows_core::HRESULT,
    pub AddChar16Array: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value_array_size: u32, value: *const u16) -> ::windows_core::HRESULT,
    pub AddChar16ArrayWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value_array_size: u32, value: *const u16, format: LoggingFieldFormat) -> ::windows_core::HRESULT,
    pub AddChar16ArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value_array_size: u32, value: *const u16, format: LoggingFieldFormat, tags: i32) -> ::windows_core::HRESULT,
    pub AddBoolean: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: bool) -> ::windows_core::HRESULT,
    pub AddBooleanWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: bool, format: LoggingFieldFormat) -> ::windows_core::HRESULT,
    pub AddBooleanWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: bool, format: LoggingFieldFormat, tags: i32) -> ::windows_core::HRESULT,
    pub AddBooleanArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value_array_size: u32, value: *const bool) -> ::windows_core::HRESULT,
    pub AddBooleanArrayWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value_array_size: u32, value: *const bool, format: LoggingFieldFormat) -> ::windows_core::HRESULT,
    pub AddBooleanArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value_array_size: u32, value: *const bool, format: LoggingFieldFormat, tags: i32) -> ::windows_core::HRESULT,
    pub AddString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AddStringWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>, format: LoggingFieldFormat) -> ::windows_core::HRESULT,
    pub AddStringWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>, format: LoggingFieldFormat, tags: i32) -> ::windows_core::HRESULT,
    pub AddStringArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value_array_size: u32, value: *const ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AddStringArrayWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value_array_size: u32, value: *const ::std::mem::MaybeUninit<::windows_core::HSTRING>, format: LoggingFieldFormat) -> ::windows_core::HRESULT,
    pub AddStringArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value_array_size: u32, value: *const ::std::mem::MaybeUninit<::windows_core::HSTRING>, format: LoggingFieldFormat, tags: i32) -> ::windows_core::HRESULT,
    pub AddGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub AddGuidWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: ::windows_core::GUID, format: LoggingFieldFormat) -> ::windows_core::HRESULT,
    pub AddGuidWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: ::windows_core::GUID, format: LoggingFieldFormat, tags: i32) -> ::windows_core::HRESULT,
    pub AddGuidArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value_array_size: u32, value: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub AddGuidArrayWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value_array_size: u32, value: *const ::windows_core::GUID, format: LoggingFieldFormat) -> ::windows_core::HRESULT,
    pub AddGuidArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value_array_size: u32, value: *const ::windows_core::GUID, format: LoggingFieldFormat, tags: i32) -> ::windows_core::HRESULT,
    pub AddDateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: super::DateTime) -> ::windows_core::HRESULT,
    pub AddDateTimeWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: super::DateTime, format: LoggingFieldFormat) -> ::windows_core::HRESULT,
    pub AddDateTimeWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: super::DateTime, format: LoggingFieldFormat, tags: i32) -> ::windows_core::HRESULT,
    pub AddDateTimeArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value_array_size: u32, value: *const super::DateTime) -> ::windows_core::HRESULT,
    pub AddDateTimeArrayWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value_array_size: u32, value: *const super::DateTime, format: LoggingFieldFormat) -> ::windows_core::HRESULT,
    pub AddDateTimeArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value_array_size: u32, value: *const super::DateTime, format: LoggingFieldFormat, tags: i32) -> ::windows_core::HRESULT,
    pub AddTimeSpan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: super::TimeSpan) -> ::windows_core::HRESULT,
    pub AddTimeSpanWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: super::TimeSpan, format: LoggingFieldFormat) -> ::windows_core::HRESULT,
    pub AddTimeSpanWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: super::TimeSpan, format: LoggingFieldFormat, tags: i32) -> ::windows_core::HRESULT,
    pub AddTimeSpanArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value_array_size: u32, value: *const super::TimeSpan) -> ::windows_core::HRESULT,
    pub AddTimeSpanArrayWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value_array_size: u32, value: *const super::TimeSpan, format: LoggingFieldFormat) -> ::windows_core::HRESULT,
    pub AddTimeSpanArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value_array_size: u32, value: *const super::TimeSpan, format: LoggingFieldFormat, tags: i32) -> ::windows_core::HRESULT,
    pub AddPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: super::Point) -> ::windows_core::HRESULT,
    pub AddPointWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: super::Point, format: LoggingFieldFormat) -> ::windows_core::HRESULT,
    pub AddPointWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: super::Point, format: LoggingFieldFormat, tags: i32) -> ::windows_core::HRESULT,
    pub AddPointArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value_array_size: u32, value: *const super::Point) -> ::windows_core::HRESULT,
    pub AddPointArrayWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value_array_size: u32, value: *const super::Point, format: LoggingFieldFormat) -> ::windows_core::HRESULT,
    pub AddPointArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value_array_size: u32, value: *const super::Point, format: LoggingFieldFormat, tags: i32) -> ::windows_core::HRESULT,
    pub AddSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: super::Size) -> ::windows_core::HRESULT,
    pub AddSizeWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: super::Size, format: LoggingFieldFormat) -> ::windows_core::HRESULT,
    pub AddSizeWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: super::Size, format: LoggingFieldFormat, tags: i32) -> ::windows_core::HRESULT,
    pub AddSizeArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value_array_size: u32, value: *const super::Size) -> ::windows_core::HRESULT,
    pub AddSizeArrayWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value_array_size: u32, value: *const super::Size, format: LoggingFieldFormat) -> ::windows_core::HRESULT,
    pub AddSizeArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value_array_size: u32, value: *const super::Size, format: LoggingFieldFormat, tags: i32) -> ::windows_core::HRESULT,
    pub AddRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: super::Rect) -> ::windows_core::HRESULT,
    pub AddRectWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: super::Rect, format: LoggingFieldFormat) -> ::windows_core::HRESULT,
    pub AddRectWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: super::Rect, format: LoggingFieldFormat, tags: i32) -> ::windows_core::HRESULT,
    pub AddRectArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value_array_size: u32, value: *const super::Rect) -> ::windows_core::HRESULT,
    pub AddRectArrayWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value_array_size: u32, value: *const super::Rect, format: LoggingFieldFormat) -> ::windows_core::HRESULT,
    pub AddRectArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value_array_size: u32, value: *const super::Rect, format: LoggingFieldFormat, tags: i32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILoggingOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILoggingOptions {
    type Vtable = ILoggingOptions_Vtbl;
}
impl ::core::clone::Clone for ILoggingOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILoggingOptions {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x90bc7850_0192_4f5d_ac26_006adaca12d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingOptions_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Keywords: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows_core::HRESULT,
    pub SetKeywords: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i64) -> ::windows_core::HRESULT,
    pub Tags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub Task: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i16) -> ::windows_core::HRESULT,
    pub SetTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i16) -> ::windows_core::HRESULT,
    pub Opcode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LoggingOpcode) -> ::windows_core::HRESULT,
    pub SetOpcode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: LoggingOpcode) -> ::windows_core::HRESULT,
    pub ActivityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub SetActivityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub RelatedActivityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub SetRelatedActivityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILoggingOptionsFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILoggingOptionsFactory {
    type Vtable = ILoggingOptionsFactory_Vtbl;
}
impl ::core::clone::Clone for ILoggingOptionsFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILoggingOptionsFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd713c6cb_98ab_464b_9f22_a3268478368a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingOptionsFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateWithKeywords: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keywords: i64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
pub struct ILoggingSession(::windows_core::IUnknown);
impl ILoggingSession {
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn SaveToFileAsync<P0>(&self, folder: P0, filename: &::windows_core::HSTRING) -> ::windows_core::Result<super::IAsyncOperation<super::super::Storage::StorageFile>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::IStorageFolder>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SaveToFileAsync)(::windows_core::Interface::as_raw(this), folder.try_into_param()?.abi(), ::core::mem::transmute_copy(filename), &mut result__).from_abi(result__)
        }
    }
    pub fn AddLoggingChannel<P0>(&self, loggingchannel: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<ILoggingChannel>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddLoggingChannel)(::windows_core::Interface::as_raw(this), loggingchannel.try_into_param()?.abi()).ok() }
    }
    pub fn AddLoggingChannelWithLevel<P0>(&self, loggingchannel: P0, maxlevel: LoggingLevel) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<ILoggingChannel>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddLoggingChannelWithLevel)(::windows_core::Interface::as_raw(this), loggingchannel.try_into_param()?.abi(), maxlevel).ok() }
    }
    pub fn RemoveLoggingChannel<P0>(&self, loggingchannel: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<ILoggingChannel>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveLoggingChannel)(::windows_core::Interface::as_raw(this), loggingchannel.try_into_param()?.abi()).ok() }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
::windows_core::imp::interface_hierarchy!(ILoggingSession, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<super::IClosable> for ILoggingSession {}
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
impl ::windows_core::RuntimeType for ILoggingSession {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{6221f306-9380-4ad7-baf5-41ea9310d768}");
}
unsafe impl ::windows_core::Interface for ILoggingSession {
    type Vtable = ILoggingSession_Vtbl;
}
impl ::core::clone::Clone for ILoggingSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILoggingSession {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6221f306_9380_4ad7_baf5_41ea9310d768);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingSession_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage")]
    pub SaveToFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folder: *mut ::core::ffi::c_void, filename: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    SaveToFileAsync: usize,
    pub AddLoggingChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, loggingchannel: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AddLoggingChannelWithLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, loggingchannel: *mut ::core::ffi::c_void, maxlevel: LoggingLevel) -> ::windows_core::HRESULT,
    pub RemoveLoggingChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, loggingchannel: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILoggingSessionFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILoggingSessionFactory {
    type Vtable = ILoggingSessionFactory_Vtbl;
}
impl ::core::clone::Clone for ILoggingSessionFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILoggingSessionFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4e937ee5_58fd_45e0_8c2f_a132eff95c1e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingSessionFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
pub struct ILoggingTarget(::windows_core::IUnknown);
impl ILoggingTarget {
    pub fn IsEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsEnabledWithLevel(&self, level: LoggingLevel) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsEnabledWithLevel)(::windows_core::Interface::as_raw(this), level, &mut result__).from_abi(result__)
        }
    }
    pub fn IsEnabledWithLevelAndKeywords(&self, level: LoggingLevel, keywords: i64) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsEnabledWithLevelAndKeywords)(::windows_core::Interface::as_raw(this), level, keywords, &mut result__).from_abi(result__)
        }
    }
    pub fn LogEvent(&self, eventname: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).LogEvent)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(eventname)).ok() }
    }
    pub fn LogEventWithFields<P0>(&self, eventname: &::windows_core::HSTRING, fields: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<LoggingFields>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).LogEventWithFields)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(eventname), fields.into_param().abi()).ok() }
    }
    pub fn LogEventWithFieldsAndLevel<P0>(&self, eventname: &::windows_core::HSTRING, fields: P0, level: LoggingLevel) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<LoggingFields>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).LogEventWithFieldsAndLevel)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(eventname), fields.into_param().abi(), level).ok() }
    }
    pub fn LogEventWithFieldsAndOptions<P0, P1>(&self, eventname: &::windows_core::HSTRING, fields: P0, level: LoggingLevel, options: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<LoggingFields>,
        P1: ::windows_core::IntoParam<LoggingOptions>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).LogEventWithFieldsAndOptions)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(eventname), fields.into_param().abi(), level, options.into_param().abi()).ok() }
    }
    pub fn StartActivity(&self, starteventname: &::windows_core::HSTRING) -> ::windows_core::Result<LoggingActivity> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartActivity)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(starteventname), &mut result__).from_abi(result__)
        }
    }
    pub fn StartActivityWithFields<P0>(&self, starteventname: &::windows_core::HSTRING, fields: P0) -> ::windows_core::Result<LoggingActivity>
    where
        P0: ::windows_core::IntoParam<LoggingFields>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartActivityWithFields)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(starteventname), fields.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn StartActivityWithFieldsAndLevel<P0>(&self, starteventname: &::windows_core::HSTRING, fields: P0, level: LoggingLevel) -> ::windows_core::Result<LoggingActivity>
    where
        P0: ::windows_core::IntoParam<LoggingFields>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartActivityWithFieldsAndLevel)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(starteventname), fields.into_param().abi(), level, &mut result__).from_abi(result__)
        }
    }
    pub fn StartActivityWithFieldsAndOptions<P0, P1>(&self, starteventname: &::windows_core::HSTRING, fields: P0, level: LoggingLevel, options: P1) -> ::windows_core::Result<LoggingActivity>
    where
        P0: ::windows_core::IntoParam<LoggingFields>,
        P1: ::windows_core::IntoParam<LoggingOptions>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartActivityWithFieldsAndOptions)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(starteventname), fields.into_param().abi(), level, options.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(ILoggingTarget, ::windows_core::IUnknown, ::windows_core::IInspectable);
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
impl ::windows_core::RuntimeType for ILoggingTarget {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{65f16c35-e388-4e26-b17a-f51cd3a83916}");
}
unsafe impl ::windows_core::Interface for ILoggingTarget {
    type Vtable = ILoggingTarget_Vtbl;
}
impl ::core::clone::Clone for ILoggingTarget {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILoggingTarget {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x65f16c35_e388_4e26_b17a_f51cd3a83916);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingTarget_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsEnabledWithLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, level: LoggingLevel, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsEnabledWithLevelAndKeywords: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, level: LoggingLevel, keywords: i64, result__: *mut bool) -> ::windows_core::HRESULT,
    pub LogEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventname: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub LogEventWithFields: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, fields: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub LogEventWithFieldsAndLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, fields: *mut ::core::ffi::c_void, level: LoggingLevel) -> ::windows_core::HRESULT,
    pub LogEventWithFieldsAndOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, fields: *mut ::core::ffi::c_void, level: LoggingLevel, options: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub StartActivity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, starteventname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub StartActivityWithFields: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, starteventname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, fields: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub StartActivityWithFieldsAndLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, starteventname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, fields: *mut ::core::ffi::c_void, level: LoggingLevel, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub StartActivityWithFieldsAndOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, starteventname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, fields: *mut ::core::ffi::c_void, level: LoggingLevel, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITracingStatusChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITracingStatusChangedEventArgs {
    type Vtable = ITracingStatusChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for ITracingStatusChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITracingStatusChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x410b7711_ff3b_477f_9c9a_d2efda302dc3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITracingStatusChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub TraceLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CausalityTraceLevel) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
pub struct AsyncCausalityTracer;
impl AsyncCausalityTracer {
    pub fn TraceOperationCreation(tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: ::windows_core::GUID, operationid: u64, operationname: &::windows_core::HSTRING, relatedcontext: u64) -> ::windows_core::Result<()> {
        Self::IAsyncCausalityTracerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).TraceOperationCreation)(::windows_core::Interface::as_raw(this), tracelevel, source, platformid, operationid, ::core::mem::transmute_copy(operationname), relatedcontext).ok() })
    }
    pub fn TraceOperationCompletion(tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: ::windows_core::GUID, operationid: u64, status: super::AsyncStatus) -> ::windows_core::Result<()> {
        Self::IAsyncCausalityTracerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).TraceOperationCompletion)(::windows_core::Interface::as_raw(this), tracelevel, source, platformid, operationid, status).ok() })
    }
    pub fn TraceOperationRelation(tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: ::windows_core::GUID, operationid: u64, relation: CausalityRelation) -> ::windows_core::Result<()> {
        Self::IAsyncCausalityTracerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).TraceOperationRelation)(::windows_core::Interface::as_raw(this), tracelevel, source, platformid, operationid, relation).ok() })
    }
    pub fn TraceSynchronousWorkStart(tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: ::windows_core::GUID, operationid: u64, work: CausalitySynchronousWork) -> ::windows_core::Result<()> {
        Self::IAsyncCausalityTracerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).TraceSynchronousWorkStart)(::windows_core::Interface::as_raw(this), tracelevel, source, platformid, operationid, work).ok() })
    }
    pub fn TraceSynchronousWorkCompletion(tracelevel: CausalityTraceLevel, source: CausalitySource, work: CausalitySynchronousWork) -> ::windows_core::Result<()> {
        Self::IAsyncCausalityTracerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).TraceSynchronousWorkCompletion)(::windows_core::Interface::as_raw(this), tracelevel, source, work).ok() })
    }
    pub fn TracingStatusChanged<P0>(handler: P0) -> ::windows_core::Result<super::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::EventHandler<TracingStatusChangedEventArgs>>,
    {
        Self::IAsyncCausalityTracerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TracingStatusChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn RemoveTracingStatusChanged(cookie: super::EventRegistrationToken) -> ::windows_core::Result<()> {
        Self::IAsyncCausalityTracerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveTracingStatusChanged)(::windows_core::Interface::as_raw(this), cookie).ok() })
    }
    #[doc(hidden)]
    pub fn IAsyncCausalityTracerStatics<R, F: FnOnce(&IAsyncCausalityTracerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<AsyncCausalityTracer, IAsyncCausalityTracerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for AsyncCausalityTracer {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.AsyncCausalityTracer";
}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
pub struct ErrorDetails(::windows_core::IUnknown);
impl ErrorDetails {
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LongDescription(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LongDescription)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HelpUri(&self) -> ::windows_core::Result<super::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HelpUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateFromHResultAsync(errorcode: i32) -> ::windows_core::Result<super::IAsyncOperation<ErrorDetails>> {
        Self::IErrorDetailsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromHResultAsync)(::windows_core::Interface::as_raw(this), errorcode, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IErrorDetailsStatics<R, F: FnOnce(&IErrorDetailsStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ErrorDetails, IErrorDetailsStatics> = ::windows_core::imp::FactoryCache::new();
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
impl ::windows_core::RuntimeType for ErrorDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.ErrorDetails;{378cbb01-2cc9-428f-8c55-2c990d463e8f})");
}
impl ::core::clone::Clone for ErrorDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ErrorDetails {
    type Vtable = IErrorDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ErrorDetails {
    const IID: ::windows_core::GUID = <IErrorDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ErrorDetails {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.ErrorDetails";
}
::windows_core::imp::interface_hierarchy!(ErrorDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ErrorDetails {}
unsafe impl ::core::marker::Sync for ErrorDetails {}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
pub struct FileLoggingSession(::windows_core::IUnknown);
impl FileLoggingSession {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AddLoggingChannel<P0>(&self, loggingchannel: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<ILoggingChannel>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddLoggingChannel)(::windows_core::Interface::as_raw(this), loggingchannel.try_into_param()?.abi()).ok() }
    }
    pub fn AddLoggingChannelWithLevel<P0>(&self, loggingchannel: P0, maxlevel: LoggingLevel) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<ILoggingChannel>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddLoggingChannelWithLevel)(::windows_core::Interface::as_raw(this), loggingchannel.try_into_param()?.abi(), maxlevel).ok() }
    }
    pub fn RemoveLoggingChannel<P0>(&self, loggingchannel: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<ILoggingChannel>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveLoggingChannel)(::windows_core::Interface::as_raw(this), loggingchannel.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn CloseAndSaveToFileAsync(&self) -> ::windows_core::Result<super::IAsyncOperation<super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CloseAndSaveToFileAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LogFileGenerated<P0>(&self, handler: P0) -> ::windows_core::Result<super::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::TypedEventHandler<IFileLoggingSession, LogFileGeneratedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LogFileGenerated)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveLogFileGenerated(&self, token: super::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveLogFileGenerated)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Create(name: &::windows_core::HSTRING) -> ::windows_core::Result<FileLoggingSession> {
        Self::IFileLoggingSessionFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IFileLoggingSessionFactory<R, F: FnOnce(&IFileLoggingSessionFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<FileLoggingSession, IFileLoggingSessionFactory> = ::windows_core::imp::FactoryCache::new();
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
impl ::windows_core::RuntimeType for FileLoggingSession {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.FileLoggingSession;{24c74216-fed2-404c-895f-1f9699cb02f7})");
}
impl ::core::clone::Clone for FileLoggingSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for FileLoggingSession {
    type Vtable = IFileLoggingSession_Vtbl;
}
unsafe impl ::windows_core::ComInterface for FileLoggingSession {
    const IID: ::windows_core::GUID = <IFileLoggingSession as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for FileLoggingSession {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.FileLoggingSession";
}
::windows_core::imp::interface_hierarchy!(FileLoggingSession, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<super::IClosable> for FileLoggingSession {}
impl ::windows_core::CanTryInto<IFileLoggingSession> for FileLoggingSession {}
unsafe impl ::core::marker::Send for FileLoggingSession {}
unsafe impl ::core::marker::Sync for FileLoggingSession {}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
pub struct LogFileGeneratedEventArgs(::windows_core::IUnknown);
impl LogFileGeneratedEventArgs {
    #[doc = "*Required features: `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn File(&self) -> ::windows_core::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).File)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
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
impl ::windows_core::RuntimeType for LogFileGeneratedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.LogFileGeneratedEventArgs;{269e976f-0d38-4c1a-b53f-b395d881df84})");
}
impl ::core::clone::Clone for LogFileGeneratedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for LogFileGeneratedEventArgs {
    type Vtable = ILogFileGeneratedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LogFileGeneratedEventArgs {
    const IID: ::windows_core::GUID = <ILogFileGeneratedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LogFileGeneratedEventArgs {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.LogFileGeneratedEventArgs";
}
::windows_core::imp::interface_hierarchy!(LogFileGeneratedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for LogFileGeneratedEventArgs {}
unsafe impl ::core::marker::Sync for LogFileGeneratedEventArgs {}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
pub struct LoggingActivity(::windows_core::IUnknown);
impl LoggingActivity {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Channel(&self) -> ::windows_core::Result<LoggingChannel> {
        let this = &::windows_core::ComInterface::cast::<ILoggingActivity2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Channel)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StopActivity(&self, stopeventname: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ILoggingActivity2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopActivity)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(stopeventname)).ok() }
    }
    pub fn StopActivityWithFields<P0>(&self, stopeventname: &::windows_core::HSTRING, fields: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<LoggingFields>,
    {
        let this = &::windows_core::ComInterface::cast::<ILoggingActivity2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopActivityWithFields)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(stopeventname), fields.into_param().abi()).ok() }
    }
    pub fn StopActivityWithFieldsAndOptions<P0, P1>(&self, stopeventname: &::windows_core::HSTRING, fields: P0, options: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<LoggingFields>,
        P1: ::windows_core::IntoParam<LoggingOptions>,
    {
        let this = &::windows_core::ComInterface::cast::<ILoggingActivity2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopActivityWithFieldsAndOptions)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(stopeventname), fields.into_param().abi(), options.into_param().abi()).ok() }
    }
    pub fn CreateLoggingActivity<P0>(activityname: &::windows_core::HSTRING, loggingchannel: P0) -> ::windows_core::Result<LoggingActivity>
    where
        P0: ::windows_core::TryIntoParam<ILoggingChannel>,
    {
        Self::ILoggingActivityFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateLoggingActivity)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(activityname), loggingchannel.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateLoggingActivityWithLevel<P0>(activityname: &::windows_core::HSTRING, loggingchannel: P0, level: LoggingLevel) -> ::windows_core::Result<LoggingActivity>
    where
        P0: ::windows_core::TryIntoParam<ILoggingChannel>,
    {
        Self::ILoggingActivityFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateLoggingActivityWithLevel)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(activityname), loggingchannel.try_into_param()?.abi(), level, &mut result__).from_abi(result__)
        })
    }
    pub fn IsEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsEnabledWithLevel(&self, level: LoggingLevel) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsEnabledWithLevel)(::windows_core::Interface::as_raw(this), level, &mut result__).from_abi(result__)
        }
    }
    pub fn IsEnabledWithLevelAndKeywords(&self, level: LoggingLevel, keywords: i64) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsEnabledWithLevelAndKeywords)(::windows_core::Interface::as_raw(this), level, keywords, &mut result__).from_abi(result__)
        }
    }
    pub fn LogEvent(&self, eventname: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ILoggingTarget>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).LogEvent)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(eventname)).ok() }
    }
    pub fn LogEventWithFields<P0>(&self, eventname: &::windows_core::HSTRING, fields: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<LoggingFields>,
    {
        let this = &::windows_core::ComInterface::cast::<ILoggingTarget>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).LogEventWithFields)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(eventname), fields.into_param().abi()).ok() }
    }
    pub fn LogEventWithFieldsAndLevel<P0>(&self, eventname: &::windows_core::HSTRING, fields: P0, level: LoggingLevel) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<LoggingFields>,
    {
        let this = &::windows_core::ComInterface::cast::<ILoggingTarget>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).LogEventWithFieldsAndLevel)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(eventname), fields.into_param().abi(), level).ok() }
    }
    pub fn LogEventWithFieldsAndOptions<P0, P1>(&self, eventname: &::windows_core::HSTRING, fields: P0, level: LoggingLevel, options: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<LoggingFields>,
        P1: ::windows_core::IntoParam<LoggingOptions>,
    {
        let this = &::windows_core::ComInterface::cast::<ILoggingTarget>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).LogEventWithFieldsAndOptions)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(eventname), fields.into_param().abi(), level, options.into_param().abi()).ok() }
    }
    pub fn StartActivity(&self, starteventname: &::windows_core::HSTRING) -> ::windows_core::Result<LoggingActivity> {
        let this = &::windows_core::ComInterface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartActivity)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(starteventname), &mut result__).from_abi(result__)
        }
    }
    pub fn StartActivityWithFields<P0>(&self, starteventname: &::windows_core::HSTRING, fields: P0) -> ::windows_core::Result<LoggingActivity>
    where
        P0: ::windows_core::IntoParam<LoggingFields>,
    {
        let this = &::windows_core::ComInterface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartActivityWithFields)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(starteventname), fields.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn StartActivityWithFieldsAndLevel<P0>(&self, starteventname: &::windows_core::HSTRING, fields: P0, level: LoggingLevel) -> ::windows_core::Result<LoggingActivity>
    where
        P0: ::windows_core::IntoParam<LoggingFields>,
    {
        let this = &::windows_core::ComInterface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartActivityWithFieldsAndLevel)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(starteventname), fields.into_param().abi(), level, &mut result__).from_abi(result__)
        }
    }
    pub fn StartActivityWithFieldsAndOptions<P0, P1>(&self, starteventname: &::windows_core::HSTRING, fields: P0, level: LoggingLevel, options: P1) -> ::windows_core::Result<LoggingActivity>
    where
        P0: ::windows_core::IntoParam<LoggingFields>,
        P1: ::windows_core::IntoParam<LoggingOptions>,
    {
        let this = &::windows_core::ComInterface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartActivityWithFieldsAndOptions)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(starteventname), fields.into_param().abi(), level, options.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn ILoggingActivityFactory<R, F: FnOnce(&ILoggingActivityFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<LoggingActivity, ILoggingActivityFactory> = ::windows_core::imp::FactoryCache::new();
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
impl ::windows_core::RuntimeType for LoggingActivity {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.LoggingActivity;{bc032941-b766-4cb5-9848-97ac6ba6d60c})");
}
impl ::core::clone::Clone for LoggingActivity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for LoggingActivity {
    type Vtable = ILoggingActivity_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LoggingActivity {
    const IID: ::windows_core::GUID = <ILoggingActivity as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LoggingActivity {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.LoggingActivity";
}
::windows_core::imp::interface_hierarchy!(LoggingActivity, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<super::IClosable> for LoggingActivity {}
impl ::windows_core::CanTryInto<ILoggingTarget> for LoggingActivity {}
unsafe impl ::core::marker::Send for LoggingActivity {}
unsafe impl ::core::marker::Sync for LoggingActivity {}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
pub struct LoggingChannel(::windows_core::IUnknown);
impl LoggingChannel {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Enabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Enabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Level(&self) -> ::windows_core::Result<LoggingLevel> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Level)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LogMessage(&self, eventstring: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).LogMessage)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(eventstring)).ok() }
    }
    pub fn LogMessageWithLevel(&self, eventstring: &::windows_core::HSTRING, level: LoggingLevel) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).LogMessageWithLevel)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(eventstring), level).ok() }
    }
    pub fn LogValuePair(&self, value1: &::windows_core::HSTRING, value2: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).LogValuePair)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value1), value2).ok() }
    }
    pub fn LogValuePairWithLevel(&self, value1: &::windows_core::HSTRING, value2: i32, level: LoggingLevel) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).LogValuePairWithLevel)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value1), value2, level).ok() }
    }
    pub fn LoggingEnabled<P0>(&self, handler: P0) -> ::windows_core::Result<super::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::TypedEventHandler<ILoggingChannel, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LoggingEnabled)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveLoggingEnabled(&self, token: super::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveLoggingEnabled)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = &::windows_core::ComInterface::cast::<ILoggingChannel2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Create(name: &::windows_core::HSTRING) -> ::windows_core::Result<LoggingChannel> {
        Self::ILoggingChannelFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateWithOptions<P0>(name: &::windows_core::HSTRING, options: P0) -> ::windows_core::Result<LoggingChannel>
    where
        P0: ::windows_core::IntoParam<LoggingChannelOptions>,
    {
        Self::ILoggingChannelFactory2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithOptions)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), options.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateWithOptionsAndId<P0>(name: &::windows_core::HSTRING, options: P0, id: ::windows_core::GUID) -> ::windows_core::Result<LoggingChannel>
    where
        P0: ::windows_core::IntoParam<LoggingChannelOptions>,
    {
        Self::ILoggingChannelFactory2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithOptionsAndId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), options.into_param().abi(), id, &mut result__).from_abi(result__)
        })
    }
    pub fn IsEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsEnabledWithLevel(&self, level: LoggingLevel) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsEnabledWithLevel)(::windows_core::Interface::as_raw(this), level, &mut result__).from_abi(result__)
        }
    }
    pub fn IsEnabledWithLevelAndKeywords(&self, level: LoggingLevel, keywords: i64) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsEnabledWithLevelAndKeywords)(::windows_core::Interface::as_raw(this), level, keywords, &mut result__).from_abi(result__)
        }
    }
    pub fn LogEvent(&self, eventname: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ILoggingTarget>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).LogEvent)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(eventname)).ok() }
    }
    pub fn LogEventWithFields<P0>(&self, eventname: &::windows_core::HSTRING, fields: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<LoggingFields>,
    {
        let this = &::windows_core::ComInterface::cast::<ILoggingTarget>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).LogEventWithFields)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(eventname), fields.into_param().abi()).ok() }
    }
    pub fn LogEventWithFieldsAndLevel<P0>(&self, eventname: &::windows_core::HSTRING, fields: P0, level: LoggingLevel) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<LoggingFields>,
    {
        let this = &::windows_core::ComInterface::cast::<ILoggingTarget>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).LogEventWithFieldsAndLevel)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(eventname), fields.into_param().abi(), level).ok() }
    }
    pub fn LogEventWithFieldsAndOptions<P0, P1>(&self, eventname: &::windows_core::HSTRING, fields: P0, level: LoggingLevel, options: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<LoggingFields>,
        P1: ::windows_core::IntoParam<LoggingOptions>,
    {
        let this = &::windows_core::ComInterface::cast::<ILoggingTarget>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).LogEventWithFieldsAndOptions)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(eventname), fields.into_param().abi(), level, options.into_param().abi()).ok() }
    }
    pub fn StartActivity(&self, starteventname: &::windows_core::HSTRING) -> ::windows_core::Result<LoggingActivity> {
        let this = &::windows_core::ComInterface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartActivity)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(starteventname), &mut result__).from_abi(result__)
        }
    }
    pub fn StartActivityWithFields<P0>(&self, starteventname: &::windows_core::HSTRING, fields: P0) -> ::windows_core::Result<LoggingActivity>
    where
        P0: ::windows_core::IntoParam<LoggingFields>,
    {
        let this = &::windows_core::ComInterface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartActivityWithFields)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(starteventname), fields.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn StartActivityWithFieldsAndLevel<P0>(&self, starteventname: &::windows_core::HSTRING, fields: P0, level: LoggingLevel) -> ::windows_core::Result<LoggingActivity>
    where
        P0: ::windows_core::IntoParam<LoggingFields>,
    {
        let this = &::windows_core::ComInterface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartActivityWithFieldsAndLevel)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(starteventname), fields.into_param().abi(), level, &mut result__).from_abi(result__)
        }
    }
    pub fn StartActivityWithFieldsAndOptions<P0, P1>(&self, starteventname: &::windows_core::HSTRING, fields: P0, level: LoggingLevel, options: P1) -> ::windows_core::Result<LoggingActivity>
    where
        P0: ::windows_core::IntoParam<LoggingFields>,
        P1: ::windows_core::IntoParam<LoggingOptions>,
    {
        let this = &::windows_core::ComInterface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartActivityWithFieldsAndOptions)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(starteventname), fields.into_param().abi(), level, options.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn ILoggingChannelFactory<R, F: FnOnce(&ILoggingChannelFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<LoggingChannel, ILoggingChannelFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ILoggingChannelFactory2<R, F: FnOnce(&ILoggingChannelFactory2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<LoggingChannel, ILoggingChannelFactory2> = ::windows_core::imp::FactoryCache::new();
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
impl ::windows_core::RuntimeType for LoggingChannel {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.LoggingChannel;{e9a50343-11d7-4f01-b5ca-cf495278c0a8})");
}
impl ::core::clone::Clone for LoggingChannel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for LoggingChannel {
    type Vtable = ILoggingChannel_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LoggingChannel {
    const IID: ::windows_core::GUID = <ILoggingChannel as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LoggingChannel {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.LoggingChannel";
}
::windows_core::imp::interface_hierarchy!(LoggingChannel, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<super::IClosable> for LoggingChannel {}
impl ::windows_core::CanTryInto<ILoggingChannel> for LoggingChannel {}
impl ::windows_core::CanTryInto<ILoggingTarget> for LoggingChannel {}
unsafe impl ::core::marker::Send for LoggingChannel {}
unsafe impl ::core::marker::Sync for LoggingChannel {}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
pub struct LoggingChannelOptions(::windows_core::IUnknown);
impl LoggingChannelOptions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<LoggingChannelOptions, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Group(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Group)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetGroup(&self, value: ::windows_core::GUID) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetGroup)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Create(group: ::windows_core::GUID) -> ::windows_core::Result<LoggingChannelOptions> {
        Self::ILoggingChannelOptionsFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), group, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILoggingChannelOptionsFactory<R, F: FnOnce(&ILoggingChannelOptionsFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<LoggingChannelOptions, ILoggingChannelOptionsFactory> = ::windows_core::imp::FactoryCache::new();
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
impl ::windows_core::RuntimeType for LoggingChannelOptions {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.LoggingChannelOptions;{c3e847ff-0ebb-4a53-8c54-dec24926cb2c})");
}
impl ::core::clone::Clone for LoggingChannelOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for LoggingChannelOptions {
    type Vtable = ILoggingChannelOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LoggingChannelOptions {
    const IID: ::windows_core::GUID = <ILoggingChannelOptions as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LoggingChannelOptions {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.LoggingChannelOptions";
}
::windows_core::imp::interface_hierarchy!(LoggingChannelOptions, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for LoggingChannelOptions {}
unsafe impl ::core::marker::Sync for LoggingChannelOptions {}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
pub struct LoggingFields(::windows_core::IUnknown);
impl LoggingFields {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<LoggingFields, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn BeginStruct(&self, name: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).BeginStruct)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name)).ok() }
    }
    pub fn BeginStructWithTags(&self, name: &::windows_core::HSTRING, tags: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).BeginStructWithTags)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), tags).ok() }
    }
    pub fn EndStruct(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).EndStruct)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn AddEmpty(&self, name: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddEmpty)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name)).ok() }
    }
    pub fn AddEmptyWithFormat(&self, name: &::windows_core::HSTRING, format: LoggingFieldFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddEmptyWithFormat)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), format).ok() }
    }
    pub fn AddEmptyWithFormatAndTags(&self, name: &::windows_core::HSTRING, format: LoggingFieldFormat, tags: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddEmptyWithFormatAndTags)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), format, tags).ok() }
    }
    pub fn AddUInt8(&self, name: &::windows_core::HSTRING, value: u8) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddUInt8)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value).ok() }
    }
    pub fn AddUInt8WithFormat(&self, name: &::windows_core::HSTRING, value: u8, format: LoggingFieldFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddUInt8WithFormat)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format).ok() }
    }
    pub fn AddUInt8WithFormatAndTags(&self, name: &::windows_core::HSTRING, value: u8, format: LoggingFieldFormat, tags: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddUInt8WithFormatAndTags)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format, tags).ok() }
    }
    pub fn AddUInt8Array(&self, name: &::windows_core::HSTRING, value: &[u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddUInt8Array)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr()).ok() }
    }
    pub fn AddUInt8ArrayWithFormat(&self, name: &::windows_core::HSTRING, value: &[u8], format: LoggingFieldFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddUInt8ArrayWithFormat)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format).ok() }
    }
    pub fn AddUInt8ArrayWithFormatAndTags(&self, name: &::windows_core::HSTRING, value: &[u8], format: LoggingFieldFormat, tags: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddUInt8ArrayWithFormatAndTags)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format, tags).ok() }
    }
    pub fn AddInt16(&self, name: &::windows_core::HSTRING, value: i16) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddInt16)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value).ok() }
    }
    pub fn AddInt16WithFormat(&self, name: &::windows_core::HSTRING, value: i16, format: LoggingFieldFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddInt16WithFormat)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format).ok() }
    }
    pub fn AddInt16WithFormatAndTags(&self, name: &::windows_core::HSTRING, value: i16, format: LoggingFieldFormat, tags: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddInt16WithFormatAndTags)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format, tags).ok() }
    }
    pub fn AddInt16Array(&self, name: &::windows_core::HSTRING, value: &[i16]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddInt16Array)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr()).ok() }
    }
    pub fn AddInt16ArrayWithFormat(&self, name: &::windows_core::HSTRING, value: &[i16], format: LoggingFieldFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddInt16ArrayWithFormat)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format).ok() }
    }
    pub fn AddInt16ArrayWithFormatAndTags(&self, name: &::windows_core::HSTRING, value: &[i16], format: LoggingFieldFormat, tags: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddInt16ArrayWithFormatAndTags)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format, tags).ok() }
    }
    pub fn AddUInt16(&self, name: &::windows_core::HSTRING, value: u16) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddUInt16)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value).ok() }
    }
    pub fn AddUInt16WithFormat(&self, name: &::windows_core::HSTRING, value: u16, format: LoggingFieldFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddUInt16WithFormat)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format).ok() }
    }
    pub fn AddUInt16WithFormatAndTags(&self, name: &::windows_core::HSTRING, value: u16, format: LoggingFieldFormat, tags: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddUInt16WithFormatAndTags)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format, tags).ok() }
    }
    pub fn AddUInt16Array(&self, name: &::windows_core::HSTRING, value: &[u16]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddUInt16Array)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr()).ok() }
    }
    pub fn AddUInt16ArrayWithFormat(&self, name: &::windows_core::HSTRING, value: &[u16], format: LoggingFieldFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddUInt16ArrayWithFormat)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format).ok() }
    }
    pub fn AddUInt16ArrayWithFormatAndTags(&self, name: &::windows_core::HSTRING, value: &[u16], format: LoggingFieldFormat, tags: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddUInt16ArrayWithFormatAndTags)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format, tags).ok() }
    }
    pub fn AddInt32(&self, name: &::windows_core::HSTRING, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddInt32)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value).ok() }
    }
    pub fn AddInt32WithFormat(&self, name: &::windows_core::HSTRING, value: i32, format: LoggingFieldFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddInt32WithFormat)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format).ok() }
    }
    pub fn AddInt32WithFormatAndTags(&self, name: &::windows_core::HSTRING, value: i32, format: LoggingFieldFormat, tags: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddInt32WithFormatAndTags)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format, tags).ok() }
    }
    pub fn AddInt32Array(&self, name: &::windows_core::HSTRING, value: &[i32]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddInt32Array)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr()).ok() }
    }
    pub fn AddInt32ArrayWithFormat(&self, name: &::windows_core::HSTRING, value: &[i32], format: LoggingFieldFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddInt32ArrayWithFormat)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format).ok() }
    }
    pub fn AddInt32ArrayWithFormatAndTags(&self, name: &::windows_core::HSTRING, value: &[i32], format: LoggingFieldFormat, tags: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddInt32ArrayWithFormatAndTags)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format, tags).ok() }
    }
    pub fn AddUInt32(&self, name: &::windows_core::HSTRING, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddUInt32)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value).ok() }
    }
    pub fn AddUInt32WithFormat(&self, name: &::windows_core::HSTRING, value: u32, format: LoggingFieldFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddUInt32WithFormat)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format).ok() }
    }
    pub fn AddUInt32WithFormatAndTags(&self, name: &::windows_core::HSTRING, value: u32, format: LoggingFieldFormat, tags: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddUInt32WithFormatAndTags)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format, tags).ok() }
    }
    pub fn AddUInt32Array(&self, name: &::windows_core::HSTRING, value: &[u32]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddUInt32Array)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr()).ok() }
    }
    pub fn AddUInt32ArrayWithFormat(&self, name: &::windows_core::HSTRING, value: &[u32], format: LoggingFieldFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddUInt32ArrayWithFormat)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format).ok() }
    }
    pub fn AddUInt32ArrayWithFormatAndTags(&self, name: &::windows_core::HSTRING, value: &[u32], format: LoggingFieldFormat, tags: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddUInt32ArrayWithFormatAndTags)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format, tags).ok() }
    }
    pub fn AddInt64(&self, name: &::windows_core::HSTRING, value: i64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddInt64)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value).ok() }
    }
    pub fn AddInt64WithFormat(&self, name: &::windows_core::HSTRING, value: i64, format: LoggingFieldFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddInt64WithFormat)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format).ok() }
    }
    pub fn AddInt64WithFormatAndTags(&self, name: &::windows_core::HSTRING, value: i64, format: LoggingFieldFormat, tags: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddInt64WithFormatAndTags)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format, tags).ok() }
    }
    pub fn AddInt64Array(&self, name: &::windows_core::HSTRING, value: &[i64]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddInt64Array)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr()).ok() }
    }
    pub fn AddInt64ArrayWithFormat(&self, name: &::windows_core::HSTRING, value: &[i64], format: LoggingFieldFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddInt64ArrayWithFormat)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format).ok() }
    }
    pub fn AddInt64ArrayWithFormatAndTags(&self, name: &::windows_core::HSTRING, value: &[i64], format: LoggingFieldFormat, tags: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddInt64ArrayWithFormatAndTags)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format, tags).ok() }
    }
    pub fn AddUInt64(&self, name: &::windows_core::HSTRING, value: u64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddUInt64)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value).ok() }
    }
    pub fn AddUInt64WithFormat(&self, name: &::windows_core::HSTRING, value: u64, format: LoggingFieldFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddUInt64WithFormat)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format).ok() }
    }
    pub fn AddUInt64WithFormatAndTags(&self, name: &::windows_core::HSTRING, value: u64, format: LoggingFieldFormat, tags: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddUInt64WithFormatAndTags)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format, tags).ok() }
    }
    pub fn AddUInt64Array(&self, name: &::windows_core::HSTRING, value: &[u64]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddUInt64Array)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr()).ok() }
    }
    pub fn AddUInt64ArrayWithFormat(&self, name: &::windows_core::HSTRING, value: &[u64], format: LoggingFieldFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddUInt64ArrayWithFormat)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format).ok() }
    }
    pub fn AddUInt64ArrayWithFormatAndTags(&self, name: &::windows_core::HSTRING, value: &[u64], format: LoggingFieldFormat, tags: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddUInt64ArrayWithFormatAndTags)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format, tags).ok() }
    }
    pub fn AddSingle(&self, name: &::windows_core::HSTRING, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddSingle)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value).ok() }
    }
    pub fn AddSingleWithFormat(&self, name: &::windows_core::HSTRING, value: f32, format: LoggingFieldFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddSingleWithFormat)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format).ok() }
    }
    pub fn AddSingleWithFormatAndTags(&self, name: &::windows_core::HSTRING, value: f32, format: LoggingFieldFormat, tags: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddSingleWithFormatAndTags)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format, tags).ok() }
    }
    pub fn AddSingleArray(&self, name: &::windows_core::HSTRING, value: &[f32]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddSingleArray)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr()).ok() }
    }
    pub fn AddSingleArrayWithFormat(&self, name: &::windows_core::HSTRING, value: &[f32], format: LoggingFieldFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddSingleArrayWithFormat)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format).ok() }
    }
    pub fn AddSingleArrayWithFormatAndTags(&self, name: &::windows_core::HSTRING, value: &[f32], format: LoggingFieldFormat, tags: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddSingleArrayWithFormatAndTags)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format, tags).ok() }
    }
    pub fn AddDouble(&self, name: &::windows_core::HSTRING, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddDouble)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value).ok() }
    }
    pub fn AddDoubleWithFormat(&self, name: &::windows_core::HSTRING, value: f64, format: LoggingFieldFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddDoubleWithFormat)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format).ok() }
    }
    pub fn AddDoubleWithFormatAndTags(&self, name: &::windows_core::HSTRING, value: f64, format: LoggingFieldFormat, tags: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddDoubleWithFormatAndTags)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format, tags).ok() }
    }
    pub fn AddDoubleArray(&self, name: &::windows_core::HSTRING, value: &[f64]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddDoubleArray)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr()).ok() }
    }
    pub fn AddDoubleArrayWithFormat(&self, name: &::windows_core::HSTRING, value: &[f64], format: LoggingFieldFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddDoubleArrayWithFormat)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format).ok() }
    }
    pub fn AddDoubleArrayWithFormatAndTags(&self, name: &::windows_core::HSTRING, value: &[f64], format: LoggingFieldFormat, tags: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddDoubleArrayWithFormatAndTags)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format, tags).ok() }
    }
    pub fn AddChar16(&self, name: &::windows_core::HSTRING, value: u16) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddChar16)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value).ok() }
    }
    pub fn AddChar16WithFormat(&self, name: &::windows_core::HSTRING, value: u16, format: LoggingFieldFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddChar16WithFormat)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format).ok() }
    }
    pub fn AddChar16WithFormatAndTags(&self, name: &::windows_core::HSTRING, value: u16, format: LoggingFieldFormat, tags: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddChar16WithFormatAndTags)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format, tags).ok() }
    }
    pub fn AddChar16Array(&self, name: &::windows_core::HSTRING, value: &[u16]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddChar16Array)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr()).ok() }
    }
    pub fn AddChar16ArrayWithFormat(&self, name: &::windows_core::HSTRING, value: &[u16], format: LoggingFieldFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddChar16ArrayWithFormat)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format).ok() }
    }
    pub fn AddChar16ArrayWithFormatAndTags(&self, name: &::windows_core::HSTRING, value: &[u16], format: LoggingFieldFormat, tags: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddChar16ArrayWithFormatAndTags)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format, tags).ok() }
    }
    pub fn AddBoolean(&self, name: &::windows_core::HSTRING, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddBoolean)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value).ok() }
    }
    pub fn AddBooleanWithFormat(&self, name: &::windows_core::HSTRING, value: bool, format: LoggingFieldFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddBooleanWithFormat)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format).ok() }
    }
    pub fn AddBooleanWithFormatAndTags(&self, name: &::windows_core::HSTRING, value: bool, format: LoggingFieldFormat, tags: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddBooleanWithFormatAndTags)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format, tags).ok() }
    }
    pub fn AddBooleanArray(&self, name: &::windows_core::HSTRING, value: &[bool]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddBooleanArray)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr()).ok() }
    }
    pub fn AddBooleanArrayWithFormat(&self, name: &::windows_core::HSTRING, value: &[bool], format: LoggingFieldFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddBooleanArrayWithFormat)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format).ok() }
    }
    pub fn AddBooleanArrayWithFormatAndTags(&self, name: &::windows_core::HSTRING, value: &[bool], format: LoggingFieldFormat, tags: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddBooleanArrayWithFormatAndTags)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format, tags).ok() }
    }
    pub fn AddString(&self, name: &::windows_core::HSTRING, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddString)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn AddStringWithFormat(&self, name: &::windows_core::HSTRING, value: &::windows_core::HSTRING, format: LoggingFieldFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddStringWithFormat)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(value), format).ok() }
    }
    pub fn AddStringWithFormatAndTags(&self, name: &::windows_core::HSTRING, value: &::windows_core::HSTRING, format: LoggingFieldFormat, tags: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddStringWithFormatAndTags)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(value), format, tags).ok() }
    }
    pub fn AddStringArray(&self, name: &::windows_core::HSTRING, value: &[::windows_core::HSTRING]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddStringArray)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    pub fn AddStringArrayWithFormat(&self, name: &::windows_core::HSTRING, value: &[::windows_core::HSTRING], format: LoggingFieldFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddStringArrayWithFormat)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format).ok() }
    }
    pub fn AddStringArrayWithFormatAndTags(&self, name: &::windows_core::HSTRING, value: &[::windows_core::HSTRING], format: LoggingFieldFormat, tags: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddStringArrayWithFormatAndTags)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    pub fn AddGuid(&self, name: &::windows_core::HSTRING, value: ::windows_core::GUID) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddGuid)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value).ok() }
    }
    pub fn AddGuidWithFormat(&self, name: &::windows_core::HSTRING, value: ::windows_core::GUID, format: LoggingFieldFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddGuidWithFormat)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format).ok() }
    }
    pub fn AddGuidWithFormatAndTags(&self, name: &::windows_core::HSTRING, value: ::windows_core::GUID, format: LoggingFieldFormat, tags: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddGuidWithFormatAndTags)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format, tags).ok() }
    }
    pub fn AddGuidArray(&self, name: &::windows_core::HSTRING, value: &[::windows_core::GUID]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddGuidArray)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr()).ok() }
    }
    pub fn AddGuidArrayWithFormat(&self, name: &::windows_core::HSTRING, value: &[::windows_core::GUID], format: LoggingFieldFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddGuidArrayWithFormat)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format).ok() }
    }
    pub fn AddGuidArrayWithFormatAndTags(&self, name: &::windows_core::HSTRING, value: &[::windows_core::GUID], format: LoggingFieldFormat, tags: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddGuidArrayWithFormatAndTags)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format, tags).ok() }
    }
    pub fn AddDateTime(&self, name: &::windows_core::HSTRING, value: super::DateTime) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddDateTime)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value).ok() }
    }
    pub fn AddDateTimeWithFormat(&self, name: &::windows_core::HSTRING, value: super::DateTime, format: LoggingFieldFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddDateTimeWithFormat)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format).ok() }
    }
    pub fn AddDateTimeWithFormatAndTags(&self, name: &::windows_core::HSTRING, value: super::DateTime, format: LoggingFieldFormat, tags: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddDateTimeWithFormatAndTags)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format, tags).ok() }
    }
    pub fn AddDateTimeArray(&self, name: &::windows_core::HSTRING, value: &[super::DateTime]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddDateTimeArray)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr()).ok() }
    }
    pub fn AddDateTimeArrayWithFormat(&self, name: &::windows_core::HSTRING, value: &[super::DateTime], format: LoggingFieldFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddDateTimeArrayWithFormat)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format).ok() }
    }
    pub fn AddDateTimeArrayWithFormatAndTags(&self, name: &::windows_core::HSTRING, value: &[super::DateTime], format: LoggingFieldFormat, tags: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddDateTimeArrayWithFormatAndTags)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format, tags).ok() }
    }
    pub fn AddTimeSpan(&self, name: &::windows_core::HSTRING, value: super::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddTimeSpan)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value).ok() }
    }
    pub fn AddTimeSpanWithFormat(&self, name: &::windows_core::HSTRING, value: super::TimeSpan, format: LoggingFieldFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddTimeSpanWithFormat)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format).ok() }
    }
    pub fn AddTimeSpanWithFormatAndTags(&self, name: &::windows_core::HSTRING, value: super::TimeSpan, format: LoggingFieldFormat, tags: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddTimeSpanWithFormatAndTags)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format, tags).ok() }
    }
    pub fn AddTimeSpanArray(&self, name: &::windows_core::HSTRING, value: &[super::TimeSpan]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddTimeSpanArray)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr()).ok() }
    }
    pub fn AddTimeSpanArrayWithFormat(&self, name: &::windows_core::HSTRING, value: &[super::TimeSpan], format: LoggingFieldFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddTimeSpanArrayWithFormat)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format).ok() }
    }
    pub fn AddTimeSpanArrayWithFormatAndTags(&self, name: &::windows_core::HSTRING, value: &[super::TimeSpan], format: LoggingFieldFormat, tags: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddTimeSpanArrayWithFormatAndTags)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format, tags).ok() }
    }
    pub fn AddPoint(&self, name: &::windows_core::HSTRING, value: super::Point) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddPoint)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value).ok() }
    }
    pub fn AddPointWithFormat(&self, name: &::windows_core::HSTRING, value: super::Point, format: LoggingFieldFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddPointWithFormat)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format).ok() }
    }
    pub fn AddPointWithFormatAndTags(&self, name: &::windows_core::HSTRING, value: super::Point, format: LoggingFieldFormat, tags: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddPointWithFormatAndTags)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format, tags).ok() }
    }
    pub fn AddPointArray(&self, name: &::windows_core::HSTRING, value: &[super::Point]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddPointArray)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr()).ok() }
    }
    pub fn AddPointArrayWithFormat(&self, name: &::windows_core::HSTRING, value: &[super::Point], format: LoggingFieldFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddPointArrayWithFormat)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format).ok() }
    }
    pub fn AddPointArrayWithFormatAndTags(&self, name: &::windows_core::HSTRING, value: &[super::Point], format: LoggingFieldFormat, tags: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddPointArrayWithFormatAndTags)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format, tags).ok() }
    }
    pub fn AddSize(&self, name: &::windows_core::HSTRING, value: super::Size) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddSize)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value).ok() }
    }
    pub fn AddSizeWithFormat(&self, name: &::windows_core::HSTRING, value: super::Size, format: LoggingFieldFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddSizeWithFormat)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format).ok() }
    }
    pub fn AddSizeWithFormatAndTags(&self, name: &::windows_core::HSTRING, value: super::Size, format: LoggingFieldFormat, tags: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddSizeWithFormatAndTags)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format, tags).ok() }
    }
    pub fn AddSizeArray(&self, name: &::windows_core::HSTRING, value: &[super::Size]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddSizeArray)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr()).ok() }
    }
    pub fn AddSizeArrayWithFormat(&self, name: &::windows_core::HSTRING, value: &[super::Size], format: LoggingFieldFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddSizeArrayWithFormat)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format).ok() }
    }
    pub fn AddSizeArrayWithFormatAndTags(&self, name: &::windows_core::HSTRING, value: &[super::Size], format: LoggingFieldFormat, tags: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddSizeArrayWithFormatAndTags)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format, tags).ok() }
    }
    pub fn AddRect(&self, name: &::windows_core::HSTRING, value: super::Rect) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddRect)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value).ok() }
    }
    pub fn AddRectWithFormat(&self, name: &::windows_core::HSTRING, value: super::Rect, format: LoggingFieldFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddRectWithFormat)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format).ok() }
    }
    pub fn AddRectWithFormatAndTags(&self, name: &::windows_core::HSTRING, value: super::Rect, format: LoggingFieldFormat, tags: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddRectWithFormatAndTags)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, format, tags).ok() }
    }
    pub fn AddRectArray(&self, name: &::windows_core::HSTRING, value: &[super::Rect]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddRectArray)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr()).ok() }
    }
    pub fn AddRectArrayWithFormat(&self, name: &::windows_core::HSTRING, value: &[super::Rect], format: LoggingFieldFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddRectArrayWithFormat)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format).ok() }
    }
    pub fn AddRectArrayWithFormatAndTags(&self, name: &::windows_core::HSTRING, value: &[super::Rect], format: LoggingFieldFormat, tags: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddRectArrayWithFormatAndTags)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.len() as u32, value.as_ptr(), format, tags).ok() }
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
impl ::windows_core::RuntimeType for LoggingFields {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.LoggingFields;{d7f6b7af-762d-4579-83bd-52c23bc333bc})");
}
impl ::core::clone::Clone for LoggingFields {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for LoggingFields {
    type Vtable = ILoggingFields_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LoggingFields {
    const IID: ::windows_core::GUID = <ILoggingFields as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LoggingFields {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.LoggingFields";
}
::windows_core::imp::interface_hierarchy!(LoggingFields, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for LoggingFields {}
unsafe impl ::core::marker::Sync for LoggingFields {}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
pub struct LoggingOptions(::windows_core::IUnknown);
impl LoggingOptions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<LoggingOptions, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Keywords(&self) -> ::windows_core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Keywords)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetKeywords(&self, value: i64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetKeywords)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Tags(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Tags)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTags(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTags)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Task(&self) -> ::windows_core::Result<i16> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Task)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTask(&self, value: i16) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTask)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Opcode(&self) -> ::windows_core::Result<LoggingOpcode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Opcode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetOpcode(&self, value: LoggingOpcode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOpcode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ActivityId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivityId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetActivityId(&self, value: ::windows_core::GUID) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetActivityId)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RelatedActivityId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RelatedActivityId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRelatedActivityId(&self, value: ::windows_core::GUID) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRelatedActivityId)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CreateWithKeywords(keywords: i64) -> ::windows_core::Result<LoggingOptions> {
        Self::ILoggingOptionsFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithKeywords)(::windows_core::Interface::as_raw(this), keywords, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILoggingOptionsFactory<R, F: FnOnce(&ILoggingOptionsFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<LoggingOptions, ILoggingOptionsFactory> = ::windows_core::imp::FactoryCache::new();
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
impl ::windows_core::RuntimeType for LoggingOptions {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.LoggingOptions;{90bc7850-0192-4f5d-ac26-006adaca12d8})");
}
impl ::core::clone::Clone for LoggingOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for LoggingOptions {
    type Vtable = ILoggingOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LoggingOptions {
    const IID: ::windows_core::GUID = <ILoggingOptions as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LoggingOptions {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.LoggingOptions";
}
::windows_core::imp::interface_hierarchy!(LoggingOptions, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for LoggingOptions {}
unsafe impl ::core::marker::Sync for LoggingOptions {}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
pub struct LoggingSession(::windows_core::IUnknown);
impl LoggingSession {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn SaveToFileAsync<P0>(&self, folder: P0, filename: &::windows_core::HSTRING) -> ::windows_core::Result<super::IAsyncOperation<super::super::Storage::StorageFile>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::IStorageFolder>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SaveToFileAsync)(::windows_core::Interface::as_raw(this), folder.try_into_param()?.abi(), ::core::mem::transmute_copy(filename), &mut result__).from_abi(result__)
        }
    }
    pub fn AddLoggingChannel<P0>(&self, loggingchannel: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<ILoggingChannel>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddLoggingChannel)(::windows_core::Interface::as_raw(this), loggingchannel.try_into_param()?.abi()).ok() }
    }
    pub fn AddLoggingChannelWithLevel<P0>(&self, loggingchannel: P0, maxlevel: LoggingLevel) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<ILoggingChannel>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddLoggingChannelWithLevel)(::windows_core::Interface::as_raw(this), loggingchannel.try_into_param()?.abi(), maxlevel).ok() }
    }
    pub fn RemoveLoggingChannel<P0>(&self, loggingchannel: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<ILoggingChannel>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveLoggingChannel)(::windows_core::Interface::as_raw(this), loggingchannel.try_into_param()?.abi()).ok() }
    }
    pub fn Create(name: &::windows_core::HSTRING) -> ::windows_core::Result<LoggingSession> {
        Self::ILoggingSessionFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILoggingSessionFactory<R, F: FnOnce(&ILoggingSessionFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<LoggingSession, ILoggingSessionFactory> = ::windows_core::imp::FactoryCache::new();
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
impl ::windows_core::RuntimeType for LoggingSession {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.LoggingSession;{6221f306-9380-4ad7-baf5-41ea9310d768})");
}
impl ::core::clone::Clone for LoggingSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for LoggingSession {
    type Vtable = ILoggingSession_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LoggingSession {
    const IID: ::windows_core::GUID = <ILoggingSession as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LoggingSession {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.LoggingSession";
}
::windows_core::imp::interface_hierarchy!(LoggingSession, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<super::IClosable> for LoggingSession {}
impl ::windows_core::CanTryInto<ILoggingSession> for LoggingSession {}
unsafe impl ::core::marker::Send for LoggingSession {}
unsafe impl ::core::marker::Sync for LoggingSession {}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
pub struct RuntimeBrokerErrorSettings(::windows_core::IUnknown);
impl RuntimeBrokerErrorSettings {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<RuntimeBrokerErrorSettings, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetErrorOptions(&self, value: ErrorOptions) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetErrorOptions)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetErrorOptions(&self) -> ::windows_core::Result<ErrorOptions> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetErrorOptions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
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
impl ::windows_core::RuntimeType for RuntimeBrokerErrorSettings {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.RuntimeBrokerErrorSettings;{16369792-b03e-4ba1-8bb8-d28f4ab4d2c0})");
}
impl ::core::clone::Clone for RuntimeBrokerErrorSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for RuntimeBrokerErrorSettings {
    type Vtable = IErrorReportingSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RuntimeBrokerErrorSettings {
    const IID: ::windows_core::GUID = <IErrorReportingSettings as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for RuntimeBrokerErrorSettings {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.RuntimeBrokerErrorSettings";
}
::windows_core::imp::interface_hierarchy!(RuntimeBrokerErrorSettings, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IErrorReportingSettings> for RuntimeBrokerErrorSettings {}
unsafe impl ::core::marker::Send for RuntimeBrokerErrorSettings {}
unsafe impl ::core::marker::Sync for RuntimeBrokerErrorSettings {}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
pub struct TracingStatusChangedEventArgs(::windows_core::IUnknown);
impl TracingStatusChangedEventArgs {
    pub fn Enabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Enabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TraceLevel(&self) -> ::windows_core::Result<CausalityTraceLevel> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TraceLevel)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
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
impl ::windows_core::RuntimeType for TracingStatusChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.TracingStatusChangedEventArgs;{410b7711-ff3b-477f-9c9a-d2efda302dc3})");
}
impl ::core::clone::Clone for TracingStatusChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for TracingStatusChangedEventArgs {
    type Vtable = ITracingStatusChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TracingStatusChangedEventArgs {
    const IID: ::windows_core::GUID = <ITracingStatusChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TracingStatusChangedEventArgs {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.TracingStatusChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(TracingStatusChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
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
impl ::windows_core::TypeKind for CausalityRelation {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CausalityRelation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CausalityRelation").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CausalityRelation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Foundation.Diagnostics.CausalityRelation;i4)");
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
impl ::windows_core::TypeKind for CausalitySource {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CausalitySource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CausalitySource").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CausalitySource {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Foundation.Diagnostics.CausalitySource;i4)");
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
impl ::windows_core::TypeKind for CausalitySynchronousWork {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CausalitySynchronousWork {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CausalitySynchronousWork").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CausalitySynchronousWork {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Foundation.Diagnostics.CausalitySynchronousWork;i4)");
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
impl ::windows_core::TypeKind for CausalityTraceLevel {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CausalityTraceLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CausalityTraceLevel").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CausalityTraceLevel {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Foundation.Diagnostics.CausalityTraceLevel;i4)");
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
impl ::windows_core::TypeKind for ErrorOptions {
    type TypeKind = ::windows_core::CopyType;
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
impl ::windows_core::RuntimeType for ErrorOptions {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Foundation.Diagnostics.ErrorOptions;u4)");
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
impl ::windows_core::TypeKind for LoggingFieldFormat {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for LoggingFieldFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LoggingFieldFormat").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for LoggingFieldFormat {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Foundation.Diagnostics.LoggingFieldFormat;i4)");
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
impl ::windows_core::TypeKind for LoggingLevel {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for LoggingLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LoggingLevel").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for LoggingLevel {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Foundation.Diagnostics.LoggingLevel;i4)");
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
impl ::windows_core::TypeKind for LoggingOpcode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for LoggingOpcode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LoggingOpcode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for LoggingOpcode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Foundation.Diagnostics.LoggingOpcode;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
