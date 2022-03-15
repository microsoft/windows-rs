#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
pub struct AsyncCausalityTracer {}
impl AsyncCausalityTracer {
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn TraceOperationCreation<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param4: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: Param2, operationid: u64, operationname: Param4, relatedcontext: u64) -> ::windows::core::Result<()> {
        Self::IAsyncCausalityTracerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).TraceOperationCreation)(::core::mem::transmute_copy(this), tracelevel, source, platformid.into_param().abi(), operationid, operationname.into_param().abi(), relatedcontext).ok() })
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn TraceOperationCompletion<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: Param2, operationid: u64, status: super::AsyncStatus) -> ::windows::core::Result<()> {
        Self::IAsyncCausalityTracerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).TraceOperationCompletion)(::core::mem::transmute_copy(this), tracelevel, source, platformid.into_param().abi(), operationid, status).ok() })
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn TraceOperationRelation<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: Param2, operationid: u64, relation: CausalityRelation) -> ::windows::core::Result<()> {
        Self::IAsyncCausalityTracerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).TraceOperationRelation)(::core::mem::transmute_copy(this), tracelevel, source, platformid.into_param().abi(), operationid, relation).ok() })
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn TraceSynchronousWorkStart<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: Param2, operationid: u64, work: CausalitySynchronousWork) -> ::windows::core::Result<()> {
        Self::IAsyncCausalityTracerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).TraceSynchronousWorkStart)(::core::mem::transmute_copy(this), tracelevel, source, platformid.into_param().abi(), operationid, work).ok() })
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn TraceSynchronousWorkCompletion(tracelevel: CausalityTraceLevel, source: CausalitySource, work: CausalitySynchronousWork) -> ::windows::core::Result<()> {
        Self::IAsyncCausalityTracerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).TraceSynchronousWorkCompletion)(::core::mem::transmute_copy(this), tracelevel, source, work).ok() })
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn TracingStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::EventHandler<TracingStatusChangedEventArgs>>>(handler: Param0) -> ::windows::core::Result<super::EventRegistrationToken> {
        Self::IAsyncCausalityTracerStatics(|this| unsafe {
            let mut result__: super::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TracingStatusChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn RemoveTracingStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::EventRegistrationToken>>(cookie: Param0) -> ::windows::core::Result<()> {
        Self::IAsyncCausalityTracerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveTracingStatusChanged)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() })
    }
    #[doc(hidden)]
    pub fn IAsyncCausalityTracerStatics<R, F: FnOnce(&IAsyncCausalityTracerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AsyncCausalityTracer, IAsyncCausalityTracerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for AsyncCausalityTracer {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.AsyncCausalityTracer";
}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for CausalityRelation {
    type Abi = Self;
}
impl ::core::fmt::Debug for CausalityRelation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CausalityRelation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CausalityRelation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Foundation.Diagnostics.CausalityRelation;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for CausalitySource {
    type Abi = Self;
}
impl ::core::fmt::Debug for CausalitySource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CausalitySource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CausalitySource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Foundation.Diagnostics.CausalitySource;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for CausalitySynchronousWork {
    type Abi = Self;
}
impl ::core::fmt::Debug for CausalitySynchronousWork {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CausalitySynchronousWork").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CausalitySynchronousWork {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Foundation.Diagnostics.CausalitySynchronousWork;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for CausalityTraceLevel {
    type Abi = Self;
}
impl ::core::fmt::Debug for CausalityTraceLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CausalityTraceLevel").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CausalityTraceLevel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Foundation.Diagnostics.CausalityTraceLevel;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
pub struct ErrorDetails(::windows::core::IUnknown);
impl ErrorDetails {
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Description)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn LongDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LongDescription)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn HelpUri(&self) -> ::windows::core::Result<super::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HelpUri)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn CreateFromHResultAsync(errorcode: i32) -> ::windows::core::Result<super::IAsyncOperation<ErrorDetails>> {
        Self::IErrorDetailsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromHResultAsync)(::core::mem::transmute_copy(this), errorcode, &mut result__).from_abi::<super::IAsyncOperation<ErrorDetails>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IErrorDetailsStatics<R, F: FnOnce(&IErrorDetailsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ErrorDetails, IErrorDetailsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ErrorDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for ErrorDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.ErrorDetails;{378cbb01-2cc9-428f-8c55-2c990d463e8f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ErrorDetails {
    type Vtable = IErrorDetails_Vtbl;
    const IID: ::windows::core::GUID = <IErrorDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ErrorDetails {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.ErrorDetails";
}
impl ::core::convert::From<ErrorDetails> for ::windows::core::IUnknown {
    fn from(value: ErrorDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ErrorDetails> for ::windows::core::IUnknown {
    fn from(value: &ErrorDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ErrorDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ErrorDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ErrorDetails> for ::windows::core::IInspectable {
    fn from(value: ErrorDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ErrorDetails> for ::windows::core::IInspectable {
    fn from(value: &ErrorDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ErrorDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ErrorDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ErrorDetails {}
unsafe impl ::core::marker::Sync for ErrorDetails {}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for ErrorOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for ErrorOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ErrorOptions").field(&self.0).finish()
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
unsafe impl ::windows::core::RuntimeType for ErrorOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Foundation.Diagnostics.ErrorOptions;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
pub struct FileLoggingSession(::windows::core::IUnknown);
impl FileLoggingSession {
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddLoggingChannel<'a, Param0: ::windows::core::IntoParam<'a, ILoggingChannel>>(&self, loggingchannel: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddLoggingChannel)(::core::mem::transmute_copy(this), loggingchannel.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddLoggingChannelWithLevel<'a, Param0: ::windows::core::IntoParam<'a, ILoggingChannel>>(&self, loggingchannel: Param0, maxlevel: LoggingLevel) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddLoggingChannelWithLevel)(::core::mem::transmute_copy(this), loggingchannel.into_param().abi(), maxlevel).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn RemoveLoggingChannel<'a, Param0: ::windows::core::IntoParam<'a, ILoggingChannel>>(&self, loggingchannel: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveLoggingChannel)(::core::mem::transmute_copy(this), loggingchannel.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`, `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn CloseAndSaveToFileAsync(&self) -> ::windows::core::Result<super::IAsyncOperation<super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CloseAndSaveToFileAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::IAsyncOperation<super::super::Storage::StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn LogFileGenerated<'a, Param0: ::windows::core::IntoParam<'a, super::TypedEventHandler<IFileLoggingSession, LogFileGeneratedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LogFileGenerated)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn RemoveLogFileGenerated<'a, Param0: ::windows::core::IntoParam<'a, super::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveLogFileGenerated)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(name: Param0) -> ::windows::core::Result<FileLoggingSession> {
        Self::IFileLoggingSessionFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<FileLoggingSession>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IFileLoggingSessionFactory<R, F: FnOnce(&IFileLoggingSessionFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FileLoggingSession, IFileLoggingSessionFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for FileLoggingSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for FileLoggingSession {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.FileLoggingSession;{24c74216-fed2-404c-895f-1f9699cb02f7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for FileLoggingSession {
    type Vtable = IFileLoggingSession_Vtbl;
    const IID: ::windows::core::GUID = <IFileLoggingSession as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FileLoggingSession {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.FileLoggingSession";
}
impl ::core::convert::From<FileLoggingSession> for ::windows::core::IUnknown {
    fn from(value: FileLoggingSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileLoggingSession> for ::windows::core::IUnknown {
    fn from(value: &FileLoggingSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FileLoggingSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FileLoggingSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FileLoggingSession> for ::windows::core::IInspectable {
    fn from(value: FileLoggingSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileLoggingSession> for ::windows::core::IInspectable {
    fn from(value: &FileLoggingSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FileLoggingSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FileLoggingSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<FileLoggingSession> for super::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: FileLoggingSession) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileLoggingSession> for super::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileLoggingSession) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IClosable> for FileLoggingSession {
    fn into_param(self) -> ::windows::core::Param<'a, super::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IClosable> for &FileLoggingSession {
    fn into_param(self) -> ::windows::core::Param<'a, super::IClosable> {
        ::core::convert::TryInto::<super::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileLoggingSession> for IFileLoggingSession {
    type Error = ::windows::core::Error;
    fn try_from(value: FileLoggingSession) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileLoggingSession> for IFileLoggingSession {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileLoggingSession) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFileLoggingSession> for FileLoggingSession {
    fn into_param(self) -> ::windows::core::Param<'a, IFileLoggingSession> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFileLoggingSession> for &FileLoggingSession {
    fn into_param(self) -> ::windows::core::Param<'a, IFileLoggingSession> {
        ::core::convert::TryInto::<IFileLoggingSession>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for FileLoggingSession {}
unsafe impl ::core::marker::Sync for FileLoggingSession {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAsyncCausalityTracerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAsyncCausalityTracerStatics {
    type Vtable = IAsyncCausalityTracerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50850b26_267e_451b_a890_ab6a370245ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsyncCausalityTracerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub TraceOperationCreation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: ::windows::core::GUID, operationid: u64, operationname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, relatedcontext: u64) -> ::windows::core::HRESULT,
    pub TraceOperationCompletion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: ::windows::core::GUID, operationid: u64, status: super::AsyncStatus) -> ::windows::core::HRESULT,
    pub TraceOperationRelation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: ::windows::core::GUID, operationid: u64, relation: CausalityRelation) -> ::windows::core::HRESULT,
    pub TraceSynchronousWorkStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: ::windows::core::GUID, operationid: u64, work: CausalitySynchronousWork) -> ::windows::core::HRESULT,
    pub TraceSynchronousWorkCompletion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tracelevel: CausalityTraceLevel, source: CausalitySource, work: CausalitySynchronousWork) -> ::windows::core::HRESULT,
    pub TracingStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::EventRegistrationToken) -> ::windows::core::HRESULT,
    pub RemoveTracingStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::EventRegistrationToken) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IErrorDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IErrorDetails {
    type Vtable = IErrorDetails_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x378cbb01_2cc9_428f_8c55_2c990d463e8f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IErrorDetails_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub LongDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub HelpUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IErrorDetailsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IErrorDetailsStatics {
    type Vtable = IErrorDetailsStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb7703750_0b1d_46c8_aa0e_4b8178e4fce9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IErrorDetailsStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateFromHResultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, errorcode: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
pub struct IErrorReportingSettings(::windows::core::IUnknown);
impl IErrorReportingSettings {
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn SetErrorOptions(&self, value: ErrorOptions) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetErrorOptions)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn GetErrorOptions(&self) -> ::windows::core::Result<ErrorOptions> {
        let this = self;
        unsafe {
            let mut result__: ErrorOptions = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetErrorOptions)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ErrorOptions>(result__)
        }
    }
}
impl ::core::convert::From<IErrorReportingSettings> for ::windows::core::IUnknown {
    fn from(value: IErrorReportingSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IErrorReportingSettings> for ::windows::core::IUnknown {
    fn from(value: &IErrorReportingSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IErrorReportingSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IErrorReportingSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IErrorReportingSettings> for ::windows::core::IInspectable {
    fn from(value: IErrorReportingSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IErrorReportingSettings> for ::windows::core::IInspectable {
    fn from(value: &IErrorReportingSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IErrorReportingSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IErrorReportingSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IErrorReportingSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::RuntimeType for IErrorReportingSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{16369792-b03e-4ba1-8bb8-d28f4ab4d2c0}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IErrorReportingSettings {
    type Vtable = IErrorReportingSettings_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x16369792_b03e_4ba1_8bb8_d28f4ab4d2c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IErrorReportingSettings_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SetErrorOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ErrorOptions) -> ::windows::core::HRESULT,
    pub GetErrorOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ErrorOptions) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
pub struct IFileLoggingSession(::windows::core::IUnknown);
impl IFileLoggingSession {
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddLoggingChannel<'a, Param0: ::windows::core::IntoParam<'a, ILoggingChannel>>(&self, loggingchannel: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddLoggingChannel)(::core::mem::transmute_copy(this), loggingchannel.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddLoggingChannelWithLevel<'a, Param0: ::windows::core::IntoParam<'a, ILoggingChannel>>(&self, loggingchannel: Param0, maxlevel: LoggingLevel) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddLoggingChannelWithLevel)(::core::mem::transmute_copy(this), loggingchannel.into_param().abi(), maxlevel).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn RemoveLoggingChannel<'a, Param0: ::windows::core::IntoParam<'a, ILoggingChannel>>(&self, loggingchannel: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveLoggingChannel)(::core::mem::transmute_copy(this), loggingchannel.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`, `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn CloseAndSaveToFileAsync(&self) -> ::windows::core::Result<super::IAsyncOperation<super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CloseAndSaveToFileAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::IAsyncOperation<super::super::Storage::StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn LogFileGenerated<'a, Param0: ::windows::core::IntoParam<'a, super::TypedEventHandler<IFileLoggingSession, LogFileGeneratedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LogFileGenerated)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn RemoveLogFileGenerated<'a, Param0: ::windows::core::IntoParam<'a, super::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveLogFileGenerated)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::convert::From<IFileLoggingSession> for ::windows::core::IUnknown {
    fn from(value: IFileLoggingSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFileLoggingSession> for ::windows::core::IUnknown {
    fn from(value: &IFileLoggingSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFileLoggingSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFileLoggingSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFileLoggingSession> for ::windows::core::IInspectable {
    fn from(value: IFileLoggingSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFileLoggingSession> for ::windows::core::IInspectable {
    fn from(value: &IFileLoggingSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IFileLoggingSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IFileLoggingSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IFileLoggingSession> for super::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: IFileLoggingSession) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IFileLoggingSession> for super::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &IFileLoggingSession) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IClosable> for IFileLoggingSession {
    fn into_param(self) -> ::windows::core::Param<'a, super::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IClosable> for &IFileLoggingSession {
    fn into_param(self) -> ::windows::core::Param<'a, super::IClosable> {
        ::core::convert::TryInto::<super::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IFileLoggingSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::RuntimeType for IFileLoggingSession {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{24c74216-fed2-404c-895f-1f9699cb02f7}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IFileLoggingSession {
    type Vtable = IFileLoggingSession_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24c74216_fed2_404c_895f_1f9699cb02f7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileLoggingSession_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AddLoggingChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, loggingchannel: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub AddLoggingChannelWithLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, loggingchannel: ::windows::core::RawPtr, maxlevel: LoggingLevel) -> ::windows::core::HRESULT,
    pub RemoveLoggingChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, loggingchannel: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage")]
    pub CloseAndSaveToFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    CloseAndSaveToFileAsync: usize,
    pub LogFileGenerated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::EventRegistrationToken) -> ::windows::core::HRESULT,
    pub RemoveLogFileGenerated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::EventRegistrationToken) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFileLoggingSessionFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFileLoggingSessionFactory {
    type Vtable = IFileLoggingSessionFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeea08dce_8447_4daa_9133_12eb46f697d4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileLoggingSessionFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILogFileGeneratedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILogFileGeneratedEventArgs {
    type Vtable = ILogFileGeneratedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x269e976f_0d38_4c1a_b53f_b395d881df84);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILogFileGeneratedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage")]
    pub File: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    File: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILoggingActivity(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILoggingActivity {
    type Vtable = ILoggingActivity_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc032941_b766_4cb5_9848_97ac6ba6d60c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingActivity_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILoggingActivity2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILoggingActivity2 {
    type Vtable = ILoggingActivity2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x26c29808_6322_456a_af82_80c8642f178b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingActivity2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Channel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub StopActivity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stopeventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub StopActivityWithFields: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stopeventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fields: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub StopActivityWithFieldsAndOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stopeventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fields: ::windows::core::RawPtr, options: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILoggingActivityFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILoggingActivityFactory {
    type Vtable = ILoggingActivityFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b33b483_e10a_4c58_97d5_10fb451074fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingActivityFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateLoggingActivity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activityname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, loggingchannel: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateLoggingActivityWithLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activityname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, loggingchannel: ::windows::core::RawPtr, level: LoggingLevel, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
pub struct ILoggingChannel(::windows::core::IUnknown);
impl ILoggingChannel {
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn Enabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Enabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn Level(&self) -> ::windows::core::Result<LoggingLevel> {
        let this = self;
        unsafe {
            let mut result__: LoggingLevel = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Level)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LoggingLevel>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn LogMessage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, eventstring: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).LogMessage)(::core::mem::transmute_copy(this), eventstring.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn LogMessageWithLevel<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, eventstring: Param0, level: LoggingLevel) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).LogMessageWithLevel)(::core::mem::transmute_copy(this), eventstring.into_param().abi(), level).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn LogValuePair<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value1: Param0, value2: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).LogValuePair)(::core::mem::transmute_copy(this), value1.into_param().abi(), value2).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn LogValuePairWithLevel<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value1: Param0, value2: i32, level: LoggingLevel) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).LogValuePairWithLevel)(::core::mem::transmute_copy(this), value1.into_param().abi(), value2, level).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn LoggingEnabled<'a, Param0: ::windows::core::IntoParam<'a, super::TypedEventHandler<ILoggingChannel, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LoggingEnabled)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn RemoveLoggingEnabled<'a, Param0: ::windows::core::IntoParam<'a, super::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveLoggingEnabled)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::convert::From<ILoggingChannel> for ::windows::core::IUnknown {
    fn from(value: ILoggingChannel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILoggingChannel> for ::windows::core::IUnknown {
    fn from(value: &ILoggingChannel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ILoggingChannel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ILoggingChannel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ILoggingChannel> for ::windows::core::IInspectable {
    fn from(value: ILoggingChannel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILoggingChannel> for ::windows::core::IInspectable {
    fn from(value: &ILoggingChannel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ILoggingChannel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ILoggingChannel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ILoggingChannel> for super::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: ILoggingChannel) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ILoggingChannel> for super::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &ILoggingChannel) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IClosable> for ILoggingChannel {
    fn into_param(self) -> ::windows::core::Param<'a, super::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IClosable> for &ILoggingChannel {
    fn into_param(self) -> ::windows::core::Param<'a, super::IClosable> {
        ::core::convert::TryInto::<super::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for ILoggingChannel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::RuntimeType for ILoggingChannel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{e9a50343-11d7-4f01-b5ca-cf495278c0a8}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ILoggingChannel {
    type Vtable = ILoggingChannel_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe9a50343_11d7_4f01_b5ca_cf495278c0a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingChannel_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Level: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LoggingLevel) -> ::windows::core::HRESULT,
    pub LogMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventstring: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub LogMessageWithLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventstring: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, level: LoggingLevel) -> ::windows::core::HRESULT,
    pub LogValuePair: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value1: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value2: i32) -> ::windows::core::HRESULT,
    pub LogValuePairWithLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value1: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value2: i32, level: LoggingLevel) -> ::windows::core::HRESULT,
    pub LoggingEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::EventRegistrationToken) -> ::windows::core::HRESULT,
    pub RemoveLoggingEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::EventRegistrationToken) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILoggingChannel2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILoggingChannel2 {
    type Vtable = ILoggingChannel2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f4c3cf3_0bac_45a5_9e33_baf3f3a246a5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingChannel2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILoggingChannelFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILoggingChannelFactory {
    type Vtable = ILoggingChannelFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4edc5b9c_af80_4a9b_b0dc_398f9ae5207b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingChannelFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILoggingChannelFactory2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILoggingChannelFactory2 {
    type Vtable = ILoggingChannelFactory2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4c6ef5dd_3b27_4dc9_99f0_299c6e4603a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingChannelFactory2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateWithOptionsAndId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: ::windows::core::RawPtr, id: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILoggingChannelOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILoggingChannelOptions {
    type Vtable = ILoggingChannelOptions_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc3e847ff_0ebb_4a53_8c54_dec24926cb2c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingChannelOptions_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Group: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILoggingChannelOptionsFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILoggingChannelOptionsFactory {
    type Vtable = ILoggingChannelOptionsFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa93151da_7faf_4191_8755_5e86dc65d896);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingChannelOptionsFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, group: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILoggingFields(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILoggingFields {
    type Vtable = ILoggingFields_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7f6b7af_762d_4579_83bd_52c23bc333bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingFields_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub BeginStruct: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub BeginStructWithTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, tags: i32) -> ::windows::core::HRESULT,
    pub EndStruct: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddEmpty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AddEmptyWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddEmptyWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddUInt8: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u8) -> ::windows::core::HRESULT,
    pub AddUInt8WithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u8, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddUInt8WithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u8, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddUInt8Array: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u8) -> ::windows::core::HRESULT,
    pub AddUInt8ArrayWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u8, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddUInt8ArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u8, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddInt16: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: i16) -> ::windows::core::HRESULT,
    pub AddInt16WithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: i16, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddInt16WithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: i16, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddInt16Array: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const i16) -> ::windows::core::HRESULT,
    pub AddInt16ArrayWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const i16, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddInt16ArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const i16, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddUInt16: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u16) -> ::windows::core::HRESULT,
    pub AddUInt16WithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u16, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddUInt16WithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u16, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddUInt16Array: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u16) -> ::windows::core::HRESULT,
    pub AddUInt16ArrayWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u16, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddUInt16ArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u16, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddInt32: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: i32) -> ::windows::core::HRESULT,
    pub AddInt32WithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: i32, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddInt32WithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: i32, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddInt32Array: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const i32) -> ::windows::core::HRESULT,
    pub AddInt32ArrayWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const i32, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddInt32ArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const i32, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddUInt32: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u32) -> ::windows::core::HRESULT,
    pub AddUInt32WithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u32, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddUInt32WithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u32, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddUInt32Array: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u32) -> ::windows::core::HRESULT,
    pub AddUInt32ArrayWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u32, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddUInt32ArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u32, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddInt64: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: i64) -> ::windows::core::HRESULT,
    pub AddInt64WithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: i64, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddInt64WithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: i64, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddInt64Array: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const i64) -> ::windows::core::HRESULT,
    pub AddInt64ArrayWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const i64, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddInt64ArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const i64, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddUInt64: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u64) -> ::windows::core::HRESULT,
    pub AddUInt64WithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u64, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddUInt64WithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u64, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddUInt64Array: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u64) -> ::windows::core::HRESULT,
    pub AddUInt64ArrayWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u64, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddUInt64ArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u64, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddSingle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: f32) -> ::windows::core::HRESULT,
    pub AddSingleWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: f32, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddSingleWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: f32, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddSingleArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const f32) -> ::windows::core::HRESULT,
    pub AddSingleArrayWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const f32, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddSingleArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const f32, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddDouble: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: f64) -> ::windows::core::HRESULT,
    pub AddDoubleWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: f64, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddDoubleWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: f64, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddDoubleArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const f64) -> ::windows::core::HRESULT,
    pub AddDoubleArrayWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const f64, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddDoubleArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const f64, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddChar16: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u16) -> ::windows::core::HRESULT,
    pub AddChar16WithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u16, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddChar16WithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u16, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddChar16Array: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u16) -> ::windows::core::HRESULT,
    pub AddChar16ArrayWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u16, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddChar16ArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u16, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddBoolean: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: bool) -> ::windows::core::HRESULT,
    pub AddBooleanWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: bool, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddBooleanWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: bool, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddBooleanArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const bool) -> ::windows::core::HRESULT,
    pub AddBooleanArrayWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const bool, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddBooleanArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const bool, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AddStringWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddStringWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddStringArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AddStringArrayWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const ::core::mem::ManuallyDrop<::windows::core::HSTRING>, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddStringArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const ::core::mem::ManuallyDrop<::windows::core::HSTRING>, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub AddGuidWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::windows::core::GUID, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddGuidWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::windows::core::GUID, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddGuidArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub AddGuidArrayWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const ::windows::core::GUID, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddGuidArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const ::windows::core::GUID, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddDateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::DateTime) -> ::windows::core::HRESULT,
    pub AddDateTimeWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::DateTime, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddDateTimeWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::DateTime, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddDateTimeArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::DateTime) -> ::windows::core::HRESULT,
    pub AddDateTimeArrayWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::DateTime, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddDateTimeArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::DateTime, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddTimeSpan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::TimeSpan) -> ::windows::core::HRESULT,
    pub AddTimeSpanWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::TimeSpan, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddTimeSpanWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::TimeSpan, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddTimeSpanArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::TimeSpan) -> ::windows::core::HRESULT,
    pub AddTimeSpanArrayWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::TimeSpan, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddTimeSpanArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::TimeSpan, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::Point) -> ::windows::core::HRESULT,
    pub AddPointWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::Point, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddPointWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::Point, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddPointArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::Point) -> ::windows::core::HRESULT,
    pub AddPointArrayWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::Point, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddPointArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::Point, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::Size) -> ::windows::core::HRESULT,
    pub AddSizeWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::Size, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddSizeWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::Size, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddSizeArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::Size) -> ::windows::core::HRESULT,
    pub AddSizeArrayWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::Size, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddSizeArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::Size, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::Rect) -> ::windows::core::HRESULT,
    pub AddRectWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::Rect, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddRectWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::Rect, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub AddRectArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::Rect) -> ::windows::core::HRESULT,
    pub AddRectArrayWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::Rect, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub AddRectArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::Rect, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILoggingOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILoggingOptions {
    type Vtable = ILoggingOptions_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x90bc7850_0192_4f5d_ac26_006adaca12d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingOptions_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd713c6cb_98ab_464b_9f22_a3268478368a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingOptionsFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateWithKeywords: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keywords: i64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
pub struct ILoggingSession(::windows::core::IUnknown);
impl ILoggingSession {
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`, `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn SaveToFileAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::IStorageFolder>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, folder: Param0, filename: Param1) -> ::windows::core::Result<super::IAsyncOperation<super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SaveToFileAsync)(::core::mem::transmute_copy(this), folder.into_param().abi(), filename.into_param().abi(), &mut result__).from_abi::<super::IAsyncOperation<super::super::Storage::StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddLoggingChannel<'a, Param0: ::windows::core::IntoParam<'a, ILoggingChannel>>(&self, loggingchannel: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddLoggingChannel)(::core::mem::transmute_copy(this), loggingchannel.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddLoggingChannelWithLevel<'a, Param0: ::windows::core::IntoParam<'a, ILoggingChannel>>(&self, loggingchannel: Param0, maxlevel: LoggingLevel) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddLoggingChannelWithLevel)(::core::mem::transmute_copy(this), loggingchannel.into_param().abi(), maxlevel).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn RemoveLoggingChannel<'a, Param0: ::windows::core::IntoParam<'a, ILoggingChannel>>(&self, loggingchannel: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveLoggingChannel)(::core::mem::transmute_copy(this), loggingchannel.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::convert::From<ILoggingSession> for ::windows::core::IUnknown {
    fn from(value: ILoggingSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILoggingSession> for ::windows::core::IUnknown {
    fn from(value: &ILoggingSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ILoggingSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ILoggingSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ILoggingSession> for ::windows::core::IInspectable {
    fn from(value: ILoggingSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILoggingSession> for ::windows::core::IInspectable {
    fn from(value: &ILoggingSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ILoggingSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ILoggingSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ILoggingSession> for super::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: ILoggingSession) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ILoggingSession> for super::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &ILoggingSession) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IClosable> for ILoggingSession {
    fn into_param(self) -> ::windows::core::Param<'a, super::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IClosable> for &ILoggingSession {
    fn into_param(self) -> ::windows::core::Param<'a, super::IClosable> {
        ::core::convert::TryInto::<super::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for ILoggingSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::RuntimeType for ILoggingSession {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{6221f306-9380-4ad7-baf5-41ea9310d768}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ILoggingSession {
    type Vtable = ILoggingSession_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6221f306_9380_4ad7_baf5_41ea9310d768);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingSession_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage")]
    pub SaveToFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folder: ::windows::core::RawPtr, filename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    SaveToFileAsync: usize,
    pub AddLoggingChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, loggingchannel: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub AddLoggingChannelWithLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, loggingchannel: ::windows::core::RawPtr, maxlevel: LoggingLevel) -> ::windows::core::HRESULT,
    pub RemoveLoggingChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, loggingchannel: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILoggingSessionFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILoggingSessionFactory {
    type Vtable = ILoggingSessionFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e937ee5_58fd_45e0_8c2f_a132eff95c1e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingSessionFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
pub struct ILoggingTarget(::windows::core::IUnknown);
impl ILoggingTarget {
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn IsEnabledWithLevel(&self, level: LoggingLevel) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsEnabledWithLevel)(::core::mem::transmute_copy(this), level, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn IsEnabledWithLevelAndKeywords(&self, level: LoggingLevel, keywords: i64) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsEnabledWithLevelAndKeywords)(::core::mem::transmute_copy(this), level, keywords, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn LogEvent<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, eventname: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).LogEvent)(::core::mem::transmute_copy(this), eventname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn LogEventWithFields<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, LoggingFields>>(&self, eventname: Param0, fields: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).LogEventWithFields)(::core::mem::transmute_copy(this), eventname.into_param().abi(), fields.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn LogEventWithFieldsAndLevel<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, LoggingFields>>(&self, eventname: Param0, fields: Param1, level: LoggingLevel) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).LogEventWithFieldsAndLevel)(::core::mem::transmute_copy(this), eventname.into_param().abi(), fields.into_param().abi(), level).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn LogEventWithFieldsAndOptions<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, LoggingFields>, Param3: ::windows::core::IntoParam<'a, LoggingOptions>>(&self, eventname: Param0, fields: Param1, level: LoggingLevel, options: Param3) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).LogEventWithFieldsAndOptions)(::core::mem::transmute_copy(this), eventname.into_param().abi(), fields.into_param().abi(), level, options.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn StartActivity<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, starteventname: Param0) -> ::windows::core::Result<LoggingActivity> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartActivity)(::core::mem::transmute_copy(this), starteventname.into_param().abi(), &mut result__).from_abi::<LoggingActivity>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn StartActivityWithFields<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, LoggingFields>>(&self, starteventname: Param0, fields: Param1) -> ::windows::core::Result<LoggingActivity> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartActivityWithFields)(::core::mem::transmute_copy(this), starteventname.into_param().abi(), fields.into_param().abi(), &mut result__).from_abi::<LoggingActivity>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn StartActivityWithFieldsAndLevel<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, LoggingFields>>(&self, starteventname: Param0, fields: Param1, level: LoggingLevel) -> ::windows::core::Result<LoggingActivity> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartActivityWithFieldsAndLevel)(::core::mem::transmute_copy(this), starteventname.into_param().abi(), fields.into_param().abi(), level, &mut result__).from_abi::<LoggingActivity>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn StartActivityWithFieldsAndOptions<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, LoggingFields>, Param3: ::windows::core::IntoParam<'a, LoggingOptions>>(&self, starteventname: Param0, fields: Param1, level: LoggingLevel, options: Param3) -> ::windows::core::Result<LoggingActivity> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartActivityWithFieldsAndOptions)(::core::mem::transmute_copy(this), starteventname.into_param().abi(), fields.into_param().abi(), level, options.into_param().abi(), &mut result__).from_abi::<LoggingActivity>(result__)
        }
    }
}
impl ::core::convert::From<ILoggingTarget> for ::windows::core::IUnknown {
    fn from(value: ILoggingTarget) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILoggingTarget> for ::windows::core::IUnknown {
    fn from(value: &ILoggingTarget) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ILoggingTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ILoggingTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ILoggingTarget> for ::windows::core::IInspectable {
    fn from(value: ILoggingTarget) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILoggingTarget> for ::windows::core::IInspectable {
    fn from(value: &ILoggingTarget) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ILoggingTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ILoggingTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ILoggingTarget {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::RuntimeType for ILoggingTarget {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{65f16c35-e388-4e26-b17a-f51cd3a83916}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ILoggingTarget {
    type Vtable = ILoggingTarget_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x65f16c35_e388_4e26_b17a_f51cd3a83916);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingTarget_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsEnabledWithLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, level: LoggingLevel, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsEnabledWithLevelAndKeywords: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, level: LoggingLevel, keywords: i64, result__: *mut bool) -> ::windows::core::HRESULT,
    pub LogEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub LogEventWithFields: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fields: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub LogEventWithFieldsAndLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fields: ::windows::core::RawPtr, level: LoggingLevel) -> ::windows::core::HRESULT,
    pub LogEventWithFieldsAndOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fields: ::windows::core::RawPtr, level: LoggingLevel, options: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub StartActivity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, starteventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub StartActivityWithFields: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, starteventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fields: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub StartActivityWithFieldsAndLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, starteventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fields: ::windows::core::RawPtr, level: LoggingLevel, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub StartActivityWithFieldsAndOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, starteventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fields: ::windows::core::RawPtr, level: LoggingLevel, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITracingStatusChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITracingStatusChangedEventArgs {
    type Vtable = ITracingStatusChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x410b7711_ff3b_477f_9c9a_d2efda302dc3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITracingStatusChangedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub TraceLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CausalityTraceLevel) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
pub struct LogFileGeneratedEventArgs(::windows::core::IUnknown);
impl LogFileGeneratedEventArgs {
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`, `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn File(&self) -> ::windows::core::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).File)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::StorageFile>(result__)
        }
    }
}
impl ::core::clone::Clone for LogFileGeneratedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for LogFileGeneratedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.LogFileGeneratedEventArgs;{269e976f-0d38-4c1a-b53f-b395d881df84})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LogFileGeneratedEventArgs {
    type Vtable = ILogFileGeneratedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ILogFileGeneratedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LogFileGeneratedEventArgs {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.LogFileGeneratedEventArgs";
}
impl ::core::convert::From<LogFileGeneratedEventArgs> for ::windows::core::IUnknown {
    fn from(value: LogFileGeneratedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LogFileGeneratedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &LogFileGeneratedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LogFileGeneratedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LogFileGeneratedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LogFileGeneratedEventArgs> for ::windows::core::IInspectable {
    fn from(value: LogFileGeneratedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LogFileGeneratedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &LogFileGeneratedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LogFileGeneratedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LogFileGeneratedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LogFileGeneratedEventArgs {}
unsafe impl ::core::marker::Sync for LogFileGeneratedEventArgs {}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
pub struct LoggingActivity(::windows::core::IUnknown);
impl LoggingActivity {
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn Channel(&self) -> ::windows::core::Result<LoggingChannel> {
        let this = &::windows::core::Interface::cast::<ILoggingActivity2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Channel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LoggingChannel>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn StopActivity<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, stopeventname: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ILoggingActivity2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StopActivity)(::core::mem::transmute_copy(this), stopeventname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn StopActivityWithFields<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, LoggingFields>>(&self, stopeventname: Param0, fields: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ILoggingActivity2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StopActivityWithFields)(::core::mem::transmute_copy(this), stopeventname.into_param().abi(), fields.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn StopActivityWithFieldsAndOptions<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, LoggingFields>, Param2: ::windows::core::IntoParam<'a, LoggingOptions>>(&self, stopeventname: Param0, fields: Param1, options: Param2) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ILoggingActivity2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StopActivityWithFieldsAndOptions)(::core::mem::transmute_copy(this), stopeventname.into_param().abi(), fields.into_param().abi(), options.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn CreateLoggingActivity<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ILoggingChannel>>(activityname: Param0, loggingchannel: Param1) -> ::windows::core::Result<LoggingActivity> {
        Self::ILoggingActivityFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateLoggingActivity)(::core::mem::transmute_copy(this), activityname.into_param().abi(), loggingchannel.into_param().abi(), &mut result__).from_abi::<LoggingActivity>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn CreateLoggingActivityWithLevel<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ILoggingChannel>>(activityname: Param0, loggingchannel: Param1, level: LoggingLevel) -> ::windows::core::Result<LoggingActivity> {
        Self::ILoggingActivityFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateLoggingActivityWithLevel)(::core::mem::transmute_copy(this), activityname.into_param().abi(), loggingchannel.into_param().abi(), level, &mut result__).from_abi::<LoggingActivity>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn IsEnabledWithLevel(&self, level: LoggingLevel) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsEnabledWithLevel)(::core::mem::transmute_copy(this), level, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn IsEnabledWithLevelAndKeywords(&self, level: LoggingLevel, keywords: i64) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsEnabledWithLevelAndKeywords)(::core::mem::transmute_copy(this), level, keywords, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn LogEvent<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, eventname: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).LogEvent)(::core::mem::transmute_copy(this), eventname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn LogEventWithFields<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, LoggingFields>>(&self, eventname: Param0, fields: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).LogEventWithFields)(::core::mem::transmute_copy(this), eventname.into_param().abi(), fields.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn LogEventWithFieldsAndLevel<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, LoggingFields>>(&self, eventname: Param0, fields: Param1, level: LoggingLevel) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).LogEventWithFieldsAndLevel)(::core::mem::transmute_copy(this), eventname.into_param().abi(), fields.into_param().abi(), level).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn LogEventWithFieldsAndOptions<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, LoggingFields>, Param3: ::windows::core::IntoParam<'a, LoggingOptions>>(&self, eventname: Param0, fields: Param1, level: LoggingLevel, options: Param3) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).LogEventWithFieldsAndOptions)(::core::mem::transmute_copy(this), eventname.into_param().abi(), fields.into_param().abi(), level, options.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn StartActivity<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, starteventname: Param0) -> ::windows::core::Result<LoggingActivity> {
        let this = &::windows::core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartActivity)(::core::mem::transmute_copy(this), starteventname.into_param().abi(), &mut result__).from_abi::<LoggingActivity>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn StartActivityWithFields<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, LoggingFields>>(&self, starteventname: Param0, fields: Param1) -> ::windows::core::Result<LoggingActivity> {
        let this = &::windows::core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartActivityWithFields)(::core::mem::transmute_copy(this), starteventname.into_param().abi(), fields.into_param().abi(), &mut result__).from_abi::<LoggingActivity>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn StartActivityWithFieldsAndLevel<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, LoggingFields>>(&self, starteventname: Param0, fields: Param1, level: LoggingLevel) -> ::windows::core::Result<LoggingActivity> {
        let this = &::windows::core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartActivityWithFieldsAndLevel)(::core::mem::transmute_copy(this), starteventname.into_param().abi(), fields.into_param().abi(), level, &mut result__).from_abi::<LoggingActivity>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn StartActivityWithFieldsAndOptions<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, LoggingFields>, Param3: ::windows::core::IntoParam<'a, LoggingOptions>>(&self, starteventname: Param0, fields: Param1, level: LoggingLevel, options: Param3) -> ::windows::core::Result<LoggingActivity> {
        let this = &::windows::core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartActivityWithFieldsAndOptions)(::core::mem::transmute_copy(this), starteventname.into_param().abi(), fields.into_param().abi(), level, options.into_param().abi(), &mut result__).from_abi::<LoggingActivity>(result__)
        }
    }
    #[doc(hidden)]
    pub fn ILoggingActivityFactory<R, F: FnOnce(&ILoggingActivityFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LoggingActivity, ILoggingActivityFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for LoggingActivity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for LoggingActivity {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.LoggingActivity;{bc032941-b766-4cb5-9848-97ac6ba6d60c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LoggingActivity {
    type Vtable = ILoggingActivity_Vtbl;
    const IID: ::windows::core::GUID = <ILoggingActivity as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LoggingActivity {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.LoggingActivity";
}
impl ::core::convert::From<LoggingActivity> for ::windows::core::IUnknown {
    fn from(value: LoggingActivity) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LoggingActivity> for ::windows::core::IUnknown {
    fn from(value: &LoggingActivity) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LoggingActivity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LoggingActivity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LoggingActivity> for ::windows::core::IInspectable {
    fn from(value: LoggingActivity) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LoggingActivity> for ::windows::core::IInspectable {
    fn from(value: &LoggingActivity) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LoggingActivity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LoggingActivity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<LoggingActivity> for super::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: LoggingActivity) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LoggingActivity> for super::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &LoggingActivity) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IClosable> for LoggingActivity {
    fn into_param(self) -> ::windows::core::Param<'a, super::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IClosable> for &LoggingActivity {
    fn into_param(self) -> ::windows::core::Param<'a, super::IClosable> {
        ::core::convert::TryInto::<super::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<LoggingActivity> for ILoggingTarget {
    type Error = ::windows::core::Error;
    fn try_from(value: LoggingActivity) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LoggingActivity> for ILoggingTarget {
    type Error = ::windows::core::Error;
    fn try_from(value: &LoggingActivity) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILoggingTarget> for LoggingActivity {
    fn into_param(self) -> ::windows::core::Param<'a, ILoggingTarget> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILoggingTarget> for &LoggingActivity {
    fn into_param(self) -> ::windows::core::Param<'a, ILoggingTarget> {
        ::core::convert::TryInto::<ILoggingTarget>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for LoggingActivity {}
unsafe impl ::core::marker::Sync for LoggingActivity {}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
pub struct LoggingChannel(::windows::core::IUnknown);
impl LoggingChannel {
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn Enabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Enabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn Level(&self) -> ::windows::core::Result<LoggingLevel> {
        let this = self;
        unsafe {
            let mut result__: LoggingLevel = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Level)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LoggingLevel>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn LogMessage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, eventstring: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).LogMessage)(::core::mem::transmute_copy(this), eventstring.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn LogMessageWithLevel<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, eventstring: Param0, level: LoggingLevel) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).LogMessageWithLevel)(::core::mem::transmute_copy(this), eventstring.into_param().abi(), level).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn LogValuePair<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value1: Param0, value2: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).LogValuePair)(::core::mem::transmute_copy(this), value1.into_param().abi(), value2).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn LogValuePairWithLevel<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value1: Param0, value2: i32, level: LoggingLevel) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).LogValuePairWithLevel)(::core::mem::transmute_copy(this), value1.into_param().abi(), value2, level).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn LoggingEnabled<'a, Param0: ::windows::core::IntoParam<'a, super::TypedEventHandler<ILoggingChannel, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LoggingEnabled)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn RemoveLoggingEnabled<'a, Param0: ::windows::core::IntoParam<'a, super::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveLoggingEnabled)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<ILoggingChannel2>(self)?;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(name: Param0) -> ::windows::core::Result<LoggingChannel> {
        Self::ILoggingChannelFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<LoggingChannel>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn CreateWithOptions<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, LoggingChannelOptions>>(name: Param0, options: Param1) -> ::windows::core::Result<LoggingChannel> {
        Self::ILoggingChannelFactory2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateWithOptions)(::core::mem::transmute_copy(this), name.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<LoggingChannel>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn CreateWithOptionsAndId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, LoggingChannelOptions>, Param2: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(name: Param0, options: Param1, id: Param2) -> ::windows::core::Result<LoggingChannel> {
        Self::ILoggingChannelFactory2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateWithOptionsAndId)(::core::mem::transmute_copy(this), name.into_param().abi(), options.into_param().abi(), id.into_param().abi(), &mut result__).from_abi::<LoggingChannel>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn IsEnabledWithLevel(&self, level: LoggingLevel) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsEnabledWithLevel)(::core::mem::transmute_copy(this), level, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn IsEnabledWithLevelAndKeywords(&self, level: LoggingLevel, keywords: i64) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsEnabledWithLevelAndKeywords)(::core::mem::transmute_copy(this), level, keywords, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn LogEvent<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, eventname: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).LogEvent)(::core::mem::transmute_copy(this), eventname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn LogEventWithFields<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, LoggingFields>>(&self, eventname: Param0, fields: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).LogEventWithFields)(::core::mem::transmute_copy(this), eventname.into_param().abi(), fields.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn LogEventWithFieldsAndLevel<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, LoggingFields>>(&self, eventname: Param0, fields: Param1, level: LoggingLevel) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).LogEventWithFieldsAndLevel)(::core::mem::transmute_copy(this), eventname.into_param().abi(), fields.into_param().abi(), level).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn LogEventWithFieldsAndOptions<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, LoggingFields>, Param3: ::windows::core::IntoParam<'a, LoggingOptions>>(&self, eventname: Param0, fields: Param1, level: LoggingLevel, options: Param3) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).LogEventWithFieldsAndOptions)(::core::mem::transmute_copy(this), eventname.into_param().abi(), fields.into_param().abi(), level, options.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn StartActivity<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, starteventname: Param0) -> ::windows::core::Result<LoggingActivity> {
        let this = &::windows::core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartActivity)(::core::mem::transmute_copy(this), starteventname.into_param().abi(), &mut result__).from_abi::<LoggingActivity>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn StartActivityWithFields<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, LoggingFields>>(&self, starteventname: Param0, fields: Param1) -> ::windows::core::Result<LoggingActivity> {
        let this = &::windows::core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartActivityWithFields)(::core::mem::transmute_copy(this), starteventname.into_param().abi(), fields.into_param().abi(), &mut result__).from_abi::<LoggingActivity>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn StartActivityWithFieldsAndLevel<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, LoggingFields>>(&self, starteventname: Param0, fields: Param1, level: LoggingLevel) -> ::windows::core::Result<LoggingActivity> {
        let this = &::windows::core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartActivityWithFieldsAndLevel)(::core::mem::transmute_copy(this), starteventname.into_param().abi(), fields.into_param().abi(), level, &mut result__).from_abi::<LoggingActivity>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn StartActivityWithFieldsAndOptions<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, LoggingFields>, Param3: ::windows::core::IntoParam<'a, LoggingOptions>>(&self, starteventname: Param0, fields: Param1, level: LoggingLevel, options: Param3) -> ::windows::core::Result<LoggingActivity> {
        let this = &::windows::core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartActivityWithFieldsAndOptions)(::core::mem::transmute_copy(this), starteventname.into_param().abi(), fields.into_param().abi(), level, options.into_param().abi(), &mut result__).from_abi::<LoggingActivity>(result__)
        }
    }
    #[doc(hidden)]
    pub fn ILoggingChannelFactory<R, F: FnOnce(&ILoggingChannelFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LoggingChannel, ILoggingChannelFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn ILoggingChannelFactory2<R, F: FnOnce(&ILoggingChannelFactory2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LoggingChannel, ILoggingChannelFactory2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for LoggingChannel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for LoggingChannel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.LoggingChannel;{e9a50343-11d7-4f01-b5ca-cf495278c0a8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LoggingChannel {
    type Vtable = ILoggingChannel_Vtbl;
    const IID: ::windows::core::GUID = <ILoggingChannel as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LoggingChannel {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.LoggingChannel";
}
impl ::core::convert::From<LoggingChannel> for ::windows::core::IUnknown {
    fn from(value: LoggingChannel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LoggingChannel> for ::windows::core::IUnknown {
    fn from(value: &LoggingChannel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LoggingChannel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LoggingChannel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LoggingChannel> for ::windows::core::IInspectable {
    fn from(value: LoggingChannel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LoggingChannel> for ::windows::core::IInspectable {
    fn from(value: &LoggingChannel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LoggingChannel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LoggingChannel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<LoggingChannel> for super::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: LoggingChannel) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LoggingChannel> for super::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &LoggingChannel) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IClosable> for LoggingChannel {
    fn into_param(self) -> ::windows::core::Param<'a, super::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IClosable> for &LoggingChannel {
    fn into_param(self) -> ::windows::core::Param<'a, super::IClosable> {
        ::core::convert::TryInto::<super::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<LoggingChannel> for ILoggingChannel {
    type Error = ::windows::core::Error;
    fn try_from(value: LoggingChannel) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LoggingChannel> for ILoggingChannel {
    type Error = ::windows::core::Error;
    fn try_from(value: &LoggingChannel) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILoggingChannel> for LoggingChannel {
    fn into_param(self) -> ::windows::core::Param<'a, ILoggingChannel> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILoggingChannel> for &LoggingChannel {
    fn into_param(self) -> ::windows::core::Param<'a, ILoggingChannel> {
        ::core::convert::TryInto::<ILoggingChannel>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<LoggingChannel> for ILoggingTarget {
    type Error = ::windows::core::Error;
    fn try_from(value: LoggingChannel) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LoggingChannel> for ILoggingTarget {
    type Error = ::windows::core::Error;
    fn try_from(value: &LoggingChannel) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILoggingTarget> for LoggingChannel {
    fn into_param(self) -> ::windows::core::Param<'a, ILoggingTarget> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILoggingTarget> for &LoggingChannel {
    fn into_param(self) -> ::windows::core::Param<'a, ILoggingTarget> {
        ::core::convert::TryInto::<ILoggingTarget>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for LoggingChannel {}
unsafe impl ::core::marker::Sync for LoggingChannel {}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
pub struct LoggingChannelOptions(::windows::core::IUnknown);
impl LoggingChannelOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LoggingChannelOptions, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn Group(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Group)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn SetGroup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetGroup)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(group: Param0) -> ::windows::core::Result<LoggingChannelOptions> {
        Self::ILoggingChannelOptionsFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::core::mem::transmute_copy(this), group.into_param().abi(), &mut result__).from_abi::<LoggingChannelOptions>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILoggingChannelOptionsFactory<R, F: FnOnce(&ILoggingChannelOptionsFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LoggingChannelOptions, ILoggingChannelOptionsFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for LoggingChannelOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for LoggingChannelOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.LoggingChannelOptions;{c3e847ff-0ebb-4a53-8c54-dec24926cb2c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LoggingChannelOptions {
    type Vtable = ILoggingChannelOptions_Vtbl;
    const IID: ::windows::core::GUID = <ILoggingChannelOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LoggingChannelOptions {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.LoggingChannelOptions";
}
impl ::core::convert::From<LoggingChannelOptions> for ::windows::core::IUnknown {
    fn from(value: LoggingChannelOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LoggingChannelOptions> for ::windows::core::IUnknown {
    fn from(value: &LoggingChannelOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LoggingChannelOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LoggingChannelOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LoggingChannelOptions> for ::windows::core::IInspectable {
    fn from(value: LoggingChannelOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LoggingChannelOptions> for ::windows::core::IInspectable {
    fn from(value: &LoggingChannelOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LoggingChannelOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LoggingChannelOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LoggingChannelOptions {}
unsafe impl ::core::marker::Sync for LoggingChannelOptions {}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for LoggingFieldFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for LoggingFieldFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LoggingFieldFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LoggingFieldFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Foundation.Diagnostics.LoggingFieldFormat;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
pub struct LoggingFields(::windows::core::IUnknown);
impl LoggingFields {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LoggingFields, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Clear)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn BeginStruct<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).BeginStruct)(::core::mem::transmute_copy(this), name.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn BeginStructWithTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).BeginStructWithTags)(::core::mem::transmute_copy(this), name.into_param().abi(), tags).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn EndStruct(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).EndStruct)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddEmpty<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddEmpty)(::core::mem::transmute_copy(this), name.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddEmptyWithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddEmptyWithFormat)(::core::mem::transmute_copy(this), name.into_param().abi(), format).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddEmptyWithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddEmptyWithFormatAndTags)(::core::mem::transmute_copy(this), name.into_param().abi(), format, tags).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddUInt8<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddUInt8)(::core::mem::transmute_copy(this), name.into_param().abi(), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddUInt8WithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: u8, format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddUInt8WithFormat)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddUInt8WithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: u8, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddUInt8WithFormatAndTags)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format, tags).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddUInt8Array<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[u8]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddUInt8Array)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddUInt8ArrayWithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[u8], format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddUInt8ArrayWithFormat)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddUInt8ArrayWithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[u8], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddUInt8ArrayWithFormatAndTags)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddInt16<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: i16) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddInt16)(::core::mem::transmute_copy(this), name.into_param().abi(), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddInt16WithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: i16, format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddInt16WithFormat)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddInt16WithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: i16, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddInt16WithFormatAndTags)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format, tags).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddInt16Array<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[i16]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddInt16Array)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddInt16ArrayWithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[i16], format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddInt16ArrayWithFormat)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddInt16ArrayWithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[i16], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddInt16ArrayWithFormatAndTags)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddUInt16<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: u16) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddUInt16)(::core::mem::transmute_copy(this), name.into_param().abi(), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddUInt16WithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: u16, format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddUInt16WithFormat)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddUInt16WithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: u16, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddUInt16WithFormatAndTags)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format, tags).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddUInt16Array<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[u16]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddUInt16Array)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddUInt16ArrayWithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[u16], format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddUInt16ArrayWithFormat)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddUInt16ArrayWithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[u16], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddUInt16ArrayWithFormatAndTags)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddInt32<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddInt32)(::core::mem::transmute_copy(this), name.into_param().abi(), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddInt32WithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: i32, format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddInt32WithFormat)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddInt32WithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: i32, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddInt32WithFormatAndTags)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format, tags).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddInt32Array<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[i32]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddInt32Array)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddInt32ArrayWithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[i32], format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddInt32ArrayWithFormat)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddInt32ArrayWithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[i32], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddInt32ArrayWithFormatAndTags)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddUInt32<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddUInt32)(::core::mem::transmute_copy(this), name.into_param().abi(), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddUInt32WithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: u32, format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddUInt32WithFormat)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddUInt32WithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: u32, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddUInt32WithFormatAndTags)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format, tags).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddUInt32Array<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[u32]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddUInt32Array)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddUInt32ArrayWithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[u32], format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddUInt32ArrayWithFormat)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddUInt32ArrayWithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[u32], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddUInt32ArrayWithFormatAndTags)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddInt64<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: i64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddInt64)(::core::mem::transmute_copy(this), name.into_param().abi(), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddInt64WithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: i64, format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddInt64WithFormat)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddInt64WithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: i64, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddInt64WithFormatAndTags)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format, tags).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddInt64Array<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[i64]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddInt64Array)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddInt64ArrayWithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[i64], format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddInt64ArrayWithFormat)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddInt64ArrayWithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[i64], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddInt64ArrayWithFormatAndTags)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddUInt64<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: u64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddUInt64)(::core::mem::transmute_copy(this), name.into_param().abi(), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddUInt64WithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: u64, format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddUInt64WithFormat)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddUInt64WithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: u64, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddUInt64WithFormatAndTags)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format, tags).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddUInt64Array<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[u64]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddUInt64Array)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddUInt64ArrayWithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[u64], format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddUInt64ArrayWithFormat)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddUInt64ArrayWithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[u64], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddUInt64ArrayWithFormatAndTags)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddSingle<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddSingle)(::core::mem::transmute_copy(this), name.into_param().abi(), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddSingleWithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: f32, format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddSingleWithFormat)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddSingleWithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: f32, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddSingleWithFormatAndTags)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format, tags).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddSingleArray<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[f32]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddSingleArray)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddSingleArrayWithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[f32], format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddSingleArrayWithFormat)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddSingleArrayWithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[f32], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddSingleArrayWithFormatAndTags)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddDouble<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddDouble)(::core::mem::transmute_copy(this), name.into_param().abi(), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddDoubleWithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: f64, format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddDoubleWithFormat)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddDoubleWithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: f64, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddDoubleWithFormatAndTags)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format, tags).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddDoubleArray<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[f64]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddDoubleArray)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddDoubleArrayWithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[f64], format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddDoubleArrayWithFormat)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddDoubleArrayWithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[f64], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddDoubleArrayWithFormatAndTags)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddChar16<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: u16) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddChar16)(::core::mem::transmute_copy(this), name.into_param().abi(), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddChar16WithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: u16, format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddChar16WithFormat)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddChar16WithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: u16, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddChar16WithFormatAndTags)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format, tags).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddChar16Array<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[u16]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddChar16Array)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddChar16ArrayWithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[u16], format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddChar16ArrayWithFormat)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddChar16ArrayWithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[u16], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddChar16ArrayWithFormatAndTags)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddBoolean<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddBoolean)(::core::mem::transmute_copy(this), name.into_param().abi(), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddBooleanWithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: bool, format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddBooleanWithFormat)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddBooleanWithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: bool, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddBooleanWithFormatAndTags)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format, tags).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddBooleanArray<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[bool]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddBooleanArray)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddBooleanArrayWithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[bool], format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddBooleanArrayWithFormat)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddBooleanArrayWithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[bool], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddBooleanArrayWithFormatAndTags)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddString<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddString)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddStringWithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: Param1, format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddStringWithFormat)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), format).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddStringWithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: Param1, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddStringWithFormatAndTags)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), format, tags).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddStringArray<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[::windows::core::HSTRING]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddStringArray)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddStringArrayWithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[::windows::core::HSTRING], format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddStringArrayWithFormat)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddStringArrayWithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[::windows::core::HSTRING], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddStringArrayWithFormatAndTags)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddGuid<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, name: Param0, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddGuid)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddGuidWithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, name: Param0, value: Param1, format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddGuidWithFormat)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), format).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddGuidWithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, name: Param0, value: Param1, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddGuidWithFormatAndTags)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), format, tags).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddGuidArray<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[::windows::core::GUID]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddGuidArray)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddGuidArrayWithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[::windows::core::GUID], format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddGuidArrayWithFormat)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddGuidArrayWithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[::windows::core::GUID], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddGuidArrayWithFormatAndTags)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddDateTime<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::DateTime>>(&self, name: Param0, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddDateTime)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddDateTimeWithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::DateTime>>(&self, name: Param0, value: Param1, format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddDateTimeWithFormat)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), format).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddDateTimeWithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::DateTime>>(&self, name: Param0, value: Param1, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddDateTimeWithFormatAndTags)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), format, tags).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddDateTimeArray<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[super::DateTime]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddDateTimeArray)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddDateTimeArrayWithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[super::DateTime], format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddDateTimeArrayWithFormat)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddDateTimeArrayWithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[super::DateTime], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddDateTimeArrayWithFormatAndTags)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddTimeSpan<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::TimeSpan>>(&self, name: Param0, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddTimeSpan)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddTimeSpanWithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::TimeSpan>>(&self, name: Param0, value: Param1, format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddTimeSpanWithFormat)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), format).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddTimeSpanWithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::TimeSpan>>(&self, name: Param0, value: Param1, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddTimeSpanWithFormatAndTags)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), format, tags).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddTimeSpanArray<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[super::TimeSpan]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddTimeSpanArray)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddTimeSpanArrayWithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[super::TimeSpan], format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddTimeSpanArrayWithFormat)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddTimeSpanArrayWithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[super::TimeSpan], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddTimeSpanArrayWithFormatAndTags)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddPoint<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::Point>>(&self, name: Param0, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddPoint)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddPointWithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::Point>>(&self, name: Param0, value: Param1, format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddPointWithFormat)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), format).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddPointWithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::Point>>(&self, name: Param0, value: Param1, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddPointWithFormatAndTags)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), format, tags).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddPointArray<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[super::Point]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddPointArray)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddPointArrayWithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[super::Point], format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddPointArrayWithFormat)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddPointArrayWithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[super::Point], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddPointArrayWithFormatAndTags)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddSize<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::Size>>(&self, name: Param0, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddSize)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddSizeWithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::Size>>(&self, name: Param0, value: Param1, format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddSizeWithFormat)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), format).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddSizeWithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::Size>>(&self, name: Param0, value: Param1, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddSizeWithFormatAndTags)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), format, tags).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddSizeArray<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[super::Size]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddSizeArray)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddSizeArrayWithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[super::Size], format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddSizeArrayWithFormat)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddSizeArrayWithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[super::Size], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddSizeArrayWithFormatAndTags)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddRect<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::Rect>>(&self, name: Param0, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddRect)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddRectWithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::Rect>>(&self, name: Param0, value: Param1, format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddRectWithFormat)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), format).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddRectWithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::Rect>>(&self, name: Param0, value: Param1, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddRectWithFormatAndTags)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), format, tags).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddRectArray<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[super::Rect]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddRectArray)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddRectArrayWithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[super::Rect], format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddRectArrayWithFormat)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddRectArrayWithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[super::Rect], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddRectArrayWithFormatAndTags)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
}
impl ::core::clone::Clone for LoggingFields {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for LoggingFields {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.LoggingFields;{d7f6b7af-762d-4579-83bd-52c23bc333bc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LoggingFields {
    type Vtable = ILoggingFields_Vtbl;
    const IID: ::windows::core::GUID = <ILoggingFields as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LoggingFields {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.LoggingFields";
}
impl ::core::convert::From<LoggingFields> for ::windows::core::IUnknown {
    fn from(value: LoggingFields) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LoggingFields> for ::windows::core::IUnknown {
    fn from(value: &LoggingFields) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LoggingFields {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LoggingFields {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LoggingFields> for ::windows::core::IInspectable {
    fn from(value: LoggingFields) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LoggingFields> for ::windows::core::IInspectable {
    fn from(value: &LoggingFields) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LoggingFields {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LoggingFields {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LoggingFields {}
unsafe impl ::core::marker::Sync for LoggingFields {}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for LoggingLevel {
    type Abi = Self;
}
impl ::core::fmt::Debug for LoggingLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LoggingLevel").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LoggingLevel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Foundation.Diagnostics.LoggingLevel;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for LoggingOpcode {
    type Abi = Self;
}
impl ::core::fmt::Debug for LoggingOpcode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LoggingOpcode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LoggingOpcode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Foundation.Diagnostics.LoggingOpcode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
pub struct LoggingOptions(::windows::core::IUnknown);
impl LoggingOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LoggingOptions, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn Keywords(&self) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Keywords)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn SetKeywords(&self, value: i64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetKeywords)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn Tags(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Tags)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn SetTags(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTags)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn Task(&self) -> ::windows::core::Result<i16> {
        let this = self;
        unsafe {
            let mut result__: i16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Task)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i16>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn SetTask(&self, value: i16) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTask)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn Opcode(&self) -> ::windows::core::Result<LoggingOpcode> {
        let this = self;
        unsafe {
            let mut result__: LoggingOpcode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Opcode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LoggingOpcode>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn SetOpcode(&self, value: LoggingOpcode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetOpcode)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn ActivityId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ActivityId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn SetActivityId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetActivityId)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn RelatedActivityId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RelatedActivityId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn SetRelatedActivityId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRelatedActivityId)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn CreateWithKeywords(keywords: i64) -> ::windows::core::Result<LoggingOptions> {
        Self::ILoggingOptionsFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateWithKeywords)(::core::mem::transmute_copy(this), keywords, &mut result__).from_abi::<LoggingOptions>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILoggingOptionsFactory<R, F: FnOnce(&ILoggingOptionsFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LoggingOptions, ILoggingOptionsFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for LoggingOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for LoggingOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.LoggingOptions;{90bc7850-0192-4f5d-ac26-006adaca12d8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LoggingOptions {
    type Vtable = ILoggingOptions_Vtbl;
    const IID: ::windows::core::GUID = <ILoggingOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LoggingOptions {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.LoggingOptions";
}
impl ::core::convert::From<LoggingOptions> for ::windows::core::IUnknown {
    fn from(value: LoggingOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LoggingOptions> for ::windows::core::IUnknown {
    fn from(value: &LoggingOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LoggingOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LoggingOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LoggingOptions> for ::windows::core::IInspectable {
    fn from(value: LoggingOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LoggingOptions> for ::windows::core::IInspectable {
    fn from(value: &LoggingOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LoggingOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LoggingOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LoggingOptions {}
unsafe impl ::core::marker::Sync for LoggingOptions {}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
pub struct LoggingSession(::windows::core::IUnknown);
impl LoggingSession {
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`, `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn SaveToFileAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::IStorageFolder>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, folder: Param0, filename: Param1) -> ::windows::core::Result<super::IAsyncOperation<super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SaveToFileAsync)(::core::mem::transmute_copy(this), folder.into_param().abi(), filename.into_param().abi(), &mut result__).from_abi::<super::IAsyncOperation<super::super::Storage::StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddLoggingChannel<'a, Param0: ::windows::core::IntoParam<'a, ILoggingChannel>>(&self, loggingchannel: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddLoggingChannel)(::core::mem::transmute_copy(this), loggingchannel.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn AddLoggingChannelWithLevel<'a, Param0: ::windows::core::IntoParam<'a, ILoggingChannel>>(&self, loggingchannel: Param0, maxlevel: LoggingLevel) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddLoggingChannelWithLevel)(::core::mem::transmute_copy(this), loggingchannel.into_param().abi(), maxlevel).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn RemoveLoggingChannel<'a, Param0: ::windows::core::IntoParam<'a, ILoggingChannel>>(&self, loggingchannel: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveLoggingChannel)(::core::mem::transmute_copy(this), loggingchannel.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(name: Param0) -> ::windows::core::Result<LoggingSession> {
        Self::ILoggingSessionFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<LoggingSession>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILoggingSessionFactory<R, F: FnOnce(&ILoggingSessionFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LoggingSession, ILoggingSessionFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for LoggingSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for LoggingSession {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.LoggingSession;{6221f306-9380-4ad7-baf5-41ea9310d768})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LoggingSession {
    type Vtable = ILoggingSession_Vtbl;
    const IID: ::windows::core::GUID = <ILoggingSession as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LoggingSession {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.LoggingSession";
}
impl ::core::convert::From<LoggingSession> for ::windows::core::IUnknown {
    fn from(value: LoggingSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LoggingSession> for ::windows::core::IUnknown {
    fn from(value: &LoggingSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LoggingSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LoggingSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LoggingSession> for ::windows::core::IInspectable {
    fn from(value: LoggingSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LoggingSession> for ::windows::core::IInspectable {
    fn from(value: &LoggingSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LoggingSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LoggingSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<LoggingSession> for super::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: LoggingSession) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LoggingSession> for super::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &LoggingSession) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IClosable> for LoggingSession {
    fn into_param(self) -> ::windows::core::Param<'a, super::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IClosable> for &LoggingSession {
    fn into_param(self) -> ::windows::core::Param<'a, super::IClosable> {
        ::core::convert::TryInto::<super::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<LoggingSession> for ILoggingSession {
    type Error = ::windows::core::Error;
    fn try_from(value: LoggingSession) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LoggingSession> for ILoggingSession {
    type Error = ::windows::core::Error;
    fn try_from(value: &LoggingSession) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILoggingSession> for LoggingSession {
    fn into_param(self) -> ::windows::core::Param<'a, ILoggingSession> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILoggingSession> for &LoggingSession {
    fn into_param(self) -> ::windows::core::Param<'a, ILoggingSession> {
        ::core::convert::TryInto::<ILoggingSession>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for LoggingSession {}
unsafe impl ::core::marker::Sync for LoggingSession {}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
pub struct RuntimeBrokerErrorSettings(::windows::core::IUnknown);
impl RuntimeBrokerErrorSettings {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RuntimeBrokerErrorSettings, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn SetErrorOptions(&self, value: ErrorOptions) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetErrorOptions)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn GetErrorOptions(&self) -> ::windows::core::Result<ErrorOptions> {
        let this = self;
        unsafe {
            let mut result__: ErrorOptions = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetErrorOptions)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ErrorOptions>(result__)
        }
    }
}
impl ::core::clone::Clone for RuntimeBrokerErrorSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for RuntimeBrokerErrorSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.RuntimeBrokerErrorSettings;{16369792-b03e-4ba1-8bb8-d28f4ab4d2c0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for RuntimeBrokerErrorSettings {
    type Vtable = IErrorReportingSettings_Vtbl;
    const IID: ::windows::core::GUID = <IErrorReportingSettings as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RuntimeBrokerErrorSettings {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.RuntimeBrokerErrorSettings";
}
impl ::core::convert::From<RuntimeBrokerErrorSettings> for ::windows::core::IUnknown {
    fn from(value: RuntimeBrokerErrorSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RuntimeBrokerErrorSettings> for ::windows::core::IUnknown {
    fn from(value: &RuntimeBrokerErrorSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RuntimeBrokerErrorSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RuntimeBrokerErrorSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RuntimeBrokerErrorSettings> for ::windows::core::IInspectable {
    fn from(value: RuntimeBrokerErrorSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RuntimeBrokerErrorSettings> for ::windows::core::IInspectable {
    fn from(value: &RuntimeBrokerErrorSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RuntimeBrokerErrorSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RuntimeBrokerErrorSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<RuntimeBrokerErrorSettings> for IErrorReportingSettings {
    type Error = ::windows::core::Error;
    fn try_from(value: RuntimeBrokerErrorSettings) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&RuntimeBrokerErrorSettings> for IErrorReportingSettings {
    type Error = ::windows::core::Error;
    fn try_from(value: &RuntimeBrokerErrorSettings) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IErrorReportingSettings> for RuntimeBrokerErrorSettings {
    fn into_param(self) -> ::windows::core::Param<'a, IErrorReportingSettings> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IErrorReportingSettings> for &RuntimeBrokerErrorSettings {
    fn into_param(self) -> ::windows::core::Param<'a, IErrorReportingSettings> {
        ::core::convert::TryInto::<IErrorReportingSettings>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for RuntimeBrokerErrorSettings {}
unsafe impl ::core::marker::Sync for RuntimeBrokerErrorSettings {}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
pub struct TracingStatusChangedEventArgs(::windows::core::IUnknown);
impl TracingStatusChangedEventArgs {
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn Enabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Enabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
    pub fn TraceLevel(&self) -> ::windows::core::Result<CausalityTraceLevel> {
        let this = self;
        unsafe {
            let mut result__: CausalityTraceLevel = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TraceLevel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CausalityTraceLevel>(result__)
        }
    }
}
impl ::core::clone::Clone for TracingStatusChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for TracingStatusChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.TracingStatusChangedEventArgs;{410b7711-ff3b-477f-9c9a-d2efda302dc3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for TracingStatusChangedEventArgs {
    type Vtable = ITracingStatusChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ITracingStatusChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for TracingStatusChangedEventArgs {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.TracingStatusChangedEventArgs";
}
impl ::core::convert::From<TracingStatusChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: TracingStatusChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TracingStatusChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &TracingStatusChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TracingStatusChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TracingStatusChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TracingStatusChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: TracingStatusChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TracingStatusChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &TracingStatusChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TracingStatusChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TracingStatusChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TracingStatusChangedEventArgs {}
unsafe impl ::core::marker::Sync for TracingStatusChangedEventArgs {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
