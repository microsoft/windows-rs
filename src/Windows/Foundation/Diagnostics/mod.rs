#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Foundation_Diagnostics`*"]
pub struct AsyncCausalityTracer {}
impl AsyncCausalityTracer {
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn TraceOperationCreation<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param4: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: Param2, operationid: u64, operationname: Param4, relatedcontext: u64) -> ::windows::core::Result<()> {
        Self::IAsyncCausalityTracerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), tracelevel, source, platformid.into_param().abi(), operationid, operationname.into_param().abi(), relatedcontext).ok() })
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn TraceOperationCompletion<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: Param2, operationid: u64, status: super::AsyncStatus) -> ::windows::core::Result<()> {
        Self::IAsyncCausalityTracerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), tracelevel, source, platformid.into_param().abi(), operationid, status).ok() })
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn TraceOperationRelation<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: Param2, operationid: u64, relation: CausalityRelation) -> ::windows::core::Result<()> {
        Self::IAsyncCausalityTracerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), tracelevel, source, platformid.into_param().abi(), operationid, relation).ok() })
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn TraceSynchronousWorkStart<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: Param2, operationid: u64, work: CausalitySynchronousWork) -> ::windows::core::Result<()> {
        Self::IAsyncCausalityTracerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), tracelevel, source, platformid.into_param().abi(), operationid, work).ok() })
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn TraceSynchronousWorkCompletion(tracelevel: CausalityTraceLevel, source: CausalitySource, work: CausalitySynchronousWork) -> ::windows::core::Result<()> {
        Self::IAsyncCausalityTracerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), tracelevel, source, work).ok() })
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn TracingStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::EventHandler<TracingStatusChangedEventArgs>>>(handler: Param0) -> ::windows::core::Result<super::EventRegistrationToken> {
        Self::IAsyncCausalityTracerStatics(|this| unsafe {
            let mut result__: super::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn RemoveTracingStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::EventRegistrationToken>>(cookie: Param0) -> ::windows::core::Result<()> {
        Self::IAsyncCausalityTracerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() })
    }
    pub fn IAsyncCausalityTracerStatics<R, F: FnOnce(&IAsyncCausalityTracerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AsyncCausalityTracer, IAsyncCausalityTracerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for AsyncCausalityTracer {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.AsyncCausalityTracer";
}
#[doc = "*Required features: `Foundation_Diagnostics`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CausalityRelation(pub i32);
impl CausalityRelation {
    pub const AssignDelegate: CausalityRelation = CausalityRelation(0i32);
    pub const Join: CausalityRelation = CausalityRelation(1i32);
    pub const Choice: CausalityRelation = CausalityRelation(2i32);
    pub const Cancel: CausalityRelation = CausalityRelation(3i32);
    pub const Error: CausalityRelation = CausalityRelation(4i32);
}
impl ::core::convert::From<i32> for CausalityRelation {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CausalityRelation {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CausalityRelation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Foundation.Diagnostics.CausalityRelation;i4)");
}
impl ::windows::core::DefaultType for CausalityRelation {
    type DefaultType = Self;
}
#[doc = "*Required features: `Foundation_Diagnostics`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CausalitySource(pub i32);
impl CausalitySource {
    pub const Application: CausalitySource = CausalitySource(0i32);
    pub const Library: CausalitySource = CausalitySource(1i32);
    pub const System: CausalitySource = CausalitySource(2i32);
}
impl ::core::convert::From<i32> for CausalitySource {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CausalitySource {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CausalitySource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Foundation.Diagnostics.CausalitySource;i4)");
}
impl ::windows::core::DefaultType for CausalitySource {
    type DefaultType = Self;
}
#[doc = "*Required features: `Foundation_Diagnostics`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CausalitySynchronousWork(pub i32);
impl CausalitySynchronousWork {
    pub const CompletionNotification: CausalitySynchronousWork = CausalitySynchronousWork(0i32);
    pub const ProgressNotification: CausalitySynchronousWork = CausalitySynchronousWork(1i32);
    pub const Execution: CausalitySynchronousWork = CausalitySynchronousWork(2i32);
}
impl ::core::convert::From<i32> for CausalitySynchronousWork {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CausalitySynchronousWork {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CausalitySynchronousWork {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Foundation.Diagnostics.CausalitySynchronousWork;i4)");
}
impl ::windows::core::DefaultType for CausalitySynchronousWork {
    type DefaultType = Self;
}
#[doc = "*Required features: `Foundation_Diagnostics`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CausalityTraceLevel(pub i32);
impl CausalityTraceLevel {
    pub const Required: CausalityTraceLevel = CausalityTraceLevel(0i32);
    pub const Important: CausalityTraceLevel = CausalityTraceLevel(1i32);
    pub const Verbose: CausalityTraceLevel = CausalityTraceLevel(2i32);
}
impl ::core::convert::From<i32> for CausalityTraceLevel {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CausalityTraceLevel {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CausalityTraceLevel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Foundation.Diagnostics.CausalityTraceLevel;i4)");
}
impl ::windows::core::DefaultType for CausalityTraceLevel {
    type DefaultType = Self;
}
#[doc = "*Required features: `Foundation_Diagnostics`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ErrorDetails(pub ::windows::core::IInspectable);
impl ErrorDetails {
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LongDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn HelpUri(&self) -> ::windows::core::Result<super::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Uri>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn CreateFromHResultAsync(errorcode: i32) -> ::windows::core::Result<super::IAsyncOperation<ErrorDetails>> {
        Self::IErrorDetailsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), errorcode, &mut result__).from_abi::<super::IAsyncOperation<ErrorDetails>>(result__)
        })
    }
    pub fn IErrorDetailsStatics<R, F: FnOnce(&IErrorDetailsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ErrorDetails, IErrorDetailsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for ErrorDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.ErrorDetails;{378cbb01-2cc9-428f-8c55-2c990d463e8f})");
}
unsafe impl ::windows::core::Interface for ErrorDetails {
    type Vtable = IErrorDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x378cbb01_2cc9_428f_8c55_2c990d463e8f);
}
impl ::windows::core::RuntimeName for ErrorDetails {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.ErrorDetails";
}
impl ::core::convert::From<ErrorDetails> for ::windows::core::IUnknown {
    fn from(value: ErrorDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ErrorDetails> for ::windows::core::IUnknown {
    fn from(value: &ErrorDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ErrorDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ErrorDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ErrorDetails> for ::windows::core::IInspectable {
    fn from(value: ErrorDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ErrorDetails> for ::windows::core::IInspectable {
    fn from(value: &ErrorDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ErrorDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ErrorDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ErrorDetails {}
unsafe impl ::core::marker::Sync for ErrorDetails {}
#[doc = "*Required features: `Foundation_Diagnostics`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ErrorOptions(pub u32);
impl ErrorOptions {
    pub const None: ErrorOptions = ErrorOptions(0u32);
    pub const SuppressExceptions: ErrorOptions = ErrorOptions(1u32);
    pub const ForceExceptions: ErrorOptions = ErrorOptions(2u32);
    pub const UseSetErrorInfo: ErrorOptions = ErrorOptions(4u32);
    pub const SuppressSetErrorInfo: ErrorOptions = ErrorOptions(8u32);
}
impl ::core::convert::From<u32> for ErrorOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ErrorOptions {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ErrorOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Foundation.Diagnostics.ErrorOptions;u4)");
}
impl ::windows::core::DefaultType for ErrorOptions {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for ErrorOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for ErrorOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for ErrorOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for ErrorOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for ErrorOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Foundation_Diagnostics`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct FileLoggingSession(pub ::windows::core::IInspectable);
impl FileLoggingSession {
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddLoggingChannel<'a, Param0: ::windows::core::IntoParam<'a, ILoggingChannel>>(&self, loggingchannel: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), loggingchannel.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddLoggingChannelWithLevel<'a, Param0: ::windows::core::IntoParam<'a, ILoggingChannel>>(&self, loggingchannel: Param0, maxlevel: LoggingLevel) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), loggingchannel.into_param().abi(), maxlevel).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn RemoveLoggingChannel<'a, Param0: ::windows::core::IntoParam<'a, ILoggingChannel>>(&self, loggingchannel: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), loggingchannel.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage")]
    #[doc = "*Required features: `Foundation_Diagnostics`, `Storage`*"]
    pub fn CloseAndSaveToFileAsync(&self) -> ::windows::core::Result<super::IAsyncOperation<super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::IAsyncOperation<super::super::Storage::StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogFileGenerated<'a, Param0: ::windows::core::IntoParam<'a, super::TypedEventHandler<IFileLoggingSession, LogFileGeneratedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn RemoveLogFileGenerated<'a, Param0: ::windows::core::IntoParam<'a, super::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(name: Param0) -> ::windows::core::Result<FileLoggingSession> {
        Self::IFileLoggingSessionFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<FileLoggingSession>(result__)
        })
    }
    pub fn IFileLoggingSessionFactory<R, F: FnOnce(&IFileLoggingSessionFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FileLoggingSession, IFileLoggingSessionFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for FileLoggingSession {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.FileLoggingSession;{24c74216-fed2-404c-895f-1f9699cb02f7})");
}
unsafe impl ::windows::core::Interface for FileLoggingSession {
    type Vtable = IFileLoggingSession_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24c74216_fed2_404c_895f_1f9699cb02f7);
}
impl ::windows::core::RuntimeName for FileLoggingSession {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.FileLoggingSession";
}
impl ::core::convert::From<FileLoggingSession> for ::windows::core::IUnknown {
    fn from(value: FileLoggingSession) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&FileLoggingSession> for ::windows::core::IUnknown {
    fn from(value: &FileLoggingSession) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FileLoggingSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FileLoggingSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<FileLoggingSession> for ::windows::core::IInspectable {
    fn from(value: FileLoggingSession) -> Self {
        value.0
    }
}
impl ::core::convert::From<&FileLoggingSession> for ::windows::core::IInspectable {
    fn from(value: &FileLoggingSession) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FileLoggingSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FileLoggingSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<FileLoggingSession> for IFileLoggingSession {
    fn from(value: FileLoggingSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileLoggingSession> for IFileLoggingSession {
    fn from(value: &FileLoggingSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFileLoggingSession> for FileLoggingSession {
    fn into_param(self) -> ::windows::core::Param<'a, IFileLoggingSession> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFileLoggingSession> for &FileLoggingSession {
    fn into_param(self) -> ::windows::core::Param<'a, IFileLoggingSession> {
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
unsafe impl ::core::marker::Send for FileLoggingSession {}
unsafe impl ::core::marker::Sync for FileLoggingSession {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IAsyncCausalityTracerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAsyncCausalityTracerStatics {
    type Vtable = IAsyncCausalityTracerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50850b26_267e_451b_a890_ab6a370245ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsyncCausalityTracerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: ::windows::core::GUID, operationid: u64, operationname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, relatedcontext: u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: ::windows::core::GUID, operationid: u64, status: super::AsyncStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: ::windows::core::GUID, operationid: u64, relation: CausalityRelation) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: ::windows::core::GUID, operationid: u64, work: CausalitySynchronousWork) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, tracelevel: CausalityTraceLevel, source: CausalitySource, work: CausalitySynchronousWork) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::EventRegistrationToken) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::EventRegistrationToken) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IErrorDetails(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IErrorDetails {
    type Vtable = IErrorDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x378cbb01_2cc9_428f_8c55_2c990d463e8f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IErrorDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IErrorDetailsStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IErrorDetailsStatics {
    type Vtable = IErrorDetailsStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb7703750_0b1d_46c8_aa0e_4b8178e4fce9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IErrorDetailsStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, errorcode: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Foundation_Diagnostics`*"]
pub struct IErrorReportingSettings(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IErrorReportingSettings {
    type Vtable = IErrorReportingSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x16369792_b03e_4ba1_8bb8_d28f4ab4d2c0);
}
impl IErrorReportingSettings {
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn SetErrorOptions(&self, value: ErrorOptions) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn GetErrorOptions(&self) -> ::windows::core::Result<ErrorOptions> {
        let this = self;
        unsafe {
            let mut result__: ErrorOptions = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ErrorOptions>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IErrorReportingSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{16369792-b03e-4ba1-8bb8-d28f4ab4d2c0}");
}
impl ::core::convert::From<IErrorReportingSettings> for ::windows::core::IUnknown {
    fn from(value: IErrorReportingSettings) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IErrorReportingSettings> for ::windows::core::IUnknown {
    fn from(value: &IErrorReportingSettings) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IErrorReportingSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IErrorReportingSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IErrorReportingSettings> for ::windows::core::IInspectable {
    fn from(value: IErrorReportingSettings) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IErrorReportingSettings> for ::windows::core::IInspectable {
    fn from(value: &IErrorReportingSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IErrorReportingSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IErrorReportingSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IErrorReportingSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ErrorOptions) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ErrorOptions) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Foundation_Diagnostics`*"]
pub struct IFileLoggingSession(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFileLoggingSession {
    type Vtable = IFileLoggingSession_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24c74216_fed2_404c_895f_1f9699cb02f7);
}
impl IFileLoggingSession {
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddLoggingChannel<'a, Param0: ::windows::core::IntoParam<'a, ILoggingChannel>>(&self, loggingchannel: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), loggingchannel.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddLoggingChannelWithLevel<'a, Param0: ::windows::core::IntoParam<'a, ILoggingChannel>>(&self, loggingchannel: Param0, maxlevel: LoggingLevel) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), loggingchannel.into_param().abi(), maxlevel).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn RemoveLoggingChannel<'a, Param0: ::windows::core::IntoParam<'a, ILoggingChannel>>(&self, loggingchannel: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), loggingchannel.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage")]
    #[doc = "*Required features: `Foundation_Diagnostics`, `Storage`*"]
    pub fn CloseAndSaveToFileAsync(&self) -> ::windows::core::Result<super::IAsyncOperation<super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::IAsyncOperation<super::super::Storage::StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogFileGenerated<'a, Param0: ::windows::core::IntoParam<'a, super::TypedEventHandler<IFileLoggingSession, LogFileGeneratedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn RemoveLogFileGenerated<'a, Param0: ::windows::core::IntoParam<'a, super::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for IFileLoggingSession {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{24c74216-fed2-404c-895f-1f9699cb02f7}");
}
impl ::core::convert::From<IFileLoggingSession> for ::windows::core::IUnknown {
    fn from(value: IFileLoggingSession) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IFileLoggingSession> for ::windows::core::IUnknown {
    fn from(value: &IFileLoggingSession) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFileLoggingSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFileLoggingSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IFileLoggingSession> for ::windows::core::IInspectable {
    fn from(value: IFileLoggingSession) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IFileLoggingSession> for ::windows::core::IInspectable {
    fn from(value: &IFileLoggingSession) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IFileLoggingSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IFileLoggingSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
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
#[repr(C)]
#[doc(hidden)]
pub struct IFileLoggingSession_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, loggingchannel: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, loggingchannel: ::windows::core::RawPtr, maxlevel: LoggingLevel) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, loggingchannel: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::EventRegistrationToken) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::EventRegistrationToken) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFileLoggingSessionFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFileLoggingSessionFactory {
    type Vtable = IFileLoggingSessionFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeea08dce_8447_4daa_9133_12eb46f697d4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileLoggingSessionFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILogFileGeneratedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILogFileGeneratedEventArgs {
    type Vtable = ILogFileGeneratedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x269e976f_0d38_4c1a_b53f_b395d881df84);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILogFileGeneratedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILoggingActivity(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILoggingActivity {
    type Vtable = ILoggingActivity_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc032941_b766_4cb5_9848_97ac6ba6d60c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingActivity_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILoggingActivity2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILoggingActivity2 {
    type Vtable = ILoggingActivity2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x26c29808_6322_456a_af82_80c8642f178b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingActivity2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, stopeventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, stopeventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fields: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, stopeventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fields: ::windows::core::RawPtr, options: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILoggingActivityFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILoggingActivityFactory {
    type Vtable = ILoggingActivityFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b33b483_e10a_4c58_97d5_10fb451074fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingActivityFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, activityname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, loggingchannel: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, activityname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, loggingchannel: ::windows::core::RawPtr, level: LoggingLevel, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Foundation_Diagnostics`*"]
pub struct ILoggingChannel(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILoggingChannel {
    type Vtable = ILoggingChannel_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe9a50343_11d7_4f01_b5ca_cf495278c0a8);
}
impl ILoggingChannel {
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Enabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Level(&self) -> ::windows::core::Result<LoggingLevel> {
        let this = self;
        unsafe {
            let mut result__: LoggingLevel = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LoggingLevel>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogMessage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, eventstring: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), eventstring.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogMessageWithLevel<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, eventstring: Param0, level: LoggingLevel) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), eventstring.into_param().abi(), level).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogValuePair<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value1: Param0, value2: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value1.into_param().abi(), value2).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogValuePairWithLevel<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value1: Param0, value2: i32, level: LoggingLevel) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value1.into_param().abi(), value2, level).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LoggingEnabled<'a, Param0: ::windows::core::IntoParam<'a, super::TypedEventHandler<ILoggingChannel, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn RemoveLoggingEnabled<'a, Param0: ::windows::core::IntoParam<'a, super::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for ILoggingChannel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{e9a50343-11d7-4f01-b5ca-cf495278c0a8}");
}
impl ::core::convert::From<ILoggingChannel> for ::windows::core::IUnknown {
    fn from(value: ILoggingChannel) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ILoggingChannel> for ::windows::core::IUnknown {
    fn from(value: &ILoggingChannel) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ILoggingChannel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ILoggingChannel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ILoggingChannel> for ::windows::core::IInspectable {
    fn from(value: ILoggingChannel) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ILoggingChannel> for ::windows::core::IInspectable {
    fn from(value: &ILoggingChannel) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ILoggingChannel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ILoggingChannel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
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
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingChannel_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut LoggingLevel) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, eventstring: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, eventstring: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, level: LoggingLevel) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value1: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value2: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value1: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value2: i32, level: LoggingLevel) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::EventRegistrationToken) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::EventRegistrationToken) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILoggingChannel2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILoggingChannel2 {
    type Vtable = ILoggingChannel2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f4c3cf3_0bac_45a5_9e33_baf3f3a246a5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingChannel2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILoggingChannelFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILoggingChannelFactory {
    type Vtable = ILoggingChannelFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4edc5b9c_af80_4a9b_b0dc_398f9ae5207b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingChannelFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILoggingChannelFactory2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILoggingChannelFactory2 {
    type Vtable = ILoggingChannelFactory2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4c6ef5dd_3b27_4dc9_99f0_299c6e4603a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingChannelFactory2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: ::windows::core::RawPtr, id: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILoggingChannelOptions(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILoggingChannelOptions {
    type Vtable = ILoggingChannelOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc3e847ff_0ebb_4a53_8c54_dec24926cb2c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingChannelOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILoggingChannelOptionsFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILoggingChannelOptionsFactory {
    type Vtable = ILoggingChannelOptionsFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa93151da_7faf_4191_8755_5e86dc65d896);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingChannelOptionsFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, group: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILoggingFields(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILoggingFields {
    type Vtable = ILoggingFields_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7f6b7af_762d_4579_83bd_52c23bc333bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingFields_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, tags: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u8, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u8, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u8, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u8, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: i16, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: i16, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const i16, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const i16, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u16, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u16, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u16, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u16, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: i32, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: i32, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const i32, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const i32, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u32, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u32, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u32, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u32, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: i64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: i64, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: i64, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const i64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const i64, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const i64, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u64, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u64, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u64, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u64, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: f32, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: f32, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const f32, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const f32, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: f64, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: f64, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const f64, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const f64, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u16, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: u16, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u16, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const u16, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: bool, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: bool, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const bool, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const bool, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const ::core::mem::ManuallyDrop<::windows::core::HSTRING>, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const ::core::mem::ManuallyDrop<::windows::core::HSTRING>, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::windows::core::GUID, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::windows::core::GUID, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const ::windows::core::GUID, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const ::windows::core::GUID, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::DateTime) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::DateTime, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::DateTime, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::DateTime) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::DateTime, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::DateTime, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::TimeSpan) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::TimeSpan, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::TimeSpan, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::TimeSpan) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::TimeSpan, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::TimeSpan, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::Point) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::Point, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::Point, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::Point) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::Point, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::Point, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::Size) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::Size, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::Size, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::Size) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::Size, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::Size, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::Rect) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::Rect, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: super::Rect, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::Rect) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::Rect, format: LoggingFieldFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value_array_size: u32, value: *const super::Rect, format: LoggingFieldFormat, tags: i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILoggingOptions(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILoggingOptions {
    type Vtable = ILoggingOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x90bc7850_0192_4f5d_ac26_006adaca12d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: i64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut LoggingOpcode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: LoggingOpcode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILoggingOptionsFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILoggingOptionsFactory {
    type Vtable = ILoggingOptionsFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd713c6cb_98ab_464b_9f22_a3268478368a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingOptionsFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, keywords: i64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Foundation_Diagnostics`*"]
pub struct ILoggingSession(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILoggingSession {
    type Vtable = ILoggingSession_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6221f306_9380_4ad7_baf5_41ea9310d768);
}
impl ILoggingSession {
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Storage")]
    #[doc = "*Required features: `Foundation_Diagnostics`, `Storage`*"]
    pub fn SaveToFileAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::IStorageFolder>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, folder: Param0, filename: Param1) -> ::windows::core::Result<super::IAsyncOperation<super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), folder.into_param().abi(), filename.into_param().abi(), &mut result__).from_abi::<super::IAsyncOperation<super::super::Storage::StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddLoggingChannel<'a, Param0: ::windows::core::IntoParam<'a, ILoggingChannel>>(&self, loggingchannel: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), loggingchannel.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddLoggingChannelWithLevel<'a, Param0: ::windows::core::IntoParam<'a, ILoggingChannel>>(&self, loggingchannel: Param0, maxlevel: LoggingLevel) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), loggingchannel.into_param().abi(), maxlevel).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn RemoveLoggingChannel<'a, Param0: ::windows::core::IntoParam<'a, ILoggingChannel>>(&self, loggingchannel: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), loggingchannel.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for ILoggingSession {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{6221f306-9380-4ad7-baf5-41ea9310d768}");
}
impl ::core::convert::From<ILoggingSession> for ::windows::core::IUnknown {
    fn from(value: ILoggingSession) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ILoggingSession> for ::windows::core::IUnknown {
    fn from(value: &ILoggingSession) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ILoggingSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ILoggingSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ILoggingSession> for ::windows::core::IInspectable {
    fn from(value: ILoggingSession) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ILoggingSession> for ::windows::core::IInspectable {
    fn from(value: &ILoggingSession) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ILoggingSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ILoggingSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
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
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingSession_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, folder: ::windows::core::RawPtr, filename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, loggingchannel: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, loggingchannel: ::windows::core::RawPtr, maxlevel: LoggingLevel) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, loggingchannel: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILoggingSessionFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILoggingSessionFactory {
    type Vtable = ILoggingSessionFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e937ee5_58fd_45e0_8c2f_a132eff95c1e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingSessionFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Foundation_Diagnostics`*"]
pub struct ILoggingTarget(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILoggingTarget {
    type Vtable = ILoggingTarget_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x65f16c35_e388_4e26_b17a_f51cd3a83916);
}
impl ILoggingTarget {
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn IsEnabledWithLevel(&self, level: LoggingLevel) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), level, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn IsEnabledWithLevelAndKeywords(&self, level: LoggingLevel, keywords: i64) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), level, keywords, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogEvent<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, eventname: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), eventname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogEventWithFields<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, LoggingFields>>(&self, eventname: Param0, fields: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), eventname.into_param().abi(), fields.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogEventWithFieldsAndLevel<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, LoggingFields>>(&self, eventname: Param0, fields: Param1, level: LoggingLevel) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), eventname.into_param().abi(), fields.into_param().abi(), level).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogEventWithFieldsAndOptions<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, LoggingFields>, Param3: ::windows::core::IntoParam<'a, LoggingOptions>>(&self, eventname: Param0, fields: Param1, level: LoggingLevel, options: Param3) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), eventname.into_param().abi(), fields.into_param().abi(), level, options.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn StartActivity<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, starteventname: Param0) -> ::windows::core::Result<LoggingActivity> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), starteventname.into_param().abi(), &mut result__).from_abi::<LoggingActivity>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn StartActivityWithFields<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, LoggingFields>>(&self, starteventname: Param0, fields: Param1) -> ::windows::core::Result<LoggingActivity> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), starteventname.into_param().abi(), fields.into_param().abi(), &mut result__).from_abi::<LoggingActivity>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn StartActivityWithFieldsAndLevel<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, LoggingFields>>(&self, starteventname: Param0, fields: Param1, level: LoggingLevel) -> ::windows::core::Result<LoggingActivity> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), starteventname.into_param().abi(), fields.into_param().abi(), level, &mut result__).from_abi::<LoggingActivity>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn StartActivityWithFieldsAndOptions<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, LoggingFields>, Param3: ::windows::core::IntoParam<'a, LoggingOptions>>(&self, starteventname: Param0, fields: Param1, level: LoggingLevel, options: Param3) -> ::windows::core::Result<LoggingActivity> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), starteventname.into_param().abi(), fields.into_param().abi(), level, options.into_param().abi(), &mut result__).from_abi::<LoggingActivity>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ILoggingTarget {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{65f16c35-e388-4e26-b17a-f51cd3a83916}");
}
impl ::core::convert::From<ILoggingTarget> for ::windows::core::IUnknown {
    fn from(value: ILoggingTarget) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ILoggingTarget> for ::windows::core::IUnknown {
    fn from(value: &ILoggingTarget) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ILoggingTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ILoggingTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ILoggingTarget> for ::windows::core::IInspectable {
    fn from(value: ILoggingTarget) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ILoggingTarget> for ::windows::core::IInspectable {
    fn from(value: &ILoggingTarget) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ILoggingTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ILoggingTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingTarget_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, level: LoggingLevel, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, level: LoggingLevel, keywords: i64, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, eventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, eventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fields: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, eventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fields: ::windows::core::RawPtr, level: LoggingLevel) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, eventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fields: ::windows::core::RawPtr, level: LoggingLevel, options: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, starteventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, starteventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fields: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, starteventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fields: ::windows::core::RawPtr, level: LoggingLevel, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, starteventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fields: ::windows::core::RawPtr, level: LoggingLevel, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITracingStatusChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITracingStatusChangedEventArgs {
    type Vtable = ITracingStatusChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x410b7711_ff3b_477f_9c9a_d2efda302dc3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITracingStatusChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut CausalityTraceLevel) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Foundation_Diagnostics`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LogFileGeneratedEventArgs(pub ::windows::core::IInspectable);
impl LogFileGeneratedEventArgs {
    #[cfg(feature = "Storage")]
    #[doc = "*Required features: `Foundation_Diagnostics`, `Storage`*"]
    pub fn File(&self) -> ::windows::core::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::StorageFile>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for LogFileGeneratedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.LogFileGeneratedEventArgs;{269e976f-0d38-4c1a-b53f-b395d881df84})");
}
unsafe impl ::windows::core::Interface for LogFileGeneratedEventArgs {
    type Vtable = ILogFileGeneratedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x269e976f_0d38_4c1a_b53f_b395d881df84);
}
impl ::windows::core::RuntimeName for LogFileGeneratedEventArgs {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.LogFileGeneratedEventArgs";
}
impl ::core::convert::From<LogFileGeneratedEventArgs> for ::windows::core::IUnknown {
    fn from(value: LogFileGeneratedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LogFileGeneratedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &LogFileGeneratedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LogFileGeneratedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LogFileGeneratedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LogFileGeneratedEventArgs> for ::windows::core::IInspectable {
    fn from(value: LogFileGeneratedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LogFileGeneratedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &LogFileGeneratedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LogFileGeneratedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LogFileGeneratedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for LogFileGeneratedEventArgs {}
unsafe impl ::core::marker::Sync for LogFileGeneratedEventArgs {}
#[doc = "*Required features: `Foundation_Diagnostics`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LoggingActivity(pub ::windows::core::IInspectable);
impl LoggingActivity {
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Channel(&self) -> ::windows::core::Result<LoggingChannel> {
        let this = &::windows::core::Interface::cast::<ILoggingActivity2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LoggingChannel>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn StopActivity<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, stopeventname: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ILoggingActivity2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), stopeventname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn StopActivityWithFields<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, LoggingFields>>(&self, stopeventname: Param0, fields: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ILoggingActivity2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), stopeventname.into_param().abi(), fields.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn StopActivityWithFieldsAndOptions<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, LoggingFields>, Param2: ::windows::core::IntoParam<'a, LoggingOptions>>(&self, stopeventname: Param0, fields: Param1, options: Param2) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ILoggingActivity2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), stopeventname.into_param().abi(), fields.into_param().abi(), options.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn IsEnabledWithLevel(&self, level: LoggingLevel) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), level, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn IsEnabledWithLevelAndKeywords(&self, level: LoggingLevel, keywords: i64) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), level, keywords, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogEvent<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, eventname: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), eventname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogEventWithFields<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, LoggingFields>>(&self, eventname: Param0, fields: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), eventname.into_param().abi(), fields.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogEventWithFieldsAndLevel<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, LoggingFields>>(&self, eventname: Param0, fields: Param1, level: LoggingLevel) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), eventname.into_param().abi(), fields.into_param().abi(), level).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogEventWithFieldsAndOptions<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, LoggingFields>, Param3: ::windows::core::IntoParam<'a, LoggingOptions>>(&self, eventname: Param0, fields: Param1, level: LoggingLevel, options: Param3) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), eventname.into_param().abi(), fields.into_param().abi(), level, options.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn StartActivity<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, starteventname: Param0) -> ::windows::core::Result<LoggingActivity> {
        let this = &::windows::core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), starteventname.into_param().abi(), &mut result__).from_abi::<LoggingActivity>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn StartActivityWithFields<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, LoggingFields>>(&self, starteventname: Param0, fields: Param1) -> ::windows::core::Result<LoggingActivity> {
        let this = &::windows::core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), starteventname.into_param().abi(), fields.into_param().abi(), &mut result__).from_abi::<LoggingActivity>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn StartActivityWithFieldsAndLevel<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, LoggingFields>>(&self, starteventname: Param0, fields: Param1, level: LoggingLevel) -> ::windows::core::Result<LoggingActivity> {
        let this = &::windows::core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), starteventname.into_param().abi(), fields.into_param().abi(), level, &mut result__).from_abi::<LoggingActivity>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn StartActivityWithFieldsAndOptions<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, LoggingFields>, Param3: ::windows::core::IntoParam<'a, LoggingOptions>>(&self, starteventname: Param0, fields: Param1, level: LoggingLevel, options: Param3) -> ::windows::core::Result<LoggingActivity> {
        let this = &::windows::core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), starteventname.into_param().abi(), fields.into_param().abi(), level, options.into_param().abi(), &mut result__).from_abi::<LoggingActivity>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn CreateLoggingActivity<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ILoggingChannel>>(activityname: Param0, loggingchannel: Param1) -> ::windows::core::Result<LoggingActivity> {
        Self::ILoggingActivityFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), activityname.into_param().abi(), loggingchannel.into_param().abi(), &mut result__).from_abi::<LoggingActivity>(result__)
        })
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn CreateLoggingActivityWithLevel<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ILoggingChannel>>(activityname: Param0, loggingchannel: Param1, level: LoggingLevel) -> ::windows::core::Result<LoggingActivity> {
        Self::ILoggingActivityFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), activityname.into_param().abi(), loggingchannel.into_param().abi(), level, &mut result__).from_abi::<LoggingActivity>(result__)
        })
    }
    pub fn ILoggingActivityFactory<R, F: FnOnce(&ILoggingActivityFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LoggingActivity, ILoggingActivityFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for LoggingActivity {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.LoggingActivity;{bc032941-b766-4cb5-9848-97ac6ba6d60c})");
}
unsafe impl ::windows::core::Interface for LoggingActivity {
    type Vtable = ILoggingActivity_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc032941_b766_4cb5_9848_97ac6ba6d60c);
}
impl ::windows::core::RuntimeName for LoggingActivity {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.LoggingActivity";
}
impl ::core::convert::From<LoggingActivity> for ::windows::core::IUnknown {
    fn from(value: LoggingActivity) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LoggingActivity> for ::windows::core::IUnknown {
    fn from(value: &LoggingActivity) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LoggingActivity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LoggingActivity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LoggingActivity> for ::windows::core::IInspectable {
    fn from(value: LoggingActivity) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LoggingActivity> for ::windows::core::IInspectable {
    fn from(value: &LoggingActivity) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LoggingActivity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LoggingActivity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
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
#[doc = "*Required features: `Foundation_Diagnostics`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LoggingChannel(pub ::windows::core::IInspectable);
impl LoggingChannel {
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Enabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Level(&self) -> ::windows::core::Result<LoggingLevel> {
        let this = self;
        unsafe {
            let mut result__: LoggingLevel = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LoggingLevel>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogMessage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, eventstring: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), eventstring.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogMessageWithLevel<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, eventstring: Param0, level: LoggingLevel) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), eventstring.into_param().abi(), level).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogValuePair<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value1: Param0, value2: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value1.into_param().abi(), value2).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogValuePairWithLevel<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value1: Param0, value2: i32, level: LoggingLevel) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value1.into_param().abi(), value2, level).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LoggingEnabled<'a, Param0: ::windows::core::IntoParam<'a, super::TypedEventHandler<ILoggingChannel, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn RemoveLoggingEnabled<'a, Param0: ::windows::core::IntoParam<'a, super::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<ILoggingChannel2>(self)?;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn IsEnabledWithLevel(&self, level: LoggingLevel) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), level, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn IsEnabledWithLevelAndKeywords(&self, level: LoggingLevel, keywords: i64) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), level, keywords, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogEvent<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, eventname: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), eventname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogEventWithFields<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, LoggingFields>>(&self, eventname: Param0, fields: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), eventname.into_param().abi(), fields.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogEventWithFieldsAndLevel<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, LoggingFields>>(&self, eventname: Param0, fields: Param1, level: LoggingLevel) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), eventname.into_param().abi(), fields.into_param().abi(), level).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogEventWithFieldsAndOptions<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, LoggingFields>, Param3: ::windows::core::IntoParam<'a, LoggingOptions>>(&self, eventname: Param0, fields: Param1, level: LoggingLevel, options: Param3) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), eventname.into_param().abi(), fields.into_param().abi(), level, options.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn StartActivity<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, starteventname: Param0) -> ::windows::core::Result<LoggingActivity> {
        let this = &::windows::core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), starteventname.into_param().abi(), &mut result__).from_abi::<LoggingActivity>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn StartActivityWithFields<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, LoggingFields>>(&self, starteventname: Param0, fields: Param1) -> ::windows::core::Result<LoggingActivity> {
        let this = &::windows::core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), starteventname.into_param().abi(), fields.into_param().abi(), &mut result__).from_abi::<LoggingActivity>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn StartActivityWithFieldsAndLevel<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, LoggingFields>>(&self, starteventname: Param0, fields: Param1, level: LoggingLevel) -> ::windows::core::Result<LoggingActivity> {
        let this = &::windows::core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), starteventname.into_param().abi(), fields.into_param().abi(), level, &mut result__).from_abi::<LoggingActivity>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn StartActivityWithFieldsAndOptions<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, LoggingFields>, Param3: ::windows::core::IntoParam<'a, LoggingOptions>>(&self, starteventname: Param0, fields: Param1, level: LoggingLevel, options: Param3) -> ::windows::core::Result<LoggingActivity> {
        let this = &::windows::core::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), starteventname.into_param().abi(), fields.into_param().abi(), level, options.into_param().abi(), &mut result__).from_abi::<LoggingActivity>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(name: Param0) -> ::windows::core::Result<LoggingChannel> {
        Self::ILoggingChannelFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<LoggingChannel>(result__)
        })
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn CreateWithOptions<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, LoggingChannelOptions>>(name: Param0, options: Param1) -> ::windows::core::Result<LoggingChannel> {
        Self::ILoggingChannelFactory2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), name.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<LoggingChannel>(result__)
        })
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn CreateWithOptionsAndId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, LoggingChannelOptions>, Param2: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(name: Param0, options: Param1, id: Param2) -> ::windows::core::Result<LoggingChannel> {
        Self::ILoggingChannelFactory2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), name.into_param().abi(), options.into_param().abi(), id.into_param().abi(), &mut result__).from_abi::<LoggingChannel>(result__)
        })
    }
    pub fn ILoggingChannelFactory<R, F: FnOnce(&ILoggingChannelFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LoggingChannel, ILoggingChannelFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ILoggingChannelFactory2<R, F: FnOnce(&ILoggingChannelFactory2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LoggingChannel, ILoggingChannelFactory2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for LoggingChannel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.LoggingChannel;{e9a50343-11d7-4f01-b5ca-cf495278c0a8})");
}
unsafe impl ::windows::core::Interface for LoggingChannel {
    type Vtable = ILoggingChannel_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe9a50343_11d7_4f01_b5ca_cf495278c0a8);
}
impl ::windows::core::RuntimeName for LoggingChannel {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.LoggingChannel";
}
impl ::core::convert::From<LoggingChannel> for ::windows::core::IUnknown {
    fn from(value: LoggingChannel) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LoggingChannel> for ::windows::core::IUnknown {
    fn from(value: &LoggingChannel) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LoggingChannel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LoggingChannel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LoggingChannel> for ::windows::core::IInspectable {
    fn from(value: LoggingChannel) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LoggingChannel> for ::windows::core::IInspectable {
    fn from(value: &LoggingChannel) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LoggingChannel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LoggingChannel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<LoggingChannel> for ILoggingChannel {
    fn from(value: LoggingChannel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LoggingChannel> for ILoggingChannel {
    fn from(value: &LoggingChannel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILoggingChannel> for LoggingChannel {
    fn into_param(self) -> ::windows::core::Param<'a, ILoggingChannel> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILoggingChannel> for &LoggingChannel {
    fn into_param(self) -> ::windows::core::Param<'a, ILoggingChannel> {
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
#[doc = "*Required features: `Foundation_Diagnostics`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LoggingChannelOptions(pub ::windows::core::IInspectable);
impl LoggingChannelOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LoggingChannelOptions, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Group(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn SetGroup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(group: Param0) -> ::windows::core::Result<LoggingChannelOptions> {
        Self::ILoggingChannelOptionsFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), group.into_param().abi(), &mut result__).from_abi::<LoggingChannelOptions>(result__)
        })
    }
    pub fn ILoggingChannelOptionsFactory<R, F: FnOnce(&ILoggingChannelOptionsFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LoggingChannelOptions, ILoggingChannelOptionsFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for LoggingChannelOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.LoggingChannelOptions;{c3e847ff-0ebb-4a53-8c54-dec24926cb2c})");
}
unsafe impl ::windows::core::Interface for LoggingChannelOptions {
    type Vtable = ILoggingChannelOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc3e847ff_0ebb_4a53_8c54_dec24926cb2c);
}
impl ::windows::core::RuntimeName for LoggingChannelOptions {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.LoggingChannelOptions";
}
impl ::core::convert::From<LoggingChannelOptions> for ::windows::core::IUnknown {
    fn from(value: LoggingChannelOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LoggingChannelOptions> for ::windows::core::IUnknown {
    fn from(value: &LoggingChannelOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LoggingChannelOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LoggingChannelOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LoggingChannelOptions> for ::windows::core::IInspectable {
    fn from(value: LoggingChannelOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LoggingChannelOptions> for ::windows::core::IInspectable {
    fn from(value: &LoggingChannelOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LoggingChannelOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LoggingChannelOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for LoggingChannelOptions {}
unsafe impl ::core::marker::Sync for LoggingChannelOptions {}
#[doc = "*Required features: `Foundation_Diagnostics`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct LoggingFieldFormat(pub i32);
impl LoggingFieldFormat {
    pub const Default: LoggingFieldFormat = LoggingFieldFormat(0i32);
    pub const Hidden: LoggingFieldFormat = LoggingFieldFormat(1i32);
    pub const String: LoggingFieldFormat = LoggingFieldFormat(2i32);
    pub const Boolean: LoggingFieldFormat = LoggingFieldFormat(3i32);
    pub const Hexadecimal: LoggingFieldFormat = LoggingFieldFormat(4i32);
    pub const ProcessId: LoggingFieldFormat = LoggingFieldFormat(5i32);
    pub const ThreadId: LoggingFieldFormat = LoggingFieldFormat(6i32);
    pub const Port: LoggingFieldFormat = LoggingFieldFormat(7i32);
    pub const Ipv4Address: LoggingFieldFormat = LoggingFieldFormat(8i32);
    pub const Ipv6Address: LoggingFieldFormat = LoggingFieldFormat(9i32);
    pub const SocketAddress: LoggingFieldFormat = LoggingFieldFormat(10i32);
    pub const Xml: LoggingFieldFormat = LoggingFieldFormat(11i32);
    pub const Json: LoggingFieldFormat = LoggingFieldFormat(12i32);
    pub const Win32Error: LoggingFieldFormat = LoggingFieldFormat(13i32);
    pub const NTStatus: LoggingFieldFormat = LoggingFieldFormat(14i32);
    pub const HResult: LoggingFieldFormat = LoggingFieldFormat(15i32);
    pub const FileTime: LoggingFieldFormat = LoggingFieldFormat(16i32);
    pub const Signed: LoggingFieldFormat = LoggingFieldFormat(17i32);
    pub const Unsigned: LoggingFieldFormat = LoggingFieldFormat(18i32);
}
impl ::core::convert::From<i32> for LoggingFieldFormat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for LoggingFieldFormat {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for LoggingFieldFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Foundation.Diagnostics.LoggingFieldFormat;i4)");
}
impl ::windows::core::DefaultType for LoggingFieldFormat {
    type DefaultType = Self;
}
#[doc = "*Required features: `Foundation_Diagnostics`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LoggingFields(pub ::windows::core::IInspectable);
impl LoggingFields {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LoggingFields, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn BeginStruct<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), name.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn BeginStructWithTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), name.into_param().abi(), tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn EndStruct(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddEmpty<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), name.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddEmptyWithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), name.into_param().abi(), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddEmptyWithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), name.into_param().abi(), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt8<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), name.into_param().abi(), value).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt8WithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: u8, format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt8WithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: u8, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt8Array<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt8ArrayWithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[<u8 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt8ArrayWithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[<u8 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddInt16<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: i16) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), name.into_param().abi(), value).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddInt16WithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: i16, format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddInt16WithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: i16, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddInt16Array<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[<i16 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddInt16ArrayWithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[<i16 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddInt16ArrayWithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[<i16 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt16<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: u16) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), name.into_param().abi(), value).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt16WithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: u16, format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt16WithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: u16, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt16Array<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[<u16 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt16ArrayWithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[<u16 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt16ArrayWithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[<u16 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddInt32<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this), name.into_param().abi(), value).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddInt32WithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: i32, format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddInt32WithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: i32, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).33)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddInt32Array<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[<i32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).34)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddInt32ArrayWithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[<i32 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).35)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddInt32ArrayWithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[<i32 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).36)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt32<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).37)(::core::mem::transmute_copy(this), name.into_param().abi(), value).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt32WithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: u32, format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).38)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt32WithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: u32, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).39)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt32Array<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[<u32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).40)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt32ArrayWithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[<u32 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).41)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt32ArrayWithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[<u32 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).42)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddInt64<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: i64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).43)(::core::mem::transmute_copy(this), name.into_param().abi(), value).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddInt64WithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: i64, format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).44)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddInt64WithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: i64, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).45)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddInt64Array<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[<i64 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).46)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddInt64ArrayWithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[<i64 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).47)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddInt64ArrayWithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[<i64 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).48)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt64<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: u64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).49)(::core::mem::transmute_copy(this), name.into_param().abi(), value).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt64WithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: u64, format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).50)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt64WithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: u64, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).51)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt64Array<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[<u64 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).52)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt64ArrayWithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[<u64 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).53)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt64ArrayWithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[<u64 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).54)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddSingle<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).55)(::core::mem::transmute_copy(this), name.into_param().abi(), value).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddSingleWithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: f32, format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).56)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddSingleWithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: f32, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).57)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddSingleArray<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[<f32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).58)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddSingleArrayWithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[<f32 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).59)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddSingleArrayWithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[<f32 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).60)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddDouble<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).61)(::core::mem::transmute_copy(this), name.into_param().abi(), value).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddDoubleWithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: f64, format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).62)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddDoubleWithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: f64, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).63)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddDoubleArray<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[<f64 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).64)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddDoubleArrayWithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[<f64 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).65)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddDoubleArrayWithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[<f64 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).66)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddChar16<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: u16) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).67)(::core::mem::transmute_copy(this), name.into_param().abi(), value).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddChar16WithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: u16, format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).68)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddChar16WithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: u16, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).69)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddChar16Array<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[<u16 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).70)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddChar16ArrayWithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[<u16 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).71)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddChar16ArrayWithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[<u16 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).72)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddBoolean<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).73)(::core::mem::transmute_copy(this), name.into_param().abi(), value).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddBooleanWithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: bool, format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).74)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddBooleanWithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: bool, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).75)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddBooleanArray<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[<bool as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).76)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddBooleanArrayWithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[<bool as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).77)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddBooleanArrayWithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[<bool as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).78)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddString<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).79)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddStringWithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: Param1, format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).80)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddStringWithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: Param1, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).81)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddStringArray<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[<::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).82)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddStringArrayWithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[<::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).83)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddStringArrayWithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[<::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).84)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddGuid<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, name: Param0, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).85)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddGuidWithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, name: Param0, value: Param1, format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).86)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddGuidWithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, name: Param0, value: Param1, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).87)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddGuidArray<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[<::windows::core::GUID as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).88)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddGuidArrayWithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[<::windows::core::GUID as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).89)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddGuidArrayWithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[<::windows::core::GUID as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).90)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddDateTime<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::DateTime>>(&self, name: Param0, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).91)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddDateTimeWithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::DateTime>>(&self, name: Param0, value: Param1, format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).92)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddDateTimeWithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::DateTime>>(&self, name: Param0, value: Param1, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).93)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddDateTimeArray<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[<super::DateTime as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).94)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddDateTimeArrayWithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[<super::DateTime as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).95)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddDateTimeArrayWithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[<super::DateTime as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).96)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddTimeSpan<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::TimeSpan>>(&self, name: Param0, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).97)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddTimeSpanWithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::TimeSpan>>(&self, name: Param0, value: Param1, format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).98)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddTimeSpanWithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::TimeSpan>>(&self, name: Param0, value: Param1, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).99)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddTimeSpanArray<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[<super::TimeSpan as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).100)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddTimeSpanArrayWithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[<super::TimeSpan as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).101)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddTimeSpanArrayWithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[<super::TimeSpan as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).102)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddPoint<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::Point>>(&self, name: Param0, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).103)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddPointWithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::Point>>(&self, name: Param0, value: Param1, format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).104)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddPointWithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::Point>>(&self, name: Param0, value: Param1, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).105)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddPointArray<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[<super::Point as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).106)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddPointArrayWithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[<super::Point as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).107)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddPointArrayWithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[<super::Point as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).108)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddSize<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::Size>>(&self, name: Param0, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).109)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddSizeWithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::Size>>(&self, name: Param0, value: Param1, format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).110)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddSizeWithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::Size>>(&self, name: Param0, value: Param1, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).111)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddSizeArray<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[<super::Size as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).112)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddSizeArrayWithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[<super::Size as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).113)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddSizeArrayWithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[<super::Size as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).114)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddRect<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::Rect>>(&self, name: Param0, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).115)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddRectWithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::Rect>>(&self, name: Param0, value: Param1, format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).116)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddRectWithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::Rect>>(&self, name: Param0, value: Param1, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).117)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddRectArray<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[<super::Rect as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).118)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddRectArrayWithFormat<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[<super::Rect as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).119)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddRectArrayWithFormatAndTags<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: &[<super::Rect as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).120)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for LoggingFields {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.LoggingFields;{d7f6b7af-762d-4579-83bd-52c23bc333bc})");
}
unsafe impl ::windows::core::Interface for LoggingFields {
    type Vtable = ILoggingFields_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7f6b7af_762d_4579_83bd_52c23bc333bc);
}
impl ::windows::core::RuntimeName for LoggingFields {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.LoggingFields";
}
impl ::core::convert::From<LoggingFields> for ::windows::core::IUnknown {
    fn from(value: LoggingFields) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LoggingFields> for ::windows::core::IUnknown {
    fn from(value: &LoggingFields) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LoggingFields {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LoggingFields {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LoggingFields> for ::windows::core::IInspectable {
    fn from(value: LoggingFields) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LoggingFields> for ::windows::core::IInspectable {
    fn from(value: &LoggingFields) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LoggingFields {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LoggingFields {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for LoggingFields {}
unsafe impl ::core::marker::Sync for LoggingFields {}
#[doc = "*Required features: `Foundation_Diagnostics`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct LoggingLevel(pub i32);
impl LoggingLevel {
    pub const Verbose: LoggingLevel = LoggingLevel(0i32);
    pub const Information: LoggingLevel = LoggingLevel(1i32);
    pub const Warning: LoggingLevel = LoggingLevel(2i32);
    pub const Error: LoggingLevel = LoggingLevel(3i32);
    pub const Critical: LoggingLevel = LoggingLevel(4i32);
}
impl ::core::convert::From<i32> for LoggingLevel {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for LoggingLevel {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for LoggingLevel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Foundation.Diagnostics.LoggingLevel;i4)");
}
impl ::windows::core::DefaultType for LoggingLevel {
    type DefaultType = Self;
}
#[doc = "*Required features: `Foundation_Diagnostics`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct LoggingOpcode(pub i32);
impl LoggingOpcode {
    pub const Info: LoggingOpcode = LoggingOpcode(0i32);
    pub const Start: LoggingOpcode = LoggingOpcode(1i32);
    pub const Stop: LoggingOpcode = LoggingOpcode(2i32);
    pub const Reply: LoggingOpcode = LoggingOpcode(6i32);
    pub const Resume: LoggingOpcode = LoggingOpcode(7i32);
    pub const Suspend: LoggingOpcode = LoggingOpcode(8i32);
    pub const Send: LoggingOpcode = LoggingOpcode(9i32);
}
impl ::core::convert::From<i32> for LoggingOpcode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for LoggingOpcode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for LoggingOpcode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Foundation.Diagnostics.LoggingOpcode;i4)");
}
impl ::windows::core::DefaultType for LoggingOpcode {
    type DefaultType = Self;
}
#[doc = "*Required features: `Foundation_Diagnostics`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LoggingOptions(pub ::windows::core::IInspectable);
impl LoggingOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LoggingOptions, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Keywords(&self) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn SetKeywords(&self, value: i64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Tags(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn SetTags(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Task(&self) -> ::windows::core::Result<i16> {
        let this = self;
        unsafe {
            let mut result__: i16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i16>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn SetTask(&self, value: i16) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Opcode(&self) -> ::windows::core::Result<LoggingOpcode> {
        let this = self;
        unsafe {
            let mut result__: LoggingOpcode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LoggingOpcode>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn SetOpcode(&self, value: LoggingOpcode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn ActivityId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn SetActivityId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn RelatedActivityId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn SetRelatedActivityId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn CreateWithKeywords(keywords: i64) -> ::windows::core::Result<LoggingOptions> {
        Self::ILoggingOptionsFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), keywords, &mut result__).from_abi::<LoggingOptions>(result__)
        })
    }
    pub fn ILoggingOptionsFactory<R, F: FnOnce(&ILoggingOptionsFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LoggingOptions, ILoggingOptionsFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for LoggingOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.LoggingOptions;{90bc7850-0192-4f5d-ac26-006adaca12d8})");
}
unsafe impl ::windows::core::Interface for LoggingOptions {
    type Vtable = ILoggingOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x90bc7850_0192_4f5d_ac26_006adaca12d8);
}
impl ::windows::core::RuntimeName for LoggingOptions {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.LoggingOptions";
}
impl ::core::convert::From<LoggingOptions> for ::windows::core::IUnknown {
    fn from(value: LoggingOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LoggingOptions> for ::windows::core::IUnknown {
    fn from(value: &LoggingOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LoggingOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LoggingOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LoggingOptions> for ::windows::core::IInspectable {
    fn from(value: LoggingOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LoggingOptions> for ::windows::core::IInspectable {
    fn from(value: &LoggingOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LoggingOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LoggingOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for LoggingOptions {}
unsafe impl ::core::marker::Sync for LoggingOptions {}
#[doc = "*Required features: `Foundation_Diagnostics`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LoggingSession(pub ::windows::core::IInspectable);
impl LoggingSession {
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Storage")]
    #[doc = "*Required features: `Foundation_Diagnostics`, `Storage`*"]
    pub fn SaveToFileAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::IStorageFolder>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, folder: Param0, filename: Param1) -> ::windows::core::Result<super::IAsyncOperation<super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), folder.into_param().abi(), filename.into_param().abi(), &mut result__).from_abi::<super::IAsyncOperation<super::super::Storage::StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddLoggingChannel<'a, Param0: ::windows::core::IntoParam<'a, ILoggingChannel>>(&self, loggingchannel: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), loggingchannel.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddLoggingChannelWithLevel<'a, Param0: ::windows::core::IntoParam<'a, ILoggingChannel>>(&self, loggingchannel: Param0, maxlevel: LoggingLevel) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), loggingchannel.into_param().abi(), maxlevel).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn RemoveLoggingChannel<'a, Param0: ::windows::core::IntoParam<'a, ILoggingChannel>>(&self, loggingchannel: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), loggingchannel.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(name: Param0) -> ::windows::core::Result<LoggingSession> {
        Self::ILoggingSessionFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<LoggingSession>(result__)
        })
    }
    pub fn ILoggingSessionFactory<R, F: FnOnce(&ILoggingSessionFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LoggingSession, ILoggingSessionFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for LoggingSession {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.LoggingSession;{6221f306-9380-4ad7-baf5-41ea9310d768})");
}
unsafe impl ::windows::core::Interface for LoggingSession {
    type Vtable = ILoggingSession_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6221f306_9380_4ad7_baf5_41ea9310d768);
}
impl ::windows::core::RuntimeName for LoggingSession {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.LoggingSession";
}
impl ::core::convert::From<LoggingSession> for ::windows::core::IUnknown {
    fn from(value: LoggingSession) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LoggingSession> for ::windows::core::IUnknown {
    fn from(value: &LoggingSession) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LoggingSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LoggingSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LoggingSession> for ::windows::core::IInspectable {
    fn from(value: LoggingSession) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LoggingSession> for ::windows::core::IInspectable {
    fn from(value: &LoggingSession) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LoggingSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LoggingSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<LoggingSession> for ILoggingSession {
    fn from(value: LoggingSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LoggingSession> for ILoggingSession {
    fn from(value: &LoggingSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILoggingSession> for LoggingSession {
    fn into_param(self) -> ::windows::core::Param<'a, ILoggingSession> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILoggingSession> for &LoggingSession {
    fn into_param(self) -> ::windows::core::Param<'a, ILoggingSession> {
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
unsafe impl ::core::marker::Send for LoggingSession {}
unsafe impl ::core::marker::Sync for LoggingSession {}
#[doc = "*Required features: `Foundation_Diagnostics`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RuntimeBrokerErrorSettings(pub ::windows::core::IInspectable);
impl RuntimeBrokerErrorSettings {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RuntimeBrokerErrorSettings, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn SetErrorOptions(&self, value: ErrorOptions) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn GetErrorOptions(&self) -> ::windows::core::Result<ErrorOptions> {
        let this = self;
        unsafe {
            let mut result__: ErrorOptions = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ErrorOptions>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for RuntimeBrokerErrorSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.RuntimeBrokerErrorSettings;{16369792-b03e-4ba1-8bb8-d28f4ab4d2c0})");
}
unsafe impl ::windows::core::Interface for RuntimeBrokerErrorSettings {
    type Vtable = IErrorReportingSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x16369792_b03e_4ba1_8bb8_d28f4ab4d2c0);
}
impl ::windows::core::RuntimeName for RuntimeBrokerErrorSettings {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.RuntimeBrokerErrorSettings";
}
impl ::core::convert::From<RuntimeBrokerErrorSettings> for ::windows::core::IUnknown {
    fn from(value: RuntimeBrokerErrorSettings) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RuntimeBrokerErrorSettings> for ::windows::core::IUnknown {
    fn from(value: &RuntimeBrokerErrorSettings) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RuntimeBrokerErrorSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RuntimeBrokerErrorSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RuntimeBrokerErrorSettings> for ::windows::core::IInspectable {
    fn from(value: RuntimeBrokerErrorSettings) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RuntimeBrokerErrorSettings> for ::windows::core::IInspectable {
    fn from(value: &RuntimeBrokerErrorSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RuntimeBrokerErrorSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RuntimeBrokerErrorSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<RuntimeBrokerErrorSettings> for IErrorReportingSettings {
    fn from(value: RuntimeBrokerErrorSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RuntimeBrokerErrorSettings> for IErrorReportingSettings {
    fn from(value: &RuntimeBrokerErrorSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IErrorReportingSettings> for RuntimeBrokerErrorSettings {
    fn into_param(self) -> ::windows::core::Param<'a, IErrorReportingSettings> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IErrorReportingSettings> for &RuntimeBrokerErrorSettings {
    fn into_param(self) -> ::windows::core::Param<'a, IErrorReportingSettings> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RuntimeBrokerErrorSettings {}
unsafe impl ::core::marker::Sync for RuntimeBrokerErrorSettings {}
#[doc = "*Required features: `Foundation_Diagnostics`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TracingStatusChangedEventArgs(pub ::windows::core::IInspectable);
impl TracingStatusChangedEventArgs {
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Enabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn TraceLevel(&self) -> ::windows::core::Result<CausalityTraceLevel> {
        let this = self;
        unsafe {
            let mut result__: CausalityTraceLevel = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CausalityTraceLevel>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for TracingStatusChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.TracingStatusChangedEventArgs;{410b7711-ff3b-477f-9c9a-d2efda302dc3})");
}
unsafe impl ::windows::core::Interface for TracingStatusChangedEventArgs {
    type Vtable = ITracingStatusChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x410b7711_ff3b_477f_9c9a_d2efda302dc3);
}
impl ::windows::core::RuntimeName for TracingStatusChangedEventArgs {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.TracingStatusChangedEventArgs";
}
impl ::core::convert::From<TracingStatusChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: TracingStatusChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TracingStatusChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &TracingStatusChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TracingStatusChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TracingStatusChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TracingStatusChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: TracingStatusChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TracingStatusChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &TracingStatusChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TracingStatusChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TracingStatusChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for TracingStatusChangedEventArgs {}
unsafe impl ::core::marker::Sync for TracingStatusChangedEventArgs {}
